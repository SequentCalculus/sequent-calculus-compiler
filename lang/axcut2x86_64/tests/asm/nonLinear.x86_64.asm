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
    ; let b: Box = B(x: prd i64);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab12
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab13

lab12:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab10
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab3
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab1
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab2

lab1:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab2:

lab3:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab6
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab4
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab5

lab4:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab5:

lab6:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab9
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab7
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab8

lab7:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    mov qword [rsp + 2008], 0
    ; let bb: BoxBox = BB(b: prd i64);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab25
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab26

lab25:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab23
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab16
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab14
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab15

lab14:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab15:

lab16:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab19
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab17
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab18

lab17:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab18:

lab19:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab22
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab20
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab21

lab20:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    mov qword [rsp + 2008], 0
    ; substitute (f1 := f1)(f2 := f2)(f3 := f3)(f5 := f5)(f6 := f6)(f7 := f7)(f4 := f4)(bb3 := bb)(bb2 := bb)(bb1 := bb);
    ; #share bb
    cmp qword [rsp + 2016], 0
    je lab27
    ; ####increment refcount
    mov rcx, [rsp + 2016]
    add qword [rcx + 0], 2

lab27:
    ; #move variables
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
    ; #there is only one clause, so we can just fall through

BoxBox_28:

BoxBox_28_BB:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab30
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab29
    ; ####increment refcount
    add qword [rcx + 0], 1

lab29:
    ; ###restore evacuated register
    mov rax, [rsp + 2040]
    jmp lab31

lab30:
    ; ##... or release blocks onto linear free list when loading
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]

lab31:
    ; switch b1 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_32:

Box_32_B:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab33
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]
    jmp lab34

lab33:
    ; ##... or release blocks onto linear free list when loading
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]

lab34:
    ; let d1: Box = B(x1: prd i64);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab46
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab47

lab46:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab37
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab35
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab36

lab35:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab36:

lab37:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab38
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39

lab38:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39:

lab40:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42

lab41:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    mov qword [rsp + 1976], 0
    ; let dd1: BoxBox = BB(d1: prd i64);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab59
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab60

lab59:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab57
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49

lab48:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49:

lab50:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52

lab51:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52:

lab53:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab56
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55

lab54:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    mov qword [rsp + 1976], 0
    ; substitute (bb2 := bb2);
    ; #erase bb3
    mov rcx, [rsp + 2016]
    cmp rcx, 0
    je lab63
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab61
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62

lab61:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62:

lab63:
    ; #erase dd1
    mov rcx, [rsp + 1984]
    cmp rcx, 0
    je lab66
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65

lab64:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65:

lab66:
    ; #move variables
    mov rax, [rsp + 2000]
    mov rdx, [rsp + 1992]
    ; lit y <- 4;
    mov rdi, 4
    ; let a1: Box = B(y: prd i64);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab78
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab79

lab78:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab76
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab69
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab67
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab68

lab67:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab68:

lab69:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab72
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab70
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab71

lab70:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab71:

lab72:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab75
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab73
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab74

lab73:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    ; substitute (a1 := a1)(bb2 := bb2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch bb2 \{ ... \};
    ; #there is only one clause, so we can just fall through

BoxBox_80:

BoxBox_80_BB:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab82
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab81
    ; ####increment refcount
    add qword [rsi + 0], 1

lab81:
    jmp lab83

lab82:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab83:
    ; switch b2 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_84:

Box_84_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab85
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab86

lab85:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab86:
    ; let a2: Box = B(x2: prd i64);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab98
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab99

lab98:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab96
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab89
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab87
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab88

lab87:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab88:

lab89:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab92
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab90
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab91

lab90:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab91:

lab92:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab95
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab93
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab94

lab93:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
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
    ; switch a2 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_100:

Box_100_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab101
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab102

lab101:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab102:
    ; substitute (x2 := x2)(a1 := a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch a1 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_103:

Box_103_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab104
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab105

lab104:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab105:
    ; res <- x1 + x2;
    mov r9, rdi
    add r9, rdx
    ; println_i64 res;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, r9
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    add rsp, 8
    ; lit ret <- 0;
    mov r11, 0
    ; exit ret
    mov rax, r11
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