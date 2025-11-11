/// Example usage of the virtual memory system
/// 
/// This module demonstrates how to use the virtual memory manager
/// to create mappings, translate addresses, and manage page tables.

use crate::arch::x86_64::memory::*;
use crate::arch::{println, print};

/// Example: Basic page mapping
pub fn example_basic_mapping() {
    println("\n=== Virtual Memory Example: Basic Mapping ===");
    
    // Get the current PML4 table from CR3
    let pml4_addr = read_cr3();
    println("Current PML4 address:");
    print("  0x");
    print_hex(pml4_addr.as_u64());
    println("");
    
    // Create a page to map
    let virt_addr = VirtAddr::new_unchecked(0xFFFF_8000_2000_0000);
    let page = Page::containing_address(virt_addr);
    
    println("Virtual page to map:");
    print("  Start: 0x");
    print_hex(page.start_address().as_u64());
    println("");
    
    // Note: Actual mapping requires a mutable reference to PML4
    // and a frame allocator, which needs careful setup
    println("(Mapping would be done with Mapper::new() and map_to())");
}

/// Example: Address translation
pub fn example_address_translation() {
    println("\n=== Virtual Memory Example: Address Translation ===");
    
    // Show how a virtual address is broken down
    let addr = VirtAddr::new_unchecked(0xFFFF_8000_1234_5678);
    
    println("Virtual address: 0x");
    print_hex(addr.as_u64());
    println("");
    
    println("Page table indices:");
    print("  PML4 (L4): ");
    print_decimal(addr.page_table_index(PageTableLevel::Four) as u64);
    println("");
    
    print("  PDPT (L3): ");
    print_decimal(addr.page_table_index(PageTableLevel::Three) as u64);
    println("");
    
    print("  PD   (L2): ");
    print_decimal(addr.page_table_index(PageTableLevel::Two) as u64);
    println("");
    
    print("  PT   (L1): ");
    print_decimal(addr.page_table_index(PageTableLevel::One) as u64);
    println("");
    
    print("  Offset:    0x");
    print_hex(addr.page_offset() as u64);
    println("");
}

/// Example: Page table flags
pub fn example_page_flags() {
    println("\n=== Virtual Memory Example: Page Table Flags ===");
    
    // Create some common flag combinations
    let kernel_code = PageTableFlags::PRESENT;
    
    let kernel_data = PageTableFlags::PRESENT
        .union(PageTableFlags::WRITABLE)
        .union(PageTableFlags::NO_EXECUTE);
    
    let user_code = PageTableFlags::PRESENT
        .union(PageTableFlags::USER_ACCESSIBLE);
    
    let user_data = PageTableFlags::PRESENT
        .union(PageTableFlags::WRITABLE)
        .union(PageTableFlags::USER_ACCESSIBLE)
        .union(PageTableFlags::NO_EXECUTE);
    
    println("Common flag combinations:");
    println("  Kernel code:  PRESENT");
    println("  Kernel data:  PRESENT | WRITABLE | NO_EXECUTE");
    println("  User code:    PRESENT | USER_ACCESSIBLE");
    println("  User data:    PRESENT | WRITABLE | USER_ACCESSIBLE | NO_EXECUTE");
    
    print("Kernel data flags: 0x");
    print_hex(kernel_data.bits());
    println("");
}

/// Run all virtual memory examples
pub fn run_examples() {
    println("\n╔════════════════════════════════════════════════╗");
    println("║  Virtual Memory System Examples               ║");
    println("╚════════════════════════════════════════════════╝");
    
    example_basic_mapping();
    example_address_translation();
    example_page_flags();
    
    println("\n=== Examples Complete ===\n");
}

/// Helper to print a hex value
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

/// Helper to print a decimal number
fn print_decimal(mut value: u64) {
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
    
    // Reverse the buffer
    for j in 0..i/2 {
        buffer.swap(j, i - 1 - j);
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer[..i]) };
    print(s);
}
