import JanusGatewayBindings

/// Asynchronous incoming event type
public enum JanusEchoTestEvent {
    case result(echotest: String, result: String)
    case resultWithJsep(echotest: String, result: String, jsep: Jsep)
    case error(errorCode: UInt16, error: String)
    case generic(GenericEvent)
}
