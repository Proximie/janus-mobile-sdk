import Foundation
import JanusGatewayBindings

/// A struct describing a janus server (e.g. url, secret).
public struct JanusConfig {
    let url: URL
    let capacity: UInt16
    let serverRoot: String
    let apisecret: String?

    var lower: Config {
        Config(
            url: url.absoluteString,
            capacity: capacity,
            apisecret: apisecret,
            serverRoot: serverRoot
        )
    }

    /// - Parameters:
    ///     - url: The URL to reach this server API
    ///     - capacity: Buffer capacity
    ///     - serverRoot: The connection server root, default ot "janus"
    ///     - apisecret: The API secret for this server
    public init(
        url: URL,
        capacity: UInt16,
        serverRoot: String = "janus",
        apisecret: String? = nil
    ) {
        self.url = url
        self.capacity = capacity
        self.serverRoot = serverRoot
        self.apisecret = apisecret
    }
}
