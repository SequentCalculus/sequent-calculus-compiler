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
    ; lit a_1 <- 1;
    mov rax, 1
    ; lit b_2 <- 3;
    mov rdx, 3
    ; c_3 <- a_1 - b_2;
    mov rsi, rax
    sub rsi, rdx
    ; lit d_4 <- 8;
    mov rdi, 8
    ; lit e_5 <- -1;
    mov r8, -1
    ; f_6 <- e_5 * d_4;
    mov r9, r8
    imul r9, rdi
    ; g_7 <- f_6 + c_3;
    mov r10, r9
    add r10, rsi
    ; lit h_8 <- -6;
    mov r11, -6
    ; i_9 <- h_8 * g_7;
    mov r12, r11
    imul r12, r10
    ; println_i64 i_9;
    ; #save caller-save registers
    mov r13, rax
    mov r14, rdx
    mov r15, rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    ; #move argument into place
    mov rdi, r12
    call println_i64
    ; #restore caller-save registers
    mov rax, r13
    mov rdx, r14
    mov rsi, r15
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    ; lit ret_10 <- 0;
    mov r13, 0
    ; exit ret_10
    mov rax, r13
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