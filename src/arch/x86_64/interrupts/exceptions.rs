/// CPU Exception Handlers
/// 
/// This module contains handlers for CPU exceptions (vectors 0-31).
/// Each exception has its own handler function with appropriate error reporting.

use crate::arch::drivers::vga::println;

/// Divide by zero exception handler (Vector 0)
/// 
/// This handler is called when the CPU encounters a division by zero.
/// It provides useful debugging information before halting the system.
pub extern "C" fn divide_by_zero_handler() {
    // Clear a few lines to make the error visible
    println("");
    println("========================================");
    println("EXCEPTION: Division by Zero (#DE)");
    println("========================================");
    println("");
    println("The CPU encountered a division by zero operation.");
    println("This is a fatal error that cannot be recovered from.");
    println("");
    println("Exception Details:");
    println("  Vector: 0 (Divide Error)");
    println("  Type: Fault");
    println("  Error Code: None");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    // Halt the CPU in a loop
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Debug exception handler (Vector 1)
/// 
/// Handles debug exceptions including hardware breakpoints and single-step.
pub extern "C" fn debug_handler() {
    println("");
    println("========================================");
    println("EXCEPTION: Debug (#DB)");
    println("========================================");
    println("");
    println("A debug exception occurred.");
    println("This could be from a hardware breakpoint or single-step.");
    println("");
    println("Exception Details:");
    println("  Vector: 1 (Debug Exception)");
    println("  Type: Fault/Trap");
    println("  Error Code: None");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Breakpoint exception handler (Vector 3)
/// 
/// Handles INT3 breakpoint instructions.
pub extern "C" fn breakpoint_handler() {
    println("");
    println("========================================");
    println("EXCEPTION: Breakpoint (#BP)");
    println("========================================");
    println("");
    println("A breakpoint exception occurred (INT3 instruction).");
    println("This is typically used by debuggers.");
    println("");
    println("Exception Details:");
    println("  Vector: 3 (Breakpoint)");
    println("  Type: Trap");
    println("  Error Code: None");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Invalid opcode exception handler (Vector 6)
/// 
/// Handles attempts to execute invalid or unsupported instructions.
pub extern "C" fn invalid_opcode_handler() {
    println("");
    println("========================================");
    println("EXCEPTION: Invalid Opcode (#UD)");
    println("========================================");
    println("");
    println("The CPU encountered an invalid or unsupported instruction.");
    println("This could indicate corrupted code or unsupported CPU features.");
    println("");
    println("Exception Details:");
    println("  Vector: 6 (Invalid Opcode)");
    println("  Type: Fault");
    println("  Error Code: None");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Double fault exception handler (Vector 8)
/// 
/// Handles double faults - when an exception occurs while handling another exception.
/// This is a critical error that indicates serious system problems.
pub extern "C" fn double_fault_handler() {
    println("");
    println("========================================");
    println("CRITICAL: Double Fault (#DF)");
    println("========================================");
    println("");
    println("A double fault occurred!");
    println("This means an exception happened while handling another exception.");
    println("This is a critical system error.");
    println("");
    println("Exception Details:");
    println("  Vector: 8 (Double Fault)");
    println("  Type: Abort");
    println("  Error Code: Always 0");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// General protection fault handler (Vector 13)
/// 
/// Handles general protection violations including privilege violations,
/// segment violations, and other protection mechanism violations.
pub extern "C" fn general_protection_fault_handler() {
    println("");
    println("========================================");
    println("EXCEPTION: General Protection Fault (#GP)");
    println("========================================");
    println("");
    println("A general protection fault occurred.");
    println("This indicates a violation of the protection mechanism:");
    println("- Privilege level violation");
    println("- Segment limit violation");
    println("- Invalid segment selector");
    println("- Other protection violations");
    println("");
    println("Exception Details:");
    println("  Vector: 13 (General Protection Fault)");
    println("  Type: Fault");
    println("  Error Code: Yes (segment selector related)");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Page fault exception handler (Vector 14)
/// 
/// Handles page faults - memory access violations.
/// This is one of the most important exception handlers for memory management.
pub extern "C" fn page_fault_handler() {
    println("");
    println("========================================");
    println("EXCEPTION: Page Fault (#PF)");
    println("========================================");
    println("");
    println("A page fault occurred.");
    println("This indicates a memory access violation:");
    println("- Access to non-present page");
    println("- Write to read-only page");
    println("- User access to supervisor page");
    println("- Instruction fetch from non-executable page");
    println("");
    println("Exception Details:");
    println("  Vector: 14 (Page Fault)");
    println("  Type: Fault");
    println("  Error Code: Yes (page fault error code)");
    println("");
    println("System halted. Please reset to continue.");
    println("========================================");
    
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
