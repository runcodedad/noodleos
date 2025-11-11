# Virtual Memory System

This directory contains the virtual memory management implementation for NoodleOS on x86_64.

## Overview

The virtual memory system provides:
- 4-level page table management (PML4/PDPT/PD/PT)
- Virtual to physical address translation
- Memory mapping and unmapping
- Frame allocation for page tables
- TLB (Translation Lookaside Buffer) management

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Virtual Memory System                   │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌────────────┐  ┌──────────────┐  ┌──────────────────┐    │
│  │   Mapper   │  │  Page Tables │  │ Frame Allocator  │    │
│  │            │──│              │──│                  │    │
│  │ - map()    │  │ - PML4       │  │ - allocate()     │    │
│  │ - unmap()  │  │ - PDPT       │  │ - deallocate()   │    │
│  │ - translate│  │ - PD         │  │                  │    │
│  └────────────┘  │ - PT         │  └──────────────────┘    │
│                  └──────────────┘                            │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Physical Memory Allocator               │    │
│  │  (Bitmap-based frame tracking)                      │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Modules

### `paging.rs`
Core page table structures and types:
- `PageTable` - 512-entry page table structure
- `PageTableEntry` - Individual page table entry with flags
- `PageTableFlags` - Flags for controlling memory access
- `VirtAddr` / `PhysAddr` - Type-safe addresses
- `Page` / `PhysFrame` - 4KB-aligned memory units

### `mapper.rs`
Virtual memory mapping functionality:
- `Mapper` - Maps and unmaps virtual pages
- `map_to()` - Map a page to a specific frame
- `map()` - Map a page (allocate frame automatically)
- `unmap()` - Remove a mapping
- `translate()` - Translate virtual to physical address
- `flush_page()` / `flush_all()` - TLB management
- `read_cr3()` / `write_cr3()` - CR3 register access

### `frame_alloc.rs`
Frame allocator trait and implementations:
- `FrameAllocator` trait - Interface for allocating physical frames
- `BitmapFrameAllocator` - Implementation using bitmap allocator
- `EmptyFrameAllocator` - Stub allocator for testing

### `physical.rs`
Physical memory management:
- Bitmap-based frame allocator
- Memory map parsing from Multiboot2
- Frame allocation and deallocation
- Memory statistics

### `examples.rs`
Example code demonstrating virtual memory usage:
- Basic mapping examples
- Address translation examples
- Page table flag examples

## Usage

### Basic Mapping

```rust
use noodleos::arch::x86_64::memory::*;

// Get the current PML4 table
let pml4_addr = read_cr3();
let pml4 = unsafe { &mut *(pml4_addr.as_u64() as *mut PageTable) };

// Create mapper with frame allocator
let frame_allocator = BitmapFrameAllocator::new();
let mut mapper = unsafe { Mapper::new(pml4, frame_allocator) };

// Map a virtual page to a physical frame
let page = Page::containing_address(VirtAddr::new_unchecked(0xFFFF_8000_1000_0000));
let frame = PhysFrame::containing_address(PhysAddr::new(0x200000));
let flags = PageTableFlags::PRESENT
    .union(PageTableFlags::WRITABLE)
    .union(PageTableFlags::NO_EXECUTE);

mapper.map_to(page, frame, flags).expect("Failed to map page");
```

### Address Translation

```rust
// Translate a virtual address
let virt_addr = VirtAddr::new_unchecked(0xFFFF_8000_1000_0000);
if let Some(phys_addr) = mapper.translate(virt_addr) {
    println!("Virtual {:#x} -> Physical {:#x}", 
             virt_addr.as_u64(), phys_addr.as_u64());
}
```

### Working with Flags

```rust
// Common flag combinations
let kernel_code = PageTableFlags::PRESENT;

let kernel_data = PageTableFlags::PRESENT
    .union(PageTableFlags::WRITABLE)
    .union(PageTableFlags::NO_EXECUTE);

let user_data = PageTableFlags::PRESENT
    .union(PageTableFlags::WRITABLE)
    .union(PageTableFlags::USER_ACCESSIBLE)
    .union(PageTableFlags::NO_EXECUTE);
```

## Page Table Hierarchy

x86_64 uses 4-level paging:

```
Virtual Address (64-bit)
┌────────┬─────┬─────┬─────┬─────┬──────────┐
│ Sign   │ PML4│ PDPT│  PD │  PT │  Offset  │
│ 63-48  │47-39│38-30│29-21│20-12│   11-0   │
└────────┴─────┴─────┴─────┴─────┴──────────┘
         │     │     │     │     │
         ▼     ▼     ▼     ▼     ▼
      ┌────┐ ┌────┐┌────┐┌────┐
      │PML4│→│PDPT││ PD ││ PT │→[Physical Frame]
      └────┘ └────┘└────┘└────┘
       512     512   512   512
      entries entries entries entries
```

Each level:
- **PML4** (Level 4): 512 entries × 512 GB = 256 TB coverage
- **PDPT** (Level 3): 512 entries × 1 GB = 512 GB coverage
- **PD** (Level 2): 512 entries × 2 MB = 1 GB coverage
- **PT** (Level 1): 512 entries × 4 KB = 2 MB coverage

## Memory Layout

Current kernel memory layout:

```
0x0000_0000_0000_0000  ┌──────────────────────────┐
                       │    User Space (128 TB)    │
0x0000_7FFF_FFFF_FFFF  ├──────────────────────────┤
                       │   Non-canonical region    │
0xFFFF_8000_0000_0000  ├──────────────────────────┤
                       │   Kernel Space (128 TB)   │
                       │                           │
                       │  - Kernel code/data       │
                       │  - Kernel heap            │
                       │  - Physical memory map    │
                       │  - I/O mappings           │
0xFFFF_FFFF_FFFF_FFFF  └──────────────────────────┘
```

## Safety Considerations

- All page tables must be 4KB-aligned
- Virtual addresses must be canonical
- TLB must be flushed after page table modifications
- Physical frames must remain valid while mapped
- The active PML4 (pointed to by CR3) must stay valid

## Testing

Run the virtual memory examples:

```rust
use noodleos::arch::x86_64::memory::examples;

examples::run_examples();
```

## Future Work

- [ ] Support for huge pages (2MB/1GB)
- [ ] Page fault handler integration
- [ ] Copy-on-write support
- [ ] Demand paging
- [ ] Memory-mapped I/O helpers
- [ ] User space page table management
- [ ] Page table entry flags validation

## Documentation

For detailed documentation, see:
- `/docs/virtual-memory.md` - Complete virtual memory documentation
- `/docs/memory_allocator.md` - Physical memory allocator details
- `/docs/architecture.md` - Overall system architecture

## References

- Intel 64 and IA-32 Architectures SDM, Volume 3, Chapter 4
- AMD64 Architecture Programmer's Manual, Volume 2, Chapter 5
- OSDev Wiki: Paging (https://wiki.osdev.org/Paging)
