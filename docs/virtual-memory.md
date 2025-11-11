# Virtual Memory Management

This document describes the virtual memory management system for NoodleOS, including page tables, address translation, and memory mapping.

## Overview

The x86_64 architecture uses a 4-level paging scheme to translate virtual addresses to physical addresses. Our virtual memory manager provides:

- **Page table structures** - Data structures representing the 4-level hierarchy
- **Address types** - Type-safe virtual and physical addresses
- **Memory mapping** - Functions to map virtual pages to physical frames
- **TLB management** - Functions to flush the Translation Lookaside Buffer
- **Frame allocation** - Interface to allocate physical memory for page tables

## x86_64 Paging Architecture

### 4-Level Page Table Hierarchy

x86_64 uses four levels of page tables:

1. **PML4 (Page Map Level 4)** - Level 4 (top level)
   - 512 entries, each covering 512 GB of virtual address space
   - Total coverage: 256 TB

2. **PDPT (Page Directory Pointer Table)** - Level 3
   - 512 entries, each covering 1 GB
   - Can map 1 GB huge pages (if supported)

3. **PD (Page Directory)** - Level 2
   - 512 entries, each covering 2 MB
   - Can map 2 MB huge pages

4. **PT (Page Table)** - Level 1 (final level)
   - 512 entries, each mapping a 4 KB page
   - Standard page size

### Virtual Address Translation

A 64-bit virtual address is divided into:

```
Bits 63-48: Sign extension (must match bit 47 for canonical addresses)
Bits 47-39: PML4 index (9 bits, 512 entries)
Bits 38-30: PDPT index (9 bits, 512 entries)
Bits 29-21: PD index (9 bits, 512 entries)
Bits 20-12: PT index (9 bits, 512 entries)
Bits 11-0:  Page offset (12 bits, 4096 bytes)
```

The CPU uses the CR3 register to locate the PML4 table, then walks through the hierarchy to find the physical frame.

### Canonical Addresses

x86_64 requires addresses to be "canonical" - bits 48-63 must be copies of bit 47:
- Lower half: `0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`
- Upper half: `0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF`

Addresses between these ranges cause a General Protection Fault.

## Core Types

### Page Table Entry

A `PageTableEntry` is a 64-bit value containing:
- **Address bits (12-51)**: Physical address of next table or frame (4KB aligned)
- **Flag bits (0-11, 63)**: Control bits for the mapping

Available flags:
```rust
PRESENT         // Page is in memory
WRITABLE        // Page can be written to
USER_ACCESSIBLE // Page is accessible from user mode
WRITE_THROUGH   // Write-through caching
NO_CACHE        // Disable caching
ACCESSED        // Page has been accessed (set by CPU)
DIRTY           // Page has been modified (set by CPU)
HUGE_PAGE       // Maps a huge page (2MB/1GB)
GLOBAL          // Don't flush from TLB on CR3 reload
NO_EXECUTE      // Prevent code execution (NX bit)
```

### Address Types

#### `PhysAddr`
Represents a physical memory address. Can be any value up to the physical address space limit.

#### `VirtAddr`
Represents a virtual memory address. Must be canonical (see above).

#### `PhysFrame`
Represents a 4KB-aligned physical memory frame. Contains a `PhysAddr` aligned to 4KB.

#### `Page`
Represents a 4KB-aligned virtual memory page. Contains a `VirtAddr` aligned to 4KB.

### Page Table

A `PageTable` contains 512 entries and is 4KB in size (page-aligned). Each level of the hierarchy uses this same structure.

## Mapping Virtual Memory

### The Mapper

The `Mapper` type provides methods to create and manage virtual memory mappings:

```rust
use noodleos::arch::x86_64::memory::*;

// Create a mapper (requires a PML4 table and frame allocator)
let mut mapper = unsafe { Mapper::new(pml4_table, frame_allocator) };

// Map a page to a specific frame
let page = Page::containing_address(VirtAddr::new(0x1000));
let frame = PhysFrame::containing_address(PhysAddr::new(0x200000));
let flags = PageTableFlags::PRESENT
    .union(PageTableFlags::WRITABLE);
mapper.map_to(page, frame, flags)?;

// Map a page (allocate frame automatically)
let page = Page::containing_address(VirtAddr::new(0x2000));
let flags = PageTableFlags::PRESENT
    .union(PageTableFlags::WRITABLE);
let frame = mapper.map(page, flags)?;

// Unmap a page
let frame = mapper.unmap(page)?;

// Translate virtual to physical address
if let Some(phys_addr) = mapper.translate(VirtAddr::new(0x1000)) {
    println!("Virtual 0x1000 maps to physical {:#x}", phys_addr.as_u64());
}
```

### Frame Allocation

The `FrameAllocator` trait provides physical memory frames for new page tables:

```rust
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Result<PhysFrame, FrameAllocError>;
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame);
}
```

Our `BitmapFrameAllocator` implements this trait using the physical memory bitmap allocator.

## TLB Management

The Translation Lookaside Buffer (TLB) caches address translations. After changing page tables, you must flush the TLB:

```rust
// Flush a single page
flush_page(VirtAddr::new(0x1000));

// Flush all non-global pages
flush_all();
```

The TLB is also flushed automatically by:
- Writing to CR3 (loads new page table)
- The `invlpg` instruction (single page)

## Implementation Details

### Creating Page Tables

When mapping a new page, the mapper traverses the 4-level hierarchy. If an intermediate table doesn't exist, it:

1. Allocates a physical frame from the frame allocator
2. Zeros the new table
3. Sets the parent entry to point to it with appropriate flags
4. Continues traversal

### Safety Considerations

- Page tables must be 4KB-aligned and accessible
- The active PML4 must remain valid while in use
- TLB must be flushed after modifying entries
- Physical frames must not be deallocated while mapped
- Raw pointers are used carefully with proper safety checks

### Memory Layout

Our kernel uses the following virtual memory layout (to be established):

```
0x0000_0000_0000_0000 - User space start
0x0000_7FFF_FFFF_FFFF - User space end (128 TB)

0xFFFF_8000_0000_0000 - Kernel space start
0xFFFF_8000_0010_0000 - Kernel text/data
0xFFFF_8000_????_???? - Kernel heap
0xFFFF_8800_0000_0000 - Physical memory mapping (direct map)
0xFFFF_FFFF_FFFF_FFFF - Top of address space
```

## Usage Example

Here's a complete example of setting up virtual memory:

```rust
use noodleos::arch::x86_64::memory::*;

// Get the current PML4 table
let pml4_addr = read_cr3();
let pml4 = unsafe { &mut *(pml4_addr.as_u64() as *mut PageTable) };

// Create a frame allocator
let mut frame_allocator = BitmapFrameAllocator::new();

// Create a mapper
let mut mapper = unsafe { Mapper::new(pml4, frame_allocator) };

// Map some memory for the kernel heap
let heap_start = VirtAddr::new(0xFFFF_8000_1000_0000);
let heap_pages = 256; // 1 MB heap

for i in 0..heap_pages {
    let page = Page::containing_address(
        VirtAddr::new(heap_start.as_u64() + i * 4096)
    );
    
    let flags = PageTableFlags::PRESENT
        .union(PageTableFlags::WRITABLE)
        .union(PageTableFlags::NO_EXECUTE);
    
    mapper.map(page, flags)
        .expect("Failed to map heap page");
}
```

## Future Enhancements

Potential improvements to the virtual memory system:

1. **Copy-on-Write (COW)** - Share pages between processes
2. **Demand Paging** - Allocate pages on first access
3. **Huge Pages** - Support for 2MB and 1GB pages
4. **Page Reclamation** - Swap pages to disk when memory is low
5. **NUMA Support** - Allocate memory close to the CPU
6. **Memory Protection Keys** - Fine-grained access control
7. **Address Space IDs** - Avoid TLB flushes on context switch

## Testing

Test the virtual memory system with:

```bash
# Build the kernel
make build

# Run in QEMU
make run

# Run tests
cargo test --target x86_64-unknown-none
```

## References

- [Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 3](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) - Chapter 4: Paging
- [AMD64 Architecture Programmer's Manual Volume 2](https://www.amd.com/en/support/tech-docs) - Chapter 5: Page Translation and Protection
- [OSDev Wiki: Paging](https://wiki.osdev.org/Paging)
- [Writing an OS in Rust: Paging](https://os.phil-opp.com/paging-introduction/)
