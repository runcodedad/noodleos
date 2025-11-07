#!/bin/bash

# NoodleOS Development Test Script
# Comprehensive testing and development helper for NoodleOS

set -e  # Exit on any error

show_help() {
    echo "NoodleOS Development Test Script"
    echo "Usage: $0 [option]"
    echo ""
    echo "Options:"
    echo "  run              - Run OS in QEMU with GUI (default)"
    echo "  debug            - Run OS with GDB debugging support (-s -S)"
    echo "  test             - Quick automated test (build + boot verification)"
    echo "  test-exceptions  - Build with exception test framework"
    echo "  test-divide-zero - Build and test divide by zero exception"
    echo "  build            - Build the OS without running"
    echo "  clean            - Clean build artifacts"
    echo "  monitor          - Run with QEMU monitor for advanced debugging"
    echo "  help             - Show this help"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run with GUI"
    echo "  $0 debug             # Start with GDB server on :1234"
    echo "  $0 test              # Quick CI-style test"
    echo "  $0 test-divide-zero  # Test divide by zero handler"
}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

check_prerequisites() {
    local missing=0
    
    if ! command -v qemu-system-x86_64 &> /dev/null; then
        log_error "qemu-system-x86_64 not found"
        missing=1
    fi
    
    if ! command -v cargo &> /dev/null; then
        log_error "cargo not found"
        missing=1
    fi
    
    if [ $missing -eq 1 ]; then
        log_error "Missing prerequisites. Please install QEMU and Rust."
        exit 1
    fi
}

build_os() {
    log_info "Building NoodleOS..."
    if make all > /dev/null 2>&1; then
        log_success "Build completed successfully"
    else
        log_error "Build failed"
        make all  # Show the actual error
        exit 1
    fi
}

run_gui() {
    log_info "Starting NoodleOS in QEMU with GUI..."
    log_info "Close QEMU window to return to terminal"
    qemu-system-x86_64 -cdrom noodleos.iso -m 128M -name "NoodleOS"
}

run_debug() {
    log_info "Starting NoodleOS in debug mode..."
    log_warning "QEMU will pause at startup"
    log_info "In another terminal, run: gdb -ex 'target remote localhost:1234'"
    log_info "Then use 'continue' in GDB to start execution"
    qemu-system-x86_64 -cdrom noodleos.iso -m 128M -s -S -name "NoodleOS Debug"
}

run_test() {
    log_info "Running automated test..."
    
    # Check if ISO exists, build if not
    if [ ! -f noodleos.iso ]; then
        build_os
    fi
    
    # Test boot (5 second timeout)
    log_info "Testing boot sequence..."
    if timeout 5 qemu-system-x86_64 \
        -cdrom noodleos.iso \
        -m 64M \
        -display none \
        -serial null \
        > /dev/null 2>&1; then
        log_success "OS boots and runs successfully"
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            log_success "OS boots successfully (normal timeout after 5s)"
        else
            log_error "OS failed to boot (exit code: $exit_code)"
            return 1
        fi
    fi
    
    log_success "All tests passed!"
}

run_monitor() {
    log_info "Starting NoodleOS with QEMU monitor..."
    log_info "Use 'quit' in monitor to exit"
    log_info "Monitor commands: info registers, info cpus, etc."
    qemu-system-x86_64 -cdrom noodleos.iso -m 128M -monitor stdio -name "NoodleOS Monitor"
}

clean_build() {
    log_info "Cleaning build artifacts..."
    make clean
    log_success "Clean completed"
}

# Main logic
case "${1:-run}" in
    "run")
        check_prerequisites
        if [ ! -f noodleos.iso ]; then
            build_os
        fi
        run_gui
        ;;
    "debug")
        check_prerequisites
        if [ ! -f noodleos.iso ]; then
            build_os
        fi
        run_debug
        ;;
    "test")
        check_prerequisites
        run_test
        ;;
    "test-exceptions")
        check_prerequisites
        log_info "Building with exception test framework..."
        make test-exceptions
        log_success "Exception test build completed"
        ;;
    "test-divide-zero")
        check_prerequisites
        log_info "Building with divide by zero test..."
        make test-divide-by-zero
        log_success "Divide by zero test build completed"
        log_warning "This build will trigger a divide by zero exception!"
        log_info "Run with: ./dev.sh run"
        ;;
    "build")
        check_prerequisites
        build_os
        ;;
    "clean")
        clean_build
        ;;
    "monitor")
        check_prerequisites
        if [ ! -f noodleos.iso ]; then
            build_os
        fi
        run_monitor
        ;;
    "help"|"-h"|"--help")
        show_help
        ;;
    *)
        log_error "Unknown option: $1"
        show_help
        exit 1
        ;;
esac
