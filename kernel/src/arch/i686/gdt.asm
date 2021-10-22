; extern gdtr
extern GDTR

global load_gdt_descriptor
load_gdt_descriptor:
    lgdt [GDTR]
    jmp 0x08:flush_segments

flush_segments:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret
