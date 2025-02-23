FAT_SIMULATOR_LIB_DIR := "target" / "ios-simulator-fat" / "release"
LIBNAME := "janus_gateway"
MODULENAME := "JanusGateway"

VERSION := `cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="rslib") .version'`
SHORTCOMMIT := `git rev-parse --short HEAD`

# Displays the available recipes
help:
	@just -l

# Build library for apple platforms
apple release="-d": apple-clean \
	apple-build-rslib \
	apple-generate-ffi \
	apple-create-fat-simulator-lib \
	(apple-build-xcframework release) \
	(apple-gh-release release)

# Build the Rust library for apple platforms
apple-build: apple-build-rslib apple-create-fat-simulator-lib

[private]
apple-build-rslib:
	@echo "Building Rust lib"
	@cargo build --lib --release --target x86_64-apple-ios
	@cargo build --lib --release --target aarch64-apple-ios-sim
	@cargo build --lib --release --target aarch64-apple-ios

# Combines two static libs to create the simulator fat lib
[private]
apple-create-fat-simulator-lib:
	@echo "Creating a fat library for x86_64 and aarch64 simulators"
	@mkdir -p {{FAT_SIMULATOR_LIB_DIR}}
	@lipo -create target/x86_64-apple-ios/release/lib{{LIBNAME}}.a target/aarch64-apple-ios-sim/release/lib{{LIBNAME}}.a -output {{FAT_SIMULATOR_LIB_DIR}}/lib{{LIBNAME}}.a


# Generate Swift ffi
apple-generate-ffi:
	@echo "Generating framework module mapping and FFI bindings"
	@cargo run -p uniffi-bindgen generate \
		--library target/aarch64-apple-ios/release/lib{{LIBNAME}}.dylib \
		--language swift \
		--out-dir target/uniffi-xcframework-staging
	@mkdir -p ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/*.swift ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/{{MODULENAME}}FFI.modulemap target/uniffi-xcframework-staging/module.modulemap

# Generate XCFramework that includes the static libs for apple platforms
apple-build-xcframework release="-d":
	@echo "Generating XCFramework"
	@rm -rf target/ios
	@xcodebuild -create-xcframework \
		-library target/aarch64-apple-ios/release/lib{{LIBNAME}}.a -headers target/uniffi-xcframework-staging \
		-library target/ios-simulator-fat/release/lib{{LIBNAME}}.a -headers target/uniffi-xcframework-staging \
		-output target/ios/lib{{LIBNAME}}-rs.xcframework
	@if [ "{{release}}" = "-r" ]; then \
		echo "Building xcframework archive"; \
		checksum=`swift package compute-checksum target/ios/lib{{LIBNAME}}-rs.xcframework.zip` \
		zip -r target/ios/lib{{LIBNAME}}-rs.xcframework.zip target/ios/lib{{LIBNAME}}-rs.xcframework; \
		sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1{{VERSION}}\2/g" ./Package.swift; \
		sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$$checksum\2/g" ./Package.swift; \
	fi

# Create a github release
apple-gh-release release="-d":
	@if [ "{{release}}" = "-r" ]; then \
		echo "Committing changes to Package.swift and tagging the release"; \
		sed -i "" -E "s/(let useLocalFramework = )true/\1false/g" ./Package.swift; \
		git add ./Package.swift; \
		git add ./rslib/Cargo.toml; \
		git add ./Cargo.lock; \
		git commit -m "Update Package.swift for {{VERSION}} release"; \
		git tag -a {{VERSION}} -m "{{VERSION}}"; \
		git push origin HEAD --tags; \
		echo "Creating draft GitHub release"; \
	fi

# Clean up the build artifacts
apple-clean:
	@echo "Cleaning up"
	@rm -rf target/ios
	@rm -rf target/uniffi-xcframework-staging
	@rm -rf {{FAT_SIMULATOR_LIB_DIR}}
