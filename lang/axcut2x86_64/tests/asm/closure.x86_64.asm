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
    ; lit a_1 <- 9;
    mov rdx, 9
    ; create f_2: Fun = (a_1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
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
    lea rdx, [rel Fun_6]
    ; create k_6: Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel Cont_7]
    ; lit y_9 <- 1;
    mov r9, 1
    ; substitute (y_9 := y_9)(k_6 := k_6)(f_2 := f_2);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f_2 apply
    ; #there is only one clause, so we can jump there directly
    jmp r9

Cont_7:

Cont_7_Ret:
    ; println_i64 r_7;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; lit ret_8 <- 0;
    mov rdi, 0
    ; exit ret_8
    mov rax, rdi
    jmp cleanup

Fun_6:

Fun_6_apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab8
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab9

lab8:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab9:
    ; b_5 <- a_1 + x_3;
    mov r11, r9
    add r11, rdx
    ; substitute (b_5 := b_5)(k_4 := k_4);
    ; #move variables
    mov rdx, r11
    ; invoke k_4 Ret
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
    je lab12
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab10
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab11

lab10:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab11:

lab12:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab15
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab13
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab14

lab13:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab14:

lab15:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
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
    jmp r15