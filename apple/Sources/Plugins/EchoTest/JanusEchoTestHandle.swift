@preconcurrency import Combine
import Foundation
import JanusGatewayBindings

/// EchoTest plugin handle,
///
/// The purpose of this plugin is for testing. A peer attaching to this plugin will receive the same packets he
/// sends.
public final class JanusEchoTestHandle {
    private let sharedPublisher: AnyPublisher<JanusEchoTestEvent, Never>
    private nonisolated let subject = PassthroughSubject<JanusEchoTestEvent, Never>()
    private var cancellables = Set<AnyCancellable>()
    private let handle: EchotestHandle

    init(handle: EchotestHandle) {
        self.handle = handle
        self.sharedPublisher = subject
            .share()
            .eraseToAnyPublisher()
    }

    /// Get an async stream of incoming Janus echotest events, check ``JanusEchoTestEvent``
    ///
    /// - Returns: An async stream of incoming events as ``JanusEchoTestEvent``
    public func events() async -> AsyncStream<JanusEchoTestEvent> {
        await handle.startEventLoop(cb: self)
        let stream = AsyncStream<JanusEchoTestEvent>.makeStream()

        sharedPublisher
            .sink { stream.continuation.yield($0) }
            .store(in: &cancellables)

        return stream.stream
    }

    /// Start the testing
    ///
    /// - Parameters:
    ///     - audio: enable/disable sending back audio
    ///     - video: enable/disable sending back video
    ///     - bitrate: to cap bitrate at the provided value
    public func start(params: EchoTestStartParams) async throws {
        try await handle.start(params: params)
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
        params: EchoTestStartParams,
        jsep: Jsep,
        timeout: TimeInterval
    ) async throws {
        try await handle.startWithJsep(
            params: params,
            jsep: jsep,
            timeout: timeout
        )
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

    /// Hang up the associated PeerConnection but keep the handle alive
    ///
    /// - Parameters:
    ///     - timeout: The maximum amount of time to wait on a response before we consider the
    ///     request as failed or times out.
    public func hangup(timeout: TimeInterval) async throws {
        try await handle.hangup(timeout: timeout)
    }

    /// Destroy the plugin handle
    ///
    /// - Parameters:
    ///     - timeout: The maximum amount of time to wait on a response before we consider the
    ///     request as failed or times out.
    public func detach(timeout: TimeInterval) async throws {
        try await handle.detach(timeout: timeout)
    }

    public func trickleSingleCandidate(
        candidate: Candidate, timeout: TimeInterval
    ) async throws {
        try await handle
            .trickleSingleCandidate(candidate: candidate, timeout: timeout)
    }

    public func trickleCandidates(
        candidates: [Candidate], timeout: TimeInterval
    ) async throws {
        try await handle
            .trickleCandidates(candidates: candidates, timeout: timeout)
    }

    public func completeTrickle(timeout: TimeInterval) async throws {
        try await handle.completeTrickle(timeout: timeout)
    }
}

extension JanusEchoTestHandle: EchotestHandleCallback {
    nonisolated public func onResult(echotest: String, result: String) {
        subject.send(.result(echotest: echotest, result: result))
    }

    nonisolated public func onResultWithJsep(echotest: String, result: String, jsep: Jsep) {
        subject.send(.resultWithJsep(echotest: echotest, result: result, jsep: jsep))
    }

    nonisolated public func onEchoTestError(errorCode: UInt16, error: String) {
        subject.send(.error(errorCode: errorCode, error: error))
    }

    nonisolated public func onHandleEvent(event: GenericEvent) {
        subject.send(.generic(event))
    }

    nonisolated public func onOther(data: Data) { }
}
