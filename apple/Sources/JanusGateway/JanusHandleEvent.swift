import Foundation
import JanusGatewayBindings

public enum JanusHandleEvent {
    case plugin(Data)
    case handle(GenericEvent)
}
