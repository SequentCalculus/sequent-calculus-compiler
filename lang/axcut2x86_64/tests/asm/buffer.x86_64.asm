    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
global asm_main0
global _asm_main0
global asm_main1
global _asm_main1
global asm_main2
global _asm_main2
global asm_main3
global _asm_main3
global asm_main4
global _asm_main4
global asm_main5
global _asm_main5

asm_main0:

_asm_main0:

asm_main1:

_asm_main1:

asm_main2:

_asm_main2:

asm_main3:

_asm_main3:

asm_main4:

_asm_main4:

asm_main5:

_asm_main5:
    ; setup
    ; save registers
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    ; move parameters into place
    ; reserve space for register spills
    sub rsp, 2048
    ; initialize heap pointer
    mov rbx, rdi
    ; initialize free pointer
    mov rbp, rbx
    add rbp, 64
    ; actual code

main:
    ; lit a <- 0;
    mov rdx, 0
    ; lit b <- 3;
    mov rdi, 3
    ; lit c <- 5;
    mov r9, 5
    ; m <- mmap_anonymous_page;
    mov r12, rax
    mov r13, rdi
    mov r14, rsi
    mov r15, rdx
    mov [rsp + 2024], r8
    mov [rsp + 2016], r9
    mov rax, 9
    mov rdi, 0
    mov rsi, 4096
    mov rdx, 3
    mov r10, 34
    mov r8, -1
    mov r9, 0
    syscall
    mov rdi, r13
    mov rsi, r14
    mov rdx, r15
    mov r8, [rsp + 2024]
    mov r9, [rsp + 2016]
    mov r11, rax
    mov rax, r12
    ; set_byte m a c;
    mov rcx, r11
    add rcx, rdx
    mov byte [rcx + 0], r9b
    ; d <- get_byte m a;
    mov rcx, r11
    add rcx, rdx
    movzx r13, byte [rcx + 0]
    ; e <- b + d;
    mov r15, rdi
    add r15, r13
    ; lit f <- 7;
    mov qword [rsp + 2024], 7
    ; set_byte m f e;
    mov rcx, r11
    add rcx, [rsp + 2024]
    mov byte [rcx + 0], r15b
    ; g <- get_byte m f;
    add r11, [rsp + 2024]
    movzx rcx, byte [r11 + 0]
    mov [rsp + 2008], rcx
    sub r11, [rsp + 2024]
    ; munmap_page m;
    mov [rsp + 2000], rax
    mov [rsp + 1992], r11
    mov [rsp + 1984], rdi
    mov [rsp + 1976], rsi
    mov rax, 11
    mov rdi, r11
    mov rsi, 4096
    syscall
    mov r11, [rsp + 1992]
    mov rdi, [rsp + 1984]
    mov rsi, [rsp + 1976]
    mov rax, [rsp + 2000]
    ; return g
    mov rdx, [rsp + 2008]
    jmp cleanup
    ; cleanup

cleanup:
    ; free space for register spills
    add rsp, 2048
    ; restore registers
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    ret