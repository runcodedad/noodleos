# Kernel Entry Point Structure

## Overview

NoodleOS uses `src/main.rs` as the kernel entry point, following standard operating system development conventions. However, due to the nature of kernel development, it's configured as a staticlib rather than a typical binary.

## Why main.rs?

- **Standard Practice**: Most OS kernels have a clear "main" entry point
- **Intuitive**: `main.rs` immediately signals "this is where the kernel starts"
- **Documentation Clarity**: Easier for new contributors to understand
- **Future Flexibility**: Allows for better separation of kernel and userland code

## Configuration Details

### Cargo.toml Setup
```toml
[lib]
name = "noodleos"
path = "src/main.rs"
crate-type = ["staticlib"]
autobins = false  # Disable automatic binary target
```

### Why Staticlib?

The kernel is built as a static library because:
1. **Assembly Integration**: Must be linked with assembly boot code
2. **Custom Entry Point**: Uses `kernel_main()` instead of standard `main()`
3. **No Runtime**: No Rust standard library or runtime dependencies
4. **Linker Control**: Full control over memory layout and linking

## Entry Flow

1. **GRUB** loads the kernel according to Multiboot2 specification
2. **Assembly boot code** (`src/arch/x86_64/boot/boot.s`) transitions to long mode  
3. **Boot code calls** `kernel_main()` in `src/main.rs`
4. **Kernel initializes** and takes control of the system

## File Structure
```
src/main.rs                     # Kernel entry point
├── mod arch                    # Architecture-specific code
├── kernel_main()               # Called from assembly
└── panic_handler()             # Panic handling
```

This structure provides the best of both worlds: conventional naming with the flexibility needed for kernel development.
