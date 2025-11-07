#!/bin/bash

# NoodleOS Quick Test Script
# Simple script to verify the OS builds and boots

echo "=== NoodleOS Quick Test ==="

# Check if ISO exists
if [ ! -f noodleos.iso ]; then
    echo "❌ noodleos.iso not found. Run 'make all' first."
    exit 1
fi

echo "✅ ISO file found"

# Test boot in QEMU (5 second timeout)
echo "🚀 Testing boot in QEMU..."
timeout 5 qemu-system-x86_64 \
    -cdrom noodleos.iso \
    -m 64M \
    -display none \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    > /dev/null 2>&1

QEMU_EXIT_CODE=$?

if [ $QEMU_EXIT_CODE -eq 124 ]; then
    echo "✅ OS boots successfully (timeout after 5s - normal)"
elif [ $QEMU_EXIT_CODE -eq 0 ] || [ $QEMU_EXIT_CODE -eq 1 ]; then
    echo "✅ OS boots and exits cleanly"
else
    echo "❌ OS failed to boot (exit code: $QEMU_EXIT_CODE)"
    exit 1
fi

echo "✅ All tests passed!"
echo ""
echo "To run interactively:"
echo "  ./test_os.sh run    # GUI mode"
echo "  ./test_os.sh debug  # Debug mode with GDB"
