/// Interrupt Management Module
/// 
/// This module provides comprehensive interrupt and exception handling for x86_64.
/// It's organized into separate submodules for better maintainability:
/// 
/// - `idt`: Core IDT data structures and management
/// - `exceptions`: CPU exception handlers (vectors 0-31)
/// - `hardware`: Hardware interrupt handlers (vectors 32-255)
/// - `setup`: Interrupt system initialization and management
/// 
/// ## Usage
/// 
/// ```rust
/// // Initialize the interrupt system
/// interrupts::setup_idt();
/// 
/// // Enable interrupts (after setting up interrupt controllers)
/// interrupts::enable_interrupts();
/// 
/// // Disable interrupts for critical sections
/// interrupts::disable_interrupts();
/// 
/// // Execute code with interrupts disabled
/// interrupts::without_interrupts(|| {
///     // Critical section code
/// });
/// ```

pub mod idt;
pub mod exceptions;
pub mod hardware;
pub mod setup;

// Re-export the main public interface
pub use setup::setup_idt;
