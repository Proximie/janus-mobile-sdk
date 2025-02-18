.PHONY: help

# Set variables
RELEASE := false
FAT_SIMULATOR_LIB_DIR := target/ios-simulator-fat/release
LIBNAME := janus_gateway
MODULENAME := JanusGateway

# Check if release flag is set
ifeq ($(filter --release,$(MAKECMDGOALS)),--release)
    RELEASE := true
endif

help:
	@echo "Usage: make <platform> [-- --release]"
	@echo ""
	@echo "Platforms:"
	@echo "  apple: Build library for apple platforms"
	@echo "  android: Build library for android platforms"
	@echo ""
	@echo "Options:"
	@echo "  --release: Build library for release"

###############
##   Apple   ##
###############
apple: apple_clean \
	apple_build_rslib \
	apple_generate_ffi \
	apple_create_fat_simulator_lib \
	apple_build_xcframework \
	apple_gh_release

apple_build_rslib:
	@cargo build --lib --release --target x86_64-apple-ios
	@cargo build --lib --release --target aarch64-apple-ios-sim
	@cargo build --lib --release --target aarch64-apple-ios

apple_generate_ffi:
	@echo "Generating framework module mapping and FFI bindings"
	@cargo run -p uniffi-bindgen generate \
		--library target/aarch64-apple-ios/release/lib$(LIBNAME).dylib \
		--language swift \
		--out-dir target/uniffi-xcframework-staging
	@mkdir -p ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/*.swift ./apple/Sources/JanusGateway/
	@mv target/uniffi-xcframework-staging/$(MODULENAME)FFI.modulemap target/uniffi-xcframework-staging/module.modulemap

apple_create_fat_simulator_lib:
	@echo "Creating a fat library for x86_64 and aarch64 simulators"
	@mkdir -p $(FAT_SIMULATOR_LIB_DIR)
	@lipo -create target/x86_64-apple-ios/release/lib$(LIBNAME).a target/aarch64-apple-ios-sim/release/lib$(LIBNAME).a -output $(FAT_SIMULATOR_LIB_DIR)/lib$(LIBNAME).a

apple_build_xcframework:
	@echo "Generating XCFramework"
	@rm -rf target/ios
	@xcodebuild -create-xcframework \
		-library target/aarch64-apple-ios/release/lib$(LIBNAME).a -headers target/uniffi-xcframework-staging \
		-library target/ios-simulator-fat/release/lib$(LIBNAME).a -headers target/uniffi-xcframework-staging \
		-output target/ios/lib$(LIBNAME)-rs.xcframework
	@if [ "$(RELEASE)" = "true" ]; then \
		echo "Building xcframework archive"; \
		zip -r target/ios/lib$(LIBNAME)-rs.xcframework.zip target/ios/lib$(LIBNAME)-rs.xcframework; \
		checksum=$$(swift package compute-checksum target/ios/lib$(LIBNAME)-rs.xcframework.zip); \
		version=$$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="rslib") .version'); \
		sed -i "" -E "s/(let releaseTag = \")[^\"]+(\")/\1$$version\2/g" ./Package.swift; \
		sed -i "" -E "s/(let releaseChecksum = \")[^\"]+(\")/\1$$checksum\2/g" ./Package.swift; \
	fi

apple_gh_release:
	@if [ "$(RELEASE)" = "true" ]; then \
		echo "Committing changes to Package.swift and tagging the release"; \
		sed -i "" -E "s/(let useLocalFramework = )true/\1false/g" ./Package.swift; \
		shortcommit=$$(git rev-parse --short HEAD); \
		version=$$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name=="rslib") .version'); \
		git add ./Package.swift; \
		git add ./rslib/Cargo.toml; \
		git add ./Cargo.lock; \
		git commit -m "Update Package.swift for $$version release"; \
		git tag -a $$version -m "$$version"; \
		git push origin HEAD --tags; \
		echo "Creating draft GitHub release"; \
		gh release create $$version target/ios/lib$(LIBNAME)-rs.xcframework.zip --title "$$version" --generate-notes --draft; \
	fi

apple_clean:
	@echo "Cleaning up"
	@rm -rf target/ios
	@rm -rf target/uniffi-xcframework-staging
	@rm -rf $(FAT_SIMULATOR_LIB_DIR)


###############
##  Android  ##
###############
android: android_clean android_build_aar

android_clean:
	@cd android && ./gradlew clean

android_build_aar:
	@if [ "$(RELEASE)" = "true" ]; then \
		echo "Release build for android"; \
		cd android && ./gradlew assembleRelease; \
	else \
		echo "Debug build for android"; \
		cd android && ./gradlew assembleDebug; \
	fi
