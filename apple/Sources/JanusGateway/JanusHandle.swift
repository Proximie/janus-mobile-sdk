@preconcurrency import Combine
import Foundation
import JanusGatewayBindings

/// General purpose plugin handle
public actor JanusHandle {
    private let sharedPublisher: AnyPublisher<JanusHandleEvent, Never>
    private nonisolated let subject = PassthroughSubject<JanusHandleEvent, Never>()
    private var cancellables = Set<AnyCancellable>()
    private let handle: Handle

    init(handle: Handle) {
        self.handle = handle
        self.sharedPublisher = subject
            .share()
            .eraseToAnyPublisher()
    }

    /// Get an async stream of incoming Janus events for this handle
    ///
    /// - Returns: An async stream of incoming events
    public func events() async -> AsyncStream<JanusHandleEvent> {
        await handle.startEventLoop(cb: self)
        let stream = AsyncStream<JanusHandleEvent>.makeStream()

        sharedPublisher
            .sink { stream.continuation.yield($0) }
            .store(in: &cancellables)

        return stream.stream
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

extension JanusHandle: HandleCallback {
    nonisolated public func onPluginEvent(event: Data) {
        subject.send(.plugin(event))
    }

    nonisolated public func onHandleEvent(event: GenericEvent) {
        subject.send(.handle(event))
    }
}
