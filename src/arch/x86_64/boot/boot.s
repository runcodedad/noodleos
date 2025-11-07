; NoodleOS Boot Code - Long Mode Transition
; This code is called by GRUB in 32-bit protected mode
; It transitions the CPU to 64-bit long mode and calls the Rust kernel

section .text
bits 32

; Entry point called by GRUB (multiboot specification)
global _start
_start:
    ; Set up stack pointer for 32-bit code
    mov esp, stack_top
    
    ; Save multiboot information (if needed later)
    mov edi, ebx    ; Multiboot info structure pointer
    mov esi, eax    ; Multiboot magic number
    
    ; Check if we're running on a supported CPU
    call check_multiboot
    call check_cpuid
    call check_long_mode
    
    ; Set up page tables for long mode
    call setup_page_tables
    call enable_paging
    
    ; Load 64-bit GDT and switch to long mode
    lgdt [gdt64.pointer]
    
    ; Jump to 64-bit code
    jmp gdt64.code_segment:long_mode_start

; Check if we were loaded by a multiboot bootloader
check_multiboot:
    cmp eax, 0x36d76289     ; Multiboot2 magic number
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "M"
    jmp error

; Check if CPUID instruction is supported
check_cpuid:
    pushfd                   ; Save EFLAGS
    pop eax                  ; Load EFLAGS into EAX
    mov ecx, eax             ; Save original EFLAGS
    xor eax, 1 << 21         ; Flip ID bit (bit 21)
    push eax                 ; Save modified EFLAGS
    popfd                    ; Load modified EFLAGS
    pushfd                   ; Save EFLAGS again
    pop eax                  ; Load EFLAGS into EAX
    push ecx                 ; Restore original EFLAGS
    popfd                    ; Load original EFLAGS
    cmp eax, ecx             ; Compare with original
    je .no_cpuid             ; If same, CPUID not supported
    ret
.no_cpuid:
    mov al, "C"
    jmp error

; Check if long mode is supported
check_long_mode:
    mov eax, 0x80000000      ; Extended CPUID function
    cpuid                    ; Call CPUID
    cmp eax, 0x80000001      ; Check if extended functions available
    jb .no_long_mode         ; If not, no long mode
    
    mov eax, 0x80000001      ; Extended feature information
    cpuid                    ; Call CPUID
    test edx, 1 << 29        ; Test long mode bit (bit 29)
    jz .no_long_mode         ; If not set, no long mode
    ret
.no_long_mode:
    mov al, "L"
    jmp error

; Set up page tables for identity mapping (virtual = physical)
setup_page_tables:
    ; Clear page table area
    mov edi, page_table_l4
    mov cr3, edi             ; Set page table root
    xor eax, eax             ; Clear EAX
    mov ecx, 4096            ; Clear 4 pages (4096 * 4 = 16KB)
    rep stosd                ; Clear memory
    mov edi, cr3             ; Restore page table pointer
    
    ; Set up PML4 (Page Map Level 4)
    mov DWORD [edi], page_table_l3 + 0x03    ; Present + Write
    
    ; Set up PDPT (Page Directory Pointer Table)  
    mov edi, page_table_l3
    mov DWORD [edi], page_table_l2 + 0x03    ; Present + Write
    
    ; Set up PDT (Page Directory Table) with 2MB pages
    mov edi, page_table_l2
    mov ebx, 0x83            ; Present + Write + Large Page (2MB)
    mov ecx, 512             ; 512 entries = 1GB of mapped memory
    
.map_page_table_l2:
    mov DWORD [edi], ebx     ; Set page entry
    add ebx, 0x200000        ; Next 2MB page
    add edi, 8               ; Next entry (8 bytes per entry)
    loop .map_page_table_l2
    ret

; Enable paging and long mode
enable_paging:
    ; Enable PAE (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5           ; Set PAE bit (bit 5)
    mov cr4, eax
    
    ; Enable long mode in EFER MSR (Model Specific Register)
    mov ecx, 0xC0000080      ; EFER MSR
    rdmsr                    ; Read MSR into EDX:EAX
    or eax, 1 << 8           ; Set Long Mode Enable bit (bit 8)
    wrmsr                    ; Write MSR
    
    ; Enable paging
    mov eax, cr0
    or eax, 1 << 31          ; Set paging bit (bit 31)
    mov cr0, eax
    ret

; Error handler - display error character and halt
error:
    ; Display error character in VGA memory (white on red)
    mov dword [0xb8000], 0x4f524f45  ; "ER" in white on red
    mov dword [0xb8004], 0x4f3a4f52  ; "R:" in white on red  
    mov dword [0xb8008], eax         ; Error code
    hlt

; 64-bit code starts here
bits 64
long_mode_start:
    ; Clear segment registers (not used in long mode)
    mov ax, gdt64.data_segment
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    ; Set up 64-bit stack
    mov rsp, stack_top
    
    ; Call Rust kernel main function
    extern kernel_main
    call kernel_main
    
    ; If kernel_main ever returns (shouldn't happen), halt
    hlt

section .bss
align 4096

; Page tables (16KB total)
page_table_l4:
    resb 4096                ; PML4 (Page Map Level 4)
page_table_l3:
    resb 4096                ; PDPT (Page Directory Pointer Table)
page_table_l2:
    resb 4096                ; PDT (Page Directory Table)

; Stack space (64KB)
stack_bottom:
    resb 65536
stack_top:

section .rodata

; Global Descriptor Table for 64-bit mode
gdt64:
    dq 0                     ; Null descriptor (required)

.code_segment: equ $ - gdt64
    ; Code segment descriptor
    ; Base: 0, Limit: 0xFFFFF (ignored in long mode)
    ; Flags: Present, Ring 0, Code, Executable, Long mode
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53)

.data_segment: equ $ - gdt64  
    ; Data segment descriptor
    ; Base: 0, Limit: 0xFFFFF (ignored in long mode)
    ; Flags: Present, Ring 0, Data, Writable
    dq (1<<44) | (1<<47) | (1<<41)

.pointer:
    dw $ - gdt64 - 1         ; GDT size
    dq gdt64                 ; GDT address
