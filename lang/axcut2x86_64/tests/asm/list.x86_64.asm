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
    ; let ws: List = Nil();
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    mov rdx, 0
    ; lit z <- 5;
    mov rdi, 5
    ; let zs: List = Cons(z: ext i64, ws: prd List);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab12
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
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
    mov rdx, 5
    ; lit y <- 7;
    mov rdi, 7
    ; let ys: List = Cons(y: ext i64, zs: prd List);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab25
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
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
    mov rdx, 5
    ; lit x <- 9;
    mov rdi, 9
    ; let xs: List = Cons(x: ext i64, ys: prd List);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab38
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab39

lab38:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab36
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab29
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab27
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab28

lab27:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab28:

lab29:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab32
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab30
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab31

lab30:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab31:

lab32:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab35
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab33
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab34

lab33:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab34:

lab35:
    jmp lab37

lab36:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab37:

lab39:
    ; #load tag
    mov rdx, 5
    ; switch xs \{ ... \};
    lea rcx, [rel List_40]
    add rcx, rdx
    jmp rcx

List_40:
    jmp near List_40_Nil
    jmp near List_40_Cons

List_40_Nil:
    ; lit err <- -1;
    mov rdx, -1
    ; exit err
    mov rax, rdx
    jmp cleanup

List_40_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab41
    ; ####increment refcount
    add qword [rax + 0], 1

lab41:
    jmp lab43

lab42:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab43:
    ; println_i64 a;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rdi, r14
    add rsp, 8
    ; lit ret <- 0;
    mov r9, 0
    ; exit ret
    mov rax, r9
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