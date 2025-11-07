# Interrupt Management Module

This module provides comprehensive interrupt and exception handling for NoodleOS on x86_64 architecture. The module is organized into separate files for better maintainability and scalability.

## Directory Structure

```
interrupts/
├── mod.rs          # Main module interface and documentation
├── idt.rs          # Core IDT data structures and management
├── exceptions.rs   # CPU exception handlers (vectors 0-31)
├── hardware.rs     # Hardware interrupt handlers (vectors 32-255)
├── setup.rs        # Interrupt system initialization and management
└── README.md       # This documentation
```

## Module Organization

### `idt.rs` - Core IDT Implementation
- **`IdtEntry`**: 16-byte IDT entry structure for x86_64
- **`IdtDescriptor`**: Descriptor structure for LIDT instruction
- **`Idt`**: Main IDT structure with 256 entries
- **`GateType`**: Interrupt gate vs trap gate types

### `exceptions.rs` - CPU Exception Handlers
Handles CPU-generated exceptions (vectors 0-31):

| Vector | Exception | Handler Function |
|--------|-----------|------------------|
| 0 | Divide by Zero (#DE) | `divide_by_zero_handler` |
| 1 | Debug (#DB) | `debug_handler` |
| 3 | Breakpoint (#BP) | `breakpoint_handler` |
| 6 | Invalid Opcode (#UD) | `invalid_opcode_handler` |
| 8 | Double Fault (#DF) | `double_fault_handler` |
| 13 | General Protection (#GP) | `general_protection_fault_handler` |
| 14 | Page Fault (#PF) | `page_fault_handler` |

### `hardware.rs` - Hardware Interrupt Handlers
Handles hardware-generated interrupts (vectors 32-255):

| Vector | IRQ | Device | Handler Function |
|--------|-----|--------|------------------|
| 32 | IRQ 0 | Timer | `timer_interrupt_handler` |
| 33 | IRQ 1 | Keyboard | `keyboard_interrupt_handler` |
| 36 | IRQ 4 | Serial Port | `serial_interrupt_handler` |
| Others | - | Unhandled | `unhandled_interrupt_handler` |

### `setup.rs` - Interrupt System Management
- **`init_idt()`**: Creates and configures the complete IDT
- **`setup_idt()`**: Initializes and loads the IDT
- **`enable_interrupts()`**: Enables hardware interrupts (STI)
- **`disable_interrupts()`**: Disables hardware interrupts (CLI)
- **`interrupts_enabled()`**: Checks if interrupts are enabled
- **`without_interrupts()`**: Executes code with interrupts disabled

## Usage

### Basic Setup
```rust
use crate::arch::x86_64::interrupts;

// Initialize the interrupt system
interrupts::setup_idt();

// Enable interrupts (after setting up PIC/APIC)
interrupts::enable_interrupts();
```

### Advanced Usage
```rust
// Disable interrupts for critical sections
interrupts::disable_interrupts();
// ... critical code ...
interrupts::enable_interrupts();

// Or use RAII-style interrupt disabling
interrupts::without_interrupts(|| {
    // This code runs with interrupts disabled
    // Interrupts are automatically restored when closure exits
});

// Check interrupt state
if interrupts::interrupts_enabled() {
    println("Interrupts are enabled");
}
```

## Adding New Handlers

### Exception Handlers
1. Add handler function to `exceptions.rs`
2. Register in `setup.rs` `init_idt()` function
3. Use `InterruptGate` for faults, `TrapGate` for traps

```rust
// In exceptions.rs
pub extern "C" fn new_exception_handler() {
    // Handler implementation
}

// In setup.rs init_idt()
idt.set_handler(VECTOR, exceptions::new_exception_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
```

### Hardware Interrupt Handlers
1. Add handler function to `hardware.rs`
2. Register in `setup.rs` `init_idt()` function
3. Don't forget to send EOI to interrupt controller

```rust
// In hardware.rs
pub extern "C" fn new_device_handler() {
    // Handle device interrupt
    // Send EOI to PIC/APIC
}

// In setup.rs init_idt()
idt.set_handler(32 + IRQ_NUMBER, hardware::new_device_handler as u64, KERNEL_CODE_SELECTOR, GateType::InterruptGate);
```

## Future Enhancements

### Planned Features
- **Interrupt Stack Table (IST)**: For critical exceptions like double fault
- **Error Code Handling**: Extract and display error codes for applicable exceptions
- **Interrupt Controller Support**: PIC and APIC initialization and management
- **Interrupt Frame Support**: Pass register state to handlers
- **Nested Interrupt Handling**: Proper interrupt nesting and priorities

### Assembly Stubs
For production-quality interrupt handling, consider adding assembly wrapper stubs:
- Save/restore all registers
- Handle error codes properly
- Provide interrupt frame to Rust handlers
- Handle stack switching for ring transitions

## Safety Notes

- All interrupt handlers use `extern "C"` calling convention
- Handlers must never return (use infinite loop with `hlt`)
- Critical sections should be kept minimal when interrupts are disabled
- Hardware interrupt handlers should send EOI to interrupt controller
- Exception handlers provide debugging information before halting

This modular structure allows for easy maintenance and extension as the interrupt system grows in complexity.
