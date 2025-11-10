; ============================================================================
; NoodleOS Boot Code - Long Mode Transition
; ============================================================================
; This code is called by GRUB in 32-bit protected mode and transitions the
; CPU from 32-bit protected mode to 64-bit long mode, then calls the Rust kernel.
;
; GRUB Entry State:
;   - CPU is in 32-bit protected mode with paging disabled
;   - A20 line is enabled
;   - EAX = 0x36d76289 (Multiboot2 magic number)
;   - EBX = physical address of Multiboot information structure
;   - CS = valid code descriptor with offset 0 and limit 0xFFFFFFFF
;   - DS, ES, FS, GS, SS = valid data descriptors with offset 0 and limit 0xFFFFFFFF
;   - Interrupts are disabled
;
; Transition Steps:
;   1. Verify Multiboot2 boot, CPUID support, and Long Mode capability
;   2. Set up identity-mapped page tables (virtual == physical for first 1GB)
;   3. Enable PAE (Physical Address Extension)
;   4. Set Long Mode Enable bit in EFER MSR
;   5. Enable paging (activates long mode)
;   6. Load 64-bit GDT
;   7. Jump to 64-bit code and call Rust kernel
; ============================================================================

section .text
bits 32

; Entry point called by GRUB (multiboot specification)
global _start
_start:
    ; Set up stack pointer for 32-bit code
    ; ESP = address of top of stack (grows downward from stack_top)
    mov esp, stack_top
    
    ; Save multiboot information for later use in kernel
    ; EBX = physical address of Multiboot2 info structure from GRUB
    ; EAX = 0x36d76289 (Multiboot2 magic number to verify bootloader)
    mov [multiboot_info_ptr], ebx  ; Save multiboot info structure pointer
    mov [multiboot_magic], eax      ; Save multiboot magic number
    
    ; Verify CPU capabilities before attempting long mode transition
    call check_multiboot     ; Verify GRUB loaded us (EAX should be 0x36d76289)
    call check_cpuid         ; Verify CPU supports CPUID instruction
    call check_long_mode     ; Verify CPU supports 64-bit long mode
    
    ; Set up page tables for long mode (required before enabling paging)
    call setup_page_tables   ; Create identity-mapped page tables (1GB)
    call enable_paging       ; Enable PAE, long mode, and paging
    
    ; Load 64-bit GDT and switch to long mode
    ; GDTR = address and size of our 64-bit GDT
    lgdt [gdt64.pointer]
    
    ; Jump to 64-bit code segment (far jump updates CS and enters long mode)
    ; CS = gdt64.code_segment (offset 8), RIP = long_mode_start
    jmp gdt64.code_segment:long_mode_start

; ============================================================================
; Check if we were loaded by a Multiboot2-compliant bootloader
; ============================================================================
; Entry: EAX = magic number passed by bootloader
; Exit:  Returns if valid, jumps to error if invalid
; ============================================================================
check_multiboot:
    ; Compare EAX with Multiboot2 magic: 0x36d76289
    ; GRUB sets EAX to this value if it loaded us correctly
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "M"             ; Error code 'M' for Multiboot failure
    jmp error

; ============================================================================
; Check if CPUID instruction is supported
; ============================================================================
; The CPUID instruction is supported if we can flip bit 21 (ID bit) of EFLAGS.
; On older CPUs, this bit cannot be modified and remains fixed.
;
; Entry: None
; Exit:  Returns if supported, jumps to error if not
; Clobbers: EAX, ECX
; ============================================================================
check_cpuid:
    pushfd                   ; Save EFLAGS to stack
    pop eax                  ; Load EFLAGS into EAX
    mov ecx, eax             ; Save original EFLAGS in ECX for later comparison
    xor eax, 1 << 21         ; Flip ID bit (bit 21) in EAX
    push eax                 ; Push modified EFLAGS to stack
    popfd                    ; Load modified EFLAGS into EFLAGS register
    pushfd                   ; Save EFLAGS again (to see if bit 21 stuck)
    pop eax                  ; Load potentially-modified EFLAGS back into EAX
    push ecx                 ; Restore original EFLAGS to stack
    popfd                    ; Load original EFLAGS back into EFLAGS register
    cmp eax, ecx             ; Compare: did bit 21 actually flip?
    je .no_cpuid             ; If EAX == ECX, bit didn't flip = no CPUID support
    ret
.no_cpuid:
    mov al, "C"              ; Error code 'C' for CPUID not supported
    jmp error

; ============================================================================
; Check if CPU supports 64-bit long mode (x86-64)
; ============================================================================
; Uses CPUID extended functions to detect long mode support.
; First checks if extended CPUID functions are available (EAX >= 0x80000001).
; Then checks bit 29 of EDX from CPUID function 0x80000001 (LM bit).
;
; Entry: None
; Exit:  Returns if supported, jumps to error if not
; Clobbers: EAX, EBX, ECX, EDX
; ============================================================================
check_long_mode:
    ; First check if extended CPUID functions are available
    mov eax, 0x80000000      ; CPUID function: Get Highest Extended Function
    cpuid                    ; Returns max extended function number in EAX
    cmp eax, 0x80000001      ; Need at least 0x80000001 for feature bits
    jb .no_long_mode         ; Jump if EAX < 0x80000001 (unsigned comparison)
    
    ; Now get extended feature information
    mov eax, 0x80000001      ; CPUID function: Extended Processor Info and Features
    cpuid                    ; Returns feature bits in EDX and ECX
    test edx, 1 << 29        ; Test bit 29 of EDX (LM: Long Mode bit)
    jz .no_long_mode         ; Jump if bit 29 is 0 (long mode not supported)
    ret
.no_long_mode:
    mov al, "L"              ; Error code 'L' for Long Mode not supported
    jmp error

; ============================================================================
; Set up page tables for identity mapping (virtual address == physical address)
; ============================================================================
; Creates a 4-level page table hierarchy for x86-64 long mode:
;   - PML4 (Page Map Level 4): Top level, 1 entry pointing to PDPT
;   - PDPT (Page Directory Pointer Table): 1 entry pointing to PDT
;   - PDT (Page Directory Table): 512 entries, each mapping 2MB (total 1GB)
;   - PT (Page Table): Not used - we use 2MB huge pages instead
;
; Memory Layout:
;   CR3 → PML4[0] → PDPT[0] → PDT[0..511] → Physical Memory (2MB pages)
;
; Each entry is 8 bytes with format:
;   Bits 0-11:  Flags (Present, Write, etc.)
;   Bits 12-51: Physical address (4KB aligned)
;   Bits 52-63: Reserved/flags
;
; Entry: None
; Exit:  Page tables configured, CR3 set to PML4 base
; Clobbers: EAX, EBX, ECX, EDI
; ============================================================================
setup_page_tables:
    ; Clear all page table memory (12KB total: 3 pages of 4KB each)
    mov edi, page_table_l4   ; EDI = destination address (PML4 start)
    mov cr3, edi             ; CR3 = page table root (PML4 physical address)
    xor eax, eax             ; EAX = 0 (value to write)
    mov ecx, 4096            ; ECX = 4096 dwords to clear (3 × 4KB ÷ 4 bytes)
    rep stosd                ; Repeat: [EDI] = EAX, EDI += 4, ECX -= 1
    mov edi, cr3             ; Restore EDI to PML4 base address
    
    ; Set up PML4 (Page Map Level 4) - top level page table
    ; PML4[0] points to PDPT (covers first 512GB of virtual address space)
    ; Flags: 0x03 = bit 0 (Present) + bit 1 (Read/Write)
    mov DWORD [edi], page_table_l3 + 0x03    ; Low 32 bits: address + flags
    
    ; Set up PDPT (Page Directory Pointer Table) - second level
    ; PDPT[0] points to PDT (covers first 1GB of virtual address space)
    ; Flags: 0x03 = Present + Read/Write
    mov edi, page_table_l3
    mov DWORD [edi], page_table_l2 + 0x03    ; Low 32 bits: address + flags
    
    ; Set up PDT (Page Directory Table) - third level with 2MB huge pages
    ; Each PDT entry maps 2MB of physical memory (no PT needed)
    ; Flags: 0x83 = bit 0 (Present) + bit 1 (Read/Write) + bit 7 (Page Size = 2MB)
    mov edi, page_table_l2
    mov ebx, 0x83            ; EBX = first page address (0x0) + flags (0x83)
    mov ecx, 512             ; ECX = 512 entries (512 × 2MB = 1GB total)
    
.map_page_table_l2:
    mov DWORD [edi], ebx     ; Set low 32 bits of page table entry
    add ebx, 0x200000        ; Next 2MB page (0x200000 = 2MB)
    add edi, 8               ; Next entry (each entry is 8 bytes = 64 bits)
    loop .map_page_table_l2  ; Decrement ECX and loop if ECX != 0
    ret

; ============================================================================
; Enable PAE, Long Mode, and Paging
; ============================================================================
; This function performs the critical steps to activate 64-bit long mode:
;   1. Enable PAE (Physical Address Extension) - required for long mode
;   2. Set LME (Long Mode Enable) bit in EFER MSR
;   3. Enable paging (which activates long mode because LME is set)
;
; After this returns, CPU is in "compatibility mode" (32-bit code in 64-bit mode)
; A far jump to 64-bit code segment is required to fully enter 64-bit long mode
;
; Entry: CR3 must be set to valid page table (done in setup_page_tables)
; Exit:  CPU in compatibility mode, ready for far jump to 64-bit code
; Clobbers: EAX, ECX, EDX
; ============================================================================
enable_paging:
    ; Step 1: Enable PAE (Physical Address Extension) in CR4
    ; PAE is required for long mode; it enables 64-bit page table entries
    mov eax, cr4             ; Read CR4 control register
    or eax, 1 << 5           ; Set PAE bit (bit 5) in CR4
    mov cr4, eax             ; Write modified value back to CR4
    
    ; Step 2: Enable long mode in EFER MSR (Model Specific Register)
    ; EFER (Extended Feature Enable Register) controls CPU extended features
    ; MSR 0xC0000080 = EFER register
    mov ecx, 0xC0000080      ; ECX = MSR number (EFER)
    rdmsr                    ; Read MSR into EDX:EAX (EDX=high 32 bits, EAX=low 32)
    or eax, 1 << 8           ; Set LME bit (Long Mode Enable, bit 8) in low 32 bits
    wrmsr                    ; Write EDX:EAX back to MSR specified in ECX
    
    ; Step 3: Enable paging in CR0 (this activates long mode)
    ; Because LME is set in EFER and PAE is enabled, enabling paging
    ; causes the CPU to enter IA-32e mode (compatibility mode)
    mov eax, cr0             ; Read CR0 control register
    or eax, 1 << 31          ; Set PG bit (Paging Enable, bit 31) in CR0
    mov cr0, eax             ; Write modified value back to CR0
    ret                      ; Return in compatibility mode (still executing 32-bit code)

; ============================================================================
; Error handler - display error code and halt
; ============================================================================
; Displays "ER:X" on screen where X is the error code, then halts the CPU.
; Uses VGA text mode memory at physical address 0xB8000.
;
; VGA Text Memory Format (each character = 2 bytes):
;   Byte 0: ASCII character code
;   Byte 1: Attribute (bits 0-3=foreground, 4-7=background)
;     0x4f = white (0xf) on red (0x4) background
;
; Entry: AL = ASCII error code character ('M', 'C', or 'L')
; Exit:  Does not return (halts CPU)
; ============================================================================
error:
    ; Display error message at VGA text buffer (physical address 0xB8000)
    ; Each DWORD writes 2 characters (4 bytes total)
    mov dword [0xb8000], 0x4f524f45  ; "ER" - 'E'=0x45, attr=0x4f, 'R'=0x52, attr=0x4f
    mov dword [0xb8004], 0x4f3a4f52  ; "R:" - 'R'=0x52, attr=0x4f, ':'=0x3a, attr=0x4f  
    mov dword [0xb8008], eax         ; Error code character in AL (with attribute 0x4f)
    hlt                              ; Halt CPU until next interrupt (but interrupts disabled)

; ============================================================================
; 64-bit Long Mode Entry Point
; ============================================================================
; This code executes in true 64-bit long mode after the far jump from 32-bit code.
; At this point:
;   - CPU is in 64-bit mode with paging enabled
;   - CS = 64-bit code segment (from GDT)
;   - Segment registers need to be loaded with 64-bit data segment
;   - Stack pointer needs to be set up for 64-bit addressing
;
; We then call the Rust kernel_main function with System V AMD64 ABI calling convention:
;   - First argument in RDI (multiboot info pointer)
;   - Second argument in RSI (multiboot magic)
; ============================================================================
bits 64
long_mode_start:
    ; Load all segment registers with data segment selector
    ; In long mode, segmentation is mostly unused (flat memory model)
    ; but segment registers must still contain valid selectors
    mov ax, gdt64.data_segment   ; AX = data segment selector (offset 16 in GDT)
    mov ss, ax                   ; SS = stack segment (data segment)
    mov ds, ax                   ; DS = data segment (general data)
    mov es, ax                   ; ES = extra segment (not really used)
    mov fs, ax                   ; FS = additional data segment (can be used for TLS)
    mov gs, ax                   ; GS = additional data segment (can be used for CPU-local)
    
    ; Set up 64-bit stack pointer
    ; RSP = top of stack (64KB stack in .bss section)
    mov rsp, stack_top
    
    ; Call Rust kernel main function using System V AMD64 calling convention
    ; RDI = first argument (physical address of Multiboot2 info structure)
    ; RSI = second argument (Multiboot2 magic number for verification)
    mov rdi, [multiboot_info_ptr]  ; Load multiboot info address into RDI
    mov rsi, [multiboot_magic]      ; Load multiboot magic number into RSI
    extern kernel_main
    call kernel_main               ; Call the Rust kernel entry point
    
    ; If kernel_main ever returns (it shouldn't - it's marked as -> !), halt
    ; This is a safety catch in case of bugs
    hlt

; ============================================================================
; Data Section - Initialized Data
; ============================================================================
section .data

; Storage for multiboot information passed from GRUB
; These are filled in at boot time from EAX and EBX registers
multiboot_info_ptr:
    dq 0                     ; Physical address of Multiboot2 info structure
multiboot_magic:
    dq 0                     ; Multiboot2 magic number (should be 0x36d76289)

; ============================================================================
; BSS Section - Uninitialized Data
; ============================================================================
section .bss
align 4096                   ; Page tables must be page-aligned (4KB = 0x1000)

; Page tables for long mode (3 levels × 4KB each = 12KB total)
; These are cleared and populated by setup_page_tables function
page_table_l4:
    resb 4096                ; PML4 (Page Map Level 4) - 512 entries × 8 bytes
page_table_l3:
    resb 4096                ; PDPT (Page Directory Pointer Table) - 512 entries × 8 bytes
page_table_l2:
    resb 4096                ; PDT (Page Directory Table) - 512 entries × 8 bytes

; Stack space (64KB = 65536 bytes)
; Stack grows downward from stack_top toward stack_bottom
stack_bottom:
    resb 65536               ; Reserve 64KB for stack
stack_top:                   ; Top of stack (highest address)

; ============================================================================
; Read-Only Data Section
; ============================================================================
section .rodata

; ============================================================================
; Global Descriptor Table (GDT) for 64-bit Long Mode
; ============================================================================
; The GDT defines memory segments. In long mode, segmentation is mostly unused
; (flat memory model), but we still need a minimal GDT with code and data segments.
;
; Each GDT entry is 8 bytes (64 bits) with this structure:
;   Bits 0-15:   Limit low (ignored in long mode)
;   Bits 16-39:  Base low (ignored in long mode)
;   Bits 40-47:  Access byte (Present, DPL, Type, etc.)
;   Bits 48-51:  Limit high (ignored in long mode)
;   Bits 52-55:  Flags (Granularity, Size, Long mode, etc.)
;   Bits 56-63:  Base high (ignored in long mode)
; ============================================================================
gdt64:
    dq 0                     ; Entry 0: Null descriptor (required by x86 architecture)

.code_segment: equ $ - gdt64 ; Offset 8: Code segment selector
    ; 64-bit code segment descriptor
    ; Bit 43: Executable (1 = code segment)
    ; Bit 44: Descriptor type (1 = code/data segment, 0 = system segment)
    ; Bit 47: Present (1 = segment is valid and in memory)
    ; Bit 53: Long mode (1 = 64-bit code segment)
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)

.data_segment: equ $ - gdt64 ; Offset 16: Data segment selector
    ; 64-bit data segment descriptor
    ; Bit 41: Writable (1 = data segment is writable)
    ; Bit 44: Descriptor type (1 = code/data segment)
    ; Bit 47: Present (1 = segment is valid and in memory)
    dq (1<<44) | (1<<47) | (1<<41)

.pointer:
    ; GDT pointer structure for LGDT instruction
    dw $ - gdt64 - 1         ; Limit: size of GDT - 1 (in bytes)
    dq gdt64                 ; Base: linear address of GDT
