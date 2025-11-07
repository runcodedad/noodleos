# NoodleOS File Organization

This document describes the reorganized file structure of NoodleOS following standard operating system development practices.

## Directory Structure

```
src/
├── main.rs                     # Main kernel entry point
└── arch/                       # Architecture-specific code
    ├── mod.rs                  # Architecture module entry
    └── x86_64/                 # x86_64 specific implementations
        ├── mod.rs              # x86_64 module entry
        ├── boot/               # Boot and initialization code
        │   ├── mod.rs          # Boot module
        │   ├── boot.s          # Long mode transition assembly
        │   └── multiboot_header.s # Multiboot2 header
        ├── interrupts/         # Interrupt handling
        │   └── mod.rs          # IDT implementation
        ├── memory/             # Memory management
        │   └── mod.rs          # Page tables, allocation (future)
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
- Interrupt Descriptor Table (IDT) setup
- Interrupt handlers
- Exception handling

#### `memory/`
- Physical and virtual memory management (future)
- Page table utilities (future)
- Heap allocator (future)

#### `drivers/`
- Hardware abstraction layer
- VGA text buffer driver
- Future: keyboard, serial, etc.

## Benefits of This Organization

1. **Separation of Concerns**: Each subsystem has its own module
2. **Architecture Independence**: Easy to add support for other architectures
3. **Maintainability**: Clear boundaries between different parts of the kernel
4. **Scalability**: Easy to add new drivers, memory managers, etc.
5. **Standard Practice**: Follows conventions used by major OS projects

## Build System

The Makefile has been updated to work with the new file locations:
- Assembly files are now in `src/arch/x86_64/boot/`
- Object files are generated in the same directories as source files
- All functionality remains the same: `make all`, `make run`, etc.
