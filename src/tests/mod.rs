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
        println("No test categories enabled.");
        println("");
        println("Usage: --features run-tests,<category>");
        println("");
        println("Available test categories:");
        println("  test-exceptions      - IDT and exception handling tests");
        println("  test-divide-by-zero  - Divide by zero exception test");
        println("  test-memory          - Physical memory allocator tests");
        println("  test-hardware        - Hardware driver tests (future)");
        println("");
        println("Example: cargo build --features run-tests,test-memory");
    }
    
    println("=== TEST SUITE COMPLETE ===");
}
