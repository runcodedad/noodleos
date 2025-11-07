# Interrupt Descriptor Table (IDT) Implementation

## Overview

This document describes the IDT implementation for NoodleOS. The IDT is a fundamental x86_64 data structure that tells the processor how to handle interrupts and exceptions.

## What We've Implemented

### IDT Structure (`src/interrupts.rs`)

1. **IdtEntry**: Each entry in the IDT (16 bytes on x86_64)
   - Contains the address of the interrupt handler
   - Specifies the code segment selector 
   - Defines gate type (interrupt gate vs trap gate)
   - Includes privilege level information

2. **IdtDescriptor**: Used by the LIDT instruction
   - Contains the base address and limit of the IDT

3. **Idt**: The main IDT structure
   - Contains 256 entries (standard for x86_64)
   - Provides methods to set handlers and load the IDT

### Key Features

- **Safe Rust Interface**: Wrapper around low-level x86_64 structures
- **Basic Setup**: Creates an empty IDT with minimal configuration
- **Dummy Handler**: Simple handler that halts on any interrupt
- **LIDT Integration**: Properly loads the IDT using the LIDT instruction

### Current Implementation Status

#### ✅ Implemented Handlers
- **Vector 0**: Divide by Zero Exception (#DE) with detailed error reporting

#### 🚧 Planned Handlers
- Vector 6: Invalid Opcode (#UD)
- Vector 13: General Protection Fault (#GP)  
- Vector 14: Page Fault (#PF)
- Vector 8: Double Fault (#DF)

#### 📋 Configuration
- 256 IDT entries (standard x86_64 configuration)
- Uses code segment selector 0x08 (assumes GDT is properly set up)
- All handlers configured as interrupt gates
- Detailed error reporting with VGA text output

## Code Structure

```
src/
├── main.rs                            # Main kernel entry point
├── arch/
│   └── x86_64/
│       ├── interrupts/               # Interrupt management module
│       │   ├── mod.rs               # Main interface and documentation
│       │   ├── idt.rs               # Core IDT data structures
│       │   ├── exceptions.rs        # CPU exception handlers (0-31)
│       │   ├── hardware.rs          # Hardware interrupt handlers (32-255)
│       │   ├── setup.rs             # Interrupt system initialization
│       │   └── README.md            # Detailed module documentation
│       └── drivers/vga.rs           # VGA text output
└── tests/                           # Test modules (feature-gated)
    ├── mod.rs                       # Main test coordinator
    ├── exceptions.rs                # Exception handler tests  
    ├── memory.rs                    # Memory tests (placeholder)
    └── hardware.rs                  # Hardware tests (placeholder)
```

## Usage

The IDT is initialized during kernel startup:

```rust
// In kernel_main()
interrupts::setup_idt();
```

This:
1. Creates a new IDT with empty entries
2. Sets up a dummy handler for divide by zero
3. Loads the IDT using the LIDT instruction

## Adding New Interrupt/Exception Handlers

### Step-by-Step Guide

#### 1. **Create the Handler Function**

Create a new `extern "C"` function for your handler:

```rust
/// Page Fault Exception Handler (Vector 14)
extern "C" fn page_fault_handler() {
    use crate::arch::drivers::vga::println;
    
    println("");
    println("EXCEPTION: Page Fault (#PF)");
    println("A page fault occurred - invalid memory access.");
    // Add specific debugging info here
    
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
```

#### 2. **Register the Handler in IDT**

Add the handler to the `init_idt()` function:

```rust
pub fn init_idt() -> Idt {
    let mut idt = Idt::new();
    
    // Existing handlers...
    idt.set_handler(0, divide_by_zero_handler as u64, 0x08, GateType::InterruptGate);
    
    // Add your new handler
    idt.set_handler(14, page_fault_handler as u64, 0x08, GateType::InterruptGate);
    
    idt
}
```

#### 3. **Handler Function Requirements**

- **Calling Convention**: Must use `extern "C"` for proper ABI
- **Function Signature**: Currently `fn()` (no parameters)
- **No Return**: Must never return (use infinite loop with `hlt`)
- **Stack Safety**: Keep stack usage minimal
- **Re-entrancy**: Assume handler may be called from interrupt context

#### 4. **Common Exception Vectors**

| Vector | Exception | Error Code | Description |
|--------|-----------|------------|-------------|
| 0 | #DE Divide Error | No | Division by zero |
| 1 | #DB Debug | No | Debug exception |
| 2 | NMI | No | Non-maskable interrupt |
| 3 | #BP Breakpoint | No | INT3 instruction |
| 4 | #OF Overflow | No | INTO instruction |
| 5 | #BR BOUND Range | No | BOUND instruction |
| 6 | #UD Invalid Opcode | No | Invalid instruction |
| 7 | #NM Device Not Available | No | No math coprocessor |
| 8 | #DF Double Fault | Yes (0) | Double fault |
| 10 | #TS Invalid TSS | Yes | Invalid Task State Segment |
| 11 | #NP Segment Not Present | Yes | Segment not present |
| 12 | #SS Stack Fault | Yes | Stack segment fault |
| 13 | #GP General Protection | Yes | General protection fault |
| 14 | #PF Page Fault | Yes | Page fault |
| 16 | #MF x87 FPU Error | No | Math fault |
| 17 | #AC Alignment Check | Yes (0) | Alignment check |
| 18 | #MC Machine Check | No | Machine check |
| 19 | #XM SIMD Exception | No | SIMD floating-point |

#### 5. **Gate Types**

- **InterruptGate**: Disables interrupts during handler execution (recommended for exceptions)
- **TrapGate**: Keeps interrupts enabled (useful for system calls)

#### 6. **Code Segment Selector**

Currently using `0x08` which assumes:
- GDT is properly set up
- Kernel code segment is at selector 0x08
- Privilege level 0 (kernel mode)

## Next Steps

With the basic IDT infrastructure in place, we can now:

1. **Add More Exception Handlers**: Implement handlers for page faults, general protection faults, etc.
2. **Add Interrupt Handlers**: Handle hardware interrupts (timer, keyboard, etc.)
3. **Enhanced Context**: Add interrupt frame support for debugging information
4. **PIC/APIC Setup**: Configure interrupt controllers
5. **Stack Switching**: Implement proper kernel stack management

## Build and Test

### Normal Operation
The IDT code compiles successfully and is integrated into the kernel. The kernel now:
- Initializes the IDT on startup
- Displays confirmation messages via VGA buffer
- Shows "Kernel initialization complete" message
- Continues to run in a halt loop

### Testing Exception Handlers

#### Testing Exception Handlers

NoodleOS uses Cargo features for professional exception testing:

##### Method 1: Using Makefile (Recommended)
```bash
# Build with exception test framework (safe)
make test-exceptions

# Build and test divide by zero exception (triggers exception)
make test-divide-by-zero

# Run the test build
./dev.sh run
```

##### Method 2: Using Development Script
```bash
# Build with exception tests enabled
./dev.sh test-exceptions

# Build with divide by zero test (triggers exception on boot)
./dev.sh test-divide-zero

# Run the test
./dev.sh run
```

##### Method 3: Manual Cargo Commands
```bash
# Build with test framework only
cargo build --release --target x86_64-unknown-none --features test-exceptions

# Build with actual exception test
cargo build --release --target x86_64-unknown-none --features test-exceptions,test-divide-by-zero
```

##### Test Output
When `test-divide-by-zero` feature is enabled, the kernel will show:
```
=== EXCEPTION TEST: Divide by Zero ===
This will trigger a divide by zero exception...
Expected: Exception handler should display error message

[Exception occurs here]

========================================
EXCEPTION: Division by Zero (#DE)
========================================
...
```

Expected output:
```
========================================
EXCEPTION: Division by Zero (#DE)
========================================

The CPU encountered a division by zero operation.
This is a fatal error that cannot be recovered from.

Exception Details:
  Vector: 0 (Divide Error)
  Type: Fault
  Error Code: None

System halted. Please reset to continue.
========================================
```

#### Build and Test Commands

```bash
# Normal build and quick test
make test

# Exception testing builds
make test-exceptions        # Safe build with test framework
make test-divide-by-zero   # Build that triggers exception

# Development script commands
./dev.sh test              # Quick automated test
./dev.sh test-exceptions   # Build with test framework
./dev.sh test-divide-zero  # Build exception test
./dev.sh run              # Run with GUI

# Manual feature-based builds
cargo build --features test-exceptions
cargo build --features test-exceptions,test-divide-by-zero
```

## Technical Notes

- **No Standard Library**: Uses `#![no_std]` environment
- **Memory Layout**: IDT entries are properly aligned and packed
- **Safety**: Uses `unsafe` blocks only where necessary for hardware access
- **Cross-Platform**: Designed specifically for x86_64 architecture

The IDT is now ready for expansion with actual interrupt and exception handlers.
