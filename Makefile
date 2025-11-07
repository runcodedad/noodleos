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

# Helper variables
COMMA := ,

# Default target
all: $(ISO_FILE)

# Build the kernel
kernel: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(CARGO) build --release --target $(TARGET)

# Assemble multiboot header
src/arch/x86_64/boot/multiboot_header.o: src/arch/x86_64/boot/multiboot_header.s
	$(NASM) -f elf64 src/arch/x86_64/boot/multiboot_header.s -o src/arch/x86_64/boot/multiboot_header.o

# Assemble boot code (long mode transition)
src/arch/x86_64/boot/boot.o: src/arch/x86_64/boot/boot.s
	$(NASM) -f elf64 src/arch/x86_64/boot/boot.s -o src/arch/x86_64/boot/boot.o

# Create the kernel binary
$(KERNEL_BIN): kernel
	$(LD) -n -T linker.ld -o $(KERNEL_BIN) src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o $(BUILD_DIR)/libnoodleos.a

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
	rm -f src/arch/x86_64/boot/multiboot_header.o
	rm -f src/arch/x86_64/boot/boot.o
	rm -f $(ISO_FILE)
	rm -f $(KERNEL_DEST)

# Clean everything including target directory
distclean: clean
	rm -rf target/

# Quick test - verify OS builds and boots
test: $(ISO_FILE)
	./quick_test.sh

# List available test targets
list-tests:
	@echo "Available test targets:"
	@echo "  Build only: $(addprefix test-,$(TEST_TARGETS))"
	@echo "  Run tests:  $(addprefix run-test-,$(TEST_TARGETS))"
	@echo "  Debug tests: $(addprefix debug-test-,$(TEST_TARGETS))"
	@echo ""
	@echo "Example usage:"
	@echo "  make run-test-divide-by-zero"
	@echo "  make debug-test-exceptions"

# Helper function to build kernel with specific features
define build_test_kernel
	$(CARGO) build --release --target $(TARGET) --features $(1)
	$(LD) -n -T linker.ld -o $(KERNEL_BIN) src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o $(BUILD_DIR)/libnoodleos.a
	cp $(KERNEL_BIN) $(KERNEL_DEST)
	$(GRUB_MKRESCUE) -o $(ISO_FILE) $(ISO_DIR)
endef

# Specific test build targets (explicit for reliability)
test-exceptions: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,test-exceptions)

test-divide-by-zero: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,test-exceptions$(COMMA)test-divide-by-zero)

test-memory: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,test-memory)

test-hardware: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,test-hardware)

# Explicit run test targets (pattern rules weren't working reliably)
run-test-exceptions: test-exceptions
	$(QEMU) -cdrom $(ISO_FILE)

run-test-divide-by-zero: test-divide-by-zero
	$(QEMU) -cdrom $(ISO_FILE)

run-test-memory: test-memory
	$(QEMU) -cdrom $(ISO_FILE)

run-test-hardware: test-hardware
	$(QEMU) -cdrom $(ISO_FILE)

# Explicit debug test targets
debug-test-exceptions: test-exceptions
	$(QEMU) -cdrom $(ISO_FILE) -s -S

debug-test-divide-by-zero: test-divide-by-zero
	$(QEMU) -cdrom $(ISO_FILE) -s -S

debug-test-memory: test-memory
	$(QEMU) -cdrom $(ISO_FILE) -s -S

debug-test-hardware: test-hardware
	$(QEMU) -cdrom $(ISO_FILE) -s -S

# Available test targets (for documentation and make completion)
TEST_TARGETS = exceptions divide-by-zero memory hardware

# Mark test targets as phony so they always rebuild
.PHONY: all kernel run debug clean distclean test list-tests $(addprefix test-,$(TEST_TARGETS)) $(addprefix run-test-,$(TEST_TARGETS)) $(addprefix debug-test-,$(TEST_TARGETS))
