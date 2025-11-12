#!/bin/bash
# NoodleOS Test Runner
# Main entry point for running OS tests

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_header() {
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║  $1${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Check dependencies
check_dependencies() {
    local missing=0
    
    if ! command -v qemu-system-x86_64 &> /dev/null; then
        print_error "qemu-system-x86_64 not found"
        missing=1
    fi
    
    if ! command -v make &> /dev/null; then
        print_error "make not found"
        missing=1
    fi
    
    if [ $missing -eq 1 ]; then
        echo ""
        print_error "Missing required dependencies"
        exit 1
    fi
    
    print_success "All dependencies found"
}

# Show usage
usage() {
    cat << EOF
Usage: $0 [OPTION] [TEST_TYPE]

Run NoodleOS tests with various configurations.

Options:
  -h, --help              Show this help message
  -q, --quick             Run quick boot test only
  -m, --memory [SIZE]     Run with specific memory size (e.g., 256M, 1G)
  -a, --all-memory        Test all memory configurations
  -v, --verbose           Show detailed output
  -d, --debug             Run in debug mode (with GDB)

Test Types:
  quick                   Quick boot test (default)
  exceptions              Exception handling tests
  memory                  Memory allocator tests
  virtual-memory          Virtual memory tests
  hardware                Hardware interrupt tests
  all                     Run all tests

Examples:
  $0 quick                          # Quick boot test
  $0 -m 512M memory                 # Memory tests with 512MB RAM
  $0 -a                             # Test all memory sizes
  $0 -d exceptions                  # Debug exception tests
  $0 all                            # Run all test suites

EOF
}

# Main test logic
main() {
    cd "$PROJECT_ROOT"
    
    local test_type="quick"
    local memory_size="128M"
    local debug_mode=false
    local verbose=false
    local all_memory=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                usage
                exit 0
                ;;
            -q|--quick)
                test_type="quick"
                shift
                ;;
            -m|--memory)
                memory_size="$2"
                shift 2
                ;;
            -a|--all-memory)
                all_memory=true
                shift
                ;;
            -v|--verbose)
                verbose=true
                shift
                ;;
            -d|--debug)
                debug_mode=true
                shift
                ;;
            *)
                test_type="$1"
                shift
                ;;
        esac
    done
    
    print_header "NoodleOS Test Runner"
    
    check_dependencies
    echo ""
    
    # Handle all-memory test
    if [ "$all_memory" = true ]; then
        "$SCRIPT_DIR/test_memory_sizes.sh"
        exit $?
    fi
    
    # Run specific test
    case $test_type in
        quick)
            print_info "Running quick boot test..."
            "$SCRIPT_DIR/quick_test.sh"
            ;;
        exceptions|memory|virtual-memory|hardware)
            print_info "Running $test_type tests with ${memory_size} RAM..."
            if [ "$debug_mode" = true ]; then
                make "debug-test-${test_type}" QEMU_MEMORY="$memory_size"
            else
                make "run-test-${test_type}" QEMU_MEMORY="$memory_size"
            fi
            ;;
        all)
            print_info "Running all test suites..."
            "$SCRIPT_DIR/quick_test.sh"
            make run-test-all QEMU_MEMORY="$memory_size"
            ;;
        *)
            print_error "Unknown test type: $test_type"
            echo ""
            usage
            exit 1
            ;;
    esac
}

main "$@"
