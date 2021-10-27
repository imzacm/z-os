BITS 32

; Stack
section .bss
align 16
stack_bottom:
resb 16384 ; 16 KiB
stack_top:

section .text
global _start:function (_start.end - _start)
_start:
    mov esp, stack_top

    extern x86_kernel_entry
    call x86_kernel_entry

    extern kernel_main
    call kernel_main

    cli
.hang: hlt
    jmp .hang
.end:

global reload_segments
reload_segments:
    jmp 0x08:reload_cs
reload_cs:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret
