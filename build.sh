#!/bin/bash

# Build script for NoodleOS

set -e

echo "Building NoodleOS..."

# Check if required tools are installed
command -v nasm >/dev/null 2>&1 || { echo >&2 "nasm is required but not installed. Aborting."; exit 1; }
command -v ld >/dev/null 2>&1 || { echo >&2 "ld (binutils) is required but not installed. Aborting."; exit 1; }
command -v grub-mkrescue >/dev/null 2>&1 || { echo >&2 "grub-mkrescue is required but not installed. Aborting."; exit 1; }

# Add the custom target if it's not already added
if ! rustup target list --installed | grep -q x86_64-unknown-none; then
    echo "Adding x86_64-unknown-none target..."
    rustup target add x86_64-unknown-none
fi

# Build the OS
make all

echo "Build complete! You can now run with: make run"
