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
    ; lit x1 <- 1;
    mov rdx, 1
    ; lit x2 <- 2;
    mov rdi, 2
    ; lit x3 <- -3;
    mov r9, -3
    ; lit x4 <- 4;
    mov r11, 4
    ; lit x5 <- 5;
    mov r13, 5
    ; lit x6 <- -6;
    mov r15, -6
    ; lit x7 <- 7;
    mov qword [rsp + 2024], 7
    ; lit x8 <- 8;
    mov qword [rsp + 2008], 8
    ; lit x9 <- 9;
    mov qword [rsp + 1992], 9
    ; lit x10 <- -10;
    mov qword [rsp + 1976], -10
    ; lit x11 <- 11;
    mov qword [rsp + 1960], 11
    ; println_i64 x10;
    ; #move argument to TEMP before adapting the stack pointer
    mov rcx, [rsp + 1976]
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
    ; substitute (x1 !-> x1)(x2 !-> x2)(x3 !-> x3)(x4 !-> x4)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8)(x9 !-> x9)(x11 !-> x11);
    ; #move variables
    mov rcx, [rsp + 1960]
    mov [rsp + 1976], rcx
    ; println_i64 x1;
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; substitute (x11 !-> x11)(x2 !-> x2)(x3 !-> x3)(x4 !-> x4)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8)(x9 !-> x9);
    ; #move variables
    mov rdx, [rsp + 1976]
    ; print_i64 x2;
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call print_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3)(x4 !-> x4)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8);
    ; #move variables
    mov rdi, [rsp + 1992]
    ; println_i64 x4;
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, r11
    call println_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3)(x8 !-> x8)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7);
    ; #move variables
    mov r11, [rsp + 2008]
    ; println_i64 x5;
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, r13
    call println_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3)(x8 !-> x8)(x7 !-> x7)(x6 !-> x6);
    ; #move variables
    mov r13, [rsp + 2024]
    ; print_i64 x8;
    ; #save caller-save registers
    push rdx
    push rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, r11
    call print_i64
    ; #restore caller-save registers
    add rsp, 8
    pop r11
    pop r9
    pop rdi
    pop rdx
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3)(x6 !-> x6)(x7 !-> x7);
    ; #move variables
    mov r11, r15
    ; println_i64 x6;
    ; #save caller-save registers
    mov r14, rdx
    mov r15, rdi
    push r9
    push r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, r11
    call println_i64
    ; #restore caller-save registers
    mov rdx, r14
    mov rdi, r15
    add rsp, 8
    pop r11
    pop r9
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3)(x7 !-> x7);
    ; #move variables
    mov r11, r13
    ; println_i64 x7;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    mov r15, r11
    sub rsp, 8
    ; #move argument into place
    mov rdi, r11
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    mov r11, r15
    add rsp, 8
    ; substitute (x11 !-> x11)(x9 !-> x9)(x3 !-> x3);
    ; print_i64 x3;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, r9
    call print_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    add rsp, 8
    ; substitute (x11 !-> x11)(x9 !-> x9);
    ; println_i64 x9;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    add rsp, 8
    ; substitute (x11 !-> x11);
    ; println_i64 x11;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; substitute ;
    ; lit x0 <- 0;
    mov rdx, 0
    ; return x0
    mov rax, rdx
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