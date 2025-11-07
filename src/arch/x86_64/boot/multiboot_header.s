section .multiboot_header
align 8

multiboot_header_start:
    dd 0xe85250d6                ; multiboot2 magic number
    dd 0                         ; architecture 0 (protected mode i386)
    dd multiboot_header_end - multiboot_header_start ; header length
    dd -(0xe85250d6 + 0 + (multiboot_header_end - multiboot_header_start)) ; checksum

    ; insert optional multiboot tags here

    ; required terminating tag
    dw 0    ; type
    dw 0    ; flags
    dd 8    ; size
multiboot_header_end:
