    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
extern print_i64
extern println_i64
global asm_main

asm_main:
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

main_:
    ; lit f_1 <- 3;
    mov rax, 3
    ; lit f_2 <- 3;
    mov rdx, 3
    ; lit f_3 <- 3;
    mov rsi, 3
    ; lit f_4 <- 3;
    mov rdi, 3
    ; lit f_5 <- 3;
    mov r8, 3
    ; lit f_6 <- 3;
    mov r9, 3
    ; lit f_7 <- 3;
    mov r10, 3
    ; lit x_8 <- 3;
    mov r11, 3
    ; let ω b_9: Box = B(x_8);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r11, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab12
    ; ####initialize refcount of just acquired block
    mov qword [r11 + 8], 0
    jmp lab13

lab12:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab10
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab3
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab1
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab2

lab1:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab2:

lab3:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab6
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab4
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab5

lab4:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab5:

lab6:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab9
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab7
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab8

lab7:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab8:

lab9:
    jmp lab11

lab10:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab11:

lab13:
    ; #load tag
    mov r12, 0
    ; let ω bb_10: BoxBox = BB(b_9);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r12
    mov [rbx + 48], r11
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r11, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab25
    ; ####initialize refcount of just acquired block
    mov qword [r11 + 8], 0
    jmp lab26

lab25:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab23
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab16
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab14
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab15

lab14:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab15:

lab16:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab19
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab17
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab18

lab17:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab18:

lab19:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab22
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab20
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab21

lab20:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab21:

lab22:
    jmp lab24

lab23:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab24:

lab26:
    ; #load tag
    mov r12, 0
    ; substitute (f_1 := f_1)(f_2 := f_2)(f_3 := f_3)(f_5 := f_5)(f_6 := f_6)(f_7 := f_7)(f_4 := f_4)(bb_13 := bb_10)(bb_12 := bb_10)(bb_11 := bb_10);
    ; #share bb_10
    cmp r11, 0
    je lab27
    ; ####increment refcount
    add qword [r11 + 8], 2

lab27:
    ; #move variables
    mov rcx, r8
    mov r8, r9
    mov r9, r10
    mov r10, rdi
    mov rdi, rcx
    mov r13, r11
    mov r15, r11
    mov r14, r12
    mov [rsp + 2032], r12
    ; switch ω bb_11 \{ ... \};
    ; #there is only one clause, so we can just fall through

BoxBox_28:

BoxBox_28_BB:
    ; #load from memory
    ; ##check refcount
    cmp qword [r15 + 8], 0
    je lab30
    ; ##either decrement refcount and share children...
    add qword [r15 + 8], -1
    ; ###load values
    mov rcx, [r15 + 56]
    mov [rsp + 2032], rcx
    mov r15, [r15 + 48]
    cmp r15, 0
    je lab29
    ; ####increment refcount
    add qword [r15 + 8], 1

lab29:
    jmp lab31

lab30:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r15 + 8], rbx
    mov rbx, r15
    ; ###load values
    mov rcx, [r15 + 56]
    mov [rsp + 2032], rcx
    mov r15, [r15 + 48]

lab31:
    ; switch ω b_14 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_32:

Box_32_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [r15 + 8], 0
    je lab33
    ; ##either decrement refcount and share children...
    add qword [r15 + 8], -1
    ; ###load values
    mov r15, [r15 + 56]
    jmp lab34

lab33:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r15 + 8], rbx
    mov rbx, r15
    ; ###load values
    mov r15, [r15 + 56]

lab34:
    ; let ω d_16: Box = B(x_15);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r15, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab46
    ; ####initialize refcount of just acquired block
    mov qword [r15 + 8], 0
    jmp lab47

lab46:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab44
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab37
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab35
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab36

lab35:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab36:

lab37:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab38
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab39

lab38:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab39:

lab40:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab41
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab42

lab41:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab42:

lab43:
    jmp lab45

lab44:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45:

lab47:
    ; #load tag
    mov qword [rsp + 2032], 0
    ; let ω dd_17: BoxBox = BB(d_16);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2032]
    mov [rbx + 56], rcx
    mov [rbx + 48], r15
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r15, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab59
    ; ####initialize refcount of just acquired block
    mov qword [r15 + 8], 0
    jmp lab60

lab59:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab57
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab48
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab49

lab48:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab49:

lab50:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab51
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab52

lab51:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab52:

lab53:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab56
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab54
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab55

lab54:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab55:

lab56:
    jmp lab58

lab57:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab58:

lab60:
    ; #load tag
    mov qword [rsp + 2032], 0
    ; substitute (bb_12 := bb_12);
    ; #erase bb_13
    cmp r11, 0
    je lab63
    ; ######check refcount
    cmp qword [r11 + 8], 0
    je lab61
    ; ######either decrement refcount ...
    add qword [r11 + 8], -1
    jmp lab62

lab61:
    ; ######... or add block to lazy free list
    mov [r11 + 8], rbp
    mov rbp, r11

lab62:

lab63:
    ; #erase dd_17
    cmp r15, 0
    je lab66
    ; ######check refcount
    cmp qword [r15 + 8], 0
    je lab64
    ; ######either decrement refcount ...
    add qword [r15 + 8], -1
    jmp lab65

lab64:
    ; ######... or add block to lazy free list
    mov [r15 + 8], rbp
    mov rbp, r15

lab65:

lab66:
    ; #move variables
    mov rax, r13
    mov rdx, r14
    ; lit y_18 <- 4;
    mov rsi, 4
    ; let ω a_19: Box = B(y_18);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rsi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab78
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 8], 0
    jmp lab79

lab78:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab76
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab69
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab67
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab68

lab67:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab68:

lab69:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab72
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab70
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab71

lab70:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab71:

lab72:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab75
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab73
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab74

lab73:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab74:

lab75:
    jmp lab77

lab76:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab77:

lab79:
    ; #load tag
    mov rdi, 0
    ; substitute (a_19 := a_19)(bb_12 := bb_12);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ω bb_12 \{ ... \};
    ; #there is only one clause, so we can just fall through

BoxBox_80:

BoxBox_80_BB:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 8], 0
    je lab82
    ; ##either decrement refcount and share children...
    add qword [rsi + 8], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab81
    ; ####increment refcount
    add qword [rsi + 8], 1

lab81:
    jmp lab83

lab82:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 8], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab83:
    ; switch ω b_20 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_84:

Box_84_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 8], 0
    je lab85
    ; ##either decrement refcount and share children...
    add qword [rsi + 8], -1
    ; ###load values
    mov rsi, [rsi + 56]
    jmp lab86

lab85:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 8], rbx
    mov rbx, rsi
    ; ###load values
    mov rsi, [rsi + 56]

lab86:
    ; let ω a_22: Box = B(x_21);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rsi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab98
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 8], 0
    jmp lab99

lab98:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 8]
    cmp rbp, 0
    je lab96
    ; ####mark linear free list empty
    mov qword [rbx + 8], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab89
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab87
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab88

lab87:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab88:

lab89:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab92
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab90
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab91

lab90:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab91:

lab92:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab95
    ; ######check refcount
    cmp qword [rcx + 8], 0
    je lab93
    ; ######either decrement refcount ...
    add qword [rcx + 8], -1
    jmp lab94

lab93:
    ; ######... or add block to lazy free list
    mov [rcx + 8], rbp
    mov rbp, rcx

lab94:

lab95:
    jmp lab97

lab96:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab97:

lab99:
    ; #load tag
    mov rdi, 0
    ; switch ω a_22 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_100:

Box_100_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 8], 0
    je lab101
    ; ##either decrement refcount and share children...
    add qword [rsi + 8], -1
    ; ###load values
    mov rsi, [rsi + 56]
    jmp lab102

lab101:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 8], rbx
    mov rbx, rsi
    ; ###load values
    mov rsi, [rsi + 56]

lab102:
    ; substitute (x_23 := x_23)(a_19 := a_19);
    ; #move variables
    mov rcx, rsi
    mov rsi, rdx
    mov rdx, rax
    mov rax, rcx
    ; switch ω a_19 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_103:

Box_103_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rdx + 8], 0
    je lab104
    ; ##either decrement refcount and share children...
    add qword [rdx + 8], -1
    ; ###load values
    mov rdx, [rdx + 56]
    jmp lab105

lab104:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rdx + 8], rbx
    mov rbx, rdx
    ; ###load values
    mov rdx, [rdx + 56]

lab105:
    ; res_25 <- x_24 + x_23;
    mov rsi, rdx
    add rsi, rax
    ; println_i64 res_25;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rsi
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rsi, r14
    add rsp, 8
    ; lit ret_26 <- 0;
    mov rdi, 0
    ; exit ret_26
    mov rax, rdi
    jmp cleanup

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