; asmsyntax=nasm
;
; To create an executable:
; $ nasm -f elf64 nonLinear.x86_64.asm
; $ gcc -o nonLinear path/to/X86_64-infrastructure/driver$MODE.c nonLinear.x86_64.o
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
mov rdx, 3
mov rdi, 3
mov r9, 3
mov r11, 3
mov r13, 3
mov r15, 3
mov qword [rsp + 2024], 3
mov qword [rsp + 2008], 3
mov rcx, [rsp + 2008]
mov [rbx + 56], rcx
mov qword [rbx + 48], 0
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rcx, rbx
mov [rsp + 2016], rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab12
mov qword [rcx + 0], 0
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
mov qword [rsp + 2008], 0
mov rcx, [rsp + 2008]
mov [rbx + 56], rcx
mov rcx, [rsp + 2016]
mov [rbx + 48], rcx
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rcx, rbx
mov [rsp + 2016], rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab25
mov qword [rcx + 0], 0
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
mov qword [rsp + 2008], 0
cmp qword [rsp + 2016], 0
je lab27
mov rcx, [rsp + 2016]
add qword [rcx + 0], 2

lab27:
mov rcx, r13
mov r13, r15
mov r15, [rsp + 2024]
mov [rsp + 2024], r11
mov r11, rcx
mov rcx, [rsp + 2016]
mov [rsp + 2000], rcx
mov rcx, [rsp + 2016]
mov [rsp + 1984], rcx
mov rcx, [rsp + 2008]
mov [rsp + 1992], rcx
mov rcx, [rsp + 2008]
mov [rsp + 1976], rcx
lea rcx, [rel BoxBox28]
jmp rcx

BoxBox28:

BoxBox28BB:
mov rcx, [rsp + 1984]
cmp qword [rcx + 0], 0
je lab33
add qword [rcx + 0], -1
mov [rsp + 2040], rdx
mov rdx, [rsp + 1984]
mov rcx, [rdx + 56]
mov [rsp + 1976], rcx
mov rcx, [rdx + 48]
mov [rsp + 1984], rcx
cmp rcx, 0
je lab32
add qword [rcx + 0], 1

lab32:
mov rdx, [rsp + 2040]
jmp lab34

lab33:
mov [rsp + 2040], rdx
mov rdx, [rsp + 1984]
mov [rdx + 0], rbx
mov rbx, rdx
mov rcx, [rdx + 56]
mov [rsp + 1976], rcx
mov rcx, [rdx + 48]
mov [rsp + 1984], rcx
mov rdx, [rsp + 2040]

lab34:
lea rcx, [rel Box35]
jmp rcx

Box35:

Box35B:
mov rcx, [rsp + 1984]
cmp qword [rcx + 0], 0
je lab38
add qword [rcx + 0], -1
mov [rsp + 2040], rdx
mov rdx, [rsp + 1984]
mov rcx, [rdx + 56]
mov [rsp + 1976], rcx
mov rdx, [rsp + 2040]
jmp lab39

lab38:
mov [rsp + 2040], rdx
mov rdx, [rsp + 1984]
mov [rdx + 0], rbx
mov rbx, rdx
mov rcx, [rdx + 56]
mov [rsp + 1976], rcx
mov rdx, [rsp + 2040]

lab39:
mov rcx, [rsp + 1976]
mov [rbx + 56], rcx
mov qword [rbx + 48], 0
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rcx, rbx
mov [rsp + 1984], rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab51
mov qword [rcx + 0], 0
jmp lab52

lab51:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab49
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
cmp rcx, 0
je lab42
cmp qword [rcx + 0], 0
je lab40
add qword [rcx + 0], -1
jmp lab41

lab40:
mov [rcx + 0], rbp
mov rbp, rcx

lab41:

lab42:
mov rcx, [rbx + 32]
cmp rcx, 0
je lab45
cmp qword [rcx + 0], 0
je lab43
add qword [rcx + 0], -1
jmp lab44

lab43:
mov [rcx + 0], rbp
mov rbp, rcx

lab44:

lab45:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab48
cmp qword [rcx + 0], 0
je lab46
add qword [rcx + 0], -1
jmp lab47

lab46:
mov [rcx + 0], rbp
mov rbp, rcx

lab47:

lab48:
jmp lab50

lab49:
mov rbp, rbx
add rbp, 64

lab50:

lab52:
mov qword [rsp + 1976], 0
mov rcx, [rsp + 1976]
mov [rbx + 56], rcx
mov rcx, [rsp + 1984]
mov [rbx + 48], rcx
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rcx, rbx
mov [rsp + 1984], rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab64
mov qword [rcx + 0], 0
jmp lab65

lab64:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab62
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
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
mov rcx, [rbx + 32]
cmp rcx, 0
je lab58
cmp qword [rcx + 0], 0
je lab56
add qword [rcx + 0], -1
jmp lab57

lab56:
mov [rcx + 0], rbp
mov rbp, rcx

lab57:

lab58:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab61
cmp qword [rcx + 0], 0
je lab59
add qword [rcx + 0], -1
jmp lab60

lab59:
mov [rcx + 0], rbp
mov rbp, rcx

lab60:

lab61:
jmp lab63

lab62:
mov rbp, rbx
add rbp, 64

lab63:

lab65:
mov qword [rsp + 1976], 0
cmp qword [rsp + 1984], 0
je lab68
mov rcx, [rsp + 1984]
cmp qword [rcx + 0], 0
je lab66
add qword [rcx + 0], -1
jmp lab67

lab66:
mov [rcx + 0], rbp
mov rbp, rcx

lab67:

lab68:
cmp qword [rsp + 2016], 0
je lab71
mov rcx, [rsp + 2016]
cmp qword [rcx + 0], 0
je lab69
add qword [rcx + 0], -1
jmp lab70

lab69:
mov [rcx + 0], rbp
mov rbp, rcx

lab70:

lab71:
mov rax, [rsp + 2000]
mov rdx, [rsp + 1992]
mov rdi, 4
mov [rbx + 56], rdi
mov qword [rbx + 48], 0
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rsi, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab83
mov qword [rsi + 0], 0
jmp lab84

lab83:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab81
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
cmp rcx, 0
je lab74
cmp qword [rcx + 0], 0
je lab72
add qword [rcx + 0], -1
jmp lab73

lab72:
mov [rcx + 0], rbp
mov rbp, rcx

lab73:

lab74:
mov rcx, [rbx + 32]
cmp rcx, 0
je lab77
cmp qword [rcx + 0], 0
je lab75
add qword [rcx + 0], -1
jmp lab76

lab75:
mov [rcx + 0], rbp
mov rbp, rcx

lab76:

lab77:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab80
cmp qword [rcx + 0], 0
je lab78
add qword [rcx + 0], -1
jmp lab79

lab78:
mov [rcx + 0], rbp
mov rbp, rcx

lab79:

lab80:
jmp lab82

lab81:
mov rbp, rbx
add rbp, 64

lab82:

lab84:
mov rdi, 0
mov rcx, rsi
mov rsi, rax
mov rax, rcx
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
lea rcx, [rel BoxBox85]
jmp rcx

BoxBox85:

BoxBox85BB:
cmp qword [rsi + 0], 0
je lab90
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
mov rsi, [rsi + 48]
cmp rsi, 0
je lab89
add qword [rsi + 0], 1

lab89:
jmp lab91

lab90:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]
mov rsi, [rsi + 48]

lab91:
lea rcx, [rel Box92]
jmp rcx

Box92:

Box92B:
cmp qword [rsi + 0], 0
je lab95
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
jmp lab96

lab95:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]

lab96:
mov [rbx + 56], rdi
mov qword [rbx + 48], 0
mov qword [rbx + 32], 0
mov qword [rbx + 16], 0
mov rsi, rbx
mov rbx, [rbx + 0]
cmp rbx, 0
je lab108
mov qword [rsi + 0], 0
jmp lab109

lab108:
mov rbx, rbp
mov rbp, [rbp + 0]
cmp rbp, 0
je lab106
mov qword [rbx + 0], 0
mov rcx, [rbx + 48]
cmp rcx, 0
je lab99
cmp qword [rcx + 0], 0
je lab97
add qword [rcx + 0], -1
jmp lab98

lab97:
mov [rcx + 0], rbp
mov rbp, rcx

lab98:

lab99:
mov rcx, [rbx + 32]
cmp rcx, 0
je lab102
cmp qword [rcx + 0], 0
je lab100
add qword [rcx + 0], -1
jmp lab101

lab100:
mov [rcx + 0], rbp
mov rbp, rcx

lab101:

lab102:
mov rcx, [rbx + 16]
cmp rcx, 0
je lab105
cmp qword [rcx + 0], 0
je lab103
add qword [rcx + 0], -1
jmp lab104

lab103:
mov [rcx + 0], rbp
mov rbp, rcx

lab104:

lab105:
jmp lab107

lab106:
mov rbp, rbx
add rbp, 64

lab107:

lab109:
mov rdi, 0
lea rcx, [rel Box110]
jmp rcx

Box110:

Box110B:
cmp qword [rsi + 0], 0
je lab113
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
jmp lab114

lab113:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]

lab114:
mov rsi, rax
mov rcx, rdi
mov rdi, rdx
mov rdx, rcx
lea rcx, [rel Box115]
jmp rcx

Box115:

Box115B:
cmp qword [rsi + 0], 0
je lab118
add qword [rsi + 0], -1
mov rdi, [rsi + 56]
jmp lab119

lab118:
mov [rsi + 0], rbx
mov rbx, rsi
mov rdi, [rsi + 56]

lab119:
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