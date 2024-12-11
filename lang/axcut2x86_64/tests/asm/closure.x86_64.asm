; asmsyntax=nasm
segment .text
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
    mov rdx, 9
    mov [rbx + 56], rdx
    mov qword [rbx + 48], 0
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    mov rax, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab12
    mov qword [rax + 0], 0
    jmp lab13

lab12:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab10
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab3
    cmp qword [rcx + 0], 0
    je lab1
    add qword [rcx + 0], -1
    jmp lab2

lab1:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab2:

lab3:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab6
    cmp qword [rcx + 0], 0
    je lab4
    add qword [rcx + 0], -1
    jmp lab5

lab4:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab5:

lab6:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab9
    cmp qword [rcx + 0], 0
    je lab7
    add qword [rcx + 0], -1
    jmp lab8

lab7:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab8:

lab9:
    jmp lab11

lab10:
    mov rbp, rbx
    add rbp, 64

lab11:

lab13:
    lea rdx, [rel Func14]
    mov rsi, 0
    lea rdi, [rel Cont15]
    mov r9, 1
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    jmp r9

Cont15:

Cont15Ret:
    mov rdx, rdx
    jmp cleanup

Func14:

Func14Ap:
    cmp qword [r8 + 0], 0
    je lab16
    add qword [r8 + 0], -1
    mov r9, [r8 + 56]
    jmp lab17

lab16:
    mov [r8 + 0], rbx
    mov rbx, r8
    mov r9, [r8 + 56]

lab17:
    mov r11, r9
    add r11, rdx
    mov rdx, r11
    jmp rdi
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