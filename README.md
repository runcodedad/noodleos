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
3. Load the kernel and transition to 64-bit long mode
4. Display "Hello from NoodleOS - 64-bit Long Mode!" and "IDT initialized successfully!" messages

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

## Testing

NoodleOS includes a comprehensive testing framework for validation and development.

### Quick Start

```bash
# Run tests using the test script
./test_vm.sh                # Virtual memory tests
./test_vm.sh memory         # All memory tests
./test_vm.sh exceptions     # Exception tests

# Or use Make targets directly
make test-virtual-memory && make run
make test-memory && make run
make list-tests             # Show all available tests
```

### Available Test Categories

| Category | Command | Description |
|----------|---------|-------------|
| Virtual Memory | `./test_vm.sh virtual` | Page tables, addresses, translation (28 tests) |
| Memory | `./test_vm.sh memory` | Physical + virtual memory allocators |
| Exceptions | `./test_vm.sh exceptions` | IDT and exception handlers |

### Documentation

For complete testing documentation, see **[Testing Guide](src/tests/docs/README.md)**

Topics covered:
- Running tests with the test script or Make
- All available test categories and flags
- Test output interpretation
- Adding new tests
- Debugging failed tests
- CI/CD integration

### Cleaning Build Artifacts
To clean up build artifacts:

```bash
# Clean build artifacts but keep target directory
make clean

# Clean everything including Rust target directory
make distclean
```

## Project Structure

NoodleOS follows a clean, modular architecture with `src/main.rs` as the kernel entry point and architecture-specific code organized under `src/arch/x86_64/`. The codebase is structured into logical subsystems: boot, interrupts, memory management, and hardware drivers.

For detailed information about the project organization and rationale, see [docs/file-organization.md](docs/file-organization.md).

## How It Works

1. **GRUB Bootloader**: GRUB loads the kernel binary according to the Multiboot2 specification
2. **Multiboot Header**: Assembly code creates a valid Multiboot2 header that GRUB recognizes  
3. **Boot Assembly**: Boot code transitions CPU from 32-bit protected mode to 64-bit long mode
4. **Kernel Entry**: The `kernel_main()` function in `main.rs` is called from assembly
5. **Architecture Setup**: Kernel initializes x86_64-specific components (IDT, VGA driver)
6. **VGA Output**: Uses VGA text buffer at memory address `0xb8000` to display boot messages
7. **Halt**: Enters an infinite loop with `hlt` instruction to save CPU cycles

For detailed explanations, see the [documentation](docs/README.md).

## Build Artifacts

When you build NoodleOS, the following files are generated:

- `target/` - Rust build artifacts (ignored by git)
- `src/arch/x86_64/boot/multiboot_header.o` - Compiled multiboot header object file  
- `src/arch/x86_64/boot/boot.o` - Compiled boot assembly object file
- `iso/boot/noodleos.bin` - Final kernel binary
- `noodleos.iso` - Bootable ISO image ready for QEMU

All build artifacts are ignored by git, so they won't be committed to your repository.

## Code Overview

### Main Kernel (`src/main.rs`)
- Entry point: `kernel_main()` function called from assembly
- Initializes architecture-specific components
- Sets up interrupt handling and displays boot message
- Enters halt loop

### Architecture Layer (`src/arch/`)
- Provides hardware abstraction and architecture-specific functionality
- Currently supports x86_64, designed to be extensible
- Organized into logical subsystems: boot, interrupts, memory, drivers

### Boot System (`src/arch/x86_64/boot/`)
- **`multiboot_header.s`**: Multiboot2 header for GRUB compatibility  
- **`boot.s`**: 32-bit to 64-bit long mode transition assembly
- **`mod.rs`**: Boot information structures and utilities

### Drivers (`src/arch/x86_64/drivers/`)
- **`vga.rs`**: VGA text buffer for kernel output
- Hardware abstraction layer for future driver additions

### Interrupt Handling (`src/arch/x86_64/interrupts/`)
- **`mod.rs`**: Interrupt Descriptor Table (IDT) setup and management
- Foundation for handling CPU exceptions and hardware interrupts

## Documentation

NoodleOS includes comprehensive documentation in the [`docs/`](docs/) directory:

- **[Documentation Index](docs/README.md)** - Overview of all documentation
- **[Architecture Overview](docs/architecture.md)** - System design and principles  
- **[Boot Process](docs/boot-process.md)** - Detailed boot sequence explanation
- **[File Organization](docs/file-organization.md)** - Project structure and rationale
- **[Development Guide](docs/development-guide.md)** - Contributing and workflow guidelines

The documentation focuses on concepts and design decisions to help understand the system architecture.

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
