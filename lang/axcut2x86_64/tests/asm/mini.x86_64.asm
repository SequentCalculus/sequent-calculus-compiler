    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
extern print_i64
extern println_i64
global asm_main

asm_main:
    ; setup
    ; save registers
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    ; reserve space for register spills
    sub rsp, 2048
    ; initialize heap pointer
    mov rbx, rdi
    ; initialize free pointer
    mov rbp, rbx
    add rbp, 64
    ; move parameters into place
    ; actual code

main_:
    ; l(...)
    jmp l_

l_:
    ; lit x_1 <- 1;
    mov rax, 1
    ; lit y_2 <- 9;
    mov rdx, 9
    ; j(...)
    jmp j_

j_:
    ; z_5 <- x_4 + y_3;
    mov rsi, rdx
    add rsi, rax
    ; println_i64 z_5;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rsi
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rsi, r14
    add rsp, 8
    ; lit ret_6 <- 0;
    mov rdi, 0
    ; exit ret_6
    mov rax, rdi
    jmp cleanup

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