@preconcurrency import Combine
import Foundation
import JanusGatewayBindings

public actor JanusAudioBridgeHandle {
    private let sharedPublisher: AnyPublisher<JanusAudioBridgeEvent, Never>
    private nonisolated let subject = PassthroughSubject<JanusAudioBridgeEvent, Never>()
    private var cancellables = Set<AnyCancellable>()
    private let handle: AudioBridgeHandle

    init(handle: AudioBridgeHandle) {
        self.handle = handle
        self.sharedPublisher = subject
            .share()
            .eraseToAnyPublisher()
    }

    public func events() async -> AsyncStream<JanusAudioBridgeEvent> {
        await handle.startEventLoop(cb: self)
        let stream = AsyncStream<JanusAudioBridgeEvent>.makeStream()

        sharedPublisher
            .sink { stream.continuation.yield($0) }
            .store(in: &cancellables)

        return stream.stream
    }

    public func createRoom(
        params: AudioBridgeCreateParams,
        timeout: TimeInterval
    ) async throws -> AudioBridgeRoomCreatedRsp {
        try await handle.createRoom(params: params, timeout: timeout)
    }

    public func exist(roomId: JanusId, timeout: TimeInterval) async throws -> Bool {
        try await handle.exist(roomId: roomId, timeout: timeout)
    }

    public func listParticipants(
        roomId: JanusId, timeout: TimeInterval
    ) async throws -> AudioBridgeListParticipantsRsp {
        try await handle.listParticipants(roomId: roomId, timeout: timeout)
    }

    public func joinRoom(
        roomId: JanusId,
        params: AudioBridgeJoinParamsOptional,
        jsep: Jsep?,
        timeout: TimeInterval
    ) async throws {
        try await handle
            .joinRoom(
                roomId: roomId,
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

extension JanusAudioBridgeHandle: AudioBridgeHandleCallback {
    nonisolated public func onRoomJoinedWithJsep(
        id: JanusId,
        room: JanusId,
        participants: [AudioBridgeParticipant],
        jsep: Jsep
    ) {
        subject.send(.roomJoinedWithJsep(
            id: id,
            room: room,
            participants: participants,
            jsep: jsep
        ))
    }

    nonisolated public func onRoomJoined(
        id: JanusId,
        room: JanusId,
        participants: [AudioBridgeParticipant]
    ) {
        subject
            .send(.roomJoined(id: id, room: room, participants: participants))
    }

    nonisolated public func onParticipantsJoined(
        room: JanusId,
        participants: [AudioBridgeParticipant]
    ) {
        subject
            .send(.participantsJoined(room: room, participants: participants))
    }

    nonisolated public func onParticipantsUpdated(
        room: JanusId,
        participants: [AudioBridgeParticipant]
    ) {
        subject
            .send(.participantsUpdated(room: room, participants: participants))
    }

    nonisolated public func onParticipantLeft(
        room: JanusId,
        participantId: JanusId
    ) {
        subject.send(.participantLeft(room: room, participantId: participantId))
    }

    nonisolated public func onHandleEvent(event: GenericEvent) {
        subject.send(.generic(event))
    }

    nonisolated public func onAudioBridgeError(errorCode: UInt16, error: String) {
        subject.send(.error(errorCode: errorCode, error: error))
    }

    nonisolated public func onOther(data: Data) { }
}
