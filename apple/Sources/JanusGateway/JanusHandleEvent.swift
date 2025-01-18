import Foundation
import UniFFI

public enum JanusHandleEvent {
    case plugin(Data)
    case handle(GenericEvent)
}
