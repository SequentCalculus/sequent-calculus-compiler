; asmsyntax=nasm
;
; To create an executable:
; $ nasm -f elf64 midi.x86_64.asm
; $ gcc -o midi path/to/X86_64-infrastructure/driver$MODE.c midi.x86_64.o
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
mov rax, 0
lea rdx, [rel ContInt3]
mov [rbx + 56], rdx
mov [rbx + 48], rax
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rax, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab15
mov qword [rax + 0], 0
jmp lab16

lab15:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab13
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
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
mov rcx, [rbx + 32]
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
mov rcx, [rbx + 16]
cmp rcx, 0
je lab12
cmp qword [rcx + 0], 0
je lab10
add qword [rcx + 0], -1
jmp lab11

lab10:
mov [rcx + 0], rbp
mov rbp, rcx

lab11:

lab12:
jmp lab14

lab13:
mov rbp, rbx
add rbp, 64

lab14:

lab16:
lea rdx, [rel ContList17]
mov rsi, 0
mov rdi, 0
mov r9, 3
jmp range

ContList17:

ContList17Retl:
cmp qword [rsi + 0], 0
je lab22
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
mov rsi, [rsi + 48]
cmp rsi, 0
je lab21
add qword [rsi + 0], 1

lab21:
jmp lab23

lab22:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]
mov rsi, [rsi + 48]

lab23:
mov rcx, rsi
mov rsi, rax
mov rax, rcx
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
jmp sum

ContInt3:

ContInt3Reti:
mov rdx, rdx
jmp cleanup

range:
cmp r9, 0
je lab24
mov r8, rsi
mov rsi, rax
mov rcx, r9
mov r11, r9
mov r9, rdi
mov rdi, rdx
mov rdx, rcx
mov [rbx + 56], r11
mov qword [rbx + 48], 0
mov [rbx + 40], r9
mov [rbx + 32], r8
mov qword [rbx + 16], 0
mov r8, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab36
mov qword [r8 + 0], 0
jmp lab37

lab36:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab34
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
cmp rcx, 0
je lab27
cmp qword [rcx + 0], 0
je lab25
add qword [rcx + 0], -1
jmp lab26

lab25:
mov [rcx + 0], rbp
mov rbp, rcx

lab26:

lab27:
mov rcx, [rbx + 32]
cmp rcx, 0
je lab30
cmp qword [rcx + 0], 0
je lab28
add qword [rcx + 0], -1
jmp lab29

lab28:
mov [rcx + 0], rbp
mov rbp, rcx

lab29:

lab30:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab33
cmp qword [rcx + 0], 0
je lab31
add qword [rcx + 0], -1
jmp lab32

lab31:
mov [rcx + 0], rbp
mov rbp, rcx

lab32:

lab33:
jmp lab35

lab34:
mov rbp, rbx
add rbp, 64

lab35:

lab37:
mov r9, 2
mov r11, -1
mov r13, rdx
add r13, r11
mov rax, rsi
mov rdx, rdi
mov rsi, r8
mov rdi, r9
mov r9, r13
jmp range

lab24:
mov rcx, rsi
mov rsi, rax
mov rax, rcx
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
jmp rdi

sum:
lea rcx, [rel List38]
add rcx, rdi
jmp rcx

List38:
jmp List38Nil
jmp List38Cons

List38Nil:
mov rdi, 0
mov rsi, rax
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
jmp rdi

List38Cons:
cmp qword [rsi + 0], 0
je lab45
add qword [rsi + 0], -1
mov r9, [rsi + 56]
mov rdi, [rsi + 40]
mov rsi, [rsi + 32]
cmp rsi, 0
je lab44
add qword [rsi + 0], 1

lab44:
jmp lab46

lab45:
mov [rsi + 0], rbx
mov rbx, rsi
mov r9, [rsi + 56]
mov rdi, [rsi + 40]
mov rsi, [rsi + 32]

lab46:
mov rcx, rsi
mov rsi, rax
mov rax, rcx
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
mov [rbx + 56], r9
mov qword [rbx + 48], 0
mov [rbx + 40], rdi
mov [rbx + 32], rsi
mov qword [rbx + 16], 0
mov rsi, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab58
mov qword [rsi + 0], 0
jmp lab59

lab58:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab56
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
cmp rcx, 0
je lab49
cmp qword [rcx + 0], 0
je lab47
add qword [rcx + 0], -1
jmp lab48

lab47:
mov [rcx + 0], rbp
mov rbp, rcx

lab48:

lab49:
mov rcx, [rbx + 32]
cmp rcx, 0
je lab52
cmp qword [rcx + 0], 0
je lab50
add qword [rcx + 0], -1
jmp lab51

lab50:
mov [rcx + 0], rbp
mov rbp, rcx

lab51:

lab52:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab55
cmp qword [rcx + 0], 0
je lab53
add qword [rcx + 0], -1
jmp lab54

lab53:
mov [rcx + 0], rbp
mov rbp, rcx

lab54:

lab55:
jmp lab57

lab56:
mov rbp, rbx
add rbp, 64

lab57:

lab59:
lea rdi, [rel ContInt60]
mov rcx, rsi
mov rsi, rax
mov rax, rcx
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
jmp sum

ContInt60:

ContInt60Reti:
cmp qword [rsi + 0], 0
je lab67
add qword [rsi + 0], -1
mov r9, [rsi + 56]
mov rdi, [rsi + 40]
mov rsi, [rsi + 32]
cmp rsi, 0
je lab66
add qword [rsi + 0], 1

lab66:
jmp lab68

lab67:
mov [rsi + 0], rbx
mov rbx, rsi
mov r9, [rsi + 56]
mov rdi, [rsi + 40]
mov rsi, [rsi + 32]

lab68:
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