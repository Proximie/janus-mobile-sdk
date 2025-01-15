//
//  JaHandleEvent.swift
//  JanusGateway
//
//  Created by Hamza Jadid on 15/01/2025.
//

import Foundation
import UniFFI

public enum JaHandleEvent {
    case plugin(Data)
    case handle(GenericEvent)
}
