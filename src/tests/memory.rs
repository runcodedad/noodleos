/// Memory Management Testing Module
/// 
/// This module contains tests for memory management functionality.

use crate::arch::drivers::vga::println;

/// Run all memory tests
pub fn run_memory_tests() {
    println("=== RUNNING MEMORY TESTS ===");
    println("");
    
    #[cfg(feature = "test-memory")]
    {
        // Run physical memory allocator tests
        crate::arch::x86_64::memory::tests::test_physical_allocator();
        
        // Run virtual memory tests
        crate::arch::x86_64::memory::tests::test_virtual_memory();
        
        // Run CR3 register test
        crate::arch::x86_64::memory::tests::test_cr3_access();
    }
    
    #[cfg(not(feature = "test-memory"))]
    {
        println("Memory tests not enabled.");
        println("Use --features test-memory to enable.");
        println("");
    }
    
    println("=== MEMORY TESTS COMPLETE ===");
    println("");
}
