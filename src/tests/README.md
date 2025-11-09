# NoodleOS Testing Framework

This directory contains the testing framework for NoodleOS. Tests are organized by category and enabled via Cargo features to keep production builds clean.

## Directory Structure

```
tests/
├── mod.rs           # Main test coordinator
├── exceptions.rs    # Exception handler tests
├── memory.rs        # Memory management tests (placeholder)
└── hardware.rs      # Hardware driver tests (placeholder)
```

## Test Categories

### Exception Tests (`exceptions.rs`)
Tests for CPU exception handlers including:
- Divide by zero exception (#DE)
- Invalid opcode exception (#UD) - future
- Breakpoint exception (#BP) - future

**Features:**
- `test-exceptions` - Enable exception test framework
- `test-divide-by-zero` - Enable divide by zero test

### Memory Tests (`memory.rs`)
Tests for memory management functionality:
- Physical memory allocator (bitmap implementation)
- Frame allocation/deallocation
- Memory statistics

**Features:**
- `test-memory` - Enable memory allocator tests

### Hardware Tests (`hardware.rs`)
Tests for hardware drivers (placeholder for future):
- VGA buffer operations
- Keyboard input
- Timer functionality

**Features:**
- `test-hardware` - Enable hardware tests (future)

## Usage

The test framework uses a two-level feature system:
1. `run-tests` - Enables the test module in the kernel
2. `test-<category>` - Enables specific test categories

### Build Commands
```bash
# Build with memory allocator tests
cargo build --release --target x86_64-unknown-none --features run-tests,test-memory

# Build with exception tests
cargo build --release --target x86_64-unknown-none --features run-tests,test-exceptions

# Build with multiple test categories
cargo build --release --target x86_64-unknown-none --features run-tests,test-exceptions,test-memory

# After building, create ISO and run
make iso
qemu-system-x86_64 -cdrom noodleos.iso
```

### Why Two Features?
- `run-tests` keeps the test module out of production builds
- Individual test categories let you control what runs
- Cleaner separation between test infrastructure and test content

## Adding New Tests

### 1. Add to Existing Category
Add new test functions to the appropriate module (exceptions.rs, memory.rs, hardware.rs).

### 2. Add New Category
1. Create new module file in `tests/`
2. Add module declaration to `tests/mod.rs`
3. Add feature to `Cargo.toml`
4. Add feature check to `run_all_tests()` in `tests/mod.rs`

### 3. Test Function Template
```rust
/// Test description
pub fn test_name() {
    use crate::arch::drivers::vga::println;
    
    println("=== TEST: Description ===");
    // Test implementation
    println("=== TEST COMPLETE ===");
}
```

## Safety Note

Exception tests use `unsafe` code and inline assembly to trigger real CPU exceptions. These tests will cause the system to halt when the exception occurs. Only run these tests in development environments.
