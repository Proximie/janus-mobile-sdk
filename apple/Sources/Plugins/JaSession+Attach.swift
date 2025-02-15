import Foundation
import JanusGateway

extension JanusSession {
    /// Attach to echotest plugin
    ///
    /// - Returns: An echotest plugin handle
    public func attachEchoTest(timeout: TimeInterval) async throws -> JanusEchoTestHandle {
        let handle = try await lower.attachEchoTest(timeout: timeout)
        return JanusEchoTestHandle(handle: handle)
    }

    public func attachAudioBridge(timeout: TimeInterval) async throws -> JanusAudioBridgeHandle {
        let handle = try await lower.attachAudioBridge(timeout: timeout)
        return JanusAudioBridgeHandle(handle: handle)
    }
}
