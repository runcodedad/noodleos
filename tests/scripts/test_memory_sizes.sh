#!/bin/bash
# Test script to validate memory allocator with different RAM sizes

set -e

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  NoodleOS Memory Size Validation Test                     ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Check if QEMU is installed
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Error: qemu-system-x86_64 not found"
    echo "Please install QEMU to run tests"
    exit 1
fi

# Build the OS if needed
if [ ! -f noodleos.iso ]; then
    echo "Building NoodleOS..."
    make
    echo ""
fi

# Test with different memory sizes
MEMORY_SIZES=("64M" "128M" "256M" "512M" "1G" "2G")

for mem in "${MEMORY_SIZES[@]}"; do
    echo "════════════════════════════════════════════════════════════"
    echo "Testing with $mem RAM"
    echo "════════════════════════════════════════════════════════════"
    
    # Run QEMU briefly to capture output
    # Using -display none and trying to get any output we can
    timeout 2 qemu-system-x86_64 -m "$mem" -cdrom noodleos.iso -display none 2>&1 || true
    
    echo ""
    sleep 1
done

echo "╔════════════════════════════════════════════════════════════╗"
echo "║  Memory tests complete                                     ║"
echo "║  Note: Run 'make test-mem-XXX' interactively to see       ║"
echo "║  the VGA output with memory statistics                    ║"
echo "╚════════════════════════════════════════════════════════════╝"
