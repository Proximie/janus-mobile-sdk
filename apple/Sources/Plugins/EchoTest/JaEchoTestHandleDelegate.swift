//
//  JaEchoTestHandleDelegate.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import UniFFI

public protocol JaEchoTestHandleDelegate {
    func didReceiveEchoTestEvent(echotest: String, result: String)
    func didReceiveEchoTestEvent(echotest: String, result: String, jsep: Jsep)
    func didReceiveEchoTestError(errorCode: UInt16, error: String)
}
