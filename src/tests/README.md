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
Tests for memory management functionality (placeholder for future):
- Memory allocation
- Page table operations

**Features:**
- `test-memory` - Enable memory tests (future)

### Hardware Tests (`hardware.rs`)
Tests for hardware drivers (placeholder for future):
- VGA buffer operations
- Keyboard input
- Timer functionality

**Features:**
- `test-hardware` - Enable hardware tests (future)

## Usage

### Build Commands
```bash
# Build with exception tests
make test-exceptions
cargo build --features test-exceptions

# Build with specific exception test
make test-divide-by-zero
cargo build --features test-exceptions,test-divide-by-zero

# Build with all test categories (future)
cargo build --features test-exceptions,test-memory,test-hardware
```

### Development Script
```bash
./dev.sh test-exceptions      # Build with exception test framework
./dev.sh test-divide-zero     # Build with divide by zero test
```

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
