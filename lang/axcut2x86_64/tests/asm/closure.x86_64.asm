    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
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

main:
    ; lit a <- 9;
    mov rdx, 9
    ; new f: Func = (a)\{ ... \};
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
    lea rdx, [rel Func_14]
    ; new k: Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel Cont_15]
    ; lit y <- 1;
    mov r9, 1
    ; substitute (y !-> y)(k !-> k)(f !-> f);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f Ap
    jmp r9

Cont_15:

Cont_15_Ret:
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

Func_14:

Func_14_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab16
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab17

lab16:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab17:
    ; b <- a + x;
    mov r11, r9
    add r11, rdx
    ; substitute (b !-> b)(k !-> k);
    ; #move variables
    mov rdx, r11
    ; invoke k Ret
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