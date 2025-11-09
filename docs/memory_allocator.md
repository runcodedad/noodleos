# Physical Memory Allocator

## Overview

The NoodleOS physical memory allocator uses a bitmap-based approach to track and allocate 4KB physical memory frames. This is a simple, efficient, and easy-to-understand allocator that serves as a solid foundation for the kernel's memory management system.

**Implementation**: `src/arch/x86_64/memory/physical.rs`

## Design

### Bitmap Approach

Each bit in the bitmap represents one 4KB physical frame:
- **0** = frame is free (available for allocation)
- **1** = frame is allocated or reserved

The bitmap itself is stored in physical memory, placed immediately after the kernel in memory.

### Key Features

1. **Simple and Efficient**: O(n) allocation time with first-fit strategy, but can be improved
2. **Space Efficient**: Only 1 bit per 4KB frame (128 KB bitmap for 4 GB RAM)
3. **Atomic Operations**: Uses `AtomicUsize` for thread-safe free frame counting
4. **Multiboot2 Integration**: Initializes from GRUB's memory map
5. **Contiguous Allocation Support**: Can allocate multiple contiguous frames

### Memory Layout

```
+------------------+
| Kernel Code      |  <- Protected (marked as reserved)
+------------------+
| Kernel Data/BSS  |
+------------------+
| Bitmap           |  <- Allocator's bitmap (marked as reserved)
+------------------+
| Available Memory |  <- Managed by allocator
+------------------+
```

## API

### Initialization

```rust
unsafe fn init_physical_allocator(
    boot_info: &BootInfo,
    kernel_start: usize,
    kernel_end: usize
)
```

Initializes the allocator during kernel boot. Must be called exactly once.

### Allocation

```rust
fn allocate_frame() -> Option<usize>
```
Allocates a single 4KB frame. Returns the physical address or `None` if no memory is available.

```rust
fn allocate_frames(count: usize) -> Option<usize>
```
Allocates multiple contiguous frames. Returns the base physical address or `None`.

### Deallocation

```rust
unsafe fn free_frame(phys_addr: usize)
```
Frees a previously allocated frame.

```rust
unsafe fn free_frames(phys_addr: usize, count: usize)
```
Frees multiple contiguous frames.

### Statistics

```rust
fn memory_stats() -> (usize, usize, usize)
```
Returns `(total_frames, free_frames, allocated_frames)`.

## Implementation Details

### First-Fit Allocation

The allocator uses a first-fit strategy with a hint optimization:
- Maintains a `start_frame` hint pointing to where the last allocation occurred
- Starts searching from the hint, wrapping around if necessary
- This improves allocation performance in common cases

### Safety Considerations

1. **Kernel Protection**: Kernel memory is marked as reserved during initialization
2. **Bitmap Protection**: The bitmap itself is marked as reserved to prevent self-corruption
3. **Bounds Checking**: All frame indices are validated before access
4. **Unsafe Blocks**: Deallocation requires `unsafe` because the caller must ensure the frame is no longer in use

### Current Limitations

This is designed as a foundational allocator that can be improved or replaced later:

1. **No Fragmentation Handling**: Large contiguous allocations may fail even if total free memory is sufficient
2. **Linear Search**: O(n) time complexity for allocation
3. **Fixed Maximum**: Limited to 16 GB of physical memory (configurable)
4. **No NUMA Awareness**: Treats all memory equally

## Future Improvements

The allocator is designed to be easily extended or replaced:

### Possible Enhancements

1. **Buddy Allocator**: Better handling of different-sized allocations and reduced fragmentation
2. **Free Lists**: Multiple free lists for different frame counts (fast path for common sizes)
3. **Bitmap Tree**: Hierarchical bitmap for O(log n) allocation
4. **Zone Allocator**: Separate zones for DMA, normal, and high memory
5. **Per-CPU Caches**: Thread-local frame caches to reduce contention
6. **NUMA Support**: Prefer allocating from local memory nodes

### Migration Path

To replace this allocator:

1. Implement new allocator with the same API
2. Update `physical::init_physical_allocator()` to use new implementation
3. Test thoroughly with `--features test-memory`
4. No changes needed in calling code

## Usage Examples

### Basic Allocation

```rust
use crate::arch::x86_64::memory::physical::{allocate_frame, free_frame};

// Allocate a single 4KB frame
if let Some(phys_addr) = allocate_frame() {
    println!("Allocated frame at: 0x{:X}", phys_addr);
    
    // Use the frame...
    
    // Free when done
    unsafe {
        free_frame(phys_addr);
    }
}
```

### Contiguous Allocation

```rust
use crate::arch::x86_64::memory::physical::{allocate_frames, free_frames};

// Allocate 16 contiguous frames (64 KB)
if let Some(base_addr) = allocate_frames(16) {
    // Use for page tables, kernel stacks, DMA buffers, etc.
    
    unsafe {
        free_frames(base_addr, 16);
    }
}
```

### Memory Statistics

```rust
use crate::arch::x86_64::memory::physical::memory_stats;

let (total, free, allocated) = memory_stats();
println!("Memory: {} total, {} free, {} allocated", total, free, allocated);
```

## Testing

Run memory allocator tests with:

```bash
cargo build --release --target x86_64-unknown-none --features test-memory
# Then rebuild ISO and run in QEMU
```

The test suite (`src/arch/x86_64/memory/tests.rs`) exercises:
- Single frame allocation/deallocation
- Multiple frame operations
- Free frame counting
- Basic allocator state verification

## References

- [OSDev Wiki - Page Frame Allocation](https://wiki.osdev.org/Page_Frame_Allocation)
- [Multiboot2 Specification](https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html)
- [Writing an OS in Rust - Memory Management](https://os.phil-opp.com/allocator-designs/)
