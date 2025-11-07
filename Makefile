# NoodleOS Makefile

# Target architecture
TARGET = x86_64-unknown-none

# Directories
BUILD_DIR = target/$(TARGET)/release
ISO_DIR = iso
KERNEL_BIN = $(BUILD_DIR)/noodleos
KERNEL_DEST = $(ISO_DIR)/boot/noodleos.bin
ISO_FILE = noodleos.iso

# Tools
RUSTC = rustc
CARGO = cargo
NASM = nasm
LD = ld
GRUB_MKRESCUE = grub-mkrescue
QEMU = qemu-system-x86_64

# Flags
RUST_TARGET_PATH = $(shell pwd)
export RUST_TARGET_PATH

# Default target
all: $(ISO_FILE)

# Build the kernel
kernel: src/multiboot_header.o src/boot.o
	$(CARGO) build --release --target $(TARGET)

# Assemble multiboot header
src/multiboot_header.o: src/multiboot_header.s
	$(NASM) -f elf64 src/multiboot_header.s -o src/multiboot_header.o

# Assemble boot code (long mode transition)
src/boot.o: src/boot.s
	$(NASM) -f elf64 src/boot.s -o src/boot.o

# Create the kernel binary
$(KERNEL_BIN): kernel
	$(LD) -n -T linker.ld -o $(KERNEL_BIN) src/multiboot_header.o src/boot.o $(BUILD_DIR)/libnoodleos.a

# Copy kernel to ISO directory
$(KERNEL_DEST): $(KERNEL_BIN)
	cp $(KERNEL_BIN) $(KERNEL_DEST)

# Create ISO image
$(ISO_FILE): $(KERNEL_DEST)
	$(GRUB_MKRESCUE) -o $(ISO_FILE) $(ISO_DIR)

# Run in QEMU
run: $(ISO_FILE)
	$(QEMU) -cdrom $(ISO_FILE)

# Run in QEMU with debugging
debug: $(ISO_FILE)
	$(QEMU) -cdrom $(ISO_FILE) -s -S

# Clean build artifacts
clean:
	$(CARGO) clean
	rm -f src/multiboot_header.o
	rm -f src/boot.o
	rm -f $(ISO_FILE)
	rm -f $(KERNEL_DEST)

# Clean everything including target directory
distclean: clean
	rm -rf target/

.PHONY: all kernel run debug clean distclean
