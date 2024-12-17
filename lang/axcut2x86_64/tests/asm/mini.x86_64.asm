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
    ; return z
    mov rdx, r9
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