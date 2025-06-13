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
    lea r9, [rel _Cont_44658]
    ; jump main_loop_
    jmp main_loop_

_Cont_44658:

_Cont_44658_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

range_:
    ; if i < n \{ ... \}
    cmp rdx, rdi
    jl lab44659
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab44659:
    ; substitute (i0 !-> i)(n !-> n)(a0 !-> a0)(i !-> i);
    ; #move variables
    mov r11, rdx
    ; new a1: List[i64] = (a0, i)\{ ... \};
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
    je lab44671
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44672

lab44671:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44669
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44661

lab44660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44661:

lab44662:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44665
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44663
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44664

lab44663:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44664:

lab44665:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44668
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44666
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44667

lab44666:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44667:

lab44668:
    jmp lab44670

lab44669:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44670:

lab44672:
    ; #load tag
    lea r9, [rel List_i64_44673]
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- i0 + x1;
    mov r13, rdx
    add r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump range_
    jmp range_

List_i64_44673:
    jmp near List_i64_44673_Nil
    jmp near List_i64_44673_Cons

List_i64_44673_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44675
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab44674
    ; ####increment refcount
    add qword [rax + 0], 1

lab44674:
    jmp lab44676

lab44675:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab44676:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (i !-> i)(x0 !-> x0)(a0 !-> a0);
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

List_i64_44673_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44678
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44677
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44677:
    jmp lab44679

lab44678:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab44679:
    ; substitute (i !-> i)(a0 !-> a0)(a3 !-> a3)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x0: List[i64] = Cons(a3, xs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44691
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44692

lab44691:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44689
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44681

lab44680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44681:

lab44682:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44684

lab44683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44684:

lab44685:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44687

lab44686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44687:

lab44688:
    jmp lab44690

lab44689:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44690:

lab44692:
    ; #load tag
    mov r9, 5
    ; substitute (i !-> i)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

sum_:
    ; substitute (a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch xs \{ ... \};
    lea rcx, [rel List_i64_44693]
    add rcx, rdi
    jmp rcx

List_i64_44693:
    jmp near List_i64_44693_Nil
    jmp near List_i64_44693_Cons

List_i64_44693_Nil:
    ; lit x1 <- 0;
    mov rdi, 0
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_44693_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44695
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44694
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44694:
    mov rdi, [rsi + 40]
    jmp lab44696

lab44695:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab44696:
    ; substitute (ys !-> ys)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: _Cont = (y, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    mov [rbx + 40], rdi
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44708
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44709

lab44708:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44706
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44698

lab44697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44698:

lab44699:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44701

lab44700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44701:

lab44702:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44705
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44703
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44704

lab44703:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44704:

lab44705:
    jmp lab44707

lab44706:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44707:

lab44709:
    ; #load tag
    lea rdi, [rel _Cont_44710]
    ; jump sum_
    jmp sum_

_Cont_44710:

_Cont_44710_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44712
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44711
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44711:
    mov rdi, [rsi + 40]
    jmp lab44713

lab44712:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab44713:
    ; x2 <- y + x0;
    mov r11, rdi
    add r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: List[i64] = (n, a0, iters)\{ ... \};
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
    je lab44725
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44726

lab44725:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44723
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44716
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44714
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44715

lab44714:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44715:

lab44716:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44719
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44717
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44718

lab44717:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44718:

lab44719:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44722
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44720
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44721

lab44720:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44721:

lab44722:
    jmp lab44724

lab44723:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44724:

lab44726:
    ; #load tag
    lea rdi, [rel List_i64_44727]
    ; lit x1 <- 0;
    mov r9, 0
    ; substitute (x1 !-> x1)(n0 !-> n0)(a2 !-> a2);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump range_
    jmp range_

List_i64_44727:
    jmp near List_i64_44727_Nil
    jmp near List_i64_44727_Cons

List_i64_44727_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44729
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab44728
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44728:
    mov rdx, [rax + 24]
    jmp lab44730

lab44729:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab44730:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_i64_44727_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44732
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab44731
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44731:
    mov r9, [r8 + 24]
    jmp lab44733

lab44732:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab44733:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(a4 !-> a4)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let x0: List[i64] = Cons(a4, xs0);
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
    je lab44745
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44746

lab44745:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44743
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44736
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44734
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44735

lab44734:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44735:

lab44736:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44739
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44737
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44738

lab44737:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44738:

lab44739:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44742
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44740
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44741

lab44740:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44741:

lab44742:
    jmp lab44744

lab44743:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44744:

lab44746:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; substitute (x0 !-> x0)(iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a3: _Cont = (iters, n, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44758
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44759

lab44758:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44756
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44749
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44747
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44748

lab44747:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44748:

lab44749:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44752
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44750
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44751

lab44750:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44751:

lab44752:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44755
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44753
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44754

lab44753:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44754:

lab44755:
    jmp lab44757

lab44756:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44757:

lab44759:
    ; #load tag
    lea rdi, [rel _Cont_44760]
    ; jump sum_
    jmp sum_

_Cont_44760:

_Cont_44760_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44762
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab44761
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44761:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab44763

lab44762:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab44763:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp rdi, r13
    je lab44764
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- iters - x3;
    mov r13, rdi
    sub r13, r11
    ; substitute (x4 !-> x4)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab44764:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
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