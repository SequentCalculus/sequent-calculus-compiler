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
    lea r9, [rel _Cont_466]
    ; jump monus_
    jmp monus_

_Cont_466:

_Cont_466_Ret:
    ; lit x2 <- -2;
    mov rdi, -2
    ; x3 <- x1 / x2;
    mov rcx, rdx
    mov r9, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rdx, rax
    mov rax, r9
    mov r9, rdx
    mov rdx, rcx
    ; substitute (x3 !-> x3);
    ; #move variables
    mov rdx, r9
    ; println_i64 x3;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; substitute ;
    ; lit x0 <- 0;
    mov rdx, 0
    ; exit x0
    mov rax, rdx
    jmp cleanup

fac_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab467
    ; substitute (n0 !-> n)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov r9, rdx
    ; create a1: _Cont = (a0, n)\{ ... \};
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
    je lab479
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab480

lab479:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab477
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab470
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab468
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab469

lab468:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab469:

lab470:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab473
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab471
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab472

lab471:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab472:

lab473:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab476
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab474
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab475

lab474:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab475:

lab476:
    jmp lab478

lab477:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab478:

lab480:
    ; #load tag
    lea rdi, [rel _Cont_481]
    ; lit x1 <- 1;
    mov r9, 1
    ; x2 <- n0 - x1;
    mov r11, rdx
    sub r11, r9
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump fac_
    jmp fac_

_Cont_481:

_Cont_481_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab483
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab482
    ; ####increment refcount
    add qword [rsi + 0], 1

lab482:
    jmp lab484

lab483:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab484:
    ; x4 <- n * x0;
    mov r11, r9
    imul r11, rdx
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab467:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x3 <- 1;
    mov rdi, 1
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

prod_:
    ; if n != 0 \{ ... \}
    cmp rdx, 0
    jne lab485
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; lit x6 <- 0;
    mov rdi, 0
    ; substitute (x6 !-> x6)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

lab485:
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; if x1 == 0 \{ ... \}
    cmp r13, 0
    je lab486
    ; substitute (n !-> n)(m0 !-> m)(a0 !-> a0)(m !-> m);
    ; #move variables
    mov r11, rdi
    ; create a1: _Cont = (a0, m)\{ ... \};
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
    je lab498
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab499

lab498:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab496
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab489
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab487
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab488

lab487:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab488:

lab489:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab492
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab490
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab491

lab490:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab491:

lab492:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab495
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab493
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab494

lab493:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab494:

lab495:
    jmp lab497

lab496:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab497:

lab499:
    ; #load tag
    lea r9, [rel _Cont_500]
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- n - x3;
    mov r13, rdx
    sub r13, r11
    ; substitute (x4 !-> x4)(m0 !-> m0)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump prod_
    jmp prod_

_Cont_500:

_Cont_500_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab502
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab501
    ; ####increment refcount
    add qword [rsi + 0], 1

lab501:
    jmp lab503

lab502:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab503:
    ; x5 <- m + x2;
    mov r11, r9
    add r11, rdx
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab486:
    ; substitute (m !-> m)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

monus_:
    ; if m == 0 \{ ... \}
    cmp rdi, 0
    je lab504
    ; if n != 0 \{ ... \}
    cmp rdx, 0
    jne lab505
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
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

lab505:
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (x1 !-> x1)(m !-> m)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- m - x2;
    mov r13, rdi
    sub r13, r11
    ; substitute (x1 !-> x1)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rdi, r13
    ; jump monus_
    jmp monus_

lab504:
    ; substitute (n !-> n)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
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