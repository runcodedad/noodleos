/// Exception Testing Module for NoodleOS
/// 
/// This module contains tests to verify exception handlers are working correctly.
/// Tests are enabled via Cargo features to keep production builds clean.

use crate::arch::drivers::vga::println;

/// Test divide by zero exception handler
/// 
/// This function uses inline assembly to trigger a real CPU divide by zero
/// exception at runtime, bypassing Rust's compile-time safety checks.
/// 
/// # Safety
/// This function will cause the system to halt when the exception occurs.
/// Only use this for testing exception handlers.
pub fn test_divide_by_zero() {
    println("=== EXCEPTION TEST: Divide by Zero ===");
    println("This will trigger a divide by zero exception...");
    println("Expected: Exception handler should display error message");
    println("");
    
    unsafe {
        // Use inline assembly to force a divide by zero at runtime
        // This bypasses Rust's compile-time checks
        core::arch::asm!(
            "mov rax, 42",    // Load dividend
            "xor rdx, rdx",   // Clear high part of dividend  
            "mov rcx, 0",     // Load divisor (zero)
            "div rcx",        // Divide - this will trigger #DE exception
            out("rax") _,
            out("rdx") _,
            out("rcx") _,
        );
    }
    
    // This line should never be reached if exception handler works
    println("ERROR: Divide by zero did not trigger exception!");
}

/// Test invalid opcode exception handler
/// 
/// This function triggers an invalid opcode exception (#UD) for testing.
/// Currently commented out as we haven't implemented the handler yet.
#[allow(dead_code)]
pub fn test_invalid_opcode() {
    println("=== EXCEPTION TEST: Invalid Opcode ===");
    println("This will trigger an invalid opcode exception...");
    println("");
    
    unsafe {
        // Trigger invalid opcode exception with undefined instruction
        core::arch::asm!("ud2");  // Undefined instruction
    }
    
    println("ERROR: Invalid opcode did not trigger exception!");
}

/// Test breakpoint exception handler
/// 
/// This function triggers a breakpoint exception (#BP) for testing.
/// Currently commented out as we haven't implemented the handler yet.
#[allow(dead_code)]
pub fn test_breakpoint() {
    println("=== EXCEPTION TEST: Breakpoint ===");
    println("This will trigger a breakpoint exception...");
    println("");
    
    unsafe {
        // Trigger breakpoint exception
        core::arch::asm!("int3");  // Software breakpoint
    }
    
    println("ERROR: Breakpoint did not trigger exception!");
}

/// Run all exception tests based on enabled features
pub fn run_exception_tests() {
    println("=== RUNNING EXCEPTION TESTS ===");
    println("");
    
    #[cfg(feature = "test-divide-by-zero")]
    {
        test_divide_by_zero();
    }
    
    // Future exception tests can be added here with their own features
    // #[cfg(feature = "test-invalid-opcode")]
    // {
    //     test_invalid_opcode();
    // }
    
    // #[cfg(feature = "test-breakpoint")]
    // {
    //     test_breakpoint();
    // }
    
    #[cfg(not(any(feature = "test-divide-by-zero")))]
    {
        println("No exception tests enabled.");
        println("Available exception test features:");
        println("  --features test-divide-by-zero");
        println("  --features test-invalid-opcode (future)");
        println("  --features test-breakpoint (future)");
    }
    
    println("");
    println("=== EXCEPTION TESTS COMPLETE ===");
}

/// Quick test that doesn't trigger exceptions
/// 
/// Useful for verifying the test framework works without causing system halt.
pub fn test_framework_check() {
    println("=== EXCEPTION TEST FRAMEWORK CHECK ===");
    println("Exception test framework is working correctly!");
    println("");
    println("To run actual exception tests, use:");
    println("  make test-divide-by-zero");
    println("  ./dev.sh test-divide-zero");
    println("  cargo build --features test-divide-by-zero");
    println("");
    println("=== FRAMEWORK CHECK COMPLETE ===");
}
