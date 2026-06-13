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
    ; lit z_1 <- 8;
    mov rax, 8
    ; lit y_2 <- 6;
    mov rdx, 6
    ; lit x_3 <- 4;
    mov rsi, 4
    ; lit w_4 <- 2;
    mov rdi, 2
    ; let ω q_5: Quad = Q(z_1, y_2, x_3, w_4);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    mov [rbx + 40], rsi
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rdx, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab12
    ; ####initialize refcount of just acquired block
    mov qword [rdx + 8], 0
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
    ; ##store link to previous block
    mov [rbx + 48], rdx
    ; ##store values
    mov [rbx + 40], rax
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 8]
    cmp rbx, 0
    je lab25
    ; ####initialize refcount of just acquired block
    mov qword [rax + 8], 0
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
    mov rdx, 0
    ; switch ω q_5 \{ ... \};
    ; #there is only one clause, so we can just fall through

Quad_27:

Quad_27_Q:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 8], 0
    je lab28
    ; ##either decrement refcount and share children...
    add qword [rax + 8], -1
    ; ###load link to next block
    mov rdx, [rax + 48]
    ; ###load values
    mov rax, [rax + 40]
    ; ###load values
    mov rdi, [rdx + 56]
    mov rsi, [rdx + 40]
    mov rdx, [rdx + 24]
    jmp lab29

lab28:
    ; ##... or release blocks onto linear free list when loading
    ; ###load link to next block
    mov rdx, [rax + 48]
    ; ###release block
    mov [rax + 8], rbx
    mov rbx, rax
    ; ###load values
    mov rax, [rax + 40]
    ; ###release block
    mov [rdx + 8], rbx
    mov rbx, rdx
    ; ###load values
    mov rdi, [rdx + 56]
    mov rsi, [rdx + 40]
    mov rdx, [rdx + 24]

lab29:
    ; lit z_10 <- 7;
    mov r8, 7
    ; e_11 <- d_6 + z_10;
    mov r9, rax
    add r9, r8
    ; println_i64 e_11;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    push r8
    push r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, r9
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rsi, r14
    mov rdi, r15
    add rsp, 8
    pop r9
    pop r8
    ; lit ret_12 <- 0;
    mov r10, 0
    ; exit ret_12
    mov rax, r10
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