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
    mov rdx, rsi
    ; actual code

main_:
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel _Cont_1]
    ; create a1: List[i64] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab13
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab14

lab13:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab11
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab4
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab2
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab3

lab2:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab3:

lab4:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab7
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab5
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab6

lab5:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab6:

lab7:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab10
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab8
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab9

lab8:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab9:

lab10:
    jmp lab12

lab11:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab12:

lab14:
    ; #load tag
    lea rdi, [rel List_i64_15]
    ; substitute (n0 !-> n)(a1 !-> a1)(n !-> n);
    ; #move variables
    mov r9, rdx
    ; create x3: Stream[i64] = (n)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab27
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab28

lab27:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab25
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab18
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab16
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab17

lab16:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab17:

lab18:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab21
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab19
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab20

lab19:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab20:

lab21:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab24
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab22
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab23

lab22:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab23:

lab24:
    jmp lab26

lab25:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab26:

lab28:
    ; #load tag
    lea r9, [rel Stream_i64_29]
    ; substitute (n0 !-> n0)(x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump take_
    jmp take_

Stream_i64_29:
    jmp near Stream_i64_29_Hd
    jmp near Stream_i64_29_Tl

Stream_i64_29_Hd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab30
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab31

lab30:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab31:
    ; substitute (n !-> n)(a00 !-> a00);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let a2: Stream[i64] = Hd(a00);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44

lab43:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab34
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab32
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab33

lab32:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab33:

lab34:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
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
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
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
    jmp lab42

lab41:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42:

lab44:
    ; #load tag
    mov rdi, 0
    ; jump repeat_
    jmp repeat_

Stream_i64_29_Tl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab46

lab45:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab46:
    ; substitute (n !-> n)(a01 !-> a01);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let a2: Stream[i64] = Tl(a01);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab58
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab59

lab58:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab56
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48

lab47:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48:

lab49:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51

lab50:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51:

lab52:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54

lab53:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54:

lab55:
    jmp lab57

lab56:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab57:

lab59:
    ; #load tag
    mov rdi, 5
    ; jump repeat_
    jmp repeat_

List_i64_15:
    jmp near List_i64_15_Nil
    jmp near List_i64_15_Cons

List_i64_15_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab61
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab60
    ; ####increment refcount
    add qword [rax + 0], 1

lab60:
    jmp lab62

lab61:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab62:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sumList_
    jmp sumList_

List_i64_15_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab63
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63:
    jmp lab65

lab64:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab65:
    ; substitute (a0 !-> a0)(x4 !-> x4)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let x2: List[i64] = Cons(x4, xs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    mov [rbx + 40], rdi
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab77
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab78

lab77:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab75
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab68
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab66
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab67

lab66:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab67:

lab68:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab71
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab69
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab70

lab69:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab70:

lab71:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab74
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab72
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab73

lab72:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab73:

lab74:
    jmp lab76

lab75:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab76:

lab78:
    ; #load tag
    mov rdi, 5
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sumList_
    jmp sumList_

_Cont_1:

_Cont_1_Ret:
    ; println_i64 x1;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; substitute ;
    ; lit x0 <- 0;
    mov rdx, 0
    ; exit x0
    mov rax, rdx
    jmp cleanup

repeat_:
    ; switch a0 \{ ... \};
    lea rcx, [rel Stream_i64_79]
    add rcx, rdi
    jmp rcx

Stream_i64_79:
    jmp near Stream_i64_79_Hd
    jmp near Stream_i64_79_Tl

Stream_i64_79_Hd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab81
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab80
    ; ####increment refcount
    add qword [rsi + 0], 1

lab80:
    jmp lab82

lab81:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab82:
    ; invoke a1 Ret
    jmp rdi

Stream_i64_79_Tl:
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
    ; jump repeat_
    jmp repeat_

const1_:
    ; switch a0 \{ ... \};
    lea rcx, [rel Stream_i64_86]
    add rcx, rdx
    jmp rcx

Stream_i64_86:
    jmp near Stream_i64_86_Hd
    jmp near Stream_i64_86_Tl

Stream_i64_86_Hd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab88
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab87
    ; ####increment refcount
    add qword [rax + 0], 1

lab87:
    jmp lab89

lab88:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab89:
    ; lit x0 <- 1;
    mov rdi, 1
    ; substitute (x0 !-> x0)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Ret
    jmp rdi

Stream_i64_86_Tl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab91
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab90
    ; ####increment refcount
    add qword [rax + 0], 1

lab90:
    jmp lab92

lab91:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab92:
    ; jump const1_
    jmp const1_

take_:
    ; lit x0 <- 0;
    mov r11, 0
    ; if n <= x0 \{ ... \}
    cmp rdx, r11
    jle lab93
    ; substitute (x7 !-> x)(x !-> x)(a0 !-> a0)(n !-> n);
    ; #share x
    cmp rsi, 0
    je lab94
    ; ####increment refcount
    add qword [rsi + 0], 1

lab94:
    ; #move variables
    mov r11, rdx
    mov rax, rsi
    mov rdx, rdi
    ; create a1: _Cont = (x, a0, n)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab106
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab107

lab106:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab104
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
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
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab100
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab98
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab99

lab98:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab99:

lab100:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab103
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab101
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab102

lab101:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab102:

lab103:
    jmp lab105

lab104:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab105:

lab107:
    ; #load tag
    lea rdi, [rel _Cont_108]
    ; substitute (a1 !-> a1)(x7 !-> x7);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x7 Hd
    add rdi, 0
    jmp rdi

_Cont_108:

_Cont_108_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab111
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab109
    ; ####increment refcount
    add qword [r8 + 0], 1

lab109:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab110
    ; ####increment refcount
    add qword [rsi + 0], 1

lab110:
    jmp lab112

lab111:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab112:
    ; substitute (n !-> n)(x !-> x)(a0 !-> a0)(x1 !-> x1);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; create a2: List[i64] = (a0, x1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab124
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab125

lab124:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab122
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab115
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab113
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab114

lab113:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab114:

lab115:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab118
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab116
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab117

lab116:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab117:

lab118:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab121
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab119
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab120

lab119:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab120:

lab121:
    jmp lab123

lab122:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab123:

lab125:
    ; #load tag
    lea r9, [rel List_i64_126]
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- n - x3;
    mov r13, rdx
    sub r13, r11
    ; substitute (x4 !-> x4)(a2 !-> a2)(x !-> x);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; create x5: Stream[i64] = (x)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab138
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab139

lab138:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab136
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab129
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab127
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab128

lab127:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab128:

lab129:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab132
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab130
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab131

lab130:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab131:

lab132:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab135
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab133
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab134

lab133:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab134:

lab135:
    jmp lab137

lab136:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab137:

lab139:
    ; #load tag
    lea r9, [rel Stream_i64_140]
    ; substitute (x4 !-> x4)(x5 !-> x5)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump take_
    jmp take_

Stream_i64_140:
    jmp near Stream_i64_140_Hd
    jmp near Stream_i64_140_Tl

Stream_i64_140_Hd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab142
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab141
    ; ####increment refcount
    add qword [rsi + 0], 1

lab141:
    jmp lab143

lab142:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab143:
    ; substitute (x !-> x)(a00 !-> a00);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let a3: Stream[i64] = Hd(a00);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab155
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab156

lab155:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab153
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab146
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab144
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab145

lab144:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab145:

lab146:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab149
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab147
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab148

lab147:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab148:

lab149:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab152
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab150
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab151

lab150:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab151:

lab152:
    jmp lab154

lab153:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab154:

lab156:
    ; #load tag
    mov rdi, 0
    ; substitute (a3 !-> a3)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Tl
    add rdi, 5
    jmp rdi

Stream_i64_140_Tl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab158
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab157
    ; ####increment refcount
    add qword [rsi + 0], 1

lab157:
    jmp lab159

lab158:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab159:
    ; substitute (x !-> x)(a01 !-> a01);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let a3: Stream[i64] = Tl(a01);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab171
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab172

lab171:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab169
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab162
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab160
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab161

lab160:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab161:

lab162:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab165
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab163
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab164

lab163:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab164:

lab165:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab168
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab166
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab167

lab166:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab167:

lab168:
    jmp lab170

lab169:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab170:

lab172:
    ; #load tag
    mov rdi, 5
    ; substitute (a3 !-> a3)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Tl
    add rdi, 5
    jmp rdi

List_i64_126:
    jmp near List_i64_126_Nil
    jmp near List_i64_126_Cons

List_i64_126_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab174
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab173
    ; ####increment refcount
    add qword [rax + 0], 1

lab173:
    jmp lab175

lab174:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab175:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_i64_126_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab177
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab176
    ; ####increment refcount
    add qword [r8 + 0], 1

lab176:
    jmp lab178

lab177:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab178:
    ; substitute (x1 !-> x1)(a0 !-> a0)(x6 !-> x6)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x2: List[i64] = Cons(x6, xs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab190
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab191

lab190:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab188
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab181
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab179
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab180

lab179:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab180:

lab181:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab184
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab182
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab183

lab182:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab183:

lab184:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab187
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab185
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab186

lab185:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab186:

lab187:
    jmp lab189

lab188:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab189:

lab191:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

lab93:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab194
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab192
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab193

lab192:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab193:

lab194:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

sumList_:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_195]
    add rcx, rdi
    jmp rcx

List_i64_195:
    jmp near List_i64_195_Nil
    jmp near List_i64_195_Cons

List_i64_195_Nil:
    ; lit x1 <- 0;
    mov rdi, 0
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_195_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab197
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab196
    ; ####increment refcount
    add qword [r8 + 0], 1

lab196:
    mov rdi, [rsi + 40]
    jmp lab198

lab197:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab198:
    ; substitute (xs !-> xs)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; create a1: _Cont = (x, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    mov [rbx + 40], rdi
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab210
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab211

lab210:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab208
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab201
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab199
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab200

lab199:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab200:

lab201:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab204
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab202
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab203

lab202:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab203:

lab204:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab207
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab205
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab206

lab205:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab206:

lab207:
    jmp lab209

lab208:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab209:

lab211:
    ; #load tag
    lea rdi, [rel _Cont_212]
    ; jump sumList_
    jmp sumList_

_Cont_212:

_Cont_212_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab214
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab213
    ; ####increment refcount
    add qword [r8 + 0], 1

lab213:
    mov rdi, [rsi + 40]
    jmp lab215

lab214:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab215:
    ; x2 <- x + x0;
    mov r11, rdi
    add r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

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