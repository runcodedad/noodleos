/// Tests for memory management (physical and virtual)

use super::physical::{allocate_frame, allocate_frames, free_frame, free_frames, memory_stats};
use super::paging::{
    Page, PageTable, PageTableEntry, PageTableFlags, PageTableLevel,
    PhysAddr, PhysFrame, VirtAddr,
};
use super::frame_alloc::{BitmapFrameAllocator, FrameAllocator};
use super::mapper::{Mapper, read_cr3};
use crate::arch::{println, print};

/// Run basic physical memory allocator tests
pub fn test_physical_allocator() {
    println("=== Testing Physical Memory Allocator ===");
    
    // Test 1: Check initial state
    let (total, free_before, allocated_before) = memory_stats();
    print("Initial state: ");
    print_decimal(free_before);
    print(" free frames, ");
    print_decimal(allocated_before);
    println(" allocated frames");
    
    // Test 2: Allocate a single frame
    print("Test 1: Allocating single frame... ");
    if let Some(frame1) = allocate_frame() {
        print("OK (0x");
        print_hex(frame1 as u64);
        println(")");
    } else {
        println("FAILED - no memory available");
        return;
    }
    
    // Test 3: Allocate another single frame
    print("Test 2: Allocating another frame... ");
    if let Some(frame2) = allocate_frame() {
        print("OK (0x");
        print_hex(frame2 as u64);
        println(")");
    } else {
        println("FAILED");
        return;
    }
    
    // Test 4: Check that free count decreased
    let (_, free_after_alloc, _) = memory_stats();
    print("Test 3: Verifying free count decreased... ");
    if free_after_alloc == free_before - 2 {
        println("OK");
    } else {
        print("FAILED (expected ");
        print_decimal(free_before - 2);
        print(", got ");
        print_decimal(free_after_alloc);
        println(")");
    }
    
    // Test 5: Allocate multiple contiguous frames
    print("Test 4: Allocating 4 contiguous frames... ");
    if let Some(frames_base) = allocate_frames(4) {
        print("OK (0x");
        print_hex(frames_base as u64);
        println(")");
        
        // Free them immediately
        unsafe {
            free_frames(frames_base, 4);
        }
        println("Test 5: Freed 4 contiguous frames... OK");
    } else {
        println("FAILED");
    }
    
    // Test 6: Free the first two frames we allocated
    print("Test 6: Freeing first allocated frame... ");
    unsafe {
        free_frame(allocate_frame().unwrap());
    }
    println("OK");
    
    // Test 7: Check final state
    let (_, free_final, allocated_final) = memory_stats();
    print("Final state: ");
    print_decimal(free_final);
    print(" free frames, ");
    print_decimal(allocated_final);
    println(" allocated frames");
    
    println("=== Physical Allocator Tests Complete ===");
    println("");
}

/// Run virtual memory tests
pub fn test_virtual_memory() {
    println("=== Testing Virtual Memory System ===");
    
    test_virtual_addresses();
    test_physical_addresses();
    test_page_and_frame();
    test_page_table_flags();
    test_page_table_entry();
    test_page_table_structure();
    test_address_translation_indices();
    
    println("=== Virtual Memory Tests Complete ===");
    println("");
}

/// Test 1: Virtual address operations
fn test_virtual_addresses() {
    println("Test 1: Virtual Address Operations");
    
    // Test canonical addresses
    print("  1a. Lower canonical address... ");
    let lower = VirtAddr::new_unchecked(0x0000_7FFF_FFFF_F000);
    if lower.as_u64() == 0x0000_7FFF_FFFF_F000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  1b. Upper canonical address... ");
    let upper = VirtAddr::new_unchecked(0xFFFF_8000_0000_0000);
    if upper.as_u64() == 0xFFFF_8000_0000_0000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    // Test alignment operations
    print("  1c. Address alignment down... ");
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_0000_1234);
    let aligned = addr.align_down(4096);
    if aligned.as_u64() == 0xFFFF_8000_0000_1000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  1d. Address alignment up... ");
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_0000_1234);
    let aligned = addr.align_up(4096);
    if aligned.as_u64() == 0xFFFF_8000_0000_2000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  1e. Page offset extraction... ");
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_0000_1ABC);
    if addr.page_offset() == 0xABC {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 2: Physical address operations
fn test_physical_addresses() {
    println("Test 2: Physical Address Operations");
    
    print("  2a. Physical address creation... ");
    let phys = PhysAddr::new(0x1234_5678);
    if phys.as_u64() == 0x1234_5678 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  2b. Physical address alignment... ");
    let phys = PhysAddr::new(0x1000_1234);
    let aligned = phys.align_down(4096);
    if aligned.as_u64() == 0x1000_1000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  2c. Check 4KB alignment... ");
    let aligned_addr = PhysAddr::new(0x2000);
    let unaligned_addr = PhysAddr::new(0x2001);
    if aligned_addr.is_aligned(4096) && !unaligned_addr.is_aligned(4096) {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 3: Page and Frame operations
fn test_page_and_frame() {
    println("Test 3: Page and Frame Operations");
    
    print("  3a. Page from virtual address... ");
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_0000_1234);
    let page = Page::containing_address(addr);
    if page.start_address().as_u64() == 0xFFFF_8000_0000_1000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  3b. Frame from physical address... ");
    let addr = PhysAddr::new(0x20_0000 + 0x234);
    let frame = PhysFrame::containing_address(addr);
    if frame.start_address().as_u64() == 0x20_0000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  3c. Page number calculation... ");
    let page = Page::containing_address(VirtAddr::new_unchecked(0x5000));
    if page.number() == 5 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  3d. Frame number calculation... ");
    let frame = PhysFrame::containing_address(PhysAddr::new(0x3000));
    if frame.number() == 3 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 4: Page table flags
fn test_page_table_flags() {
    println("Test 4: Page Table Flags");
    
    print("  4a. Flag creation and testing... ");
    let flags = PageTableFlags::PRESENT;
    if flags.contains(PageTableFlags::PRESENT) {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  4b. Flag combination with union... ");
    let flags = PageTableFlags::PRESENT
        .union(PageTableFlags::WRITABLE)
        .union(PageTableFlags::NO_EXECUTE);
    if flags.contains(PageTableFlags::PRESENT) 
        && flags.contains(PageTableFlags::WRITABLE)
        && flags.contains(PageTableFlags::NO_EXECUTE) {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  4c. Flag bits extraction... ");
    let flags = PageTableFlags::PRESENT.union(PageTableFlags::WRITABLE);
    if flags.bits() == 0b11 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  4d. Empty flags... ");
    let flags = PageTableFlags::empty();
    if !flags.contains(PageTableFlags::PRESENT) {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 5: Page table entries
fn test_page_table_entry() {
    println("Test 5: Page Table Entries");
    
    print("  5a. New entry is unused... ");
    let entry = PageTableEntry::new();
    if entry.is_unused() {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  5b. Set entry address and flags... ");
    let mut entry = PageTableEntry::new();
    let addr = PhysAddr::new(0x1000);
    let flags = PageTableFlags::PRESENT.union(PageTableFlags::WRITABLE);
    entry.set_addr(addr, flags);
    if entry.addr().as_u64() == 0x1000 
        && entry.flags().contains(PageTableFlags::PRESENT)
        && entry.flags().contains(PageTableFlags::WRITABLE) {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  5c. Entry frame extraction... ");
    let mut entry = PageTableEntry::new();
    entry.set_addr(PhysAddr::new(0x2000), PageTableFlags::PRESENT);
    let frame = entry.frame();
    if frame.start_address().as_u64() == 0x2000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  5d. Clear entry... ");
    let mut entry = PageTableEntry::new();
    entry.set_addr(PhysAddr::new(0x3000), PageTableFlags::PRESENT);
    entry.set_unused();
    if entry.is_unused() {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 6: Page table structure
fn test_page_table_structure() {
    println("Test 6: Page Table Structure");
    
    print("  6a. New page table is zeroed... ");
    let table = PageTable::new();
    let all_unused = table.iter().all(|entry| entry.is_unused());
    if all_unused {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  6b. Page table indexing... ");
    let mut table = PageTable::new();
    table[0].set_addr(PhysAddr::new(0x1000), PageTableFlags::PRESENT);
    if table[0].addr().as_u64() == 0x1000 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    print("  6c. Page table iteration... ");
    let table = PageTable::new();
    let count = table.iter().count();
    if count == 512 {
        println("OK");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Test 7: Address translation indices
fn test_address_translation_indices() {
    println("Test 7: Address Translation Indices");
    
    // Test address: 0xFFFF_8000_1234_5678
    // Binary breakdown of bits 47-12:
    // Bits 47-39 (PML4): 256 (0x100)
    // Bits 38-30 (PDPT): 0
    // Bits 29-21 (PD):   145 (0x91)
    // Bits 20-12 (PT):   69 (0x45)
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_1234_5678);
    
    print("  7a. PML4 index extraction... ");
    let pml4_idx = addr.page_table_index(PageTableLevel::Four);
    if pml4_idx == 256 {
        println("OK");
    } else {
        print("FAILED (got ");
        print_decimal(pml4_idx);
        println(")");
    }
    
    print("  7b. PDPT index extraction... ");
    let pdpt_idx = addr.page_table_index(PageTableLevel::Three);
    if pdpt_idx == 0 {
        println("OK");
    } else {
        print("FAILED (got ");
        print_decimal(pdpt_idx);
        println(")");
    }
    
    print("  7c. PD index extraction... ");
    let pd_idx = addr.page_table_index(PageTableLevel::Two);
    if pd_idx == 145 {
        println("OK");
    } else {
        print("FAILED (got ");
        print_decimal(pd_idx);
        println(")");
    }
    
    print("  7d. PT index extraction... ");
    let pt_idx = addr.page_table_index(PageTableLevel::One);
    if pt_idx == 69 {
        println("OK");
    } else {
        print("FAILED (got ");
        print_decimal(pt_idx);
        println(")");
    }
    
    println("");
}

/// Test 8: CR3 register reading (read-only test)
pub fn test_cr3_access() {
    println("Test 8: CR3 Register Access");
    
    print("  8a. Read CR3 register... ");
    let cr3_addr = read_cr3();
    // CR3 should be page-aligned and non-zero
    if cr3_addr.as_u64() != 0 && cr3_addr.is_aligned(4096) {
        print("OK (PML4 at 0x");
        print_hex(cr3_addr.as_u64());
        println(")");
    } else {
        println("FAILED");
    }
    
    println("");
}

/// Helper to print hex value
fn print_hex(value: u64) {
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    let mut buffer = [0u8; 16];
    
    for i in 0..16 {
        let nibble = ((value >> (60 - i * 4)) & 0xF) as usize;
        buffer[i] = HEX_CHARS[nibble];
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
    print(s);
}

/// Helper to print decimal
fn print_decimal(mut value: usize) {
    if value == 0 {
        print("0");
        return;
    }
    
    let mut buffer = [0u8; 20];
    let mut i = 0;
    
    while value > 0 {
        buffer[i] = b'0' + (value % 10) as u8;
        value /= 10;
        i += 1;
    }
    
    // Reverse
    for j in 0..i/2 {
        buffer.swap(j, i - 1 - j);
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer[..i]) };
    print(s);
}
