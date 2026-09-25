// Reference implementation — NOT part of the JanusGateway SPM target.
//
// Shows how a host app supplies its own Socket.IO transport to JanusGateway instead of
// a jarust-native transport. Drop an adaptation of this into the iOS app (e.g. under
// LiveSessionEngine, alongside DataRegionSocket) and wire it with:
//
//     let connection = try await janusConnectWithTransport(
//         config: Config(url: url, capacity: 32),
//         transport: SocketIOJanusTransport(accessToken: token, sessionID: id)
//     )
//
// jarust owns the entire Janus protocol (transactions, demux, routing, response
// polling); this transport only moves bytes. Idioms mirror
// ios/Modules/LiveSessionEngine/Sources/DataRegionSocket.

import Foundation
import JanusGateway
@preconcurrency import SocketIO

/// The Socket.IO event Janus messages are carried on, matching the gateway.
private let janusEvent = "janus"

public final class SocketIOJanusTransport: JanusTransport, @unchecked Sendable {
    private let accessToken: String
    private let sessionID: String
    private let secure: Bool

    private var manager: SocketManager?
    private var client: SocketIOClient?
    private var sink: TransportInbound?

    public init(accessToken: String, sessionID: String, secure: Bool = true) {
        self.accessToken = accessToken
        self.sessionID = sessionID
        self.secure = secure
    }

    // MARK: - JanusTransport

    /// Called by jarust. Establishes the connection, retains `sink`, and forwards every
    /// inbound `janus` payload into it. Awaits the Socket.IO handshake before returning
    /// (mirrors DataRegionSocket.connectHelper).
    public func connect(url: String, sink: TransportInbound) async throws {
        self.sink = sink

        guard let parsed = URL(string: url) else {
            throw TransportError.ConnectionFailure(reason: "Invalid URL: \(url)")
        }

        let configuration: SocketIOClientConfiguration = [
            .forceWebsockets(true),
            .secure(secure),
            .reconnects(true),
            .connectParams([
                "token": accessToken,
                "mediaSessionId": sessionID.lowercased()
            ])
        ]

        let manager = SocketManager(socketURL: parsed, config: configuration)
        manager.handleQueue = DispatchQueue(
            label: "com.proximie.janus.socketIOTransport",
            qos: .userInitiated
        )
        let client = manager.defaultSocket
        self.manager = manager
        self.client = client

        try await withCheckedThrowingContinuation {
            (continuation: CheckedContinuation<Void, Error>) in
            client.on(clientEvent: .connect) { _, _ in
                // Swap the one-shot connect handler for the steady-state janus handler.
                client.off(clientEvent: .connect)
                self.installJanusHandler(on: client)
                continuation.resume()
            }

            client.on(clientEvent: .error) { _, _ in
                client.removeAllHandlers()
                continuation.resume(
                    throwing: TransportError.ConnectionFailure(
                        reason: "Socket.IO error during connect"
                    )
                )
            }

            client.connect()
        }
    }

    /// Fire-and-forget send. jarust already serialized a full `{"janus": ...}` request;
    /// forward it as the payload of the `janus` event.
    public func send(data: Data) {
        guard let manager, let client else { return }
        // Decode on the handle queue: `data` is Sendable, the decoded dictionary is not.
        manager.handleQueue.async {
            guard let json = Self.jsonObject(from: data) else { return }
            client.emit(Self.janusEvent, json)
        }
    }

    public func disconnect() {
        client?.removeAllHandlers()
        client?.disconnect()
        client = nil
        manager = nil
        sink = nil
    }

    // MARK: - Inbound

    private func installJanusHandler(on client: SocketIOClient) {
        client.on(Self.janusEvent) { [weak self] data, _ in
            guard let self, let sink = self.sink else { return }
            // jarust's demuxer expects exactly one `{"janus": ...}` object per receive,
            // so flatten however the gateway delivered them into individual frames.
            for payload in Self.janusPayloads(from: data) {
                sink.receive(data: payload)
            }
        }
    }

    // MARK: - Payload helpers

    /// jarust hands us a serialized `{"janus": ...}` request; Socket.IO's `emit` wants a
    /// JSON-compatible object, so decode back to a dictionary (which conforms to
    /// `SocketData`).
    private static func jsonObject(from data: Data) -> [String: Any]? {
        try? JSONSerialization.jsonObject(with: data, options: []) as? [String: Any]
    }

    /// Extracts the bytes of every `{"janus": ...}` message from a Socket.IO `on`
    /// callback's `data` array.
    ///
    /// jarust expects one `{"janus": ...}` object per receive, so we flatten every shape
    /// the gateway may use (matching the Rust client this replaces):
    ///  - `data = [ {janus:...} ]`            — the only/first arg
    ///  - `data = [ meta, {janus:...} ]`      — one arg among several
    ///  - `data = [ [ {janus:...}, ... ] ]`   — args nested in an array
    ///  - `data = [ "{\"janus\":...}" ]`      — a raw JSON string arg
    private static func janusPayloads(from data: [Any]) -> [Data] {
        var payloads: [Data] = []
        for element in data {
            collectJanusPayloads(from: element, into: &payloads)
        }
        return payloads
    }

    private static func collectJanusPayloads(from element: Any, into payloads: inout [Data]) {
        switch element {
        case let dict as [String: Any]:
            guard dict["janus"] != nil else { return }
            if let bytes = try? JSONSerialization.data(withJSONObject: dict) {
                payloads.append(bytes)
            }
        case let array as [Any]:
            for item in array {
                collectJanusPayloads(from: item, into: &payloads)
            }
        case let string as String:
            guard let data = string.data(using: .utf8),
                  let parsed = try? JSONSerialization.jsonObject(with: data, options: [])
            else { return }
            collectJanusPayloads(from: parsed, into: &payloads)
        default:
            return
        }
    }
}
