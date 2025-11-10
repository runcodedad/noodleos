# NoodleOS Documentation

Welcome to the NoodleOS documentation! This directory contains comprehensive documentation about the operating system's architecture, design decisions, and development approach.

## Documentation Structure

- **[Architecture Overview](architecture.md)** - System design principles and component organization
- **[Boot Process](boot-process.md)** - How NoodleOS transitions from bootloader to kernel execution
- **[File Organization](file-organization.md)** - Project structure, module organization, and rationale
- **[Kernel Entry Point](kernel-entry-point.md)** - Main kernel entry structure and configuration
- **[Long Mode Transition](long-mode-transition.md)** - Concepts behind 32-bit to 64-bit CPU mode switching
- **[Build System](build-system.md)** - Toolchain requirements and compilation process overview
- **[Makefile Architecture](makefile-architecture.md)** - Scalable build system design and test infrastructure
- **[VGA Buffer](vga-buffer.md)** - Text output system and hardware interface concepts
- **[IDT Implementation](idt-implementation.md)** - Interrupt Descriptor Table setup and concepts
- **[Interrupt System](interrupt-system.md)** - Complete interrupt handling architecture and implementation
- **[Memory Allocator](memory_allocator.md)** - Physical memory allocator design and implementation
- **[Development Guide](development-guide.md)** - Contributing guidelines and development workflow

## Documentation Philosophy

These documents focus on **concepts and design decisions** rather than implementation details. The goal is to help readers understand:

- **Why** specific approaches were chosen
- **How** different components work together
- **What** design principles guide the project
- **Where** to find more detailed information in the source code

## Learning Path

### For Beginners
1. Start with **Architecture Overview** to understand the big picture
2. Read **File Organization** to understand the project structure
3. Read **Boot Process** to understand system initialization
4. Explore **VGA Buffer** to see hardware interaction concepts
5. Learn about **Interrupt System** to understand exception and interrupt handling

### For Contributors  
1. Review **Development Guide** for workflow and standards
2. Study **File Organization** to understand the codebase structure
3. Understand **Build System** and **Makefile Architecture** for toolchain and testing
4. Study **Long Mode Transition** for low-level system programming concepts
5. Review **IDT Implementation** and **Interrupt System** for handler development
6. Explore **Memory Allocator** for physical memory management

## Current Implementation Status

NoodleOS currently implements:

### ✅ Completed Features
- **Boot System**: Multiboot2 compliance with manual long mode transition
- **VGA Text Driver**: Color text output at 80×25 resolution
- **Interrupt System**: Complete IDT with 7 exception handlers and 5 hardware interrupt handlers
- **Physical Memory Allocator**: Bitmap-based 4KB frame allocator with Multiboot2 integration
- **Testing Infrastructure**: Feature-gated testing for exceptions and memory management
- **Build System**: Scalable Makefile with test support

### 🚧 In Development
- Virtual memory management and page table utilities
- Kernel heap allocator
- Enhanced hardware interrupt handling (PIC/APIC configuration)

### 📋 Planned Features
- Process management and task switching
- File system implementation
- User mode execution
- System call interface

## Project Goals

NoodleOS serves as an educational platform for learning:
- **Systems Programming**: Low-level hardware interaction and kernel development
- **Modern Tools**: Rust language applied to operating system development
- **Industry Standards**: Real-world approaches used in production operating systems
- **Computer Architecture**: x86_64 platform and CPU behavior understanding

The documentation supports these goals by explaining concepts clearly while pointing readers to the source code for implementation details.
