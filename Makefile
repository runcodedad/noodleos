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

# QEMU Configuration
QEMU_MEMORY ?= 128M  # Default memory size, can be overridden: make run QEMU_MEMORY=256M
QEMU_FLAGS = -m $(QEMU_MEMORY)

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
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

# Run in QEMU with debugging
debug: $(ISO_FILE)
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

# Memory size tests - verify allocator with different RAM amounts
test-mem-64m: $(ISO_FILE)
	@echo "Testing with 64MB RAM..."
	$(QEMU) -m 64M -cdrom $(ISO_FILE)

test-mem-128m: $(ISO_FILE)
	@echo "Testing with 128MB RAM (default)..."
	$(QEMU) -m 128M -cdrom $(ISO_FILE)

test-mem-256m: $(ISO_FILE)
	@echo "Testing with 256MB RAM..."
	$(QEMU) -m 256M -cdrom $(ISO_FILE)

test-mem-512m: $(ISO_FILE)
	@echo "Testing with 512MB RAM..."
	$(QEMU) -m 512M -cdrom $(ISO_FILE)

test-mem-1g: $(ISO_FILE)
	@echo "Testing with 1GB RAM..."
	$(QEMU) -m 1G -cdrom $(ISO_FILE)

test-mem-2g: $(ISO_FILE)
	@echo "Testing with 2GB RAM..."
	$(QEMU) -m 2G -cdrom $(ISO_FILE)

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
	tests/scripts/quick_test.sh

# Build all tests
test-all: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	@echo "Building kernel with all tests enabled..."
	$(call build_test_kernel,run-tests$(COMMA)test-exceptions$(COMMA)test-memory$(COMMA)test-virtual-memory$(COMMA)test-hardware)

# Run all tests
run-test-all: test-all
	@echo "Running all tests..."
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

# Debug all tests
debug-test-all: test-all
	@echo "Running all tests with debugger..."
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

# List available test targets
list-tests:
	@echo "Available test targets:"
	@echo "  Build all:  test-all"
	@echo "  Run all:    run-test-all"
	@echo "  Debug all:  debug-test-all"
	@echo ""
	@echo "  Build only: $(addprefix test-,$(TEST_TARGETS))"
	@echo "  Run tests:  $(addprefix run-test-,$(TEST_TARGETS))"
	@echo "  Debug tests: $(addprefix debug-test-,$(TEST_TARGETS))"
	@echo ""
	@echo "Example usage:"
	@echo "  make run-test-all           # Run all tests"
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
	$(call build_test_kernel,run-tests$(COMMA)test-memory)

test-virtual-memory: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,run-tests$(COMMA)test-virtual-memory)

test-hardware: src/arch/x86_64/boot/multiboot_header.o src/arch/x86_64/boot/boot.o
	$(call build_test_kernel,run-tests$(COMMA)test-hardware)

# Explicit run test targets (pattern rules weren't working reliably)
run-test-exceptions: test-exceptions
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

run-test-divide-by-zero: test-divide-by-zero
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

run-test-memory: test-memory
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

run-test-hardware: test-hardware
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE)

# Explicit debug test targets
debug-test-exceptions: test-exceptions
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

debug-test-divide-by-zero: test-divide-by-zero
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

debug-test-memory: test-memory
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

debug-test-hardware: test-hardware
	$(QEMU) $(QEMU_FLAGS) -cdrom $(ISO_FILE) -s -S

# Available test targets (for documentation and make completion)
TEST_TARGETS = exceptions divide-by-zero memory virtual-memory hardware
MEMORY_TEST_TARGETS = test-mem-64m test-mem-128m test-mem-256m test-mem-512m test-mem-1g test-mem-2g

# Mark test targets as phony so they always rebuild
.PHONY: all kernel run debug clean distclean test list-tests test-all run-test-all debug-test-all $(addprefix test-,$(TEST_TARGETS)) $(addprefix run-test-,$(TEST_TARGETS)) $(addprefix debug-test-,$(TEST_TARGETS)) $(MEMORY_TEST_TARGETS)
