# NoodleOS File Organization

This document describes the reorganized file structure of NoodleOS following standard operating system development practices.

## Directory Structure

```
src/
├── main.rs                     # Main kernel entry point
├── tests/                      # Test infrastructure (feature-gated)
│   ├── mod.rs                  # Test coordinator
│   ├── exceptions.rs           # Exception handler tests
│   ├── memory.rs               # Memory allocator tests
│   └── hardware.rs             # Hardware tests (placeholder)
└── arch/                       # Architecture-specific code
    ├── mod.rs                  # Architecture module entry
    └── x86_64/                 # x86_64 specific implementations
        ├── mod.rs              # x86_64 module entry
        ├── boot/               # Boot and initialization code
        │   ├── mod.rs          # Boot module
        │   ├── boot.s          # Long mode transition assembly
        │   ├── multiboot_header.s # Multiboot2 header
        │   └── multiboot2.rs   # Multiboot2 parsing
        ├── interrupts/         # Interrupt handling (complete)
        │   ├── mod.rs          # Module interface and exports
        │   ├── idt.rs          # IDT data structures
        │   ├── exceptions.rs   # CPU exception handlers (0-31)
        │   ├── hardware.rs     # Hardware interrupt handlers (32-255)
        │   ├── setup.rs        # Interrupt system initialization
        │   └── README.md       # Module documentation
        ├── memory/             # Memory management
        │   ├── mod.rs          # Memory module interface
        │   ├── physical.rs     # Physical frame allocator
        │   └── tests.rs        # Memory allocator tests
        └── drivers/            # Hardware drivers
            ├── mod.rs          # Driver module entry
            └── vga.rs          # VGA text buffer driver
```

## Module Organization

### `src/main.rs`
- Main kernel entry point (follows standard OS conventions)
- Imports architecture-specific functionality
- Contains the kernel_main() function called from assembly
- Configured as a staticlib for linking with assembly boot code

### `src/arch/`
- Architecture-abstraction layer
- Currently supports x86_64, designed to be extensible
- Re-exports current architecture functionality

### `src/arch/x86_64/`
- All x86_64 specific code
- Organized into logical subsystems

#### `boot/`
- Multiboot2 header and compliance
- 32-bit to 64-bit long mode transition
- Initial page table setup
- GDT configuration

#### `interrupts/`
- Interrupt Descriptor Table (IDT) setup and core structures
- CPU exception handlers (divide by zero, page fault, GPF, etc.)
- Hardware interrupt handlers (timer, keyboard, spurious)
- Modular organization with separate files for different handler types
- Comprehensive testing infrastructure

#### `memory/`
- Physical memory allocator (bitmap-based frame allocator)
- Multiboot2 memory map parsing
- Memory statistics tracking
- Virtual memory management (future)
- Page table utilities (future)
- Heap allocator (future)

#### `drivers/`
- Hardware abstraction layer
- VGA text buffer driver
- Future: keyboard, serial, etc.

## Benefits of This Organization

1. **Separation of Concerns**: Each subsystem has its own module with clear responsibilities
2. **Architecture Independence**: Easy to add support for other architectures (ARM, RISC-V, etc.)
3. **Maintainability**: Clear boundaries between different parts of the kernel
4. **Scalability**: Easy to add new drivers, memory managers, interrupt handlers
5. **Standard Practice**: Follows conventions used by major OS projects (Linux, FreeBSD)
6. **Testability**: Feature-gated tests allow isolated component testing
7. **Modularity**: Related functionality grouped together (e.g., all interrupt code in one directory)

## Build System

The Makefile has been updated to work with the new file locations:
- Assembly files are now in `src/arch/x86_64/boot/`
- Object files are generated in the same directories as source files
- All functionality remains the same: `make all`, `make run`, etc.
