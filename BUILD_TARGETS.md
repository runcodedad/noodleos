# NoodleOS Build Targets Quick Reference

## Standard Build & Run
```bash
make           # Build kernel
make run       # Build and run in QEMU
make debug     # Build and run with debugging (-s -S)
```

## Testing Targets

### Build Only (for CI/validation)
```bash
make test-exceptions        # Build with exception tests
make test-divide-by-zero   # Build with divide by zero test
```

### Build and Run (for interactive testing)
```bash
make run-test-exceptions        # Build exception tests and run in QEMU
make run-test-divide-by-zero   # Build divide by zero test and run in QEMU
```

### Build and Debug (for development)
```bash
make debug-test-exceptions        # Build exception tests and run with debugger
make debug-test-divide-by-zero   # Build divide by zero test and run with debugger
```

## Maintenance
```bash
make clean      # Remove build artifacts
make distclean  # Remove everything including target/
make test       # Quick build verification
```

## Common Workflows

### Testing Exception Handling
```bash
# Test specific exception
make run-test-divide-by-zero

# Debug exception handler
make debug-test-divide-by-zero
# In another terminal: gdb target/x86_64-unknown-none/release/noodleos
# (gdb) target remote :1234
```

### Adding New Tests
```bash
# 1. Add test target to Makefile (ONE line):
test-page-fault: $(BOOT_DEPS)
	$(call build_test_kernel,test-exceptions,test-page-fault)

# 2. Run/debug targets work automatically:
make run-test-page-fault    # ✅ Auto-generated
make debug-test-page-fault  # ✅ Auto-generated
```

### Normal Development
```bash
# Regular kernel development
make run

# Quick verification after changes
make test
```

### CI/Automated Testing
```bash
# Verify all builds work
make clean
make test-exceptions
make test-divide-by-zero
make all
```

## Important Notes

⚠️ **Don't mix build types**: 
- `make test-divide-by-zero && make run` will NOT test - `make run` rebuilds without test features
- Use `make run-test-divide-by-zero` instead

✅ **Correct test workflow**:
- Use `run-test-*` targets for interactive testing
- Use `test-*` targets for build validation only
- Use `debug-test-*` targets for development debugging
