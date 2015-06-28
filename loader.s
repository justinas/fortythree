bits 32

section .text
    ;multiboot spec
    align 4
    dd 0x1BADB002            ;magic
    dd 0x00                  ;flags
    dd - (0x1BADB002 + 0x00) ;checksum. m+f+c should be zero

global load_gdt
load_gdt:
    mov eax, [esp+4]
    lgdt [eax]
    jmp 0x08:.reload_segments
.reload_segments:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret

global start
extern setup_gdt
extern setup_idt
extern kmain
start:
    cli

    mov esp, stack_space
    call setup_gdt
    call setup_idt
    sti

    call kmain
.halt:
    hlt
    jmp .halt

section .bss
resb 8192
stack_space:
