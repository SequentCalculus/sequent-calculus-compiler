; asmsyntax=nasm
;
; To create an executable:
; $ nasm -f elf64 quad.x86_64.asm
; $ gcc -o quad path/to/X86_64-infrastructure/driver$MODE.c quad.x86_64.o
; where $MODE = Args | Debug

segment .text
  global asm_main0, _asm_main0
  global asm_main1, _asm_main1
  global asm_main2, _asm_main2
  global asm_main3, _asm_main3
  global asm_main4, _asm_main4
  global asm_main5, _asm_main5
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
mov rdx, 8
mov rdi, 6
mov r9, 4
mov r11, 2
mov [rbx + 56], r11
mov qword [rbx + 48], 0
mov [rbx + 40], r9
mov qword [rbx + 32], 0
mov [rbx + 24], rdi
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
mov [rbx + 48], rsi
mov [rbx + 40], rdx
mov qword [rbx + 32], 0
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
mov rdx, 0
lea rcx, [rel Quad27]
jmp rcx

Quad27:

Quad27Q:
cmp qword [rax + 0], 0
je lab38
add qword [rax + 0], -1
mov rsi, [rax + 48]
mov rdx, [rax + 40]
mov r11, [rsi + 56]
mov r9, [rsi + 40]
mov rdi, [rsi + 24]
jmp lab39

lab38:
mov [rax + 0], rbx
mov rbx, rax
mov rsi, [rax + 48]
mov rdx, [rax + 40]
mov [rsi + 0], rbx
mov rbx, rsi
mov r11, [rsi + 56]
mov r9, [rsi + 40]
mov rdi, [rsi + 24]

lab39:
mov r13, 7
mov r15, rdx
add r15, r13
mov rdx, r15
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