# Copilot Instructions for This Project

This repository is a learning-focused operating system project written in **Rust**.  
The OS will be booted using **GRUB**, run in **QEMU**, and built with a **Rust + `x86_64-elf` cross compiler toolchain**.

When generating code or suggestions, follow these guidelines:

---

## Project Goals
- Implement an x86_64 operating system kernel from scratch.
- Use **Rust** for safety and clarity, minimizing `unsafe` where possible.
- Use GRUB as the bootloader, conforming to the **Multiboot2** specification.
- Run the OS in QEMU during development.
- Extend the OS step-by-step: boot → memory → interrupts → drivers → tasks → filesystem → userland.

---

## Language & Coding Rules

### Prefer:
- Rust stable when possible.
- Minimal and clearly justified use of `unsafe`.
- Clear and well-commented implementation.
- Modular code organization (separate architecture-specific and generic logic).

### Avoid:
- Using the Rust standard library (we will use `#![no_std]`).
- Dependencies that assume a hosted environment.
- High-level abstractions that hide hardware behavior.

---

## Build & Toolchain Requirements

### We will:
- Build a **custom cross-compiler** target: `x86_64-unknown-none`.
- Use `bootimage` or pure Multiboot2 loading via GRUB.
- Use `cargo xbuild` / `cargo build -Z build-std` as needed.

### Always generate commands assuming:
```sh
rustup target add x86_64-unknown-none
```

### Cross-compiler reference tools:
- `nasm` (if using assembly stubs)
- `ld` from `binutils`
- `grub-mkrescue`
- `qemu-system-x86_64`

---

## File and Module Layout (Preferred)
Suggest code organized like this:

```
src/
  lib.rs
  main.rs (kernel entry, if used)
  arch/
    x86_64/
      boot/
      interrupts/
      memory/
      drivers/
  task/
  fs/
  user/
build/
scripts/
```

---

## When Copilot Generates Code:
1. Assume **Multiboot2** booting environment via GRUB.
2. Assume `#![no_std]` and `#![no_main]`.
3. Provide helpful inline documentation for key low-level concepts:
   - GDT / IDT setup
   - paging and memory maps
   - PIC/APIC and timer interrupts
   - PS/2 keyboard interaction
4. Avoid suggesting platform-specific APIs or `std`.

---

## Example Kernel Entry Expectations

Copilot should generate kernel entry points like:

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Kernel entry point
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

---

## If the user asks about framebuffer output:
Suggest using:
- VGA text buffer for early output
- Later allow switch to linear framebuffer for graphics modes

---

## If the user requests help writing a driver:
- Start with polling-based basic implementations
- Later refactor to interrupt-driven versions

---

## If the user requests debugging help:
Suggest using:
```sh
qemu-system-x86_64 -kernel target/... -s -S
gdb
```

---

### End of Copilot Instructions
This file should influence all Copilot completions in this repository.
