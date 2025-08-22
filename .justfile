FAT_SIMULATOR_LIB_DIR := "target" / "ios-simulator-fat" / "release"
LIBNAME := "janus_gateway"
MODULENAME := "JanusGateway"

VERSION := `cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="rslib") .version'`
SHORTCOMMIT := `git rev-parse --short HEAD`

LATEST_TAG := `git tag --sort=-version:refname | head -n 1 2>/dev/null || echo "0.0.0"`

# Displays the available recipes
help:
	@just -l

# Build library for apple platforms. Pass '-r` to build release version
[group: 'apple']
[macos]
apple release="": \
	apple-clean apple-build apple-generate-ffi (apple-build-xcframework release) (apple-gh-release release)

# Build the Rust library for apple platforms
[group: 'apple']
[macos]
apple-build: apple-build-rslib apple-create-fat-simulator-lib

[private]
[macos]
apple-build-rslib:
	@echo "Building Rust lib"
	@cargo build --lib --release --target x86_64-apple-ios
	@cargo build --lib --release --target aarch64-apple-ios-sim
	@cargo build --lib --release --target aarch64-apple-ios

# Combines two static libs to create the simulator fat lib
[private]
[macos]
apple-create-fat-simulator-lib:
	@echo "Creating a fat library for x86_64 and aarch64 simulators"
	@mkdir -p {{FAT_SIMULATOR_LIB_DIR}}
	@lipo -create target/x86_64-apple-ios/release/lib{{LIBNAME}}.a target/aarch64-apple-ios-sim/release/lib{{LIBNAME}}.a -output {{FAT_SIMULATOR_LIB_DIR}}/lib{{LIBNAME}}.a

# Generate Swift ffi
[group: 'apple']
[macos]
apple-generate-ffi:
	@echo "Generating framework module mapping and FFI bindings"
	@cargo run -p uniffi-bindgen generate \
		--library target/aarch64-apple-ios/release/lib{{LIBNAME}}.dylib \
		--language swift \
		--out-dir target/uniffi-xcframework-staging
	@mkdir -p ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/*.swift ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/{{MODULENAME}}FFI.modulemap target/uniffi-xcframework-staging/module.modulemap

# Generate XCFramework that includes the static libs for apple platforms. When passing '-r' it will compute the zip checksum and modify the Package.swift accordingly
[group: 'apple']
[macos]
apple-build-xcframework release="":
	@echo "Generating XCFramework"
	@rm -rf target/ios
	@xcodebuild -create-xcframework \
		-library target/aarch64-apple-ios/release/lib{{LIBNAME}}.a -headers target/uniffi-xcframework-staging \
		-library target/ios-simulator-fat/release/lib{{LIBNAME}}.a -headers target/uniffi-xcframework-staging \
		-output target/ios/lib{{LIBNAME}}-rs.xcframework
	@if [ "{{release}}" = "-r" ]; then \
		echo "Building xcframework archive"; \
		zip -r target/ios/lib{{LIBNAME}}-rs.xcframework.zip target/ios/lib{{LIBNAME}}-rs.xcframework; \
		checksum=`swift package compute-checksum target/ios/lib{{LIBNAME}}-rs.xcframework.zip`; \
		sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1{{VERSION}}\2/g" ./Package.swift; \
		sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$checksum\2/g" ./Package.swift; \
	fi

# Create a github release. Only works when '-r' is passed.
[group: 'apple']
[macos]
apple-gh-release release="":
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
		gh release create {{VERSION}} target/ios/lib{{LIBNAME}}-rs.xcframework.zip --title "{{VERSION}}" --generate-notes --draft; \
	fi

# Clean up the build artifacts
[group: 'apple']
[macos]
apple-clean:
	@echo "Cleaning up"
	@rm -rf target/ios
	@rm -rf target/uniffi-xcframework-staging
	@rm -rf {{FAT_SIMULATOR_LIB_DIR}}

# Build library for android. Pass '-r` to build release version
[group: 'android']
android release="": android-clean (android-build release)

# Clean up the build artifacts
[working-directory: 'android']
[group: 'android']
android-clean:
	./gradlew clean

# Build the Rust lib, generate kotlin bindings, then bundle them inside aar. Pass '-r` to build release version
[working-directory: 'android']
[group: 'android']
android-build release="":
	@if [ "{{release}}" = "-r" ]; then \
		echo "Release build for android"; \
		./gradlew assembleRelease; \
	else \
		echo "Debug build for android"; \
		./gradlew assembleDebug; \
	fi

[group: 'utils']
[no-exit-message]
update-version version:
	@echo "Updating janus-mobile-sdk versions (rslib and Package.swift) to {{version}}"

	@sed -i.bak 's/^version = ".*"/version = "'{{version}}'"/' rslib/Cargo.toml && rm rslib/Cargo.toml.bak
	@sed -i.bak 's/^let releaseTag = ".*"/let releaseTag = "'{{version}}'"/' Package.swift && rm Package.swift.bak

	@echo "✓ Updated all janus-mobile-sdk versions to {{version}}"

# Validate version is semantic version and higher than latest tag
[group: 'utils']
[no-exit-message]
validate-version version: (validate-semver-regex version) (version-compare-with-latest version)

[private]
[no-exit-message]
validate-semver-regex version:
	@if [[ ! "{{version}}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then \
		echo "Error: Version must be semantic version major.minor.patch (e.g., 1.2.3)"; \
		exit 1; \
	fi
	@echo "✓ Version {{version}} is a semver"

[private]
[no-exit-message]
version-compare-with-latest version:
	#!/bin/bash
	latest_version=$(echo "{{LATEST_TAG}}" | sed 's/^v//')

	latest_major=$(echo "$latest_version" | cut -d. -f1)
	latest_minor=$(echo "$latest_version" | cut -d. -f2)
	latest_patch=$(echo "$latest_version" | cut -d. -f3)
	input_major=$(echo "{{version}}" | cut -d. -f1)
	input_minor=$(echo "{{version}}" | cut -d. -f2)
	input_patch=$(echo "{{version}}" | cut -d. -f3)

	is_greater_than_latest=0

	if [ "$input_major" -gt "$latest_major" ]; then
		is_greater_than_latest=1
	elif [ "$input_major" -eq "$latest_major" ]; then
		if [ "$input_minor" -gt "$latest_minor" ]; then
			is_greater_than_latest=1
		elif [ "$input_minor" -eq "$latest_minor" ]; then
			if [ "$input_patch" -gt "$latest_patch" ]; then
				is_greater_than_latest=1
			fi
		fi
	fi

	if [ "$is_greater_than_latest" -eq 1 ]; then
		echo "✓ Version {{version}} is higher than $latest_version (latest)"
	else
		echo "✗ Version {{version}} is not higher than $latest_version (latest)"
		exit 1
	fi
