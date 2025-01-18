import Foundation
import UniFFI

/// Client-Server Session with Janus
public struct JanusSession {
    let session: Session

    public var lower: Session { session }

    /// Attach plugin to a janus session
    ///
    /// - Parameters:
    ///     - pluginId: Plugin identifier _e.g: janus.plugin.echotest_
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    /// - Returns: A general purpose plugin handle
    public func attach(
        pluginId: String, timeout: TimeInterval
    ) async throws -> JanusHandle {
        let handle = try await session.attach(pluginId: pluginId, timeout: timeout)
        return JanusHandle(handle: handle)
    }

    /// Destroy the current session
    ///
    /// - Parameters:
    ///     - timeout: The maximum amount of time to wait on an acknowledgment before we consider
    ///     the request as failed or times out.
    public func destory(timeout: TimeInterval) async throws {
        try await session.destory(timeout: timeout)
    }
}
