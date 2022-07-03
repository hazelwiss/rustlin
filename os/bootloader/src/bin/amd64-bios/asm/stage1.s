.section .boot.initial, "awx"
.code16

.global _start
_start:
    jmp _start

.org 510
.byte 0x55, 0xAA