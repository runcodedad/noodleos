/// x86_64 architecture-specific code
/// 
/// This module contains all the x86_64 specific implementations including:
/// - Boot process and initialization
/// - Interrupt handling (IDT)
/// - Memory management (paging, etc.)
/// - Hardware drivers (VGA, keyboard, etc.)

pub mod boot;
pub mod interrupts;
pub mod memory;
pub mod drivers;

// Re-export commonly used functionality for convenience
pub use interrupts::setup_idt;
pub use drivers::{clear_screen, print, println};
pub use memory::init_memory;
