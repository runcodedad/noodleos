/// NoodleOS Testing Module
/// 
/// This module contains all testing functionality for the operating system.
/// Tests are organized by category and enabled via Cargo features.

pub mod exceptions;
pub mod memory;
pub mod hardware;

/// Run all enabled tests based on Cargo features
pub fn run_all_tests() {
    use crate::arch::drivers::vga::println;
    
    println("=== NOODLEOS TEST SUITE ===");
    
    // Exception tests
    #[cfg(feature = "test-exceptions")]
    {
        exceptions::run_exception_tests();
    }
    
    // Memory tests (placeholder for future)
    #[cfg(feature = "test-memory")]
    {
        memory::run_memory_tests();
    }
    
    // Hardware tests (placeholder for future)
    #[cfg(feature = "test-hardware")]
    {
        hardware::run_hardware_tests();
    }
    
    // Show available tests if none are enabled
    #[cfg(not(any(
        feature = "test-exceptions",
        feature = "test-memory", 
        feature = "test-hardware"
    )))]
    {
        println("No tests enabled.");
        println("");
        println("Available test categories:");
        println("  Exception Tests:");
        println("    --features test-exceptions");
        println("    --features test-divide-by-zero");
        println("  Memory Tests (future):");
        println("    --features test-memory");
        println("  Hardware Tests (future):");
        println("    --features test-hardware");
    }
    
    println("=== TEST SUITE COMPLETE ===");
}
