# NoodleOS Boot Process

This document explains how NoodleOS boots from power-on to kernel execution.

## Overview

NoodleOS uses the **Multiboot2** specification with **GRUB** as the bootloader. The boot process follows these stages:

```
Power-On → BIOS/UEFI → GRUB → Multiboot2 → NoodleOS Kernel
```

## Boot Flow

### 1. System Initialization
- System firmware (BIOS/UEFI) initializes hardware
- Firmware locates bootable media (our ISO file)
- Firmware loads and executes GRUB from the boot sector

### 2. GRUB Bootloader Stage
- GRUB reads configuration from `grub.cfg`
- GRUB verifies Multiboot2 header in our kernel binary
- **GRUB boots kernel in 32-bit protected mode** (industry standard)
- GRUB provides system information to the kernel

### 3. Manual Long Mode Transition
NoodleOS performs a manual transition from 32-bit to 64-bit mode, following the same approach used by Linux, FreeBSD, and other production operating systems.

**Transition Steps:**
1. **CPU Verification**: Check that processor supports 64-bit long mode
2. **Memory Setup**: Configure identity paging (virtual = physical addresses)
3. **Page Tables**: Set up 64-bit page table hierarchy (PML4, PDPT, PDT)
4. **CPU Mode Switch**: Enable PAE, long mode, and paging
5. **64-bit Jump**: Transfer control to 64-bit kernel code

**Why Manual Transition:**
- **Educational Value**: Complete understanding of x86_64 boot process
- **Industry Standard**: Same approach used by major operating systems
- **Full Control**: Every aspect of boot process is understood and controllable
- **Professional Skills**: Directly applicable to real-world OS development

### 4. Kernel Entry
Once the transition completes, the kernel starts execution in genuine 64-bit long mode with:
- **CPU**: x86_64 long mode active
- **Memory**: Identity-mapped virtual memory
- **Stack**: Properly aligned kernel stack
- **Hardware Access**: VGA text mode available for output

## Boot Standards and Specifications

### Multiboot2 Specification
- **Purpose**: Standardized interface between bootloader and kernel
- **Benefits**: Bootloader independence and standardized system information
- **Memory Map**: Hardware memory layout provided by bootloader
- **Module Loading**: Support for loading additional kernel modules

### Binary Format
- **ELF Format**: Standard executable format for Unix-like systems
- **Linker Script**: Custom memory layout targeting 1MB load address
- **Sections**: Organized code, data, and metadata sections

## Architecture Comparison

### Our Approach: Manual Long Mode Transition
```
GRUB (32-bit) → Boot Assembly (32-bit) → Manual Transition → Kernel (64-bit)
```

**Advantages:**
- Complete understanding of hardware transition process
- Industry-standard approach used by major operating systems  
- Full control over page table setup and CPU configuration
- Enhanced debugging capabilities at each transition step

### Alternative: Bootloader-Managed Transition
Some bootloaders can handle the 64-bit transition automatically, but this approach:
- Hides critical system programming concepts
- Reduces control over low-level system configuration
- Limits portability to other bootloader systems
- Provides less educational value for OS development

## Memory Layout

The kernel is loaded at the 1MB mark (0x100000) to avoid conflicts with:
- **Real Mode IVT**: Interrupt vector table in low memory
- **BIOS Data Area**: System firmware working space
- **Video Memory**: VGA buffer and other display hardware

**Kernel Structure:**
- Multiboot header (bootloader detection)
- Boot code (32-bit assembly for transition)
- Kernel code (64-bit Rust implementation)
- Data sections (initialized and uninitialized data)

## Post-Boot System State

After successful boot, NoodleOS has:
- **64-bit Environment**: True long mode execution
- **Basic Output**: VGA text buffer for console output
- **Stable Platform**: Ready for additional kernel subsystems
- **Hardware Access**: Direct memory-mapped I/O capabilities

## Kernel Initialization Sequence

Once in 64-bit mode, the kernel (`kernel_main` in `src/main.rs`) performs:

1. **VGA Initialization**: Clear screen and prepare text output
2. **Welcome Message**: Display system identification
3. **IDT Setup**: Initialize interrupt descriptor table with exception and hardware interrupt handlers
4. **Memory Management**: Initialize physical memory allocator from Multiboot2 memory map
5. **Testing** (if enabled): Run feature-gated tests for exceptions or memory
6. **Ready State**: Enter halt loop, ready to handle interrupts

This foundation provides a solid base for implementing additional operating system features.
