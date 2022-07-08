.section .boot, "awx"
.code32

.global 
stage3:
    hlt
    jmp stage3

    mov eax, offset stage4