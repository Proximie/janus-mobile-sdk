//
//  JaEchoTestEvent.swift
//  JanusGateway
//
//  Created by Hamza Jadid on 10/01/2025.
//

import UniFFI

/// Asynchronous incoming event type
public enum JaEchoTestEvent {
    case result(echotest: String, result: String)
    case resultWithJsep(echotest: String, result: String, jsep: Jsep)
    case error(errorCode: UInt16, error: String)
}
