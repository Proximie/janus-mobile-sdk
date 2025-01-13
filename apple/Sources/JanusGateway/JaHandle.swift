//
//  JaHandle.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import Foundation
import UniFFI

/// General purpose plugin handle
public final class JaHandle {
    let handle: Handle
    public var delegate: JaHandleDelegate?
    private var continuation: AsyncStream<String>.Continuation?

    /// Get an async stream of incoming Janus events for this handle
    ///
    /// - Returns: An async stream of incoming events
    public var events: AsyncStream<String> {
        get async {
            await handle.startEventLoop(cb: self)

            return AsyncStream { continuation in
                self.continuation = continuation
            }
        }
    }

    init(handle: Handle) {
        self.handle = handle
    }

    /// Sends a message without waiting for any response or acknowledgment
    ///
    /// - Parameters:
    ///     - data: Data to be sent
    ///     - jsep: JavaScript Session Establishment Protocol
    public func fireAndForget(data: Data, jsep: Jsep? = nil) async throws {
        if let jsep {
            try await handle.fireAndForgetWithJsep(data: data, jsep: jsep)
        } else {
            try await handle.fireAndForget(data: data)
        }
    }

    /// Sends a message and waits until the server acknowledges or timeout
    ///
    /// - Parameters:
    ///     - data: Data to be sent
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    public func sendWaitonAck(
        data: Data, timeout: TimeInterval
    ) async throws {
        try await handle.sendWaitonAck(data: data, timeout: timeout)
    }

    /// Sends a message and waits until the server returns a response or timeout
    ///
    /// - Parameters:
    ///     - data: Data to be sent
    ///     - timeout: The maximum amount of time to wait on a response before we consider the
    ///     request as failed or times out.
    public func sendWaitonResult(
        data: Data,
        timeout: TimeInterval
    ) async throws -> Data {
        try await handle.sendWaitonResult(data: data, timeout: timeout)
    }
}

extension JaHandle: HandleCallback {
    public func onEvent(event: String) {
        delegate?.didReceive(event: event)
        continuation?.yield(event)
    }
}
