import JanusGatewayBindings

public protocol JanusEchoTestHandleDelegate {
    func didReceiveEchoTestEvent(echotest: String, result: String)
    func didReceiveEchoTestEvent(echotest: String, result: String, jsep: Jsep)
    func didReceiveEchoTestError(errorCode: UInt16, error: String)
}
