//
//  JaConnection.swift
//
//
//  Created by Hamza Jadid on 16/09/2024.
//

import Foundation
import UniFFI

/// Connection with a Janus server
public struct JaConnection {
    let connection: Connection

    private init(connection: Connection) {
        self.connection = connection
    }

    /// Connect using the provided configuration.
    ///
    /// - Parameters:
    ///     - config: Janus connection configuration
    /// - Returns: A connection with janus server
    public static func connect(config: JaConfig) async throws -> Self {
        let connection = try await rawJanusConnect(config: config.lower)
        return JaConnection(connection: connection)
    }

    /// Create a client-server session
    ///
    /// - Parameters:
    ///     - kaInterval: The time interval (seconds) for session keep-alive requests
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    /// - Returns: The newly created session
    public func createSession(kaInterval: UInt32, timeout: TimeInterval) async throws -> JaSession {
        let session = try await connection.createSession(kaInterval: kaInterval, timeout: timeout)
        return JaSession(session: session)
    }

    public func serverInfo(timeout: TimeInterval) async throws -> ServerInfoRsp {
        return try await connection.serverInfo(timeout: timeout)
    }
}
