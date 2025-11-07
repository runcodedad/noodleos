/// IDT (Interrupt Descriptor Table) Core Implementation
/// 
/// This module contains the core IDT data structures and management functions.
/// Individual handlers are implemented in separate modules.

use core::mem::size_of;

/// Number of IDT entries (256 for x86_64)
const IDT_ENTRIES: usize = 256;

/// IDT gate types
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GateType {
    InterruptGate = 0b1110,
    TrapGate = 0b1111,
}

/// IDT entry structure (16 bytes each on x86_64)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    /// Lower 16 bits of the interrupt service routine (ISR) address
    offset_low: u16,
    /// Code segment selector (GDT selector)
    selector: u16,
    /// Interrupt Stack Table offset (3 bits) + reserved (5 bits)
    ist: u8,
    /// Gate type and attributes
    type_attributes: u8,
    /// Middle 16 bits of the ISR address
    offset_middle: u16,
    /// Upper 32 bits of the ISR address
    offset_high: u32,
    /// Reserved (must be zero)
    reserved: u32,
}

impl IdtEntry {
    /// Create a new IDT entry for an interrupt gate
    pub fn new(handler: u64, selector: u16, gate_type: GateType) -> Self {
        Self {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            ist: 0, // No IST for now
            type_attributes: (gate_type as u8) | (1 << 7), // Present bit set
            offset_middle: ((handler >> 16) & 0xFFFF) as u16,
            offset_high: ((handler >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }

    /// Create an empty IDT entry
    pub fn empty() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

/// IDT descriptor structure for LIDT instruction
#[repr(C, packed)]
#[derive(Debug)]
pub struct IdtDescriptor {
    /// Size of IDT in bytes minus 1
    limit: u16,
    /// Linear address of IDT
    base: u64,
}

/// The Interrupt Descriptor Table
pub struct Idt {
    entries: [IdtEntry; IDT_ENTRIES],
}

impl Idt {
    /// Create a new empty IDT
    pub fn new() -> Self {
        Self {
            entries: [IdtEntry::empty(); IDT_ENTRIES],
        }
    }

    /// Set an IDT entry
    pub fn set_handler(&mut self, vector: u8, handler: u64, selector: u16, gate_type: GateType) {
        self.entries[vector as usize] = IdtEntry::new(handler, selector, gate_type);
    }

    /// Load the IDT using the LIDT instruction
    pub fn load(&self) {
        let descriptor = IdtDescriptor {
            limit: (size_of::<[IdtEntry; IDT_ENTRIES]>() - 1) as u16,
            base: self.entries.as_ptr() as u64,
        };

        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &descriptor, options(readonly, nostack, preserves_flags));
        }
    }
}
