# NoodleOS Test Scripts

Test automation scripts for NoodleOS.

## Scripts Overview

### Main Test Runner

**`run_tests.sh`** - Unified test runner with comprehensive options

```bash
# Usage examples
./run_tests.sh quick                      # Quick boot test
./run_tests.sh exceptions                 # Exception tests
./run_tests.sh memory                     # Memory tests
./run_tests.sh --memory 512M memory       # Custom RAM size
./run_tests.sh --all-memory               # Test all RAM configurations
./run_tests.sh --debug exceptions         # Debug mode
./run_tests.sh --help                     # Show all options
```

**Features:**
- Unified interface for all test types
- Configurable memory sizes
- Debug mode support
- Colored output
- Dependency checking

### Individual Test Scripts

**`quick_test.sh`** - Fast boot verification

Quick sanity check that the OS builds and boots. Useful for CI/CD.

```bash
./quick_test.sh
```

**`test_vm.sh`** - Virtual memory test runner

Specialized runner for memory-related tests.

```bash
./test_vm.sh virtual      # Virtual memory tests
./test_vm.sh memory       # All memory tests
./test_vm.sh exceptions   # Exception tests
```

**`test_memory_sizes.sh`** - Memory configuration validator

Tests the physical memory allocator with various RAM sizes to validate
correct memory detection and allocation.

```bash
./test_memory_sizes.sh
```

Automatically tests: 64M, 128M, 256M, 512M, 1G, 2G

## Running from Project Root

All scripts are designed to work from the project root:

```bash
# From project root
tests/scripts/run_tests.sh quick

# Or from scripts directory
cd tests/scripts
./run_tests.sh quick
```

## Integration with Make

These scripts integrate with the Makefile system:

```bash
# Make uses these scripts internally
make test               # Calls quick_test.sh
make run-test-memory    # Uses configured QEMU settings
make test-mem-512m      # Specific memory test target
```

## Script Standards

All test scripts follow these conventions:

1. **Shebang**: `#!/bin/bash`
2. **Error handling**: `set -e` for fail-fast
3. **Help text**: All scripts support `-h` or `--help`
4. **Exit codes**: 
   - `0` = success
   - `1` = failure
   - `124` = timeout (acceptable for boot tests)
5. **Output**: Colored output with clear success/failure indicators
6. **Dependencies**: Check for required tools before running

## Adding New Test Scripts

When creating new test scripts:

1. Place in `tests/scripts/`
2. Make executable: `chmod +x script_name.sh`
3. Follow the standard template:

```bash
#!/bin/bash
# Script description

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Check dependencies
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Error: qemu-system-x86_64 not found"
    exit 1
fi

# Main logic
cd "$PROJECT_ROOT"
# ... test implementation
```

4. Update this README
5. Add integration to `run_tests.sh` if applicable

## Environment Variables

Scripts respect these environment variables:

- `QEMU_MEMORY` - Override default memory size (default: 128M)
- `QEMU_FLAGS` - Additional QEMU flags
- `NO_COLOR` - Disable colored output

## Examples

### Daily Development Workflow

```bash
# Quick check before commit
tests/scripts/run_tests.sh quick

# Test specific feature
tests/scripts/run_tests.sh memory

# Debug failing test
tests/scripts/run_tests.sh --debug --memory 256M exceptions
```

### CI/CD Pipeline

```bash
# Fast verification
tests/scripts/quick_test.sh

# Comprehensive test
tests/scripts/run_tests.sh all

# Memory validation
tests/scripts/run_tests.sh --all-memory
```

### Memory Allocator Development

```bash
# Test with different RAM sizes
tests/scripts/run_tests.sh --memory 64M memory
tests/scripts/run_tests.sh --memory 512M memory
tests/scripts/run_tests.sh --memory 2G memory

# Or test all at once
tests/scripts/test_memory_sizes.sh
```

## Troubleshooting

**Script not executable:**
```bash
chmod +x tests/scripts/script_name.sh
```

**QEMU not found:**
```bash
# Install QEMU
sudo apt install qemu-system-x86  # Ubuntu/Debian
sudo pacman -S qemu               # Arch
brew install qemu                 # macOS
```

**Tests timing out:**
- Increase timeout in script
- Check if OS is actually running (open QEMU GUI)
- Review kernel logs for hangs

**Debug output not showing:**
- Use `--verbose` flag
- Check QEMU serial output
- Run with QEMU GUI to see VGA output

## See Also

- [Testing Guide](../docs/README.md) - Complete testing documentation
- [Memory Testing](../docs/testing-memory-sizes.md) - Memory validation guide
- [Virtual Memory Tests](../docs/virtual-memory.md) - VM test documentation
- [Makefile](../../Makefile) - Build system integration
