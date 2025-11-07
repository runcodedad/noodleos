/// Interrupt Management and Setup
/// 
/// This module coordinates interrupt setup and provides the main
/// interface for interrupt management.

use super::idt::{Idt, GateType};
use super::exceptions;
use super::hardware;

/// Code segment selector for kernel code
/// This assumes the GDT has kernel code segment at selector 0x08
const KERNEL_CODE_SELECTOR: u16 = 0x08;

/// Initialize the IDT with all exception and interrupt handlers
pub fn init_idt() -> Idt {
    let mut idt = Idt::new();
    
    // === CPU Exception Handlers (Vectors 0-31) ===
    
    // Vector 0: Divide by Zero Exception (#DE)
    idt.set_handler(0, exceptions::divide_by_zero_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 1: Debug Exception (#DB)
    idt.set_handler(1, exceptions::debug_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 3: Breakpoint Exception (#BP)
    idt.set_handler(3, exceptions::breakpoint_handler as u64, KERNEL_CODE_SELECTOR, GateType::TrapGate);
    
    // Vector 6: Invalid Opcode Exception (#UD)
    idt.set_handler(6, exceptions::invalid_opcode_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 8: Double Fault Exception (#DF)
    idt.set_handler(8, exceptions::double_fault_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 13: General Protection Fault (#GP)
    idt.set_handler(13, exceptions::general_protection_fault_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 14: Page Fault Exception (#PF)
    idt.set_handler(14, exceptions::page_fault_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // === Hardware Interrupt Handlers (Vectors 32-255) ===
    
    // Vector 32: Timer (IRQ 0)
    idt.set_handler(32, hardware::timer_interrupt_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 33: Keyboard (IRQ 1)
    idt.set_handler(33, hardware::keyboard_interrupt_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Vector 36: Serial Port (IRQ 4)
    idt.set_handler(36, hardware::serial_interrupt_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
    
    // Fill remaining vectors with unhandled interrupt handler
    for vector in 32..=255_u8 {
        // Skip vectors we've already set
        if vector != 32 && vector != 33 && vector != 36 {
            idt.set_handler(vector, hardware::unhandled_interrupt_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
        }
    }
    
    idt
}

/// Global IDT instance
static mut IDT: Option<Idt> = None;

/// Initialize and load the IDT
pub fn setup_idt() {
    unsafe {
        IDT = Some(init_idt());
        if let Some(ref idt) = IDT {
            idt.load();
        }
    }
}

/// Enable interrupts
/// 
/// This function enables hardware interrupts by setting the interrupt flag.
/// Call this after setting up the IDT and interrupt controllers.
pub fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti");
    }
}

/// Disable interrupts
/// 
/// This function disables hardware interrupts by clearing the interrupt flag.
/// Useful for critical sections that must not be interrupted.
pub fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cli");
    }
}

/// Check if interrupts are enabled
/// 
/// Returns true if the interrupt flag is set.
pub fn interrupts_enabled() -> bool {
    let flags: u64;
    unsafe {
        core::arch::asm!("pushfq; pop {}", out(reg) flags);
    }
    (flags & 0x200) != 0  // Check IF flag (bit 9)
}

/// Execute a closure with interrupts disabled
/// 
/// This function temporarily disables interrupts, executes the closure,
/// and then restores the previous interrupt state.
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let were_enabled = interrupts_enabled();
    disable_interrupts();
    let result = f();
    if were_enabled {
        enable_interrupts();
    }
    result
}
