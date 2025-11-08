/// Boot module for x86_64 architecture
/// 
/// This module handles the early boot process, including:
/// - Multiboot2 header setup
/// - Long mode transition from 32-bit protected mode
/// - Initial page table setup
/// - GDT configuration for 64-bit mode

pub mod multiboot2;

pub use multiboot2::{BootInfo, MULTIBOOT2_MAGIC};
