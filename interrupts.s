bits 32

%define PIC1_OFFSET 0x20
%define PIC2_OFFSET 0x28

%macro outb 2
    push eax
    mov al, %2
    out %1, al
    pop eax
%endmacro

%macro io_wait 0
    push eax
    mov al, 0
    out 0x80, al
    pop eax
%endmacro

%macro make_interrupt_handler_code 1
global interrupt_handler_%1
interrupt_handler_%1:
    push %1
    call interrupt_handler
    add esp, 8 ; revert err_code + int_no push
    iret
%endmacro

%macro make_interrupt_handler_no_code 1
global interrupt_handler_%1
interrupt_handler_%1:
    push -1 ; dummy error code
    push %1
    call interrupt_handler
    add esp, 8
    iret
%endmacro

%macro make_master_irq_handler 1
global interrupt_handler_%1
interrupt_handler_%1:
    push -1
    push %1

    call interrupt_handler
    add esp, 8

    outb 0x20, 0x20 ; send EOI
    iret
%endmacro

%macro make_slave_irq_handler 1
global interrupt_handler_%1
interrupt_handler_%1:
    push -1
    push %1

    call interrupt_handler
    add esp, 8

    ; send EOI
    outb 0x20, 0x20
    outb 0xa0, 0x20
    iret
%endmacro

section .text

extern interrupt_handler

make_interrupt_handler_no_code 0
make_interrupt_handler_no_code 1
make_interrupt_handler_no_code 2
make_interrupt_handler_no_code 3
make_interrupt_handler_no_code 4
make_interrupt_handler_no_code 5
make_interrupt_handler_no_code 6
make_interrupt_handler_no_code 7
make_interrupt_handler_code 8
make_interrupt_handler_no_code 9
make_interrupt_handler_code 10
make_interrupt_handler_code 11
make_interrupt_handler_code 12
make_interrupt_handler_code 13
make_interrupt_handler_code 14
make_interrupt_handler_no_code 15
make_interrupt_handler_no_code 16
make_interrupt_handler_code 17
make_interrupt_handler_no_code 18
make_interrupt_handler_no_code 19
make_interrupt_handler_no_code 20
make_interrupt_handler_no_code 21
make_interrupt_handler_no_code 22
make_interrupt_handler_no_code 23
make_interrupt_handler_no_code 24
make_interrupt_handler_no_code 25
make_interrupt_handler_no_code 26
make_interrupt_handler_no_code 27
make_interrupt_handler_no_code 28
make_interrupt_handler_no_code 29
make_interrupt_handler_code 30
make_interrupt_handler_no_code 31
make_master_irq_handler 32
make_master_irq_handler 33
make_master_irq_handler 34
make_master_irq_handler 35
make_master_irq_handler 36
make_master_irq_handler 37
make_master_irq_handler 38
make_master_irq_handler 39
make_slave_irq_handler 40
make_slave_irq_handler 41
make_slave_irq_handler 42
make_slave_irq_handler 43
make_slave_irq_handler 44
make_slave_irq_handler 45
make_slave_irq_handler 46
make_slave_irq_handler 47


global load_idt
load_idt:
    mov eax, [esp+4]
    lidt [eax]

global setup_pic
setup_pic:
    outb 0x20, 0x10 + 0x01 ; init sequence
    io_wait
    outb 0xA0, 0x10 + 0x01 ; ditto, slave PIC
    io_wait
    outb 0x21, PIC1_OFFSET
    io_wait
    outb 0xA1, PIC2_OFFSET
    io_wait

    outb 0x21, 0x01 ; x86 mode
    io_wait
    outb 0xA1, 0x01 ; ditto
    io_wait

    outb 0x21, 0x00
    io_wait
    outb 0xA1, 0x00
    io_wait

    ret

