/// Hardware drivers for x86_64 architecture
/// 
/// This module contains device drivers and hardware abstraction layers
/// specific to the x86_64 architecture.

pub mod vga;

// Re-export commonly used driver functionality
pub use vga::{clear_screen, print, println};
