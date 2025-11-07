# NoodleOS

A minimal operating system kernel written in Rust that boots with GRUB, transitions to 64-bit long mode, and prints "Hello from NoodleOS - 64-bit Long Mode!" to the screen.

## Features

- **Multiboot2 compliant**: Works with GRUB bootloader
- **Manual Long Mode Transition**: Boots in 32-bit mode and manually transitions to 64-bit
- **VGA Text Mode**: Basic text output to screen
- **Rust no_std**: Bare metal Rust kernel
- **QEMU Ready**: Designed to run in QEMU emulator
- **Educational**: Well-documented architecture and boot process

## Prerequisites

Before building NoodleOS, you need to install the following tools:

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install build-essential nasm qemu-system-x86 grub-pc-bin xorriso mtools
```

### Arch Linux
```bash
sudo pacman -S base-devel nasm qemu grub xorriso mtools
```

### macOS (using Homebrew)
```bash
brew install nasm qemu grub xorriso mtools
# You may also need to install binutils
brew install binutils
```

### Rust Toolchain

#### Option 1: Install via rustup (Recommended)
Install Rust using the official rustup installer from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
rustup target add x86_64-unknown-none
```

#### Option 2: Install via Package Manager
If you prefer using your system's package manager:

**Arch Linux:**
```bash
# Remove the rust package if already installed
sudo pacman -Rs rust
# Install rustup instead
sudo pacman -S rustup
# Set up the toolchain
rustup install stable
rustup target add x86_64-unknown-none
```

**Ubuntu/Debian:**
```bash
sudo apt install rustup
rustup install stable
rustup target add x86_64-unknown-none
```

## Building

### Quick Build
Use the provided build script:

```bash
./build.sh
```

### Manual Build
Alternatively, you can build manually using the Makefile:

```bash
# Build the kernel and create ISO
make all

# Or build step by step:
make kernel    # Build the Rust kernel
make           # Create ISO image
```

## Running

### Run in QEMU
```bash
make run
```

This will:
1. Start QEMU with the NoodleOS ISO
2. Boot using GRUB 
3. Load the kernel
4. Display "Hi from NoodleOS!" in yellow text

### Debug Mode
To run with GDB debugging support:

```bash
make debug
```

Then in another terminal:
```bash
gdb
(gdb) target remote :1234
(gdb) continue
```

### Cleaning Build Artifacts
To clean up build artifacts:

```bash
# Clean build artifacts but keep target directory
make clean

# Clean everything including Rust target directory
make distclean
```

## Project Structure

```
noodleos/
├── src/
│   ├── lib.rs               # Kernel entry point and main library
│   ├── vga_buffer.rs        # VGA text mode driver
│   └── multiboot_header.s   # Multiboot2 header assembly
├── iso/
│   └── boot/
│       ├── grub/
│       │   └── grub.cfg     # GRUB configuration
│       └── noodleos.bin     # Kernel binary (generated)
├── target/                  # Build artifacts (ignored by git)
├── .gitignore              # Git ignore file
├── Cargo.toml              # Rust project configuration
├── Makefile                # Build automation
├── linker.ld               # Linker script
├── build.sh                # Build script
├── rust-toolchain.toml     # Rust toolchain specification
├── x86_64-unknown-none.json # Custom target specification
├── noodleos.iso            # Final ISO image (generated, ignored by git)
└── README.md               # This file
```

## How It Works

1. **GRUB Bootloader**: GRUB loads the kernel binary according to the Multiboot2 specification
2. **Multiboot Header**: The assembly file creates a valid Multiboot2 header that GRUB recognizes
3. **Kernel Entry**: The `_start` function in `main.rs` is called by GRUB
4. **VGA Output**: The kernel uses the VGA text buffer at memory address `0xb8000` to display text
5. **Halt**: The kernel enters an infinite loop with `hlt` instruction to save CPU cycles

## Build Artifacts

When you build NoodleOS, the following files are generated:

- `target/` - Rust build artifacts (ignored by git)
- `src/multiboot_header.o` - Compiled multiboot header object file
- `iso/boot/noodleos.bin` - Final kernel binary
- `noodleos.iso` - Bootable ISO image ready for QEMU

All build artifacts are ignored by git, so they won't be committed to your repository.

## Code Overview

### Main Kernel (`src/lib.rs`)
- Entry point: `_start()` function
- Clears screen and prints message
- Enters halt loop

### VGA Buffer (`src/vga_buffer.rs`)
- Provides text output functionality
- Manages VGA color attributes
- Implements screen clearing and scrolling

### Multiboot Header (`src/multiboot_header.s`)
- Assembly code that creates the Multiboot2 header
- Required for GRUB to recognize the kernel as bootable

## Troubleshooting

### Build Issues

**Error: `rustup: command not found`**
You need to install rustup (the Rust toolchain installer). See the Rust Toolchain section above for installation options.

**Error: `x86_64-unknown-none` target not found**
```bash
rustup target add x86_64-unknown-none
```

**Error: `nasm` command not found**
Install NASM assembler for your platform (see Prerequisites section).

**Error: `grub-mkrescue` command not found**
Install GRUB tools for your platform (see Prerequisites section).

**Error: `xorriso not found`**
Install xorriso package:
- Arch Linux: `sudo pacman -S xorriso`
- Ubuntu/Debian: `sudo apt install xorriso`

**Error: `mtools not found` (during ISO creation)**
Install mtools package:
- Arch Linux: `sudo pacman -S mtools`
- Ubuntu/Debian: `sudo apt install mtools`

**Error: `rustup and rust are in conflict`**
If you have the system rust package installed, remove it first:
```bash
sudo pacman -Rs rust  # On Arch Linux
sudo pacman -S rustup
```

### Runtime Issues

**QEMU doesn't start**
- Ensure QEMU is installed: `qemu-system-x86_64 --version`
- Try running with explicit path: `/usr/bin/qemu-system-x86_64 -cdrom noodleos.iso`

**Screen shows only GRUB menu**
- Check that `noodleos.bin` exists in `iso/boot/`
- Verify GRUB configuration in `iso/boot/grub/grub.cfg`

**Kernel doesn't print message**
- The kernel might have crashed during boot
- Try debugging with: `make debug` and connecting with GDB

## Next Steps

This minimal kernel can be extended with:

- Interrupt handling (IDT setup)
- Memory management (paging, heap allocation)
- Keyboard input handling
- Basic shell/command interface
- File system support
- Process management
- Device drivers

## Resources

- [OSDev Wiki](https://wiki.osdev.org/) - Comprehensive OS development resource
- [Multiboot2 Specification](https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html)
- [Intel 64 and IA-32 Architecture Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [Rust Embedded Book](https://docs.rust-embedded.org/book/)

## License

This project is released into the public domain. Feel free to use it as a starting point for your own OS development adventures!
