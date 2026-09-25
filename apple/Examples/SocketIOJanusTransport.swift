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
        guard let client, let json = Self.jsonObject(from: data) else { return }
        manager?.handleQueue.async {
            client.emit(janusEvent, json)
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
        client.on(janusEvent) { [weak self] data, _ in
            guard let self, let sink = self.sink else { return }
            // The gateway may deliver one object or an array of them. Forward each
            // `{"janus": ...}` object as its own frame — jarust's demuxer expects one
            // message per receive (mirrors the old Rust `forward_json`).
            for payload in Self.janusPayloads(from: data) {
                sink.receive(data: payload)
            }
        }
    }

    // MARK: - Payload helpers

    /// jarust hands us serialized JSON; Socket.IO's `emit` wants a JSON-compatible
    /// object, so decode back to one.
    private static func jsonObject(from data: Data) -> Any? {
        try? JSONSerialization.jsonObject(with: data, options: [])
    }

    /// Extracts the bytes of every `{"janus": ...}` object from a Socket.IO `on`
    /// callback's `data` array.
    private static func janusPayloads(from data: [Any]) -> [Data] {
        var out: [Data] = []
        for element in data {
            switch element {
            case let dict as [String: Any] where dict["janus"] != nil:
                if let bytes = try? JSONSerialization.data(withJSONObject: dict) {
                    out.append(bytes)
                }
            case let array as [Any]:
                for item in array {
                    if let dict = item as? [String: Any], dict["janus"] != nil,
                       let bytes = try? JSONSerialization.data(withJSONObject: dict) {
                        out.append(bytes)
                    }
                }
            default:
                continue
            }
        }
        return out
    }
}
