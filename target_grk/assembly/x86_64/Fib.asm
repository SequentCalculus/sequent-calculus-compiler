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
    lea r9, [rel _Cont_44603]
    ; jump main_loop_
    jmp main_loop_

_Cont_44603:

_Cont_44603_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

fib_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab44604
    ; lit x0 <- 1;
    mov r9, 1
    ; if n == x0 \{ ... \}
    cmp rdx, r9
    je lab44605
    ; substitute (n0 !-> n)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov r9, rdx
    ; new a1: _Cont = (a0, n)\{ ... \};
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
    je lab44617
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44618

lab44617:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44615
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44608
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44606
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44607

lab44606:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44607:

lab44608:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44611
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44609
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44610

lab44609:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44610:

lab44611:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44614
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44612
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44613

lab44612:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44613:

lab44614:
    jmp lab44616

lab44615:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44616:

lab44618:
    ; #load tag
    lea rdi, [rel _Cont_44619]
    ; lit x2 <- 1;
    mov r9, 1
    ; x3 <- n0 - x2;
    mov r11, rdx
    sub r11, r9
    ; substitute (x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump fib_
    jmp fib_

_Cont_44619:

_Cont_44619_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44621
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44620
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44620:
    jmp lab44622

lab44621:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44622:
    ; substitute (n !-> n)(a0 !-> a0)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a3: _Cont = (a0, x1)\{ ... \};
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
    je lab44634
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44635

lab44634:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44632
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44625
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44623
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44624

lab44623:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44624:

lab44625:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44628
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44626
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44627

lab44626:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44627:

lab44628:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44631
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44629
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44630

lab44629:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44630:

lab44631:
    jmp lab44633

lab44632:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44633:

lab44635:
    ; #load tag
    lea rdi, [rel _Cont_44636]
    ; lit x5 <- 2;
    mov r9, 2
    ; x6 <- n - x5;
    mov r11, rdx
    sub r11, r9
    ; substitute (x6 !-> x6)(a3 !-> a3);
    ; #move variables
    mov rdx, r11
    ; jump fib_
    jmp fib_

_Cont_44636:

_Cont_44636_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44638
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44637
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44637:
    jmp lab44639

lab44638:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44639:
    ; x9 <- x1 + x4;
    mov r11, r9
    add r11, rdx
    ; substitute (x9 !-> x9)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab44605:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x8 <- 1;
    mov rdi, 1
    ; substitute (x8 !-> x8)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

lab44604:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x7 <- 0;
    mov rdi, 0
    ; substitute (x7 !-> x7)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

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
    je lab44651
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44652

lab44651:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44649
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44642
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44640
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44641

lab44640:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44641:

lab44642:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44645
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44643
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44644

lab44643:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44644:

lab44645:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44648
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44646
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44647

lab44646:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44647:

lab44648:
    jmp lab44650

lab44649:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44650:

lab44652:
    ; #load tag
    lea rdi, [rel _Cont_44653]
    ; jump fib_
    jmp fib_

_Cont_44653:

_Cont_44653_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44655
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44654
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44654:
    mov rdi, [rsi + 24]
    jmp lab44656

lab44655:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab44656:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab44657
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

lab44657:
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