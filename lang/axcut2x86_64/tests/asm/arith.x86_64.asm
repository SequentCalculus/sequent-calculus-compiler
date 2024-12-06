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
mov rdx, 1
mov rdi, 3
mov r9, rdx
sub r9, rdi
mov r11, 8
mov r13, -1
mov r15, r13
imul r15, r11
mov rcx, r15
add rcx, r9
mov [rsp + 2024], rcx
mov qword [rsp + 2008], -6
mov rcx, [rsp + 2008]
imul rcx, [rsp + 2024]
mov [rsp + 1992], rcx
mov rdx, [rsp + 1992]
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