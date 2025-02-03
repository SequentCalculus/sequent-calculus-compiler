    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
extern println_i64
global asm_main0
global asm_main1
global asm_main2
global asm_main3
global asm_main4
global asm_main5

asm_main0:

asm_main1:

asm_main2:

asm_main3:

asm_main4:

asm_main5:
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

main:
    ; jump l
    jmp l

l:
    ; lit x <- 1;
    mov rdx, 1
    ; lit y <- 9;
    mov rdi, 9
    ; jump j
    jmp j

j:
    ; z <- x + y;
    mov r9, rdi
    add r9, rdx
    ; println_i64 z;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, r9
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    add rsp, 8
    ; lit ret <- 0;
    mov r11, 0
    ; return ret
    mov rax, r11
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