/// Boot module for x86_64 architecture
/// 
/// This module handles the early boot process, including:
/// - Multiboot2 header setup
/// - Long mode transition from 32-bit protected mode
/// - Initial page table setup
/// - GDT configuration for 64-bit mode

// Re-export any boot-related functionality here
// For now, this is mainly assembly code, but we may add Rust boot utilities later

/// Boot information and utilities
pub mod info {
    /// Multiboot2 magic number
    pub const MULTIBOOT2_MAGIC: u32 = 0x36d76289;
    
    /// Basic boot information structure
    /// This can be expanded as we add more boot features
    #[repr(C)]
    pub struct BootInfo {
        pub magic: u32,
        pub multiboot_info: u32,
    }
}
