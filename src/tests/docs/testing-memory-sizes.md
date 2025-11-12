# Testing Memory Allocator with Different RAM Sizes

This document explains how to validate the physical memory allocator with different RAM configurations in QEMU.

## Quick Reference

### Run with Custom Memory Size

```bash
# Using make variable
make run QEMU_MEMORY=256M

# Using dedicated test targets
make test-mem-64m
make test-mem-128m
make test-mem-256m
make test-mem-512m
make test-mem-1g
make test-mem-2g
```

### Run All Memory Size Tests

```bash
tests/scripts/test_memory_sizes.sh
# Or use the main test runner:
tests/scripts/run_tests.sh --all-memory
```

## Understanding Memory Statistics

After fixing the allocator (commit: calculating total from available memory, not highest address), the memory statistics should now correctly reflect:

### Example Output with 128MB RAM

```
Total frames:     32768
Free frames:      ~32000 (127 MB)
Allocated frames: ~768 (3 MB)
```

### What Each Value Means

- **Total frames**: Total usable RAM / 4KB
  - Calculated by summing all "Available" memory regions from multiboot memory map
  - Does NOT include reserved regions, PCI holes, or address space gaps
  
- **Free frames**: RAM available for allocation
  - Should be close to total minus kernel and bitmap overhead
  
- **Allocated frames**: RAM currently in use
  - Includes kernel code/data, bitmap storage, and any allocations
  - Should be a small percentage of total (typically < 5% for idle kernel)

## Expected Values for Different RAM Sizes

| RAM Size | Total Frames | Expected Free | Expected Allocated |
|----------|-------------|---------------|-------------------|
| 64M      | ~16,384     | ~16,000       | ~384              |
| 128M     | ~32,768     | ~32,000       | ~768              |
| 256M     | ~65,536     | ~64,000       | ~1,536            |
| 512M     | ~131,072    | ~128,000      | ~3,072            |
| 1G       | ~262,144    | ~256,000      | ~6,144            |
| 2G       | ~524,288    | ~512,000      | ~12,288           |

Note: Actual values may vary slightly based on:
- Kernel size (depends on code compiled in)
- Bitmap size (grows with total RAM)
- Memory map fragmentation from bootloader

## The Bug We Fixed

### Before (Incorrect)

```rust
// Found HIGHEST address in memory map (included holes!)
let mut highest_addr = 0u64;
for entry in mmap {
    let end_addr = entry.base_addr + entry.length;
    if end_addr > highest_addr {
        highest_addr = end_addr;
    }
}
self.total_frames = (highest_addr as usize) / PAGE_SIZE;
```

**Problem**: With 128MB RAM, QEMU's memory map includes entries up to ~13GB for PCI holes and reserved regions. This made `total_frames` = 3.15 million (~12GB), showing 12GB "allocated" when we only had 128MB!

### After (Correct)

```rust
// Sum only AVAILABLE memory regions
let mut total_available = 0u64;
for entry in mmap {
    if MemoryType::from_u32(entry.mem_type) == Some(MemoryType::Available) {
        total_available += entry.length;  // Sum lengths!
    }
}
self.total_frames = (total_available as usize) / PAGE_SIZE;
```

**Solution**: Only count memory marked as "Available" by summing the lengths of available regions, not their addresses. Now `total_frames` correctly represents actual usable RAM.

## Manual Testing

To visually verify the fix:

1. Build and run with default memory:
   ```bash
   make run
   ```

2. Check the VGA output shows reasonable values:
   - Total should match QEMU's `-m` value
   - Free should be close to total
   - Allocated should be small (~1-3 MB)

3. Test with different sizes:
   ```bash
   make test-mem-64m   # Should show ~64MB total
   make test-mem-256m  # Should show ~256MB total
   make test-mem-1g    # Should show ~1GB total
   ```

## Automated Testing

The `tests/scripts/test_memory_sizes.sh` script runs the OS with multiple RAM sizes. While it can't easily capture VGA output, you can use it to verify the OS boots successfully with each configuration.

You can also use the main test runner:

```bash
# Test all memory sizes
tests/scripts/run_tests.sh --all-memory

# Test specific memory size with any test
tests/scripts/run_tests.sh --memory 512M memory

# Run with debug mode
tests/scripts/run_tests.sh --debug --memory 256M exceptions
```

## Debugging Memory Issues

If you see unexpected values:

1. **Total is much larger than RAM**: The old bug - counting address space instead of available memory
2. **Free is much smaller than expected**: Check for memory leaks or incorrect reservations
3. **Allocated grows over time**: Memory leak in allocator
4. **Total is zero or very small**: Memory map not being parsed correctly

### Enable Debug Output

To add detailed memory map debugging, edit `src/arch/x86_64/memory/physical.rs`:

```rust
if let Some(mmap) = boot_info.memory_map() {
    for entry in mmap {
        // Add this debugging output
        println!("Memory region: ");
        println!("  Base: 0x");
        print_hex(entry.base_addr);
        println!("  Length: 0x");
        print_hex(entry.length);
        println!("  Type: ");
        println!(MemoryType::from_u32(entry.mem_type).unwrap().as_str());
        println!("");
        
        if MemoryType::from_u32(entry.mem_type) == Some(MemoryType::Available) {
            total_available += entry.length;
        }
    }
}
```

## QEMU Memory Layout

QEMU's x86_64 emulation creates a memory map similar to physical hardware:

- **0x0 - 0x9FFFF**: Low memory (640KB)
- **0xA0000 - 0xFFFFF**: VGA/BIOS (384KB) - Reserved
- **0x100000 - RAM_END**: Main RAM (your `-m` value)
- **0xE0000000 - 0xFFFFFFFF**: PCI hole - Reserved
- **Above 4GB**: Additional RAM (if > 4GB configured)

The allocator now only counts the "Main RAM" regions as available, ignoring the reserved areas.

## See Also

- [Memory Allocator Documentation](memory_allocator.md)
- [Virtual Memory Documentation](virtual-memory.md)
- [Build System](build-system.md)
