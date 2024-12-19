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
    ; lit c <- 5;
    mov rdx, 5
    ; m <- mmap_anonymous_page;
    mov r12, rax
    mov r15, rdx
    mov rax, 9
    mov rdi, 0
    mov rsi, 4096
    mov rdx, 3
    mov r10, 34
    mov r8, -1
    mov r9, 0
    syscall
    mov rdx, r15
    mov rdi, rax
    mov rax, r12
    ; r <- read_stdin m c;
    mov r12, rax
    mov r13, rdi
    mov r14, rsi
    mov r15, rdx
    mov rsi, rdi
    mov rdx, rdx
    mov rax, 0
    mov rdi, 0
    syscall
    mov rdi, r13
    mov rsi, r14
    mov rdx, r15
    mov r9, rax
    mov rax, r12
    ; w <- write_stdout m r;
    mov r12, rax
    mov r13, rdi
    mov r14, rsi
    mov r15, rdx
    mov rsi, rdi
    mov rdx, r9
    mov rax, 1
    mov rdi, 1
    syscall
    mov rdi, r13
    mov rsi, r14
    mov rdx, r15
    mov r11, rax
    mov rax, r12
    ; munmap_page m;
    mov r12, rax
    mov r13, r11
    mov r14, rdi
    mov r15, rsi
    mov rax, 11
    mov rdi, rdi
    mov rsi, 4096
    syscall
    mov r11, r13
    mov rdi, r14
    mov rsi, r15
    mov rax, r12
    ; return w
    mov rdx, r11
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