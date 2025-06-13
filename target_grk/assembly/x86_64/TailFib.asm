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
    mov rdi, rdx
    ; move parameters into place
    mov rdx, rsi
    ; actual code

main_:
    ; new a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel _Cont_54809]
    ; jump main_loop_
    jmp main_loop_

_Cont_54809:

_Cont_54809_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

tfib_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab54810
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- n - x0;
    mov r15, rdx
    sub r15, r13
    ; substitute (x1 !-> x1)(a !-> a)(b !-> b)(a0 !-> a0);
    ; #move variables
    mov rdx, r15
    ; x2 <- a + b;
    mov r13, rdi
    add r13, r9
    ; substitute (x1 !-> x1)(x2 !-> x2)(a !-> a)(a0 !-> a0);
    ; #move variables
    mov r9, rdi
    mov rdi, r13
    ; jump tfib_
    jmp tfib_

lab54810:
    ; substitute (a !-> a)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

fib_:
    ; lit x0 <- 0;
    mov r9, 0
    ; lit x1 <- 1;
    mov r11, 1
    ; substitute (n !-> n)(x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump tfib_
    jmp tfib_

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: _Cont = (n, a0, iters)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54822
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54823

lab54822:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54820
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54812

lab54811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54812:

lab54813:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54815

lab54814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54815:

lab54816:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54818

lab54817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54818:

lab54819:
    jmp lab54821

lab54820:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54821:

lab54823:
    ; #load tag
    lea rdi, [rel _Cont_54824]
    ; jump fib_
    jmp fib_

_Cont_54824:

_Cont_54824_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54826
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54825
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54825:
    mov rdi, [rsi + 24]
    jmp lab54827

lab54826:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54827:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab54828
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- iters - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab54828:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; println_i64 res;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rsi
    mov r14, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rsi, r13
    mov rdi, r14
    add rsp, 8
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x3 <- 0;
    mov rdi, 0
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
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