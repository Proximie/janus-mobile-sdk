// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.2.0"
let releaseChecksum = "b6fe9b6186521a9fac3008c965f6b8406915cf07c4a4dd4e27469124ce2a0209"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "JanusGatewayFFI",
        path: "./target/ios/libjanus_gateway-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "JanusGatewayFFI",
        url: "https://github.com/Ghamza-Jd/janus-mobile-sdk/releases/download/\(releaseTag)/libjanus_gateway-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "JanusGateway",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "JanusGateway", targets: ["JanusGateway"]),
        .library(name: "JanusGatewayPlugins", targets: ["JanusGatewayPlugins"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "JanusGateway",
            dependencies: [.target(name: "UniFFI")],
            path: "apple/Sources/JanusGateway",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        ),
        .target(
            name: "JanusGatewayPlugins",
            dependencies: [
                .target(name: "UniFFI"),
                .target(name: "JanusGateway")
            ],
            path: "apple/Sources/Plugins",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        ),
        .target(
            name: "UniFFI",
            dependencies: [.target(name: "JanusGatewayFFI")],
            path: "apple/Sources/UniFFI"
        ),
        .testTarget(
            name: "JanusGatewayTests",
            dependencies: ["JanusGateway"],
            path: "apple/Tests/JanusGatewayTests"
        ),
    ]
)
