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
    ; lit z_1 <- 1;
    mov rdx, 1
    ; lit x_2 <- 9;
    mov rdi, 9
    ; let p_3: Either = Right(x_2);
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
    je lab4
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
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
    mov rdi, 5
    ; switch p_3 \{ ... \};
    lea rcx, [rel Either_6]
    add rcx, rdi
    jmp rcx

Either_6:
    jmp near Either_6_Left
    jmp near Either_6_Right

Either_6_Left:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab7
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab8

lab7:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab8:
    ; lit err_5 <- -1;
    mov r9, -1
    ; exit err_5
    mov rax, r9
    jmp cleanup

Either_6_Right:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab9
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab10

lab9:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab10:
    ; c_7 <- b_6 + z_1;
    mov r9, rdi
    add r9, rdx
    ; println_i64 c_7;
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
    ; lit ret_8 <- 0;
    mov r11, 0
    ; exit ret_8
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

acquire_block_slow_path:
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab13
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab11
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab12

lab11:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab12:

lab13:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
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
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
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
    jmp r15