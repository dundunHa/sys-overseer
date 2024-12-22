#!/bin/bash

# 创建发布目录
mkdir -p releases

# 编译所有平台版本
platforms=(
    "x86_64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-gnu"
)

for platform in "${platforms[@]}"
do
    echo "Building for $platform..."
    cross build --target "$platform" --release
    
    # 创建发布包
    if [[ $platform == *"windows"* ]]; then
        cp "target/$platform/release/system_monitor.exe" "releases/system_monitor-$platform.exe"
    else
        cp "target/$platform/release/system_monitor" "releases/system_monitor-$platform"
    fi
done

echo "All builds completed!"