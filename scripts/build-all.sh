#!/bin/bash

echo "Building for all platforms..."

docker build -f docker/Dockerfile.windows-x86 -t crackle-builder-win32 .
docker build -f docker/Dockerfile.windows-x64 -t crackle-builder-win64 .
docker build -f docker/Dockerfile.linux-x64 -t crackle-builder-linux-x64 .

# Run builds
echo "Building Windows x86..."
docker run --rm -v $(pwd):/workspace crackle-builder-win32

echo "Building Windows x64..."
docker run --rm -v $(pwd):/workspace crackle-builder-win64

echo "Building Linux x64..."
docker run --rm -v $(pwd):/workspace crackle-builder-linux-x64

# Native Mac build
echo "Building for macOS (native)..."
cargo build --release




echo "Organizing binaries..."
mkdir -p dist
cp target/release/crackle dist/crackle-macos-$(uname -m)  # adds arm64 or x86_64
cp target/i686-pc-windows-gnu/release/crackle.exe dist/crackle-windows-x86.exe
cp target/x86_64-pc-windows-gnu/release/crackle.exe dist/crackle-windows-x64.exe
cp target/x86_64-unknown-linux-gnu/release/crackle dist/crackle-linux-x64

echo "Build complete! Check dist/ folder"
