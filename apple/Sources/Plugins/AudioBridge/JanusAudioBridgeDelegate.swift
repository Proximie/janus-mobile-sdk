public protocol JanusAudioBridgeDelegate: AnyObject, Sendable {
    func didReceive(audioBridgeEvent: JanusAudioBridgeEvent)
}
