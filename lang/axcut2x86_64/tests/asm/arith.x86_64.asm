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
    mov rdx, 1
    ; lit b_2 <- 3;
    mov rdi, 3
    ; c_3 <- a_1 - b_2;
    mov r9, rdx
    sub r9, rdi
    ; lit d_4 <- 8;
    mov r11, 8
    ; lit e_5 <- -1;
    mov r13, -1
    ; f_6 <- e_5 * d_4;
    mov r15, r13
    imul r15, r11
    ; g_7 <- f_6 + c_3;
    mov rcx, r15
    add rcx, r9
    mov [rsp + 2024], rcx
    ; lit h_8 <- -6;
    mov qword [rsp + 2008], -6
    ; i_9 <- h_8 * g_7;
    mov rcx, [rsp + 2008]
    imul rcx, [rsp + 2024]
    mov [rsp + 1992], rcx
    ; println_i64 i_9;
    ; #move argument to TEMP before adapting the stack pointer
    mov rcx, [rsp + 1992]
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, rcx
    call println_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; lit ret_10 <- 0;
    mov qword [rsp + 1976], 0
    ; exit ret_10
    mov rax, [rsp + 1976]
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