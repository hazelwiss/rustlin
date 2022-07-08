ENTRY(stage1)

SECTIONS {
    __disk_load_buffer = 500;
    __bootloader_load_adr = 0x01000000;
    
    .bootstrap 0x7C00 : AT (.) {
        *(.boot.stage1)
        __bootstrap_load_adr = .;
        *(.boot.stage2) 
        *(.boot .boot.*)
        . = ALIGN(512);
        __bootstrap_size = . - __bootstrap_load_adr;
    }
    __bootstrap_end = ADDR(.bootstrap) + SIZEOF(.bootstrap);

    .bootloader __bootloader_load_adr : AT(__bootstrap_end) {
        __bootloader_load_beg = .;
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.bss .bss.*)
        . = ALIGN(512);
        __bootloader_load_size = . - __bootloader_load_beg;
    }
}


