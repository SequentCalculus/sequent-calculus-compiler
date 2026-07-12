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
    ; let ws_1: List = Nil();
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    mov rdx, 0
    ; lit z_2 <- 5;
    mov rdi, 5
    ; let zs_3: List = Cons(z_2, ws_1);
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
    je lab4
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab5

lab4:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab2
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path1]
    jmp acquire_block_slow_path

return_from_slow_path1:
    jmp lab3

lab2:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab3:

lab5:
    ; #load tag
    mov rdx, 5
    ; lit y_4 <- 7;
    mov rdi, 7
    ; let ys_5: List = Cons(y_4, zs_3);
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
    je lab9
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab10

lab9:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab7
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path6]
    jmp acquire_block_slow_path

return_from_slow_path6:
    jmp lab8

lab7:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab8:

lab10:
    ; #load tag
    mov rdx, 5
    ; lit x_6 <- 9;
    mov rdi, 9
    ; let xs_7: List = Cons(x_6, ys_5);
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
    je lab14
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab15

lab14:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab12
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path11]
    jmp acquire_block_slow_path

return_from_slow_path11:
    jmp lab13

lab12:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab13:

lab15:
    ; #load tag
    mov rdx, 5
    ; switch xs_7 \{ ... \};
    lea rcx, [rel List_16]
    add rcx, rdx
    jmp rcx

List_16:
    jmp near List_16_Nil
    jmp near List_16_Cons

List_16_Nil:
    ; lit err_8 <- -1;
    mov rdx, -1
    ; exit err_8
    mov rax, rdx
    jmp cleanup

List_16_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab18
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab17
    ; ####increment refcount
    add qword [rax + 0], 1

lab17:
    jmp lab19

lab18:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab19:
    ; println_i64 a_10;
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
    ; lit ret_11 <- 0;
    mov r9, 0
    ; exit ret_11
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

acquire_block_slow_path:
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
    jmp r15