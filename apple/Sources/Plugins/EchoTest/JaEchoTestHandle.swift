//
//  Dummy.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import Foundation
import UniFFI

/// EchoTest plugin handle,
///
/// The purpose of this plugin is for testing. A peer attaching to this plugin will receive the same packets he
/// sends.
public final class JaEchoTestHandle {
    let handle: EchotestHandle
    public var delegate: JaEchoTestHandleDelegate?
    private var continuation: AsyncStream<JaEchoTestEvent>.Continuation?

    /// Get an async stream of incoming Janus echotest events, check ``JaEchotestEvent``
    ///
    /// - Returns: An async stream of incoming events as ``JaEchotestEvent``
    public var events: AsyncStream<JaEchoTestEvent> {
        get async {
            await handle.startEventLoop(cb: self)

            return AsyncStream { continuation in
                self.continuation = continuation
            }
        }
    }

    init(handle: EchotestHandle) {
        self.handle = handle
    }

    /// Start the testing
    ///
    /// - Parameters:
    ///     - audio: enable/disable sending back audio
    ///     - video: enable/disable sending back video
    ///     - bitrate: to cap bitrate at the provided value
    public func start(
        audio: Bool = false,
        video: Bool = false,
        bitrate: UInt32? = nil
    ) async throws {
        try await handle.start(
            audio: audio,
            video: video,
            bitrate: bitrate
        )
    }

    /// Start the testing
    ///
    /// - Parameters:
    ///     - audio: enable/disable sending back audio
    ///     - video: enable/disable sending back video
    ///     - bitrate: to cap bitrate at the provided value
    ///     - jsep: JavaScript establishment protocol
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out
    public func start(
        audio: Bool = false,
        video: Bool = false,
        bitrate: UInt32? = nil,
        jsep: Jsep,
        timeout: TimeInterval
    ) async throws {
        try await handle.startWithJsep(
            audio: audio,
            video: video,
            bitrate: bitrate,
            jsep: jsep,
            timeout: timeout
        )
    }
}

extension JaEchoTestHandle: EchotestHandleCallback {
    public func onResult(echotest: String, result: String) {
        delegate?.didReceiveEchoTestEvent(echotest: echotest, result: result)
        continuation?.yield(.result(echotest: echotest, result: result))
    }

    public func onResultWithJsep(echotest: String, result: String, jsep: Jsep) {
        delegate?.didReceiveEchoTestEvent(
            echotest: echotest, result: result, jsep: jsep
        )
        continuation?.yield(
            .resultWithJsep(echotest: echotest, result: result, jsep: jsep)
        )
    }

    public func onEchoTestError(errorCode: UInt16, error: String) {
        delegate?.didReceiveEchoTestError(errorCode: errorCode, error: error)
        continuation?.yield(.error(errorCode: errorCode, error: error))
    }
}
