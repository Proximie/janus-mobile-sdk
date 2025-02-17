import Foundation
import JanusGatewayBindings

/// Connection with a Janus server
public actor JanusConnection {
    let connection: Connection

    private init(connection: Connection) {
        self.connection = connection
    }

    /// Connect using the provided configuration.
    ///
    /// - Parameters:
    ///     - config: Janus connection configuration
    /// - Returns: A connection with janus server
    public static func connect(config: JanusConfig) async throws -> JanusConnection {
        let connection = try await rawJanusConnect(config: config.lower)
        return JanusConnection(connection: connection)
    }

    /// Create a client-server session
    ///
    /// - Parameters:
    ///     - kaInterval: The time interval (seconds) for session keep-alive requests
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    /// - Returns: The newly created session
    public func createSession(
        kaInterval: UInt32, timeout: TimeInterval
    ) async throws -> JanusSession {
        let session = try await connection.createSession(kaInterval: kaInterval, timeout: timeout)
        return JanusSession(session: session)
    }

    /// Retrieve Janus server info
    ///
    /// - Parameters:
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    /// - Returns: Janus server info
    public func serverInfo(timeout: TimeInterval) async throws -> ServerInfoRsp {
        return try await connection.serverInfo(timeout: timeout)
    }
}
