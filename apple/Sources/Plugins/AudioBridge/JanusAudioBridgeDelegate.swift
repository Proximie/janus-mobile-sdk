public protocol JanusAudioBridgeDelegate: Sendable {
    func didReceive(audioBridgeEvent: JanusAudioBridgeEvent)
}
