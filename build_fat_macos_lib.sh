#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Step 1: Add the necessary targets
echo "Adding targets for x86_64-apple-darwin and aarch64-apple-darwin..."
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Step 2: Build for x86_64-apple-darwin
echo "Building for x86_64-apple-darwin..."
cargo build --release --target x86_64-apple-darwin

# Step 3: Build for aarch64-apple-darwin
echo "Building for aarch64-apple-darwin..."
cargo build --release --target aarch64-apple-darwin

# Step 4: Create a universal binary using lipo
echo "Creating a universal binary..."
lipo -create -output xcstringstodocx target/x86_64-apple-darwin/release/xcstringsdocx target/aarch64-apple-darwin/release/xcstringsdocx

# Step 5: Verify the created binary
echo "Verifying the universal binary..."
file_output=$(file xcstringstodocx)
echo "$file_output"

echo "Build and packaging completed successfully!"