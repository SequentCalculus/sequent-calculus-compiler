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
mov rdx, 1
mov rdi, 9
mov [rbx + 56], rdi
mov qword [rbx + 48], 0
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rsi, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab12
mov qword [rsi + 0], 0
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
mov rdi, 2
lea rcx, [rel Either14]
add rcx, rdi
jmp rcx

Either14:
jmp Either14Left
jmp Either14Right

Either14Left:
cmp qword [rsi + 0], 0
je lab15
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
jmp lab16

lab15:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]

lab16:
jmp cleanup

Either14Right:
cmp qword [rsi + 0], 0
je lab17
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
jmp lab18

lab17:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]

lab18:
mov r9, rdi
add r9, rdx
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