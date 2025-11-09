/// Memory management for x86_64 architecture
/// 
/// This module handles memory-related functionality including:
/// - Boot-time memory map initialization
/// - Page table management
/// - Physical and virtual memory allocation
/// - Memory mapping and protection
/// - Heap management

use crate::arch::println;
use super::boot::{BootInfo, MULTIBOOT2_MAGIC};

pub mod physical;

/// Basic memory constants for x86_64
pub mod constants {
    /// Page size (4KB)
    pub const PAGE_SIZE: usize = 4096;
    
    /// Canonical address space limits
    pub const CANONICAL_LOWER_LIMIT: u64 = 0x0000_7FFF_FFFF_FFFF;
    pub const CANONICAL_UPPER_LIMIT: u64 = 0xFFFF_8000_0000_0000;
}

// Kernel boundaries (defined in linker script)
extern "C" {
    static __kernel_start: u8;
    static __kernel_end: u8;
}

/// Initialize memory subsystem from multiboot information
/// 
/// Validates the multiboot magic number, parses boot info,
/// displays the memory map, and initializes the physical memory allocator.
pub fn init_memory(multiboot_info_addr: usize, multiboot_magic: usize) {
    if multiboot_magic != MULTIBOOT2_MAGIC as usize {
        println("Invalid multiboot magic number!");
        return;
    }
    
    if let Some(boot_info) = unsafe { BootInfo::new(multiboot_info_addr) } {
        boot_info.print_memory_map();
        
        // Initialize physical memory allocator
        let kernel_start = unsafe { &__kernel_start as *const u8 as usize };
        let kernel_end = unsafe { &__kernel_end as *const u8 as usize };
        
        println("Initializing physical memory allocator...");
        crate::arch::print("  Kernel: 0x");
        print_hex(kernel_start as u64);
        crate::arch::print(" - 0x");
        print_hex(kernel_end as u64);
        println("");
        
        unsafe {
            physical::init_physical_allocator(&boot_info, kernel_start, kernel_end);
        }
        
        // Print memory statistics
        let (total, free, allocated) = physical::memory_stats();
        crate::arch::print("  Total frames:     ");
        print_decimal(total as u64);
        println("");
        crate::arch::print("  Free frames:      ");
        print_decimal(free as u64);
        crate::arch::print(" (");
        print_size((free * constants::PAGE_SIZE) as u64);
        println(")");
        crate::arch::print("  Allocated frames: ");
        print_decimal(allocated as u64);
        crate::arch::print(" (");
        print_size((allocated * constants::PAGE_SIZE) as u64);
        println(")");
        println("");
    } else {
        println("Failed to parse multiboot info!");
    }
}

/// Helper function to print a hex value
fn print_hex(value: u64) {
    use crate::arch::print;
    
    const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";
    let mut buffer = [0u8; 16];
    
    for i in 0..16 {
        let nibble = ((value >> (60 - i * 4)) & 0xF) as usize;
        buffer[i] = HEX_CHARS[nibble];
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer) };
    print(s);
}

/// Helper function to print size in human-readable format
fn print_size(bytes: u64) {
    use crate::arch::print;
    
    let kb = bytes / 1024;
    let mb = kb / 1024;
    
    if mb > 0 {
        print_decimal(mb);
        print(" MB");
    } else if kb > 0 {
        print_decimal(kb);
        print(" KB");
    } else {
        print_decimal(bytes);
        print(" bytes");
    }
}

/// Helper function to print a decimal number
fn print_decimal(mut value: u64) {
    use crate::arch::print;
    
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
