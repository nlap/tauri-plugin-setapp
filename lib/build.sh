#!/bin/bash
xcodebuild -project SetappBridge/SetappBridge.xcodeproj -scheme "SetappBridge Release" build

build_setting=$(xcodebuild -project SetappBridge/SetappBridge.xcodeproj -scheme "SetappBridge Release" -showBuildSettings | grep CONFIGURATION_BUILD_DIR)

build_dir=$(echo "$build_setting" | cut -d'=' -f2 | xargs)

cp "$build_dir/libSetappBridge.a" libSetappBridge.a
