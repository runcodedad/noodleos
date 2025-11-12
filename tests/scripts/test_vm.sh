#!/bin/bash
# Quick test script for virtual memory system

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  NoodleOS Virtual Memory Test Runner                      ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if QEMU is installed
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Error: qemu-system-x86_64 not found"
    echo "Please install QEMU to run tests"
    exit 1
fi

# Parse arguments
TEST_TYPE="${1:-virtual}"

case $TEST_TYPE in
    virtual|vm)
        echo "Building with virtual memory tests..."
        MAKE_TARGET="test-virtual-memory"
        ;;
    memory|mem)
        echo "Building with all memory tests..."
        MAKE_TARGET="test-memory"
        ;;
    exceptions)
        echo "Building with exception tests..."
        MAKE_TARGET="test-exceptions"
        ;;
    *)
        echo "Usage: $0 [virtual|memory|exceptions]"
        echo ""
        echo "Options:"
        echo "  virtual (default) - Run virtual memory tests only"
        echo "  memory            - Run all memory tests (physical + virtual)"
        echo "  exceptions        - Run exception handling tests"
        exit 1
        ;;
esac

echo "Target: $MAKE_TARGET"
echo ""

# Build
echo "Building kernel..."
make $MAKE_TARGET

if [ $? -ne 0 ]; then
    echo ""
    echo "✗ Build failed"
    exit 1
fi

echo ""
echo "✓ Build successful"
echo ""

# Run in QEMU
echo "Starting QEMU..."
echo "Press Ctrl+A then X to exit QEMU"
echo ""
echo "─────────────────────────────────────────────────────────────"

qemu-system-x86_64 \
    -cdrom noodleos.iso \
    -serial stdio \
    -display none \
    -no-reboot \
    -no-shutdown

echo ""
echo "─────────────────────────────────────────────────────────────"
echo ""
echo "Test run complete!"
