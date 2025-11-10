# NoodleOS Architecture Overview

This document provides a high-level view of NoodleOS architecture, design principles, and system components.

## System Architecture

```
┌─────────────────────────────────────┐
│           User Space                │
│         (Future Development)        │
├─────────────────────────────────────┤
│          Kernel Space               │
│  ┌─────────────┐  ┌─────────────┐  │
│  │ VGA Buffer  │  │   Drivers   │  │
│  │   Driver    │  │  (Future)   │  │
│  └─────────────┘  └─────────────┘  │
│  ┌─────────────┐  ┌─────────────┐  │
│  │   Memory    │  │ Interrupts  │  │
│  │ Allocator   │  │   System    │  │
│  │ (Physical)  │  │  (IDT/ISR)  │  │
│  └─────────────┘  └─────────────┘  │
├─────────────────────────────────────┤
│         Hardware Layer              │
│  ┌─────────────┐  ┌─────────────┐  │
│  │ VGA Device  │  │  CPU/APIC   │  │
│  └─────────────┘  └─────────────┘  │
└─────────────────────────────────────┘
```

## Design Principles

### 1. Safety First
- **Rust Language**: Memory safety without garbage collection
- **Minimal `unsafe`**: Only where absolutely necessary for hardware access
- **Type Safety**: Strong typing prevents common kernel bugs

### 2. Modular Design
- **Separate Modules**: Each subsystem in its own module
- **Clean Interfaces**: Well-defined APIs between components
- **Future Extension**: Easy to add new features

### 3. Educational Focus
- **Clear Code**: Well-commented, readable implementation
- **Step-by-Step**: Incremental complexity
- **Documentation**: Comprehensive explanations

## Current Components

### Core Kernel (`src/main.rs`)
- **Entry Point**: `kernel_main()` function called by boot assembly
- **Panic Handler**: Basic panic handling for system errors
- **Module Coordination**: Imports and coordinates other modules
- **Test Infrastructure**: Feature-gated testing system

### Interrupt System (`src/arch/x86_64/interrupts/`)
- **IDT Setup**: Complete Interrupt Descriptor Table implementation
- **Exception Handlers**: CPU exceptions (divide by zero, page fault, GPF, etc.)
- **Hardware Interrupts**: Timer, keyboard, and spurious interrupt handlers
- **Modular Design**: Separate files for exceptions, hardware, and setup

**Key Features:**
- 7 implemented exception handlers with detailed error reporting
- 3 hardware interrupt handlers
- Feature-gated testing system
- Professional code organization

### Memory Management (`src/arch/x86_64/memory/`)
- **Physical Allocator**: Bitmap-based frame allocator
- **4KB Frame Management**: Allocation and deallocation of physical frames
- **Multiboot2 Integration**: Memory map parsing from bootloader
- **Statistics Tracking**: Real-time memory usage information

**Key Features:**
- First-fit allocation with hint optimization
- Contiguous frame allocation support
- Thread-safe with atomic operations
- Comprehensive testing infrastructure

### VGA Buffer Module (`src/arch/x86_64/drivers/vga.rs`)
- **Direct Hardware Access**: Memory-mapped VGA text buffer
- **Text Output**: Functions to write characters and strings
- **Screen Management**: Clear screen, scroll, positioning

**Key Features:**
- Color support (16 foreground × 8 background colors)
- 80×25 character display
- Hardware scrolling capability

### Boot Components (`src/arch/x86_64/boot/`)
- **Multiboot Header**: Assembly code for bootloader interface
- **Long Mode Transition**: 32-bit to 64-bit mode switching
- **Page Table Setup**: Initial identity mapping
- **Linker Script**: Memory layout and section organization
- **GRUB Configuration**: Bootloader setup

## Memory Layout

```
High Memory
│
├─── Stack (grows down)
├─── Heap (future - grows up)
├─── BSS (uninitialized data)
├─── Data (initialized data)
├─── RO Data (read-only data)
├─── Text (executable code)
├─── Multiboot Header
├─── 0x100000 (1MB - kernel load address)
│
├─── VGA Buffer (0xB8000)
├─── BIOS/Hardware reserved
├─── 0x0 (start of memory)
│
Low Memory
```

## Hardware Abstraction

### Current Level
- **Minimal Abstraction**: Direct hardware register access
- **VGA Text Mode**: Memory-mapped I/O for display
- **CPU Instructions**: Direct assembly for halt, etc.

### Future Abstraction Layers
- **Device Drivers**: Standardized interfaces for hardware
- **Interrupt Handling**: IRQ management and handlers
- **Memory Management**: Virtual memory, paging, allocation
- **System Calls**: User-kernel interface

## Compilation Target

### x86_64-unknown-none Target
Custom target specification for bare-metal x86_64 development:

**Key Characteristics:**
- **No Operating System**: Bare metal execution environment
- **Abort on Panic**: No exception unwinding in kernel space
- **No Red Zone**: Required for safe kernel code execution
- **Software Floating Point**: Avoids hardware FPU dependencies

## Error Handling Strategy

### Current Approach
- **Simple Panic Handler**: Infinite loop on kernel panic
- **Fail-Fast Philosophy**: System halts on unrecoverable errors
- **No Recovery**: Focus on preventing errors rather than handling them

### Future Improvements
- **Panic Information Display**: Show panic message on screen
- **Stack Trace**: Debug information for development
- **Recovery Mechanisms**: Attempt graceful degradation where possible
- **Logging**: Persistent error recording system

## Extension Points

### Implemented Features ✅
1. **Interrupt Handling**: IDT setup with exception and hardware interrupt handlers
2. **Physical Memory Management**: Bitmap-based frame allocator

### Planned Features (in order)
1. **Virtual Memory Management**: Page table utilities, higher-half kernel mapping
2. **Heap Allocator**: Dynamic memory allocation for kernel
3. **Process Management**: Task switching, scheduler
4. **File System**: Simple filesystem implementation
5. **User Mode**: Ring 3 execution, system calls

### Module Structure (Future Development)
The planned directory structure will organize code by functionality:

- **Architecture Layer** (`arch/x86_64/`): CPU and platform-specific code
- **Hardware Drivers** (`drivers/`): Device-specific interfaces
- **Memory Management** (`memory/`): Virtual memory and allocation
- **Task Management** (`task/`): Process scheduling and switching
- **File System** (`fs/`): Storage abstraction and implementation

## Performance Considerations

### Current State
- **Minimal Overhead**: Direct hardware access
- **No Dynamic Allocation**: Static memory layout
- **Single Threaded**: No concurrency overhead

### Future Optimizations
- **Efficient Scheduling**: O(1) scheduler
- **Memory Coalescing**: Smart allocator design  
- **Interrupt Latency**: Fast ISR implementation
- **Cache Optimization**: Memory layout for performance

This architecture provides a solid foundation for learning operating system concepts while maintaining simplicity and safety through Rust's type system.
