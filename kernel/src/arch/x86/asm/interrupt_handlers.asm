%macro interrupt_handler 1
global interrupt_handler_%1
interrupt_handler_%1:
    push dword 0
    push dword %1
    jmp common_interrupt_handler
%endmacro

%macro interrupt_error_handler 1
global interrupt_handler_%1
interrupt_handler_%1:
    push dword %1
    jmp common_interrupt_handler
%endmacro

extern x86_interrupt_handler
common_interrupt_handler:
    push ds
    push es
    push fs
    push gs
    pushad
    call x86_interrupt_handler
    popad
    pop gs
    pop fs
    pop es
    pop ds
    add esp, 8
    iret

interrupt_handler 0
interrupt_handler 1
interrupt_handler 2
interrupt_handler 3
interrupt_handler 4
interrupt_handler 5
interrupt_handler 6
interrupt_handler 7
interrupt_error_handler 8
interrupt_handler 9
interrupt_error_handler 10
interrupt_error_handler 11
interrupt_error_handler 12
interrupt_error_handler 13
interrupt_error_handler 14
interrupt_handler 15
interrupt_handler 16
interrupt_error_handler 17
interrupt_handler 18
interrupt_handler 19
interrupt_handler 20
interrupt_handler 21
interrupt_handler 22
interrupt_handler 23
interrupt_handler 24
interrupt_handler 25
interrupt_handler 26
interrupt_handler 27
interrupt_handler 28
interrupt_handler 29
interrupt_error_handler 30
interrupt_handler 31

%macro pic_interrupt_handler 1
global pic_interrupt_handler_%1
pic_interrupt_handler_%1:
    push dword 0
    push dword %1
    jmp common_pic_interrupt_handler
%endmacro

extern x86_pic_interrupt_handler
common_pic_interrupt_handler:
    push ds
    push es
    push fs
    push gs
    pushad
    call x86_pic_interrupt_handler
    popad
    pop gs
    pop fs
    pop es
    pop ds
    add esp, 8
    iret

pic_interrupt_handler 0
pic_interrupt_handler 1
pic_interrupt_handler 2
pic_interrupt_handler 3
pic_interrupt_handler 4
pic_interrupt_handler 5
pic_interrupt_handler 6
pic_interrupt_handler 7
pic_interrupt_handler 8
pic_interrupt_handler 9
pic_interrupt_handler 10
pic_interrupt_handler 11
pic_interrupt_handler 12
pic_interrupt_handler 13
pic_interrupt_handler 14
pic_interrupt_handler 15
