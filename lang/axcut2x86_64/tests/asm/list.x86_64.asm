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
    mov rax, 0
    mov rdx, 0
    mov rdi, 5
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
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
    mov rdx, 2
    mov rdi, 7
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    mov qword [rbx + 16], 0
    mov rax, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab25
    mov qword [rax + 0], 0
    jmp lab26

lab25:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab23
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab16
    cmp qword [rcx + 0], 0
    je lab14
    add qword [rcx + 0], -1
    jmp lab15

lab14:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab15:

lab16:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab19
    cmp qword [rcx + 0], 0
    je lab17
    add qword [rcx + 0], -1
    jmp lab18

lab17:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab18:

lab19:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab22
    cmp qword [rcx + 0], 0
    je lab20
    add qword [rcx + 0], -1
    jmp lab21

lab20:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab21:

lab22:
    jmp lab24

lab23:
    mov rbp, rbx
    add rbp, 64

lab24:

lab26:
    mov rdx, 2
    mov rdi, 9
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    mov qword [rbx + 16], 0
    mov rax, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab38
    mov qword [rax + 0], 0
    jmp lab39

lab38:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab36
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab29
    cmp qword [rcx + 0], 0
    je lab27
    add qword [rcx + 0], -1
    jmp lab28

lab27:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab28:

lab29:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab32
    cmp qword [rcx + 0], 0
    je lab30
    add qword [rcx + 0], -1
    jmp lab31

lab30:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab31:

lab32:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab35
    cmp qword [rcx + 0], 0
    je lab33
    add qword [rcx + 0], -1
    jmp lab34

lab33:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab34:

lab35:
    jmp lab37

lab36:
    mov rbp, rbx
    add rbp, 64

lab37:

lab39:
    mov rdx, 2
    lea rcx, [rel List40]
    add rcx, rdx
    jmp rcx

List40:
    jmp List40Nil
    jmp List40Cons

List40Nil:
    jmp cleanup

List40Cons:
    cmp qword [rax + 0], 0
    je lab43
    add qword [rax + 0], -1
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab42
    add qword [rax + 0], 1

lab42:
    jmp lab44

lab43:
    mov [rax + 0], rbx
    mov rbx, rax
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab44:
    mov rdx, rdi
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