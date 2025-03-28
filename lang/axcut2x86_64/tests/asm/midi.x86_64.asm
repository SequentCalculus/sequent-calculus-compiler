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
    ; new t: ContInt = ()\{ ... \};
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    lea rdx, [rel ContInt_1]
    ; new k: ContList = (t)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab13
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
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
    lea rdx, [rel ContList_15]
    ; let zs: List = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; lit n <- 3;
    mov r9, 3
    ; substitute (k !-> k)(zs !-> zs)(n !-> n);
    ; jump range_
    jmp range_

ContList_15:

ContList_15_Retl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab17
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab16
    ; ####increment refcount
    add qword [rsi + 0], 1

lab16:
    jmp lab18

lab17:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab18:
    ; substitute (t !-> t)(as !-> as);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sum_
    jmp sum_

ContInt_1:

ContInt_1_Reti:
    ; println_i64 r;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; lit ret <- 0;
    mov rdi, 0
    ; return ret
    mov rax, rdi
    jmp cleanup

range_:
    ; if i == 0 \{ ... \}
    cmp r9, 0
    je lab19
    ; substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
    ; #move variables
    mov r8, rsi
    mov rsi, rax
    mov rcx, r9
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let ys: List = Cons(xs, i);
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
    je lab31
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab32

lab31:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab29
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
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
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab25
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab23
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab24

lab23:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab24:

lab25:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab28
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab26
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab27

lab26:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab27:

lab28:
    jmp lab30

lab29:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab30:

lab32:
    ; #load tag
    mov r9, 5
    ; lit o <- -1;
    mov r11, -1
    ; j <- n + o;
    mov r13, rdx
    add r13, r11
    ; substitute (k !-> k)(ys !-> ys)(j !-> j);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    mov r9, r13
    ; jump range_
    jmp range_

lab19:
    ; substitute (xs !-> xs)(k !-> k);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k Retl
    jmp rdi

sum_:
    ; switch xs \{ ... \};
    lea rcx, [rel List_33]
    add rcx, rdi
    jmp rcx

List_33:
    jmp near List_33_Nil
    jmp near List_33_Cons

List_33_Nil:
    ; lit z <- 0;
    mov rdi, 0
    ; substitute (z !-> z)(k !-> k);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k Reti
    jmp rdi

List_33_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab35
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab34
    ; ####increment refcount
    add qword [rsi + 0], 1

lab34:
    jmp lab36

lab35:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab36:
    ; substitute (ys !-> ys)(k !-> k)(y !-> y);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new j: ContInt = (k, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab49

lab48:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab37
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab38

lab37:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab38:

lab39:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41

lab40:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41:

lab42:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44

lab43:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44:

lab45:
    jmp lab47

lab46:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47:

lab49:
    ; #load tag
    lea rdi, [rel ContInt_50]
    ; substitute (j !-> j)(ys !-> ys);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump sum_
    jmp sum_

ContInt_50:

ContInt_50_Reti:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51:
    jmp lab53

lab52:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab53:
    ; s <- y + r;
    mov r11, r9
    add r11, rdx
    ; substitute (s !-> s)(k !-> k);
    ; #move variables
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