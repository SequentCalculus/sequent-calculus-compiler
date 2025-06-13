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
    lea r9, [rel _Cont_65702]
    ; jump main_loop_
    jmp main_loop_

_Cont_65702:

_Cont_65702_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

useless_:
    ; if i < n \{ ... \}
    cmp rdx, rdi
    jl lab65703
    ; substitute (i !-> i)(a0 !-> a0);
    ; #erase b
    cmp r8, 0
    je lab65706
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab65704
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab65705

lab65704:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab65705:

lab65706:
    ; #move variables
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

lab65703:
    ; substitute (i !-> i)(n !-> n)(a0 !-> a0);
    ; #erase b
    cmp r8, 0
    je lab65709
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab65707
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab65708

lab65707:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab65708:

lab65709:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- i + x0;
    mov r13, rdx
    add r13, r11
    ; substitute (i !-> i)(n !-> n)(a0 !-> a0)(x1 !-> x1);
    ; #move variables
    mov r11, r13
    ; new a2: List[i64] = (n, a0, x1)\{ ... \};
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
    je lab65721
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65722

lab65721:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65719
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65712
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65710
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65711

lab65710:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65711:

lab65712:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65715
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65713
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65714

lab65713:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65714:

lab65715:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65718
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65716
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65717

lab65716:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65717:

lab65718:
    jmp lab65720

lab65719:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65720:

lab65722:
    ; #load tag
    lea rdi, [rel List_i64_65723]
    ; lit x3 <- 0;
    mov r9, 0
    ; let x4: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x3 !-> x3)(i !-> i)(x4 !-> x4)(a2 !-> a2);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r10
    mov r10, rsi
    ; jump replicate_
    jmp replicate_

List_i64_65723:
    jmp near List_i64_65723_Nil
    jmp near List_i64_65723_Cons

List_i64_65723_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65725
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65724
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65724:
    mov rdx, [rax + 24]
    jmp lab65726

lab65725:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab65726:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x1 !-> x1)(n !-> n)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r10
    mov r10, rsi
    ; jump useless_
    jmp useless_

List_i64_65723_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65728
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65727
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65727:
    mov r9, [r8 + 24]
    jmp lab65729

lab65728:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab65729:
    ; substitute (x1 !-> x1)(a0 !-> a0)(n !-> n)(x5 !-> x5)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let x2: List[i64] = Cons(x5, xs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65741
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65742

lab65741:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65739
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65732
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65730
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65731

lab65730:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65731:

lab65732:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65734

lab65733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65734:

lab65735:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65737

lab65736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65737:

lab65738:
    jmp lab65740

lab65739:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65740:

lab65742:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(n !-> n)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump useless_
    jmp useless_

replicate_:
    ; if n == 0 \{ ... \}
    cmp rdi, 0
    je lab65743
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- n - x0;
    mov r15, rdi
    sub r15, r13
    ; substitute (v !-> v)(x1 !-> x1)(a0 !-> a0)(v0 !-> v)(a !-> a);
    ; #move variables
    mov r13, r9
    mov r9, r11
    mov r11, rdx
    mov r12, r8
    mov r8, r10
    mov rdi, r15
    ; let x2: List[i64] = Cons(v0, a);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65755
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65756

lab65755:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65753
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65746
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65744
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65745

lab65744:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65745:

lab65746:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65749
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65747
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65748

lab65747:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65748:

lab65749:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65752
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65750
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65751

lab65750:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65751:

lab65752:
    jmp lab65754

lab65753:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65754:

lab65756:
    ; #load tag
    mov r11, 5
    ; substitute (v !-> v)(x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump replicate_
    jmp replicate_

lab65743:
    ; substitute (a0 !-> a0)(a !-> a);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rax, r10
    mov rdx, r11
    ; switch a \{ ... \};
    lea rcx, [rel List_i64_65757]
    add rcx, rdi
    jmp rcx

List_i64_65757:
    jmp near List_i64_65757_Nil
    jmp near List_i64_65757_Cons

List_i64_65757_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_65757_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65759
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65758
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65758:
    mov rdi, [rsi + 40]
    jmp lab65760

lab65759:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab65760:
    ; substitute (x3 !-> x3)(xs0 !-> xs0)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

main_loop_:
    ; lit x0 <- 0;
    mov r11, 0
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (x1 !-> x1)(n0 !-> n)(x0 !-> x0)(a0 !-> a0)(n !-> n)(iters !-> iters);
    ; #move variables
    mov r15, rdx
    mov rdx, r13
    mov r13, rdi
    mov r10, r8
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    mov rax, r12
    ; new a2: _Cont = (a0, n, iters)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    mov [rbx + 24], r11
    mov [rbx + 16], r10
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65772
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65773

lab65772:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65770
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65763
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65761
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65762

lab65761:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65762:

lab65763:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65766
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65764
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65765

lab65764:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65765:

lab65766:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65769
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65767
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65768

lab65767:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65768:

lab65769:
    jmp lab65771

lab65770:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65771:

lab65773:
    ; #load tag
    lea r11, [rel _Cont_65774]
    ; substitute (x0 !-> x0)(n0 !-> n0)(x1 !-> x1)(a2 !-> a2);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump useless_
    jmp useless_

_Cont_65774:

_Cont_65774_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65776
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab65775
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65775:
    jmp lab65777

lab65776:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab65777:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp r11, r13
    je lab65778
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov rdx, r11
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- iters - x3;
    mov r13, rdx
    sub r13, r11
    ; substitute (x4 !-> x4)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab65778:
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
    ; lit x5 <- 0;
    mov rdi, 0
    ; substitute (x5 !-> x5)(a0 !-> a0);
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