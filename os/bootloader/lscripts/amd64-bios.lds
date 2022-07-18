ENTRY(stage1)

SECTIONS {
    __bootloader_load_adr = 0x01000000;
    __bootloader_stack_top = 0x00EFFFF0; 

    . = 0x500;
    __disk_load_buffer = .;
    . += 512;
    . = ALIGN(0x1000);
    __pml4t_adr = .;
    . += 0x1000;
    __pd = .;

    .bootstrap 0x7C00 : AT (0) {
        *(.boot.stage1)
        __bootstrap_load_adr = .;
        *(.boot .boot.*)
        . = ALIGN(512);
        __bootstrap_size = . - __bootstrap_load_adr;
    }
    __bootstrap_size = SIZEOF(.bootstrap);

    .bootloader __bootloader_load_adr : AT(__bootstrap_size) {
        __bootloader_load_beg = .;
        *(.setup)
        *(.text .text.*)
        *(.rodata .rodata.*)
        *(.data .data.*)
        *(.bss .bss.*)
        *(.eh_frame)
        *(.eh_frame_hdr)
        . = ALIGN(512);
        __bootloader_load_size = . - __bootloader_load_beg;
    }
}


