ENTRY(_start)

SECTIONS {
    . = 1M;

    __kernel_start = .;

    .boot :
    {
        /* ensure that the multiboot header is at the beginning */
        *(.multiboot_header)
    }

    .text :
    {
        /* 32-bit boot code comes first */
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        /* Page tables and stack from boot.s */
        *(.bss)
    }

    __kernel_end = .;
}
