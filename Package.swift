// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = true
let releaseTag = "0.1.0"
let releaseChecksum = "512ec41bc933fd7da15161676bfb65963b986a5778f4e1f91a4f5908a04a174d"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "JanusGatewayFFI",
        path: "./target/ios/libjanus_gateway-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "JanusGatewayFFI",
        url: "https://github.com/Proximie/jarust-ios-package/releases/download/\(releaseTag)/libjanus_gateway-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "JanusGateway",
    platforms: [.iOS(.v13), .macOS(.v11)],
    products: [
        .library(
            name: "JanusGateway",
            targets: ["JanusGateway"]
        ),
        .library(
            name: "JanusGatewayPlugins",
            targets: ["JanusGatewayPlugins"]
        ),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "JanusGateway",
            dependencies: [.target(name: "UniFFI")],
            path: "apple/Sources/JanusGateway"
        ),
        .target(
            name: "JanusGatewayPlugins",
            dependencies: [
                .target(name: "UniFFI"),
                .target(name: "JanusGateway")
            ],
            path: "apple/Sources/Plugins"
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
