// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

// Never push to remote with this flag set to true
let useLocalFramework = false
let releaseTag = "0.16.0"
let releaseChecksum = "01b965d9977fd292498b9957bda6fa25da21052d8f70d54a6e3ccf8c8bb612a7"

let binaryTarget: Target = if useLocalFramework {
    .binaryTarget(
        name: "JanusGatewayFFI",
        path: "./target/ios/libjanus_gateway-rs.xcframework"
    )
} else {
    .binaryTarget(
        name: "JanusGatewayFFI",
        url: "https://github.com/Proximie/janus-mobile-sdk/releases/download/\(releaseTag)/libjanus_gateway-rs.xcframework.zip",
        checksum: releaseChecksum
    )
}

let package = Package(
    name: "JanusGateway",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "JanusGateway", targets: ["JanusGateway"]),
    ],
    targets: [
        binaryTarget,
        .target(
            name: "JanusGateway",
            dependencies: [.target(name: "JanusGatewayFFI")],
            path: "apple/Sources/JanusGateway",
            resources: [
                .process("Resources/PrivacyInfo.xcprivacy")
            ]
        )
    ]
)
