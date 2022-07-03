ENTRY(_start)

SECTIONS {
    . = 0x7C00;
    
    .bootloader : {
        *(.boot.initial .boot .boot.*)
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.bss .bss.*)
    }
}