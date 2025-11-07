# NoodleOS Documentation

Welcome to the NoodleOS documentation! This directory contains comprehensive documentation about the operating system's architecture, design decisions, and development approach.

## Documentation Structure

- **[Architecture Overview](architecture.md)** - System design principles and component organization
- **[Boot Process](boot-process.md)** - How NoodleOS transitions from bootloader to kernel execution
- **[File Organization](file-organization.md)** - Project structure, module organization, and rationale
- **[Long Mode Transition](long-mode-transition.md)** - Concepts behind 32-bit to 64-bit CPU mode switching
- **[Build System](build-system.md)** - Toolchain requirements and compilation process overview
- **[VGA Buffer](vga-buffer.md)** - Text output system and hardware interface concepts
- **[IDT Implementation](idt-implementation.md)** - Interrupt Descriptor Table setup and concepts
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

### For Contributors  
1. Review **Development Guide** for workflow and standards
2. Study **File Organization** to understand the codebase structure
3. Understand **Build System** for toolchain requirements
4. Study **Long Mode Transition** for low-level system programming concepts

## Project Goals

NoodleOS serves as an educational platform for learning:
- **Systems Programming**: Low-level hardware interaction and kernel development
- **Modern Tools**: Rust language applied to operating system development
- **Industry Standards**: Real-world approaches used in production operating systems
- **Computer Architecture**: x86_64 platform and CPU behavior understanding

The documentation supports these goals by explaining concepts clearly while pointing readers to the source code for implementation details.
