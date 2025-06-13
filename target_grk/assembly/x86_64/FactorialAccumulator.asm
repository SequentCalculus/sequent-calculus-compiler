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
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel _Cont_950]
    ; jump main_loop_
    jmp main_loop_

_Cont_950:

_Cont_950_Ret:
    ; exit x0
    mov rax, rdx
    jmp cleanup

factorial_:
    ; if i == 0 \{ ... \}
    cmp rdi, 0
    je lab951
    ; x0 <- i * a;
    mov r11, rdi
    imul r11, rdx
    ; substitute (x0 !-> x0)(i !-> i)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x1 <- 1000000007;
    mov r11, 1000000007
    ; x2 <- x0 % x1;
    mov rcx, rdx
    mov r13, rax
    mov rax, rdx
    cqo
    idiv r11
    mov rax, r13
    mov r13, rdx
    mov rdx, rcx
    ; substitute (x2 !-> x2)(i !-> i)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- i - x3;
    mov r13, rdi
    sub r13, r11
    ; substitute (x2 !-> x2)(x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rdi, r13
    ; jump factorial_
    jmp factorial_

lab951:
    ; substitute (a !-> a)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; lit x0 <- 1;
    mov r11, 1
    ; substitute (x0 !-> x0)(n0 !-> n)(a0 !-> a0)(n !-> n)(iters !-> iters);
    ; #move variables
    mov r13, rdx
    mov rdx, r11
    mov r11, rdi
    ; create a1: _Cont = (a0, n, iters)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov [rbx + 16], r8
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab963
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab964

lab963:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab961
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab954
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab952
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab953

lab952:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab953:

lab954:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab957
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab955
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab956

lab955:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab956:

lab957:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab960
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab958
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab959

lab958:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab959:

lab960:
    jmp lab962

lab961:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab962:

lab964:
    ; #load tag
    lea r9, [rel _Cont_965]
    ; jump factorial_
    jmp factorial_

_Cont_965:

_Cont_965_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab967
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab966
    ; ####increment refcount
    add qword [rsi + 0], 1

lab966:
    jmp lab968

lab967:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab968:
    ; lit x1 <- 1;
    mov r13, 1
    ; if iters == x1 \{ ... \}
    cmp r11, r13
    je lab969
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov rdx, r11
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- iters - x2;
    mov r13, rdx
    sub r13, r11
    ; substitute (x3 !-> x3)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab969:
    ; substitute (res !-> res)(a0 !-> a0);
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
    ; lit x4 <- 0;
    mov rdi, 0
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
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