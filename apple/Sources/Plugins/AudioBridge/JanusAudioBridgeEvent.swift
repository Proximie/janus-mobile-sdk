import JanusGatewayBindings

public enum JanusAudioBridgeEvent {
    case roomJoinedWithJsep(
        id: JanusId,
        room: JanusId,
        participants: [AudioBridgeParticipant],
        jsep: Jsep
    )
    case roomJoined(
        id: JanusId,
        room: JanusId,
        participants: [AudioBridgeParticipant]
    )
    case participantsJoined(
        room: JanusId,
        participants: [AudioBridgeParticipant]
    )
    case participantsUpdated(
        room: JanusId,
        participants: [AudioBridgeParticipant]
    )
    case participantLeft(
        room: JanusId,
        participantId: JanusId
    )
    case generic(GenericEvent)
    case error(errorCode: UInt16, error: String)
}
