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
    ; lit f1 <- 3;
    mov rdx, 3
    ; lit f2 <- 3;
    mov rdi, 3
    ; lit f3 <- 3;
    mov r9, 3
    ; lit f4 <- 3;
    mov r11, 3
    ; lit f5 <- 3;
    mov r13, 3
    ; lit f6 <- 3;
    mov r15, 3
    ; lit f7 <- 3;
    mov qword [rsp + 2024], 3
    ; lit x <- 3;
    mov qword [rsp + 2008], 3
    ; leta b: Box = B(x);
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
    ; leta bb: BoxBox = BB(b);
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
    ; substitute (f1 !-> f1)(f2 !-> f2)(f3 !-> f3)(f5 !-> f5)(f6 !-> f6)(f7 !-> f7)(f4 !-> f4)(bb3 !-> bb)(bb2 !-> bb)(bb1 !-> bb);
    ;  share bb
    cmp qword [rsp + 2016], 0
    je lab27
    mov rcx, [rsp + 2016]
    add qword [rcx + 0], 2

lab27:
    ;  move variables
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
    ; switch bb1 \{ ... \};
    lea rcx, [rel BoxBox28]
    jmp rcx

BoxBox28:

BoxBox28BB:
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab31
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab30
    add qword [rcx + 0], 1

lab30:
    mov rax, [rsp + 2040]
    jmp lab32

lab31:
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    mov [rax + 0], rbx
    mov rbx, rax
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    mov rax, [rsp + 2040]

lab32:
    ; switch b1 \{ ... \};
    lea rcx, [rel Box33]
    jmp rcx

Box33:

Box33B:
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab34
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]
    jmp lab35

lab34:
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    mov [rax + 0], rbx
    mov rbx, rax
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]

lab35:
    ; leta d1: Box = B(x1);
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    mov rcx, rbx
    mov [rsp + 1984], rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab47
    mov qword [rcx + 0], 0
    jmp lab48

lab47:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab38
    cmp qword [rcx + 0], 0
    je lab36
    add qword [rcx + 0], -1
    jmp lab37

lab36:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab37:

lab38:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41
    cmp qword [rcx + 0], 0
    je lab39
    add qword [rcx + 0], -1
    jmp lab40

lab39:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40:

lab41:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44
    cmp qword [rcx + 0], 0
    je lab42
    add qword [rcx + 0], -1
    jmp lab43

lab42:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43:

lab44:
    jmp lab46

lab45:
    mov rbp, rbx
    add rbp, 64

lab46:

lab48:
    mov qword [rsp + 1976], 0
    ; leta dd1: BoxBox = BB(d1);
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
    je lab60
    mov qword [rcx + 0], 0
    jmp lab61

lab60:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab58
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51
    cmp qword [rcx + 0], 0
    je lab49
    add qword [rcx + 0], -1
    jmp lab50

lab49:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50:

lab51:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54
    cmp qword [rcx + 0], 0
    je lab52
    add qword [rcx + 0], -1
    jmp lab53

lab52:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53:

lab54:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab57
    cmp qword [rcx + 0], 0
    je lab55
    add qword [rcx + 0], -1
    jmp lab56

lab55:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab56:

lab57:
    jmp lab59

lab58:
    mov rbp, rbx
    add rbp, 64

lab59:

lab61:
    mov qword [rsp + 1976], 0
    ; substitute (bb2 !-> bb2);
    ;  erase dd1
    cmp qword [rsp + 1984], 0
    je lab64
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab62
    add qword [rcx + 0], -1
    jmp lab63

lab62:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63:

lab64:
    ;  erase bb3
    cmp qword [rsp + 2016], 0
    je lab67
    mov rcx, [rsp + 2016]
    cmp qword [rcx + 0], 0
    je lab65
    add qword [rcx + 0], -1
    jmp lab66

lab65:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab66:

lab67:
    ;  move variables
    mov rax, [rsp + 2000]
    mov rdx, [rsp + 1992]
    ; lit y <- 4;
    mov rdi, 4
    ; leta a1: Box = B(y);
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    mov rsi, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab79
    mov qword [rsi + 0], 0
    jmp lab80

lab79:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab77
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab70
    cmp qword [rcx + 0], 0
    je lab68
    add qword [rcx + 0], -1
    jmp lab69

lab68:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab69:

lab70:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab73
    cmp qword [rcx + 0], 0
    je lab71
    add qword [rcx + 0], -1
    jmp lab72

lab71:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab72:

lab73:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab76
    cmp qword [rcx + 0], 0
    je lab74
    add qword [rcx + 0], -1
    jmp lab75

lab74:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab75:

lab76:
    jmp lab78

lab77:
    mov rbp, rbx
    add rbp, 64

lab78:

lab80:
    mov rdi, 0
    ; substitute (a1 !-> a1)(bb2 !-> bb2);
    ;  move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch bb2 \{ ... \};
    lea rcx, [rel BoxBox81]
    jmp rcx

BoxBox81:

BoxBox81BB:
    cmp qword [rsi + 0], 0
    je lab84
    add qword [rsi + 0], -1
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab83
    add qword [rsi + 0], 1

lab83:
    jmp lab85

lab84:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab85:
    ; switch b2 \{ ... \};
    lea rcx, [rel Box86]
    jmp rcx

Box86:

Box86B:
    cmp qword [rsi + 0], 0
    je lab87
    add qword [rsi + 0], -1
    mov rdi, [rsi + 56]
    jmp lab88

lab87:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov rdi, [rsi + 56]

lab88:
    ; leta a2: Box = B(x2);
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    mov rsi, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab100
    mov qword [rsi + 0], 0
    jmp lab101

lab100:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab98
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab91
    cmp qword [rcx + 0], 0
    je lab89
    add qword [rcx + 0], -1
    jmp lab90

lab89:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab90:

lab91:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab94
    cmp qword [rcx + 0], 0
    je lab92
    add qword [rcx + 0], -1
    jmp lab93

lab92:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab93:

lab94:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab97
    cmp qword [rcx + 0], 0
    je lab95
    add qword [rcx + 0], -1
    jmp lab96

lab95:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab96:

lab97:
    jmp lab99

lab98:
    mov rbp, rbx
    add rbp, 64

lab99:

lab101:
    mov rdi, 0
    ; switch a2 \{ ... \};
    lea rcx, [rel Box102]
    jmp rcx

Box102:

Box102B:
    cmp qword [rsi + 0], 0
    je lab103
    add qword [rsi + 0], -1
    mov rdi, [rsi + 56]
    jmp lab104

lab103:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov rdi, [rsi + 56]

lab104:
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ;  move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch a1 \{ ... \};
    lea rcx, [rel Box105]
    jmp rcx

Box105:

Box105B:
    cmp qword [rsi + 0], 0
    je lab106
    add qword [rsi + 0], -1
    mov rdi, [rsi + 56]
    jmp lab107

lab106:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov rdi, [rsi + 56]

lab107:
    ; res <- x1 + x2;
    mov r9, rdi
    add r9, rdx
    ; return res
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