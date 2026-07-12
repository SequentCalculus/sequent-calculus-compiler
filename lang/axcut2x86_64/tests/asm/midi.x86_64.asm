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
    ; create t_1: ContInt = ()\{ ... \};
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    lea rdx, [rel ContInt_1]
    ; create k_4: ContList = (t_1)\{ ... \};
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
    je lab5
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab6

lab5:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab3
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path2]
    jmp acquire_block_slow_path

return_from_slow_path2:
    jmp lab4

lab3:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab4:

lab6:
    ; #load tag
    lea rdx, [rel ContList_7]
    ; let zs_6: List = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; lit n_7 <- 3;
    mov r9, 3
    ; substitute (k_4 := k_4)(zs_6 := zs_6)(n_7 := n_7);
    ; range(...)
    jmp range_

ContList_7:

ContList_7_Retl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab9
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab8
    ; ####increment refcount
    add qword [rsi + 0], 1

lab8:
    jmp lab10

lab9:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab10:
    ; substitute (t_1 := t_1)(as_5 := as_5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; sum(...)
    jmp sum_

ContInt_1:

ContInt_1_Reti:
    ; println_i64 r_2;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; lit ret_3 <- 0;
    mov rdi, 0
    ; exit ret_3
    mov rax, rdi
    jmp cleanup

range_:
    ; if i_10 == 0 \{ ... \}
    cmp r9, 0
    je lab11
    ; else branch
    ; substitute (n_11 := i_10)(k_8 := k_8)(xs_9 := xs_9)(i_10 := i_10);
    ; #move variables
    mov r8, rsi
    mov rsi, rax
    mov rcx, r9
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let ys_12: List = Cons(xs_9, i_10);
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
    je lab15
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab16

lab15:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab13
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path12]
    jmp acquire_block_slow_path

return_from_slow_path12:
    jmp lab14

lab13:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab14:

lab16:
    ; #load tag
    mov r9, 5
    ; lit o_13 <- -1;
    mov r11, -1
    ; j_14 <- n_11 + o_13;
    mov r13, rdx
    add r13, r11
    ; substitute (k_8 := k_8)(ys_12 := ys_12)(j_14 := j_14);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    mov r9, r13
    ; range(...)
    jmp range_

lab11:
    ; then branch
    ; substitute (xs_9 := xs_9)(k_8 := k_8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k_8 Retl
    ; #there is only one clause, so we can jump there directly
    jmp rdi

sum_:
    ; switch xs_16 \{ ... \};
    lea rcx, [rel List_17]
    add rcx, rdi
    jmp rcx

List_17:
    jmp near List_17_Nil
    jmp near List_17_Cons

List_17_Nil:
    ; lit z_17 <- 0;
    mov rdi, 0
    ; substitute (z_17 := z_17)(k_15 := k_15);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k_15 Reti
    ; #there is only one clause, so we can jump there directly
    jmp rdi

List_17_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab19
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab18
    ; ####increment refcount
    add qword [rsi + 0], 1

lab18:
    jmp lab20

lab19:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab20:
    ; substitute (ys_18 := ys_18)(k_15 := k_15)(y_19 := y_19);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; create j_20: ContInt = (k_15, y_19)\{ ... \};
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
    je lab24
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab25

lab24:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab22
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path21]
    jmp acquire_block_slow_path

return_from_slow_path21:
    jmp lab23

lab22:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab23:

lab25:
    ; #load tag
    lea rdi, [rel ContInt_26]
    ; substitute (j_20 := j_20)(ys_18 := ys_18);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; sum(...)
    jmp sum_

ContInt_26:

ContInt_26_Reti:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab28
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab27
    ; ####increment refcount
    add qword [rsi + 0], 1

lab27:
    jmp lab29

lab28:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab29:
    ; s_22 <- y_19 + r_21;
    mov r11, r9
    add r11, rdx
    ; substitute (s_22 := s_22)(k_15 := k_15);
    ; #move variables
    mov rdx, r11
    ; invoke k_15 Reti
    ; #there is only one clause, so we can jump there directly
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

acquire_block_slow_path:
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
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
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
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
    jmp r15