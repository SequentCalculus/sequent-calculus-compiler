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
    ; let y: TestTable = C3();
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    mov rdx, 15
    ; switch y \{ ... \};
    lea rcx, [rel TestTable_74983]
    add rcx, rdx
    jmp rcx

TestTable_74983:
    jmp near TestTable_74983_C0
    jmp near TestTable_74983_C1
    jmp near TestTable_74983_C2
    jmp near TestTable_74983_C3

TestTable_74983_C0:
    ; lit x1 <- 1;
    mov rdx, 1
    ; lit x2 <- 2;
    mov rdi, 2
    ; x3 <- x1 + x2;
    mov r9, rdx
    add r9, rdi
    ; substitute (x3 !-> x3);
    ; #move variables
    mov rdx, r9
    ; lit x4 <- 3;
    mov rdi, 3
    ; x5 <- x3 + x4;
    mov r9, rdx
    add r9, rdi
    ; substitute (x5 !-> x5);
    ; #move variables
    mov rdx, r9
    ; lit x6 <- 4;
    mov rdi, 4
    ; x7 <- x5 + x6;
    mov r9, rdx
    add r9, rdi
    ; substitute (x7 !-> x7);
    ; #move variables
    mov rdx, r9
    ; lit x8 <- 5;
    mov rdi, 5
    ; x9 <- x7 + x8;
    mov r9, rdx
    add r9, rdi
    ; substitute (x9 !-> x9);
    ; #move variables
    mov rdx, r9
    ; lit x10 <- 6;
    mov rdi, 6
    ; x11 <- x9 + x10;
    mov r9, rdx
    add r9, rdi
    ; substitute (x11 !-> x11);
    ; #move variables
    mov rdx, r9
    ; lit x12 <- 7;
    mov rdi, 7
    ; x13 <- x11 + x12;
    mov r9, rdx
    add r9, rdi
    ; substitute (x13 !-> x13);
    ; #move variables
    mov rdx, r9
    ; lit x14 <- 8;
    mov rdi, 8
    ; x15 <- x13 + x14;
    mov r9, rdx
    add r9, rdi
    ; substitute (x15 !-> x15);
    ; #move variables
    mov rdx, r9
    ; lit x16 <- 9;
    mov rdi, 9
    ; x17 <- x15 + x16;
    mov r9, rdx
    add r9, rdi
    ; substitute (x17 !-> x17);
    ; #move variables
    mov rdx, r9
    ; lit x18 <- 10;
    mov rdi, 10
    ; x19 <- x17 + x18;
    mov r9, rdx
    add r9, rdi
    ; substitute (x19 !-> x19);
    ; #move variables
    mov rdx, r9
    ; lit x20 <- 11;
    mov rdi, 11
    ; x21 <- x19 + x20;
    mov r9, rdx
    add r9, rdi
    ; substitute (x21 !-> x21);
    ; #move variables
    mov rdx, r9
    ; lit x22 <- 12;
    mov rdi, 12
    ; x23 <- x21 + x22;
    mov r9, rdx
    add r9, rdi
    ; substitute (x23 !-> x23);
    ; #move variables
    mov rdx, r9
    ; lit x24 <- 13;
    mov rdi, 13
    ; x25 <- x23 + x24;
    mov r9, rdx
    add r9, rdi
    ; substitute (x25 !-> x25);
    ; #move variables
    mov rdx, r9
    ; lit x26 <- 14;
    mov rdi, 14
    ; x27 <- x25 + x26;
    mov r9, rdx
    add r9, rdi
    ; substitute (x27 !-> x27);
    ; #move variables
    mov rdx, r9
    ; lit x28 <- 15;
    mov rdi, 15
    ; x29 <- x27 + x28;
    mov r9, rdx
    add r9, rdi
    ; substitute (x29 !-> x29);
    ; #move variables
    mov rdx, r9
    ; lit x30 <- 16;
    mov rdi, 16
    ; res <- x29 + x30;
    mov r9, rdx
    add r9, rdi
    ; substitute (res !-> res);
    ; #move variables
    mov rdx, r9
    ; jump share_main_0_
    jmp share_main_0_

TestTable_74983_C1:
    ; lit res <- 1;
    mov rdx, 1
    ; jump share_main_0_
    jmp share_main_0_

TestTable_74983_C2:
    ; lit res <- 2;
    mov rdx, 2
    ; jump share_main_0_
    jmp share_main_0_

TestTable_74983_C3:
    ; lit res <- 3;
    mov rdx, 3
    ; jump share_main_0_
    jmp share_main_0_

share_main_0_:
    ; println_i64 res;
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