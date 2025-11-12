# NoodleOS Testing Guide

Complete guide for running and understanding tests in NoodleOS.

## Navigation

- [Quick Start](#quick-start) - Get started running tests
- [Test Categories](#test-categories) - All available test types
- [Build System](#build-system-integration) - Makefile targets and features
- [Test Implementation](#test-implementation) - Code structure and patterns
- [Adding Tests](#adding-new-tests) - How to write new tests
- [Debugging](#debugging-failed-tests) - Troubleshooting and tools
- [CI/CD](#cicd-integration) - Automated testing

## Related Documentation

- [Test Framework Overview](../README.md) - Module-level documentation
- [Virtual Memory Architecture](../../../docs/virtual-memory.md) - VM system design
- [Memory Allocator](../../../docs/memory_allocator.md) - Physical allocator
- [Main README](../../../README.md) - Project overview

## Quick Start

### Using the Test Runner (Recommended)

The easiest way to run tests is using the unified test runner:

```bash
# Quick boot test
tests/scripts/run_tests.sh quick

# Run specific test category
tests/scripts/run_tests.sh exceptions
tests/scripts/run_tests.sh memory
tests/scripts/run_tests.sh virtual-memory
tests/scripts/run_tests.sh hardware

# Test with different memory sizes
tests/scripts/run_tests.sh --memory 256M memory
tests/scripts/run_tests.sh --memory 1G virtual-memory

# Test all memory configurations
tests/scripts/run_tests.sh --all-memory

# Debug mode
tests/scripts/run_tests.sh --debug exceptions

# Show all options
tests/scripts/run_tests.sh --help
```

### Using Legacy Test Scripts

Individual test scripts are also available:

```bash
# Quick boot test
tests/scripts/quick_test.sh

# Virtual memory tests
tests/scripts/test_vm.sh virtual

# Memory size validation
tests/scripts/test_memory_sizes.sh
```

### Using Make Targets Directly

You can also use Make directly:

```bash
# Build and run tests
make run-test-exceptions
make run-test-memory
make run-test-virtual-memory
make run-test-hardware

# Test with specific memory size
make run-test-memory QEMU_MEMORY=512M

# Debug tests
make debug-test-exceptions
```

### List All Available Tests

```bash
make list-tests
```

### Quick Reference

| Test Category | Test Count | Command |
|--------------|------------|---------|
| Virtual Memory | 28 | `./test_vm.sh virtual` |
| Physical Memory | 7 | Included in `./test_vm.sh memory` |
| All Memory | 35 | `./test_vm.sh memory` |
| Exceptions | 2 | `./test_vm.sh exceptions` |
| Divide-by-Zero | 1 | `make test-divide-by-zero && make run` |

## Test Categories

### 1. Virtual Memory Tests

**Feature Flag:** `test-virtual-memory`

Tests the virtual memory management system including page tables, address translation, and mapping.

**What's Tested:**
- Virtual address operations (canonical addresses, alignment, offsets)
- Physical address operations (creation, alignment, validation)
- Page and frame operations (creation, indexing, numbering)
- Page table flags (creation, combination, testing)
- Page table entries (setting, clearing, frame extraction)
- Page table structure (indexing, iteration, initialization)
- Address translation indices (PML4, PDPT, PD, PT)
- CR3 register access

**Run:**
```bash
./test_vm.sh virtual
# or
make test-virtual-memory
make run
```

**Test Count:** 28 individual test cases

### 2. Physical Memory Tests

**Feature Flag:** `test-memory`

Tests the physical memory allocator (bitmap-based) and includes virtual memory tests.

**What's Tested:**
- Frame allocation (single and multiple)
- Frame deallocation
- Memory statistics tracking
- Contiguous frame allocation
- All virtual memory tests

**Run:**
```bash
./test_vm.sh memory
# or
make test-memory
make run
```

### 3. Exception Tests

**Feature Flag:** `test-exceptions`

Tests CPU exception handlers and the Interrupt Descriptor Table (IDT).

**What's Tested:**
- IDT initialization
- Exception handler registration
- Basic exception handling flow

**Run:**
```bash
./test_vm.sh exceptions
# or
make test-exceptions
make run
```

#### 3a. Divide by Zero Test

**Feature Flag:** `test-divide-by-zero`

Actually triggers a divide-by-zero exception to test the handler.

**Run:**
```bash
make test-divide-by-zero
make run
```

### 4. Hardware Tests (Future)

**Feature Flag:** `test-hardware`

Placeholder for future hardware driver tests.

**Planned Tests:**
- VGA buffer operations
- Keyboard input handling
- Timer functionality
- Serial port communication

## Test Output

### Successful Test Output

When tests pass, you'll see output like:

```
=== Testing Virtual Memory System ===
Test 1: Virtual Address Operations
  1a. Lower canonical address... OK
  1b. Upper canonical address... OK
  1c. Address alignment down... OK
  1d. Address alignment up... OK
  1e. Page offset extraction... OK

Test 2: Physical Address Operations
  2a. Physical address creation... OK
  2b. Physical address alignment... OK
  2c. Check 4KB alignment... OK
...
=== Virtual Memory Tests Complete ===
```

### Failed Test Output

If a test fails, you'll see:

```
Test 3: Page and Frame Operations
  3a. Page from virtual address... FAILED (got 0x1234, expected 0x1000)
```

## Build System Integration

### Makefile Targets

The Makefile provides these test-related targets:

| Target | Description |
|--------|-------------|
| `make list-tests` | List all available test targets |
| `make test-exceptions` | Build with exception tests |
| `make test-divide-by-zero` | Build with divide-by-zero test |
| `make test-memory` | Build with memory tests |
| `make test-virtual-memory` | Build with virtual memory tests |
| `make test-hardware` | Build with hardware tests |
| `make run-test-*` | Build and immediately run test |
| `make debug-test-*` | Build and run test with GDB |

### Feature Flags

Tests are controlled by Cargo feature flags defined in `Cargo.toml`:

| Flag | Purpose |
|------|---------|
| `run-tests` | Enable test framework (required) |
| `test-exceptions` | Enable exception tests |
| `test-divide-by-zero` | Enable divide-by-zero test |
| `test-memory` | Enable memory tests |
| `test-virtual-memory` | Enable virtual memory tests |
| `test-hardware` | Enable hardware tests |

## Test Implementation

### Test File Structure

```
src/tests/
├── docs/
│   ├── README.md              # This file
│   └── virtual-memory.md      # Virtual memory test details
├── mod.rs                     # Test coordinator
├── exceptions.rs              # Exception tests
├── memory.rs                  # Memory test runner
└── hardware.rs               # Hardware tests (future)

src/arch/x86_64/memory/
└── tests.rs                   # Memory-specific tests
```

### Adding New Tests

To add a new test:

1. **Add test function** to appropriate test file:
```rust
fn test_my_feature() {
    println("Test X: My Feature");
    
    print("  Xa. First check... ");
    if check_passed {
        println("OK");
    } else {
        println("FAILED");
    }
}
```

2. **Call from test suite:**
```rust
pub fn run_my_tests() {
    test_my_feature();
}
```

3. **Add feature flag** to `Cargo.toml`:
```toml
[features]
test-my-feature = []
```

4. **Add Makefile target:**
```makefile
test-my-feature: $(BOOT_OBJECTS)
	$(call build_test_kernel,run-tests$(COMMA)test-my-feature)
```

## Test Patterns

### Basic Test Pattern

```rust
print("  Test description... ");
if condition {
    println("OK");
} else {
    println("FAILED");
}
```

### Test with Value Display

```rust
print("  Test description... ");
if result == expected {
    print("OK (value: ");
    print_hex(result);
    println(")");
} else {
    print("FAILED (expected ");
    print_hex(expected);
    print(", got ");
    print_hex(result);
    println(")");
}
```

### Multiple Conditions

```rust
print("  Test description... ");
if cond1 && cond2 && cond3 {
    println("OK");
} else {
    println("FAILED");
}
```

## Debugging Failed Tests

### Using QEMU Monitor

When running tests, you can access QEMU monitor:

```bash
make run
# Press Ctrl+A then C for QEMU monitor
# Type 'info registers' to see CPU state
# Type 'quit' to exit
```

### Using GDB

For deep debugging:

```bash
make debug-test-virtual-memory
# In another terminal:
gdb target/x86_64-unknown-none/release/noodleos
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

### Adding Debug Output

Use helper functions in tests:

```rust
print_hex(value);      // Print hex value
print_decimal(value);  // Print decimal value
println("message");    // Print with newline
```

## CI/CD Integration

### Automated Test Script

Example script for continuous integration:

```bash
#!/bin/bash
set -e

# Run all test categories
for test in virtual-memory memory exceptions; do
    echo "Running $test tests..."
    make test-$test
    
    # Run with timeout
    timeout 30s qemu-system-x86_64 \
        -cdrom noodleos.iso \
        -serial stdio \
        -display none \
        -no-reboot
        
    if [ $? -ne 0 ]; then
        echo "Test $test failed!"
        exit 1
    fi
done

echo "All tests passed!"
```

## Performance Considerations

Tests are designed to be:

- **Fast** - Complete in seconds
- **Non-destructive** - Don't modify active system state
- **Isolated** - Each test is independent
- **Repeatable** - Same results every run

## Test Coverage Summary

| Component | Tests | Status |
|-----------|-------|--------|
| Virtual Memory | 28 | ✅ Complete |
| Physical Memory | 7 | ✅ Complete |
| Exception Handlers | 2 | ✅ Complete |
| Page Tables | Included | ✅ Complete |
| Hardware Drivers | 0 | 🔄 Planned |

## Troubleshooting

### Tests Don't Run

**Problem:** Built kernel doesn't run tests

**Solution:** Make sure you're using a test target:
```bash
make test-virtual-memory  # Not just 'make'
make run
```

### QEMU Hangs

**Problem:** QEMU doesn't exit after tests

**Solution:** Use the test script which handles this:
```bash
./test_vm.sh virtual
```

### Build Fails with Feature Errors

**Problem:** Unknown feature flag

**Solution:** Check available features:
```bash
grep '\[features\]' -A 20 Cargo.toml
```

## Additional Resources

- **Virtual Memory Tests:** `src/tests/docs/virtual-memory.md`
- **Test Framework:** `src/tests/mod.rs`
- **Memory Tests Implementation:** `src/arch/x86_64/memory/tests.rs`
- **Makefile:** See test targets and build rules
- **Main Documentation:** `docs/` directory

## Contributing Tests

When contributing new tests:

1. ✅ Follow existing test patterns
2. ✅ Provide clear test descriptions
3. ✅ Test both success and failure cases
4. ✅ Add documentation
5. ✅ Update this README
6. ✅ Keep tests fast and focused

## Questions?

If you have questions about testing:

1. Check this documentation
2. Look at existing test implementations
3. Review `src/tests/README.md`
4. Check the main project README
