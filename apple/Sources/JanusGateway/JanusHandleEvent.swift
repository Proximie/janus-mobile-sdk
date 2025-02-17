import Foundation
import JanusGatewayBindings

public enum JanusHandleEvent: Sendable {
    case plugin(Data)
    case handle(GenericEvent)
}
