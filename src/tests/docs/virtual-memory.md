# Virtual Memory Testing Details

Detailed information about virtual memory system tests.

**📚 Main Documentation:**
- [Testing Guide](README.md) - Complete testing documentation
- [Virtual Memory Architecture](../../../docs/virtual-memory.md) - Virtual memory system design

---

This document describes how to test the virtual memory management system in NoodleOS.

## Running Tests

### Quick Test with Script

```bash
# Run virtual memory tests (default)
./test_vm.sh

# Or specify category
./test_vm.sh virtual    # Virtual memory only
./test_vm.sh memory     # All memory tests
./test_vm.sh exceptions # Exception tests
```

### Build and Run All Memory Tests

```bash
# Build with all memory tests
make test-memory

# Run in QEMU
make run
```

### Build and Run Virtual Memory Tests Only

```bash
# Build with only virtual memory tests
make test-virtual-memory

# Run in QEMU
make run
```

### Available Test Features

The following test features are available:

- `run-tests` - Enable the test framework (required)
- `test-memory` - Run all memory tests (physical + virtual)
- `test-virtual-memory` - Run only virtual memory tests
- `test-exceptions` - Run exception handling tests
- `test-hardware` - Run hardware driver tests

Build specific test categories:
```bash
make test-memory          # Physical + virtual memory
make test-virtual-memory  # Virtual memory only  
make test-exceptions      # Exception handlers
```

## Test Coverage

### Virtual Memory Tests

The virtual memory test suite (`test_virtual_memory()`) includes:

#### Test 1: Virtual Address Operations
- ✓ Lower canonical address handling
- ✓ Upper canonical address handling
- ✓ Address alignment down
- ✓ Address alignment up
- ✓ Page offset extraction

#### Test 2: Physical Address Operations
- ✓ Physical address creation
- ✓ Physical address alignment
- ✓ 4KB alignment checking

#### Test 3: Page and Frame Operations
- ✓ Page from virtual address
- ✓ Frame from physical address
- ✓ Page number calculation
- ✓ Frame number calculation

#### Test 4: Page Table Flags
- ✓ Flag creation and testing
- ✓ Flag combination with union
- ✓ Flag bits extraction
- ✓ Empty flags

#### Test 5: Page Table Entries
- ✓ New entry is unused
- ✓ Set entry address and flags
- ✓ Entry frame extraction
- ✓ Clear entry

#### Test 6: Page Table Structure
- ✓ New page table is zeroed
- ✓ Page table indexing
- ✓ Page table iteration (512 entries)

#### Test 7: Address Translation Indices
- ✓ PML4 index extraction
- ✓ PDPT index extraction
- ✓ PD index extraction
- ✓ PT index extraction

#### Test 8: CR3 Register Access
- ✓ Read CR3 register
- ✓ Verify PML4 address is page-aligned

### Physical Memory Tests

The physical memory test suite (`test_physical_allocator()`) includes:

- ✓ Initial state verification
- ✓ Single frame allocation
- ✓ Multiple frame allocation
- ✓ Free count tracking
- ✓ Contiguous frame allocation
- ✓ Frame deallocation
- ✓ Final state verification

## Test Output

When tests run successfully, you should see output similar to:

```
=== NOODLEOS TEST SUITE ===

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

Test 3: Page and Frame Operations
  3a. Page from virtual address... OK
  3b. Frame from physical address... OK
  3c. Page number calculation... OK
  3d. Frame number calculation... OK

Test 4: Page Table Flags
  4a. Flag creation and testing... OK
  4b. Flag combination with union... OK
  4c. Flag bits extraction... OK
  4d. Empty flags... OK

Test 5: Page Table Entries
  5a. New entry is unused... OK
  5b. Set entry address and flags... OK
  5c. Entry frame extraction... OK
  5d. Clear entry... OK

Test 6: Page Table Structure
  6a. New page table is zeroed... OK
  6b. Page table indexing... OK
  6c. Page table iteration... OK

Test 7: Address Translation Indices
  7a. PML4 index extraction... OK
  7b. PDPT index extraction... OK
  7c. PD index extraction... OK
  7d. PT index extraction... OK

=== Virtual Memory Tests Complete ===

Test 8: CR3 Register Access
  8a. Read CR3 register... OK (PML4 at 0x...)

=== MEMORY TESTS COMPLETE ===
```

## Adding New Tests

To add new virtual memory tests:

1. Add your test function to `src/arch/x86_64/memory/tests.rs`:
```rust
fn test_my_feature() {
    println("Test X: My Feature");
    
    print("  Xa. First check... ");
    // Test code here
    if check_passed {
        println("OK");
    } else {
        println("FAILED");
    }
}
```

2. Call it from `test_virtual_memory()`:
```rust
pub fn test_virtual_memory() {
    println("=== Testing Virtual Memory System ===");
    
    test_virtual_addresses();
    test_physical_addresses();
    // ... existing tests ...
    test_my_feature();  // Add your test here
    
    println("=== Virtual Memory Tests Complete ===");
}
```

3. Rebuild and run:
```bash
make test-virtual-memory
make run
```

## Test Patterns

### Testing Success Conditions

```rust
print("  Test description... ");
if condition_is_true {
    println("OK");
} else {
    println("FAILED");
}
```

### Testing with Value Display

```rust
print("  Test description... ");
if result == expected {
    print("OK (value: ");
    print_decimal(result as u64);
    println(")");
} else {
    print("FAILED (expected ");
    print_decimal(expected as u64);
    print(", got ");
    print_decimal(result as u64);
    println(")");
}
```

### Testing Multiple Conditions

```rust
print("  Test description... ");
if condition1 && condition2 && condition3 {
    println("OK");
} else {
    println("FAILED");
}
```

## Debugging Failed Tests

If a test fails:

1. **Check the output** - The test will show which specific check failed
2. **Use QEMU debugging** - Run with `-s -S` flags to attach GDB
3. **Add more detailed output** - Use `print_hex()` and `print_decimal()` helpers
4. **Check assumptions** - Verify that prerequisites are met

## Testing in CI/CD

To integrate tests in automated builds:

```bash
# Build and test script
#!/bin/bash
set -e

# Build with tests
./build.sh run-tests,test-virtual-memory

# Run in QEMU with timeout
timeout 30s qemu-system-x86_64 \
    -cdrom noodleos.iso \
    -serial stdio \
    -nographic \
    -no-reboot

# Check exit code
if [ $? -eq 0 ]; then
    echo "Tests passed!"
else
    echo "Tests failed or timed out"
    exit 1
fi
```

## Performance Considerations

Tests are designed to be fast and non-destructive:

- No actual page table modifications are made to the active tables
- Tests use stack-allocated structures when possible
- Memory allocations are minimal and cleaned up
- CR3 is only read, never written

## Future Test Enhancements

Planned test additions:

- [ ] Test page table traversal with mock tables
- [ ] Test mapper with controlled allocator
- [ ] Test TLB flush operations
- [ ] Test huge page detection
- [ ] Test page fault scenarios (with handler)
- [ ] Stress test with many allocations
- [ ] Test edge cases (null addresses, misaligned, etc.)

## References

- Main test module: `src/tests/memory.rs`
- Virtual memory tests: `src/arch/x86_64/memory/tests.rs`
- Test framework: `src/tests/mod.rs`
- Build system: `build.sh`, `Makefile`
