    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
extern println_i64
global asm_main0
global asm_main1
global asm_main2
global asm_main3
global asm_main4
global asm_main5

asm_main0:

asm_main1:

asm_main2:

asm_main3:

asm_main4:

asm_main5:
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
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
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
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
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
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
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
    ; leta bb: BoxBox = BB(b);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
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
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
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
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
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
    ; substitute (f1 !-> f1)(f2 !-> f2)(f3 !-> f3)(f5 !-> f5)(f6 !-> f6)(f7 !-> f7)(f4 !-> f4)(bb3 !-> bb)(bb2 !-> bb)(bb1 !-> bb);
    ; #share bb
    cmp qword [rsp + 2016], 0
    je lab27
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
    lea rcx, [rel BoxBox28]
    jmp rcx

BoxBox28:

BoxBox28BB:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab31
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab30
    ; ####increment refcount
    add qword [rcx + 0], 1

lab30:
    mov rax, [rsp + 2040]
    jmp lab32

lab31:
    ; ##... or release blocks onto linear free list when loading
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
    mov rax, [rsp + 2040]

lab32:
    ; switch b1 \{ ... \};
    lea rcx, [rel Box33]
    jmp rcx

Box33:

Box33B:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab34
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]
    jmp lab35

lab34:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]

lab35:
    ; leta d1: Box = B(x1);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab47
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48

lab47:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab38
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab36
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab37

lab36:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab37:

lab38:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40

lab39:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40:

lab41:
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43

lab42:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43:

lab44:
    jmp lab46

lab45:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46:

lab48:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; leta dd1: BoxBox = BB(d1);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab60
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab61

lab60:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab58
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50

lab49:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50:

lab51:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53

lab52:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53:

lab54:
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab57
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab56

lab55:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab56:

lab57:
    jmp lab59

lab58:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab59:

lab61:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; substitute (bb2 !-> bb2);
    ; #erase dd1
    cmp qword [rsp + 1984], 0
    je lab64
    ; ######check refcount
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab62
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63

lab62:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63:

lab64:
    ; #erase bb3
    cmp qword [rsp + 2016], 0
    je lab67
    ; ######check refcount
    mov rcx, [rsp + 2016]
    cmp qword [rcx + 0], 0
    je lab65
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab66

lab65:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab66:

lab67:
    ; #move variables
    mov rax, [rsp + 2000]
    mov rdx, [rsp + 1992]
    ; lit y <- 4;
    mov rdi, 4
    ; leta a1: Box = B(y);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab79
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab80

lab79:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab77
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab70
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab68
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab69

lab68:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab69:

lab70:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab73
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab71
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab72

lab71:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab72:

lab73:
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab76
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab74
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab75

lab74:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab75:

lab76:
    jmp lab78

lab77:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab78:

lab80:
    ; #load tag
    mov rdi, 0
    ; substitute (a1 !-> a1)(bb2 !-> bb2);
    ; #move variables
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
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab84
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab83
    ; ####increment refcount
    add qword [rsi + 0], 1

lab83:
    jmp lab85

lab84:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab85:
    ; switch b2 \{ ... \};
    lea rcx, [rel Box86]
    jmp rcx

Box86:

Box86B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab87
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab88

lab87:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab88:
    ; leta a2: Box = B(x2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 32], 0
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab100
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab101

lab100:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab98
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab91
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab89
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab90

lab89:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab90:

lab91:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab94
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab92
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab93

lab92:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab93:

lab94:
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab97
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab95
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab96

lab95:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab96:

lab97:
    jmp lab99

lab98:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab99:

lab101:
    ; #load tag
    mov rdi, 0
    ; switch a2 \{ ... \};
    lea rcx, [rel Box102]
    jmp rcx

Box102:

Box102B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab103
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab104

lab103:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab104:
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch a1 \{ ... \};
    lea rcx, [rel Box105]
    jmp rcx

Box105:

Box105B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab106
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab107

lab106:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab107:
    ; res <- x1 + x2;
    mov r9, rdi
    add r9, rdx
    ; return res
    mov rax, r9
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