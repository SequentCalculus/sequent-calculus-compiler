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
    ; new t: ContInt = ()\{ ... \};
    mov rax, 0
    lea rdx, [rel ContInt3]
    ; new k: ContList = (t)\{ ... \};
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
    ; leta zs: List = Nil();
    mov rsi, 0
    mov rdi, 0
    ; lit n <- 3;
    mov r9, 3
    ; substitute (k !-> k)(zs !-> zs)(n !-> n);
    ; jump range
    jmp range

ContList17:

ContList17Retl:
    cmp qword [rsi + 0], 0
    je lab20
    add qword [rsi + 0], -1
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab19
    add qword [rsi + 0], 1

lab19:
    jmp lab21

lab20:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab21:
    ; substitute (t !-> t)(as !-> as);
    ;  move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sum
    jmp sum

ContInt3:

ContInt3Reti:
    ; return r
    mov rdx, rdx
    jmp cleanup

range:
    ; ifz i \{ ... \}
    cmp r9, 0
    je lab22
    ; substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
    ;  move variables
    mov r8, rsi
    mov rsi, rax
    mov rcx, r9
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; leta ys: List = Cons(xs, i);
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov qword [rbx + 16], 0
    mov r8, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab34
    mov qword [r8 + 0], 0
    jmp lab35

lab34:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab32
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab25
    cmp qword [rcx + 0], 0
    je lab23
    add qword [rcx + 0], -1
    jmp lab24

lab23:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab24:

lab25:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab28
    cmp qword [rcx + 0], 0
    je lab26
    add qword [rcx + 0], -1
    jmp lab27

lab26:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab27:

lab28:
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab31
    cmp qword [rcx + 0], 0
    je lab29
    add qword [rcx + 0], -1
    jmp lab30

lab29:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab30:

lab31:
    jmp lab33

lab32:
    mov rbp, rbx
    add rbp, 64

lab33:

lab35:
    mov r9, 5
    ; lit o <- -1;
    mov r11, -1
    ; j <- n + o;
    mov r13, rdx
    add r13, r11
    ; substitute (k !-> k)(ys !-> ys)(j !-> j);
    ;  move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    mov r9, r13
    ; jump range
    jmp range

lab22:
    ; substitute (xs !-> xs)(k !-> k);
    ;  move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k Retl
    jmp rdi

sum:
    ; switch xs \{ ... \};
    lea rcx, [rel List36]
    add rcx, rdi
    jmp rcx

List36:
    jmp near List36Nil
    jmp near List36Cons

List36Nil:
    ; lit z <- 0;
    mov rdi, 0
    ; substitute (z !-> z)(k !-> k);
    ;  move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k Reti
    jmp rdi

List36Cons:
    cmp qword [rsi + 0], 0
    je lab39
    add qword [rsi + 0], -1
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab38
    add qword [rsi + 0], 1

lab38:
    jmp lab40

lab39:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab40:
    ; substitute (ys !-> ys)(k !-> k)(y !-> y);
    ;  move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new j: ContInt = (k, y)\{ ... \};
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    mov qword [rbx + 16], 0
    mov rsi, rbx
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52
    mov qword [rsi + 0], 0
    jmp lab53

lab52:
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50
    mov qword [rbx + 0], 0
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43
    cmp qword [rcx + 0], 0
    je lab41
    add qword [rcx + 0], -1
    jmp lab42

lab41:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42:

lab43:
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46
    cmp qword [rcx + 0], 0
    je lab44
    add qword [rcx + 0], -1
    jmp lab45

lab44:
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45:

lab46:
    mov rcx, [rbx + 16]
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
    jmp lab51

lab50:
    mov rbp, rbx
    add rbp, 64

lab51:

lab53:
    lea rdi, [rel ContInt54]
    ; substitute (j !-> j)(ys !-> ys);
    ;  move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sum
    jmp sum

ContInt54:

ContInt54Reti:
    cmp qword [rsi + 0], 0
    je lab57
    add qword [rsi + 0], -1
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab56
    add qword [rsi + 0], 1

lab56:
    jmp lab58

lab57:
    mov [rsi + 0], rbx
    mov rbx, rsi
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab58:
    ; s <- y + r;
    mov r11, r9
    add r11, rdx
    ; substitute (s !-> s)(k !-> k);
    ;  move variables
    mov rdx, r11
    ; invoke k Reti
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