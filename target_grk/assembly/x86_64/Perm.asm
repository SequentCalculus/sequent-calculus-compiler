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
    mov r9, rcx
    ; move parameters into place
    mov rdi, rdx
    ; move parameters into place
    mov rdx, rsi
    ; actual code

main_:
    ; new a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel _Cont_50675]
    ; jump main_loop_
    jmp main_loop_

_Cont_50675:

_Cont_50675_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

empty_i_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_50676]
    add rcx, rdi
    jmp rcx

List_i64_50676:
    jmp near List_i64_50676_Nil
    jmp near List_i64_50676_Cons

List_i64_50676_Nil:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_i64_50676_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50678
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50677
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50677:
    mov rdi, [rsi + 40]
    jmp lab50679

lab50678:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50679:
    ; substitute (a0 !-> a0);
    ; #erase is
    cmp r8, 0
    je lab50682
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab50680
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab50681

lab50680:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab50681:

lab50682:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

tail_i_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_50683]
    add rcx, rdi
    jmp rcx

List_i64_50683:
    jmp near List_i64_50683_Nil
    jmp near List_i64_50683_Cons

List_i64_50683_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_50683_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50685
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50684
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50684:
    mov rdi, [rsi + 40]
    jmp lab50686

lab50685:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50686:
    ; substitute (a0 !-> a0)(is !-> is);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch is \{ ... \};
    lea rcx, [rel List_i64_50687]
    add rcx, rdi
    jmp rcx

List_i64_50687:
    jmp near List_i64_50687_Nil
    jmp near List_i64_50687_Cons

List_i64_50687_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_50687_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50689
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50688
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50688:
    mov rdi, [rsi + 40]
    jmp lab50690

lab50689:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50690:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

head_i_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_50691]
    add rcx, rdi
    jmp rcx

List_i64_50691:
    jmp near List_i64_50691_Nil
    jmp near List_i64_50691_Cons

List_i64_50691_Nil:
    ; lit x0 <- 0;
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_50691_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50693
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50692
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50692:
    mov rdi, [rsi + 40]
    jmp lab50694

lab50693:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50694:
    ; substitute (i !-> i)(a0 !-> a0);
    ; #erase is
    cmp r8, 0
    je lab50697
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab50695
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab50696

lab50695:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab50696:

lab50697:
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

len_i_loop_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_50698]
    add rcx, r9
    jmp rcx

List_i64_50698:
    jmp near List_i64_50698_Nil
    jmp near List_i64_50698_Cons

List_i64_50698_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_50698_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50700
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50699
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50699:
    mov r9, [r8 + 40]
    jmp lab50701

lab50700:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab50701:
    ; substitute (a0 !-> a0)(acc !-> acc)(iss !-> iss);
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- acc + x0;
    mov r13, rdi
    add r13, r11
    ; substitute (iss !-> iss)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump len_i_loop_
    jmp len_i_loop_

len_i_:
    ; lit x0 <- 0;
    mov r9, 0
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump len_i_loop_
    jmp len_i_loop_

empty_l_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_50702]
    add rcx, rdi
    jmp rcx

List_List_i64_50702:
    jmp near List_List_i64_50702_Nil
    jmp near List_List_i64_50702_Cons

List_List_i64_50702_Nil:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_List_i64_50702_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50705
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50703
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50703:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab50704
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50704:
    jmp lab50706

lab50705:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab50706:
    ; substitute (a0 !-> a0);
    ; #erase is
    cmp rsi, 0
    je lab50709
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab50707
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab50708

lab50707:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab50708:

lab50709:
    ; #erase iss
    cmp r8, 0
    je lab50712
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab50710
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab50711

lab50710:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab50711:

lab50712:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

tail_l_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_50713]
    add rcx, rdi
    jmp rcx

List_List_i64_50713:
    jmp near List_List_i64_50713_Nil
    jmp near List_List_i64_50713_Cons

List_List_i64_50713_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_50713_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50716
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50714
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50714:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab50715
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50715:
    jmp lab50717

lab50716:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab50717:
    ; substitute (a0 !-> a0)(iss !-> iss);
    ; #erase is
    cmp rsi, 0
    je lab50720
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab50718
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab50719

lab50718:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab50719:

lab50720:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch iss \{ ... \};
    lea rcx, [rel List_List_i64_50721]
    add rcx, rdi
    jmp rcx

List_List_i64_50721:
    jmp near List_List_i64_50721_Nil
    jmp near List_List_i64_50721_Cons

List_List_i64_50721_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_50721_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50724
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50722
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50722:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab50723
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50723:
    jmp lab50725

lab50724:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab50725:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

head_l_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_50726]
    add rcx, rdi
    jmp rcx

List_List_i64_50726:
    jmp near List_List_i64_50726_Nil
    jmp near List_List_i64_50726_Cons

List_List_i64_50726_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_50726_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50729
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50727
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50727:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab50728
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50728:
    jmp lab50730

lab50729:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab50730:
    ; substitute (a0 !-> a0)(is !-> is);
    ; #erase iss
    cmp r8, 0
    je lab50733
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab50731
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab50732

lab50731:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab50732:

lab50733:
    ; switch is \{ ... \};
    lea rcx, [rel List_i64_50734]
    add rcx, rdi
    jmp rcx

List_i64_50734:
    jmp near List_i64_50734_Nil
    jmp near List_i64_50734_Cons

List_i64_50734_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_50734_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50736
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50735
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50735:
    mov rdi, [rsi + 40]
    jmp lab50737

lab50736:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50737:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

loop_p_:
    ; if j == 0 \{ ... \}
    cmp rdx, 0
    je lab50738
    ; lit x4 <- 1;
    mov r15, 1
    ; x5 <- n - x4;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x5 !-> x5)(perms !-> perms)(x !-> x)(n !-> n)(a0 !-> a0)(j !-> j);
    ; #move variables
    mov r15, rdx
    mov rdx, [rsp + 2024]
    ; new a4: Pair[List[List[i64]], List[i64]] = (n, a0, j)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    mov [rbx + 40], r13
    mov [rbx + 32], r12
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50750
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50751

lab50750:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50748
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50740

lab50739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50740:

lab50741:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50744
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50742
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50743

lab50742:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50743:

lab50744:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50747
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50745
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50746

lab50745:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50746:

lab50747:
    jmp lab50749

lab50748:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50749:

lab50751:
    ; #load tag
    lea r11, [rel Pair_List_List_i64_List_i64_50752]
    ; jump p_
    jmp p_

Pair_List_List_i64_List_i64_50752:

Pair_List_List_i64_List_i64_50752_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50754
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab50753
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50753:
    mov r9, [r8 + 24]
    jmp lab50755

lab50754:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab50755:
    ; substitute (j !-> j)(a0 !-> a0)(n !-> n)(a7 !-> a7)(b1 !-> b1);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let pair_perms_x: Pair[List[List[i64]], List[i64]] = Tup(a7, b1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50767
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50768

lab50767:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50765
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50758
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50756
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50757

lab50756:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50757:

lab50758:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50760

lab50759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50760:

lab50761:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50763

lab50762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50763:

lab50764:
    jmp lab50766

lab50765:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50766:

lab50768:
    ; #load tag
    mov r11, 0
    ; switch pair_perms_x \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_List_List_i64_List_i64_50769:

Pair_List_List_i64_List_i64_50769_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab50772
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab50770
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50770:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab50771
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50771:
    jmp lab50773

lab50772:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab50773:
    ; substitute (x0 !-> x0)(perms0 !-> perms0)(n0 !-> n)(n !-> n)(a0 !-> a0)(j !-> j);
    ; #move variables
    mov r15, rdx
    mov rax, r12
    mov r12, rsi
    mov rdx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, r9
    mov rsi, r10
    ; new a5: Pair[List[List[i64]], List[i64]] = (n, a0, j)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    mov [rbx + 40], r13
    mov [rbx + 32], r12
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50785
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50786

lab50785:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50783
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50776
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50774
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50775

lab50774:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50775:

lab50776:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50779
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50777
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50778

lab50777:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50778:

lab50779:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50782
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50780
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50781

lab50780:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50781:

lab50782:
    jmp lab50784

lab50783:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50784:

lab50786:
    ; #load tag
    lea r11, [rel Pair_List_List_i64_List_i64_50787]
    ; substitute (n0 !-> n0)(perms0 !-> perms0)(x0 !-> x0)(a5 !-> a5);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump f_
    jmp f_

Pair_List_List_i64_List_i64_50787:

Pair_List_List_i64_List_i64_50787_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50789
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab50788
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50788:
    mov r9, [r8 + 24]
    jmp lab50790

lab50789:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab50790:
    ; substitute (j !-> j)(a0 !-> a0)(n !-> n)(a6 !-> a6)(b0 !-> b0);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let pair_perms_x0: Pair[List[List[i64]], List[i64]] = Tup(a6, b0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50802
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50803

lab50802:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50800
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50793
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50791
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50792

lab50791:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50792:

lab50793:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50795

lab50794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50795:

lab50796:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50798

lab50797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50798:

lab50799:
    jmp lab50801

lab50800:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50801:

lab50803:
    ; #load tag
    mov r11, 0
    ; switch pair_perms_x0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_List_List_i64_List_i64_50804:

Pair_List_List_i64_List_i64_50804_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab50807
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab50805
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50805:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab50806
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50806:
    jmp lab50808

lab50807:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab50808:
    ; lit x6 <- 1;
    mov r15, 1
    ; x7 <- j - x6;
    mov rcx, rdx
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x7 !-> x7)(perms1 !-> perms1)(x1 !-> x1)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, r12
    mov r12, rsi
    mov rcx, r11
    mov r11, r9
    mov r9, r13
    mov r13, rdi
    mov rdi, rcx
    mov rsi, r10
    mov rdx, [rsp + 2024]
    ; jump loop_p_
    jmp loop_p_

lab50738:
    ; substitute (a0 !-> a0)(perms !-> perms)(x !-> x)(n !-> n);
    ; #move variables
    mov rax, r12
    mov rdx, r13
    ; lit x2 <- 1;
    mov r13, 1
    ; x3 <- n - x2;
    mov r15, r11
    sub r15, r13
    ; substitute (x3 !-> x3)(perms !-> perms)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rdx, r15
    ; jump p_
    jmp p_

p_:
    ; lit x0 <- 1;
    mov r13, 1
    ; if x0 < n \{ ... \}
    cmp r13, rdx
    jl lab50809
    ; substitute (perms !-> perms)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    mov r8, r10
    mov r9, r11
    ; invoke a0 Tup
    jmp r9

lab50809:
    ; substitute (n !-> n)(perms !-> perms)(x !-> x)(a0 !-> a0);
    ; lit x1 <- 1;
    mov r13, 1
    ; x2 <- n - x1;
    mov r15, rdx
    sub r15, r13
    ; substitute (x2 !-> x2)(perms !-> perms)(x !-> x)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r13, r11
    mov r11, rdx
    mov r12, r10
    mov rdx, r15
    ; jump loop_p_
    jmp loop_p_

f_:
    ; substitute (n0 !-> n)(x2 !-> x)(x !-> x)(a0 !-> a0)(n !-> n)(perms !-> perms);
    ; #share x
    cmp r8, 0
    je lab50810
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50810:
    ; #move variables
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a1: List[i64] = (x, a0, n, perms)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
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
    je lab50822
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50823

lab50822:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50820
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50812

lab50811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50812:

lab50813:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50815

lab50814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50815:

lab50816:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50818

lab50817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50818:

lab50819:
    jmp lab50821

lab50820:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50821:

lab50823:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab50835
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50836

lab50835:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50833
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50826
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50824
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50825

lab50824:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50825:

lab50826:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50829
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50827
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50828

lab50827:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50828:

lab50829:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50831

lab50830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50831:

lab50832:
    jmp lab50834

lab50833:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50834:

lab50836:
    ; #load tag
    lea r9, [rel List_i64_50837]
    ; substitute (x2 !-> x2)(n0 !-> n0)(a1 !-> a1);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump list_tail_
    jmp list_tail_

List_i64_50837:
    jmp near List_i64_50837_Nil
    jmp near List_i64_50837_Cons

List_i64_50837_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50841
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50838
    ; ####increment refcount
    add qword [rax + 0], 1

lab50838:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab50839
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50839:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab50840
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50840:
    jmp lab50842

lab50841:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab50842:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(n !-> n)(perms !-> perms)(x !-> x)(x1 !-> x1);
    ; #move variables
    mov r8, r10
    mov r10, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_f_0_
    jmp lift_f_0_

List_i64_50837_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50846
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50843
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50843:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab50844
    ; ####increment refcount
    add qword [r14 + 0], 1

lab50844:
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab50845
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50845:
    jmp lab50847

lab50846:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab50847:
    ; substitute (perms !-> perms)(n !-> n)(x !-> x)(a0 !-> a0)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    ; let x1: List[i64] = Cons(a4, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50859
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab50860

lab50859:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50857
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50850
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50848
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50849

lab50848:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50849:

lab50850:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50853
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50851
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50852

lab50851:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50852:

lab50853:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50856
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50854
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50855

lab50854:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50855:

lab50856:
    jmp lab50858

lab50857:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50858:

lab50860:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(n !-> n)(perms !-> perms)(x !-> x)(x1 !-> x1);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_f_0_
    jmp lift_f_0_

lift_f_0_:
    ; substitute (x1 !-> x1)(n !-> n)(x !-> x)(perms !-> perms)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; new a2: List[i64] = (perms, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50872
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50873

lab50872:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50870
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50863
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50861
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50862

lab50861:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50862:

lab50863:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50866
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50864
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50865

lab50864:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50865:

lab50866:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50869
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50867
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50868

lab50867:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50868:

lab50869:
    jmp lab50871

lab50870:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50871:

lab50873:
    ; #load tag
    lea r11, [rel List_i64_50874]
    ; substitute (x !-> x)(n !-> n)(x1 !-> x1)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_loop_
    jmp rev_loop_

List_i64_50874:
    jmp near List_i64_50874_Nil
    jmp near List_i64_50874_Cons

List_i64_50874_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50877
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab50875
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50875:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50876
    ; ####increment refcount
    add qword [rax + 0], 1

lab50876:
    jmp lab50878

lab50877:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50878:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(perms !-> perms)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_f_1_
    jmp lift_f_1_

List_i64_50874_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50881
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50879
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50879:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50880
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50880:
    jmp lab50882

lab50881:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50882:
    ; substitute (a0 !-> a0)(perms !-> perms)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x0: List[i64] = Cons(a3, as0);
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
    je lab50894
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50895

lab50894:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50892
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50885
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50883
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50884

lab50883:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50884:

lab50885:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50888
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50886
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50887

lab50886:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50887:

lab50888:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50891
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50889
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50890

lab50889:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50890:

lab50891:
    jmp lab50893

lab50892:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50893:

lab50895:
    ; #load tag
    mov r9, 5
    ; jump lift_f_1_
    jmp lift_f_1_

lift_f_1_:
    ; substitute (a0 !-> a0)(x0 !-> x0)(x00 !-> x0)(perms !-> perms);
    ; #share x0
    cmp r8, 0
    je lab50896
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50896:
    ; #move variables
    mov r10, rsi
    mov r11, rdi
    mov rsi, r8
    mov rdi, r9
    ; let perms0: List[List[i64]] = Cons(x00, perms);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab50908
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50909

lab50908:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50906
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50899
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50897
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50898

lab50897:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50898:

lab50899:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50902
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50900
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50901

lab50900:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50901:

lab50902:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50905
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50903
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50904

lab50903:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50904:

lab50905:
    jmp lab50907

lab50906:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50907:

lab50909:
    ; #load tag
    mov r9, 5
    ; substitute (perms0 !-> perms0)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Tup
    jmp r9

rev_loop_:
    ; if n == 0 \{ ... \}
    cmp rdi, 0
    je lab50910
    ; substitute (x5 !-> x)(n !-> n)(y !-> y)(a0 !-> a0)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab50911
    ; ####increment refcount
    add qword [rax + 0], 1

lab50911:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    ; new a1: List[i64] = (n, y, a0, x)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    mov [rbx + 24], r9
    mov [rbx + 16], r8
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50923
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50924

lab50923:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50921
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50914
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50912
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50913

lab50912:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50913:

lab50914:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50917
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50915
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50916

lab50915:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50916:

lab50917:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50920
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50918
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50919

lab50918:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50919:

lab50920:
    jmp lab50922

lab50921:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50922:

lab50924:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab50936
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50937

lab50936:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50934
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50927
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50925
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50926

lab50925:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50926:

lab50927:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50930
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50928
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50929

lab50928:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50929:

lab50930:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50933
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50931
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50932

lab50931:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50932:

lab50933:
    jmp lab50935

lab50934:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50935:

lab50937:
    ; #load tag
    lea rdi, [rel List_i64_50938]
    ; jump tail_i_
    jmp tail_i_

List_i64_50938:
    jmp near List_i64_50938_Nil
    jmp near List_i64_50938_Cons

List_i64_50938_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50942
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab50939
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50939:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab50940
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50940:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab50941
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50941:
    jmp lab50943

lab50942:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab50943:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(n !-> n)(x !-> x)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, r10
    mov r10, r12
    mov r12, rsi
    ; jump lift_rev_loop_0_
    jmp lift_rev_loop_0_

List_i64_50938_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50947
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab50944
    ; ####increment refcount
    add qword [r14 + 0], 1

lab50944:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab50945
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50945:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab50946
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50946:
    jmp lab50948

lab50947:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab50948:
    ; substitute (x !-> x)(a0 !-> a0)(n !-> n)(y !-> y)(a5 !-> a5)(as1 !-> as1);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    mov rsi, r12
    ; let x0: List[i64] = Cons(a5, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50960
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab50961

lab50960:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50958
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50951
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50949
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50950

lab50949:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50950:

lab50951:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50954
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50952
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50953

lab50952:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50953:

lab50954:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50957
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50955
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50956

lab50955:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50956:

lab50957:
    jmp lab50959

lab50958:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50959:

lab50961:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(n !-> n)(x !-> x)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_rev_loop_0_
    jmp lift_rev_loop_0_

lab50910:
    ; substitute (a0 !-> a0)(y !-> y);
    ; #erase x
    cmp rax, 0
    je lab50964
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab50962
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab50963

lab50962:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab50963:

lab50964:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rax, r10
    mov rdx, r11
    ; switch y \{ ... \};
    lea rcx, [rel List_i64_50965]
    add rcx, rdi
    jmp rcx

List_i64_50965:
    jmp near List_i64_50965_Nil
    jmp near List_i64_50965_Cons

List_i64_50965_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_50965_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50967
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50966
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50966:
    mov rdi, [rsi + 40]
    jmp lab50968

lab50967:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50968:
    ; substitute (a4 !-> a4)(as0 !-> as0)(a0 !-> a0);
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

lift_rev_loop_0_:
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- n - x1;
    mov rcx, rdi
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x !-> x)(a0 !-> a0)(x2 !-> x2)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    mov r9, [rsp + 2024]
    ; new a3: _Cont = (a0, x2, x0, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    mov [rbx + 24], r9
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50980
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50981

lab50980:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50978
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50970

lab50969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50970:

lab50971:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50974
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50972
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50973

lab50972:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50973:

lab50974:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50977
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50975
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50976

lab50975:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50976:

lab50977:
    jmp lab50979

lab50978:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50979:

lab50981:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab50993
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50994

lab50993:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50991
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50983

lab50982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50983:

lab50984:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50987
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50985
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50986

lab50985:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50986:

lab50987:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50990
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50988
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50989

lab50988:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50989:

lab50990:
    jmp lab50992

lab50991:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50992:

lab50994:
    ; #load tag
    lea rdi, [rel _Cont_50995]
    ; jump head_i_
    jmp head_i_

_Cont_50995:

_Cont_50995_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50999
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab50996
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50996:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab50997
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50997:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab50998
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50998:
    mov r9, [r8 + 24]
    jmp lab51000

lab50999:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab51000:
    ; substitute (x0 !-> x0)(a0 !-> a0)(x2 !-> x2)(x4 !-> x4)(y !-> y);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r10
    ; let x3: List[i64] = Cons(x4, y);
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
    je lab51012
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51013

lab51012:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51010
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51002

lab51001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51002:

lab51003:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51005

lab51004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51005:

lab51006:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51009
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51007
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51008

lab51007:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51008:

lab51009:
    jmp lab51011

lab51010:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51011:

lab51013:
    ; #load tag
    mov r11, 5
    ; substitute (x0 !-> x0)(x2 !-> x2)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump rev_loop_
    jmp rev_loop_

list_tail_:
    ; if n == 0 \{ ... \}
    cmp rdi, 0
    je lab51014
    ; new a1: List[i64] = (n, a0)\{ ... \};
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
    je lab51026
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51027

lab51026:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51024
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51017
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51015
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51016

lab51015:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51016:

lab51017:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51020
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51018
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51019

lab51018:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51019:

lab51020:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51023
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51021
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51022

lab51021:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51022:

lab51023:
    jmp lab51025

lab51024:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51025:

lab51027:
    ; #load tag
    lea rdi, [rel List_i64_51028]
    ; jump tail_i_
    jmp tail_i_

List_i64_51028:
    jmp near List_i64_51028_Nil
    jmp near List_i64_51028_Cons

List_i64_51028_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51030
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab51029
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51029:
    mov rdx, [rax + 40]
    jmp lab51031

lab51030:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab51031:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_list_tail_0_
    jmp lift_list_tail_0_

List_i64_51028_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51033
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab51032
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51032:
    mov r9, [r8 + 40]
    jmp lab51034

lab51033:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab51034:
    ; substitute (a0 !-> a0)(n !-> n)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x0: List[i64] = Cons(a4, as1);
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
    je lab51046
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51047

lab51046:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51044
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51037
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51035
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51036

lab51035:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51036:

lab51037:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51040
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51038
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51039

lab51038:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51039:

lab51040:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51042

lab51041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51042:

lab51043:
    jmp lab51045

lab51044:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51045:

lab51047:
    ; #load tag
    mov r9, 5
    ; jump lift_list_tail_0_
    jmp lift_list_tail_0_

lab51014:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; switch x \{ ... \};
    lea rcx, [rel List_i64_51048]
    add rcx, rdi
    jmp rcx

List_i64_51048:
    jmp near List_i64_51048_Nil
    jmp near List_i64_51048_Cons

List_i64_51048_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_51048_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51050
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51049
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51049:
    mov rdi, [rsi + 40]
    jmp lab51051

lab51050:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab51051:
    ; substitute (a3 !-> a3)(as0 !-> as0)(a0 !-> a0);
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

lift_list_tail_0_:
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- n - x1;
    mov r13, rdi
    sub r13, r11
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump list_tail_
    jmp list_tail_

permutations_:
    ; substitute (x01 !-> x0)(a0 !-> a0)(x0 !-> x0);
    ; #share x0
    cmp rax, 0
    je lab51052
    ; ####increment refcount
    add qword [rax + 0], 1

lab51052:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a1: _Cont = (a0, x0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
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
    je lab51064
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51065

lab51064:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51062
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51055
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51053
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51054

lab51053:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51054:

lab51055:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51058
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51056
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51057

lab51056:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51057:

lab51058:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51061
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51059
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51060

lab51059:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51060:

lab51061:
    jmp lab51063

lab51062:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51063:

lab51065:
    ; #load tag
    lea rdi, [rel _Cont_51066]
    ; jump len_i_
    jmp len_i_

_Cont_51066:

_Cont_51066_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51069
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51067
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51067:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51068
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51068:
    jmp lab51070

lab51069:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51070:
    ; let x3: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x1 !-> x1)(a0 !-> a0)(x0 !-> x0)(x00 !-> x0)(x3 !-> x3);
    ; #share x0
    cmp r8, 0
    je lab51071
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51071:
    ; #move variables
    mov r12, r10
    mov r10, r8
    mov r13, r11
    mov r11, r9
    ; let x2: List[List[i64]] = Cons(x00, x3);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51083
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51084

lab51083:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51081
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51073

lab51072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51073:

lab51074:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51077
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51075
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51076

lab51075:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51076:

lab51077:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51080
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51078
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51079

lab51078:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51079:

lab51080:
    jmp lab51082

lab51081:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51082:

lab51084:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a2: Pair[List[List[i64]], List[i64]] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51096
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51097

lab51096:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51094
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51086

lab51085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51086:

lab51087:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51089

lab51088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51089:

lab51090:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51093
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51091
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51092

lab51091:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51092:

lab51093:
    jmp lab51095

lab51094:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51095:

lab51097:
    ; #load tag
    lea r11, [rel Pair_List_List_i64_List_i64_51098]
    ; jump p_
    jmp p_

Pair_List_List_i64_List_i64_51098:

Pair_List_List_i64_List_i64_51098_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51100
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab51099
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51099:
    jmp lab51101

lab51100:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab51101:
    ; substitute (a0 !-> a0)(final_perms !-> final_perms);
    ; #erase x
    cmp rsi, 0
    je lab51104
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab51102
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab51103

lab51102:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab51103:

lab51104:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; switch final_perms \{ ... \};
    lea rcx, [rel List_List_i64_51105]
    add rcx, rdi
    jmp rcx

List_List_i64_51105:
    jmp near List_List_i64_51105_Nil
    jmp near List_List_i64_51105_Cons

List_List_i64_51105_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_51105_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51108
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51106
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51106:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51107
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51107:
    jmp lab51109

lab51108:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51109:
    ; substitute (a3 !-> a3)(as0 !-> as0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

loop_sum2_:
    ; substitute (y1 !-> y)(sum !-> sum)(a0 !-> a0)(y !-> y);
    ; #share y
    cmp rax, 0
    je lab51110
    ; ####increment refcount
    add qword [rax + 0], 1

lab51110:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; new a4: Bool = (sum, a0, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51122
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51123

lab51122:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51120
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51113
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51111
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51112

lab51111:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51112:

lab51113:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51116
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51114
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51115

lab51114:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51115:

lab51116:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51119
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51117
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51118

lab51117:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51118:

lab51119:
    jmp lab51121

lab51120:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51121:

lab51123:
    ; #load tag
    lea rdi, [rel Bool_51124]
    ; jump empty_i_
    jmp empty_i_

Bool_51124:
    jmp near Bool_51124_True
    jmp near Bool_51124_False

Bool_51124_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51127
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab51125
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51125:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51126
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51126:
    mov rdx, [rax + 24]
    jmp lab51128

lab51127:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab51128:
    ; substitute (sum !-> sum)(a0 !-> a0);
    ; #erase y
    cmp r8, 0
    je lab51131
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab51129
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab51130

lab51129:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab51130:

lab51131:
    ; invoke a0 Ret
    jmp rdi

Bool_51124_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51134
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab51132
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51132:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51133
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51133:
    mov rdx, [rax + 24]
    jmp lab51135

lab51134:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab51135:
    ; substitute (y0 !-> y)(a0 !-> a0)(y !-> y)(sum !-> sum);
    ; #share y
    cmp r8, 0
    je lab51136
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51136:
    ; #move variables
    mov r11, rdx
    mov rax, r8
    mov rdx, r9
    ; new a1: List[i64] = (a0, y, sum)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51148
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51149

lab51148:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51146
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51139
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51137
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51138

lab51137:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51138:

lab51139:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51142
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51140
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51141

lab51140:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51141:

lab51142:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51145
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51143
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51144

lab51143:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51144:

lab51145:
    jmp lab51147

lab51146:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51147:

lab51149:
    ; #load tag
    lea rdi, [rel List_i64_51150]
    ; jump tail_i_
    jmp tail_i_

List_i64_51150:
    jmp near List_i64_51150_Nil
    jmp near List_i64_51150_Cons

List_i64_51150_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51153
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51151
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51151:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab51152
    ; ####increment refcount
    add qword [rax + 0], 1

lab51152:
    jmp lab51154

lab51153:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab51154:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(sum !-> sum)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump lift_loop_sum2_0_
    jmp lift_loop_sum2_0_

List_i64_51150_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51157
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51155
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51155:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51156
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51156:
    jmp lab51158

lab51157:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab51158:
    ; substitute (sum !-> sum)(y !-> y)(a0 !-> a0)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let x0: List[i64] = Cons(a5, as0);
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
    je lab51170
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51171

lab51170:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51168
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51161
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51159
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51160

lab51159:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51160:

lab51161:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51164
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51162
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51163

lab51162:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51163:

lab51164:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51167
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51165
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51166

lab51165:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51166:

lab51167:
    jmp lab51169

lab51168:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51169:

lab51171:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(sum !-> sum)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, r10
    mov r10, rsi
    ; jump lift_loop_sum2_0_
    jmp lift_loop_sum2_0_

lift_loop_sum2_0_:
    ; substitute (y !-> y)(sum !-> sum)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a3: _Cont = (sum, x0, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51183
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51184

lab51183:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51181
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51174
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51172
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51173

lab51172:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51173:

lab51174:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51177
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51175
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51176

lab51175:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51176:

lab51177:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51180
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51178
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51179

lab51178:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51179:

lab51180:
    jmp lab51182

lab51181:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51182:

lab51184:
    ; #load tag
    lea rdi, [rel _Cont_51185]
    ; jump head_i_
    jmp head_i_

_Cont_51185:

_Cont_51185_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51188
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab51186
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51186:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab51187
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51187:
    mov rdi, [rsi + 24]
    jmp lab51189

lab51188:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab51189:
    ; x2 <- sum + x1;
    mov r13, rdi
    add r13, rdx
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    mov r8, r10
    mov r9, r11
    mov rdi, r13
    ; jump loop_sum2_
    jmp loop_sum2_

loop_sum1_:
    ; substitute (x4 !-> x)(sum !-> sum)(a0 !-> a0)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab51190
    ; ####increment refcount
    add qword [rax + 0], 1

lab51190:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; new a4: Bool = (sum, a0, x)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51202
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51203

lab51202:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51200
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51193
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51191
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51192

lab51191:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51192:

lab51193:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51196
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51194
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51195

lab51194:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51195:

lab51196:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51199
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51197
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51198

lab51197:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51198:

lab51199:
    jmp lab51201

lab51200:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51201:

lab51203:
    ; #load tag
    lea rdi, [rel Bool_51204]
    ; jump empty_l_
    jmp empty_l_

Bool_51204:
    jmp near Bool_51204_True
    jmp near Bool_51204_False

Bool_51204_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51207
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab51205
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51205:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51206
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51206:
    mov rdx, [rax + 24]
    jmp lab51208

lab51207:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab51208:
    ; substitute (sum !-> sum)(a0 !-> a0);
    ; #erase x
    cmp r8, 0
    je lab51211
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab51209
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab51210

lab51209:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab51210:

lab51211:
    ; invoke a0 Ret
    jmp rdi

Bool_51204_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51214
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab51212
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51212:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51213
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51213:
    mov rdx, [rax + 24]
    jmp lab51215

lab51214:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab51215:
    ; substitute (x3 !-> x)(a0 !-> a0)(x !-> x)(sum !-> sum);
    ; #share x
    cmp r8, 0
    je lab51216
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51216:
    ; #move variables
    mov r11, rdx
    mov rax, r8
    mov rdx, r9
    ; new a1: List[List[i64]] = (a0, x, sum)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51228
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51229

lab51228:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51226
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51219
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51217
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51218

lab51217:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51218:

lab51219:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51222
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51220
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51221

lab51220:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51221:

lab51222:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51225
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51223
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51224

lab51223:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51224:

lab51225:
    jmp lab51227

lab51226:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51227:

lab51229:
    ; #load tag
    lea rdi, [rel List_List_i64_51230]
    ; jump tail_l_
    jmp tail_l_

List_List_i64_51230:
    jmp near List_List_i64_51230_Nil
    jmp near List_List_i64_51230_Cons

List_List_i64_51230_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51233
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab51231
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51231:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab51232
    ; ####increment refcount
    add qword [rax + 0], 1

lab51232:
    jmp lab51234

lab51233:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab51234:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(sum !-> sum)(x !-> x)(x0 !-> x0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_loop_sum1_0_
    jmp lift_loop_sum1_0_

List_List_i64_51230_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51237
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51235
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51235:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51236
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51236:
    jmp lab51238

lab51237:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab51238:
    ; substitute (sum !-> sum)(x !-> x)(a0 !-> a0)(a6 !-> a6)(as1 !-> as1);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a6, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51250
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51251

lab51250:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51248
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51241
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51239
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51240

lab51239:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51240:

lab51241:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51244
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51242
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51243

lab51242:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51243:

lab51244:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51247
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51245
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51246

lab51245:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51246:

lab51247:
    jmp lab51249

lab51248:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51249:

lab51251:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(sum !-> sum)(x !-> x)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump lift_loop_sum1_0_
    jmp lift_loop_sum1_0_

lift_loop_sum1_0_:
    ; substitute (x !-> x)(sum !-> sum)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: _Cont = (a0, x0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51263
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51264

lab51263:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51261
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51254
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51252
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51253

lab51252:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51253:

lab51254:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51257
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51255
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51256

lab51255:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51256:

lab51257:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51260
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51258
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51259

lab51258:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51259:

lab51260:
    jmp lab51262

lab51261:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51262:

lab51264:
    ; #load tag
    lea r9, [rel _Cont_51265]
    ; new a3: List[i64] = (sum, a2)\{ ... \};
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
    je lab51277
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51278

lab51277:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51275
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51268
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51266
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51267

lab51266:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51267:

lab51268:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51271
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51269
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51270

lab51269:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51270:

lab51271:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51274
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51272
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51273

lab51272:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51273:

lab51274:
    jmp lab51276

lab51275:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51276:

lab51278:
    ; #load tag
    lea rdi, [rel List_i64_51279]
    ; jump head_l_
    jmp head_l_

List_i64_51279:
    jmp near List_i64_51279_Nil
    jmp near List_i64_51279_Cons

List_i64_51279_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51281
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab51280
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51280:
    mov rdx, [rax + 40]
    jmp lab51282

lab51281:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab51282:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(sum !-> sum)(a2 !-> a2);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump loop_sum2_
    jmp loop_sum2_

List_i64_51279_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51284
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab51283
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51283:
    mov r9, [r8 + 40]
    jmp lab51285

lab51284:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab51285:
    ; substitute (a2 !-> a2)(sum !-> sum)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x2: List[i64] = Cons(a5, as0);
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
    je lab51297
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51298

lab51297:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51295
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51288
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51286
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51287

lab51286:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51287:

lab51288:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51291
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51289
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51290

lab51289:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51290:

lab51291:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51294
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51292
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51293

lab51292:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51293:

lab51294:
    jmp lab51296

lab51295:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51296:

lab51298:
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(sum !-> sum)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump loop_sum2_
    jmp loop_sum2_

_Cont_51265:

_Cont_51265_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51301
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51299
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51299:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51300
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51300:
    jmp lab51302

lab51301:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51302:
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump loop_sum1_
    jmp loop_sum1_

sumlists_:
    ; lit x0 <- 0;
    mov r9, 0
    ; substitute (x !-> x)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump loop_sum1_
    jmp loop_sum1_

loop_one2n_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab51303
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (x1 !-> x1)(a0 !-> a0)(n !-> n)(p !-> p);
    ; #move variables
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov r10, rsi
    mov rsi, r8
    mov rdx, r13
    ; let x2: List[i64] = Cons(n, p);
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
    je lab51315
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51316

lab51315:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51313
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51306
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51304
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51305

lab51304:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51305:

lab51306:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51309
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51307
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51308

lab51307:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51308:

lab51309:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51312
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51310
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51311

lab51310:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51311:

lab51312:
    jmp lab51314

lab51313:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51314:

lab51316:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump loop_one2n_
    jmp loop_one2n_

lab51303:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch p \{ ... \};
    lea rcx, [rel List_i64_51317]
    add rcx, rdi
    jmp rcx

List_i64_51317:
    jmp near List_i64_51317_Nil
    jmp near List_i64_51317_Cons

List_i64_51317_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_51317_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51319
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51318
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51318:
    mov rdi, [rsi + 40]
    jmp lab51320

lab51319:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab51320:
    ; substitute (a2 !-> a2)(as0 !-> as0)(a0 !-> a0);
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

one2n_:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (n !-> n)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump loop_one2n_
    jmp loop_one2n_

loop_run_:
    ; substitute (work0 !-> work)(work !-> work)(result !-> result)(a0 !-> a0)(iters !-> iters);
    ; #share work
    cmp rsi, 0
    je lab51321
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51321:
    ; #move variables
    mov r13, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a2: List[List[i64]] = (work, result, a0, iters)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov [rbx + 32], r10
    mov [rbx + 24], r9
    mov [rbx + 16], r8
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51333
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51334

lab51333:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51331
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51323

lab51322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51323:

lab51324:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51327
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51325
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51326

lab51325:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51326:

lab51327:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51330
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51328
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51329

lab51328:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51329:

lab51330:
    jmp lab51332

lab51331:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51332:

lab51334:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab51346
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51347

lab51346:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51344
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51336

lab51335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51336:

lab51337:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51340
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51338
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51339

lab51338:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51339:

lab51340:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51343
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51341
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51342

lab51341:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51342:

lab51343:
    jmp lab51345

lab51344:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51345:

lab51347:
    ; #load tag
    lea rdi, [rel List_List_i64_51348]
    ; let x1: Unit = Unit();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(a2 !-> a2)(work0 !-> work0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke work0 Apply
    jmp r9

List_List_i64_51348:
    jmp near List_List_i64_51348_Nil
    jmp near List_List_i64_51348_Cons

List_List_i64_51348_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51352
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab51349
    ; ####increment refcount
    add qword [rax + 0], 1

lab51349:
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab51350
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51350:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51351
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51351:
    jmp lab51353

lab51352:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51353:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(result !-> result)(work !-> work)(x0 !-> x0);
    ; #move variables
    mov r10, rax
    mov rcx, r9
    mov r9, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump lift_loop_run_0_
    jmp lift_loop_run_0_

List_List_i64_51348_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51357
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab51354
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51354:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab51355
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51355:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab51356
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51356:
    jmp lab51358

lab51357:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab51358:
    ; substitute (iters !-> iters)(a0 !-> a0)(work !-> work)(result !-> result)(a4 !-> a4)(as0 !-> as0);
    ; #move variables
    mov r14, rsi
    mov rsi, r12
    mov r12, rax
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a4, as0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov [rbx + 32], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51370
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51371

lab51370:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51368
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51361
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51359
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51360

lab51359:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51360:

lab51361:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51364
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51362
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51363

lab51362:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51363:

lab51364:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51367
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51365
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51366

lab51365:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51366:

lab51367:
    jmp lab51369

lab51368:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51369:

lab51371:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(result !-> result)(work !-> work)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_loop_run_0_
    jmp lift_loop_run_0_

lift_loop_run_0_:
    ; substitute (x0 !-> x0)(result0 !-> result)(result !-> result)(work !-> work)(iters !-> iters)(a0 !-> a0);
    ; #share result
    cmp r8, 0
    je lab51372
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51372:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rdx, r13
    mov r13, rdi
    mov rsi, r8
    mov rdi, r9
    mov rax, r12
    ; new a3: Bool = (result, work, iters, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
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
    je lab51384
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51385

lab51384:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51382
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51375
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51373
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51374

lab51373:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51374:

lab51375:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51378
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51376
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51377

lab51376:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51377:

lab51378:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51381
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51379
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51380

lab51379:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51380:

lab51381:
    jmp lab51383

lab51382:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51383:

lab51385:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab51397
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51398

lab51397:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51395
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51388
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51386
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51387

lab51386:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51387:

lab51388:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51391
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51389
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51390

lab51389:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51390:

lab51391:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51394
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51392
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51393

lab51392:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51393:

lab51394:
    jmp lab51396

lab51395:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51396:

lab51398:
    ; #load tag
    lea r9, [rel Bool_51399]
    ; substitute (x0 !-> x0)(a3 !-> a3)(result0 !-> result0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke result0 Apply
    jmp r9

Bool_51399:
    jmp near Bool_51399_True
    jmp near Bool_51399_False

Bool_51399_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51403
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab51400
    ; ####increment refcount
    add qword [rax + 0], 1

lab51400:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab51401
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51401:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51402
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51402:
    jmp lab51404

lab51403:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51404:
    ; let res: Bool = True();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(res !-> res)(result !-> result)(work !-> work);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, r12
    mov r12, rsi
    mov rcx, r9
    mov r9, r13
    mov r13, rdi
    mov rdi, rcx
    ; jump lift_loop_run_1_
    jmp lift_loop_run_1_

Bool_51399_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51408
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab51405
    ; ####increment refcount
    add qword [rax + 0], 1

lab51405:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab51406
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51406:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51407
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51407:
    jmp lab51409

lab51408:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51409:
    ; let res: Bool = False();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(res !-> res)(result !-> result)(work !-> work);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, r12
    mov r12, rsi
    mov rcx, r9
    mov r9, r13
    mov r13, rdi
    mov rdi, rcx
    ; jump lift_loop_run_1_
    jmp lift_loop_run_1_

lift_loop_run_1_:
    ; if iters == 0 \{ ... \}
    cmp rdi, 0
    je lab51410
    ; substitute (a0 !-> a0)(iters !-> iters)(work !-> work)(result !-> result);
    ; #erase res
    cmp r8, 0
    je lab51413
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab51411
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab51412

lab51411:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab51412:

lab51413:
    ; #move variables
    mov r8, r12
    mov r9, r13
    ; lit x2 <- 1;
    mov r13, 1
    ; x3 <- iters - x2;
    mov r15, rdi
    sub r15, r13
    ; substitute (x3 !-> x3)(work !-> work)(result !-> result)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, r15
    ; jump loop_run_
    jmp loop_run_

lab51410:
    ; substitute (a0 !-> a0)(res !-> res);
    ; #erase result
    cmp r10, 0
    je lab51416
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab51414
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab51415

lab51414:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab51415:

lab51416:
    ; #erase work
    cmp r12, 0
    je lab51419
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab51417
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab51418

lab51417:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab51418:

lab51419:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch res \{ ... \};
    lea rcx, [rel Bool_51420]
    add rcx, rdi
    jmp rcx

Bool_51420:
    jmp near Bool_51420_True
    jmp near Bool_51420_False

Bool_51420_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_51420_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

run_benchmark_:
    ; jump loop_run_
    jmp loop_run_

factorial_:
    ; lit x0 <- 1;
    mov r9, 1
    ; if n == x0 \{ ... \}
    cmp rdx, r9
    je lab51421
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
    je lab51433
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51434

lab51433:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51431
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51424
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51422
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51423

lab51422:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51423:

lab51424:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51427
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51425
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51426

lab51425:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51426:

lab51427:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51430
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51428
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51429

lab51428:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51429:

lab51430:
    jmp lab51432

lab51431:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51432:

lab51434:
    ; #load tag
    lea rdi, [rel _Cont_51435]
    ; lit x2 <- 1;
    mov r9, 1
    ; x3 <- n0 - x2;
    mov r11, rdx
    sub r11, r9
    ; substitute (x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump factorial_
    jmp factorial_

_Cont_51435:

_Cont_51435_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51437
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51436
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51436:
    jmp lab51438

lab51437:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51438:
    ; x5 <- n * x1;
    mov r11, r9
    imul r11, rdx
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab51421:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x4 <- 1;
    mov rdi, 1
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

loop_work_:
    ; if m == 0 \{ ... \}
    cmp rdx, 0
    je lab51439
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- m - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (perms !-> perms)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rdi, r13
    ; new a2: List[List[i64]] = (x1, a0)\{ ... \};
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
    je lab51451
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51452

lab51451:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51449
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51442
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51440
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51441

lab51440:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51441:

lab51442:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51445
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51443
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51444

lab51443:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51444:

lab51445:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51448
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51446
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51447

lab51446:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51447:

lab51448:
    jmp lab51450

lab51449:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51450:

lab51452:
    ; #load tag
    lea rdi, [rel List_List_i64_51453]
    ; new a3: List[i64] = (a2)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51465
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51466

lab51465:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51463
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51456
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51454
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51455

lab51454:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51455:

lab51456:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51459
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51457
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51458

lab51457:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51458:

lab51459:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51462
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51460
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51461

lab51460:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51461:

lab51462:
    jmp lab51464

lab51463:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51464:

lab51466:
    ; #load tag
    lea rdi, [rel List_i64_51467]
    ; jump head_l_
    jmp head_l_

List_i64_51467:
    jmp near List_i64_51467_Nil
    jmp near List_i64_51467_Cons

List_i64_51467_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51469
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab51468
    ; ####increment refcount
    add qword [rax + 0], 1

lab51468:
    jmp lab51470

lab51469:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab51470:
    ; let x3: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x3 !-> x3)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump permutations_
    jmp permutations_

List_i64_51467_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51472
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab51471
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51471:
    jmp lab51473

lab51472:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab51473:
    ; substitute (a2 !-> a2)(a6 !-> a6)(as2 !-> as2);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let x3: List[i64] = Cons(a6, as2);
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
    je lab51485
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51486

lab51485:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51483
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51476
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51474
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51475

lab51474:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51475:

lab51476:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51479
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51477
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51478

lab51477:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51478:

lab51479:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51481

lab51480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51481:

lab51482:
    jmp lab51484

lab51483:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51484:

lab51486:
    ; #load tag
    mov rdi, 5
    ; substitute (x3 !-> x3)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump permutations_
    jmp permutations_

List_List_i64_51453:
    jmp near List_List_i64_51453_Nil
    jmp near List_List_i64_51453_Cons

List_List_i64_51453_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51488
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab51487
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51487:
    mov rdx, [rax + 40]
    jmp lab51489

lab51488:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab51489:
    ; let x2: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump loop_work_
    jmp loop_work_

List_List_i64_51453_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51491
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab51490
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51490:
    mov r9, [r8 + 40]
    jmp lab51492

lab51491:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab51492:
    ; substitute (a0 !-> a0)(x1 !-> x1)(a5 !-> a5)(as1 !-> as1);
    ; #move variables
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x2: List[List[i64]] = Cons(a5, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51504
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51505

lab51504:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51502
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51495
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51493
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51494

lab51493:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51494:

lab51495:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51498
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51496
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51497

lab51496:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51497:

lab51498:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51501
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51499
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51500

lab51499:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51500:

lab51501:
    jmp lab51503

lab51502:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51503:

lab51505:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump loop_work_
    jmp loop_work_

lab51439:
    ; substitute (a0 !-> a0)(perms !-> perms);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch perms \{ ... \};
    lea rcx, [rel List_List_i64_51506]
    add rcx, rdi
    jmp rcx

List_List_i64_51506:
    jmp near List_List_i64_51506_Nil
    jmp near List_List_i64_51506_Cons

List_List_i64_51506_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_51506_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51509
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51507
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51507:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51508
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51508:
    jmp lab51510

lab51509:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51510:
    ; substitute (a4 !-> a4)(as0 !-> as0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

perm9_:
    ; lit x0 <- 1;
    mov r11, 1
    ; substitute (x0 !-> x0)(n0 !-> n)(a0 !-> a0)(n !-> n)(m !-> m);
    ; #move variables
    mov r13, rdx
    mov rdx, r11
    mov r11, rdi
    ; new x1: Fun[Unit, List[List[i64]]] = (n, m)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab51522
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51523

lab51522:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51520
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51512

lab51511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51512:

lab51513:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51515

lab51514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51515:

lab51516:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51518

lab51517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51518:

lab51519:
    jmp lab51521

lab51520:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51521:

lab51523:
    ; #load tag
    lea r11, [rel Fun_Unit_List_List_i64_51524]
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0)(n0 !-> n0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rsi, r10
    ; new x2: Fun[List[List[i64]], Bool] = (n0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51536
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51537

lab51536:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51534
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51527
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51525
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51526

lab51525:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51526:

lab51527:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51530
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51528
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51529

lab51528:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51529:

lab51530:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51533
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51531
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51532

lab51531:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51532:

lab51533:
    jmp lab51535

lab51534:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51535:

lab51537:
    ; #load tag
    lea r11, [rel Fun_List_List_i64_Bool_51538]
    ; substitute (x0 !-> x0)(x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump run_benchmark_
    jmp run_benchmark_

Fun_List_List_i64_Bool_51538:

Fun_List_List_i64_Bool_51538_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51539
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab51540

lab51539:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab51540:
    ; new a5: _Cont = (a4, n0)\{ ... \};
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
    je lab51552
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51553

lab51552:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51550
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51543
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51541
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51542

lab51541:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51542:

lab51543:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51546
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51544
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51545

lab51544:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51545:

lab51546:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51549
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51547
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51548

lab51547:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51548:

lab51549:
    jmp lab51551

lab51550:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51551:

lab51553:
    ; #load tag
    lea rdi, [rel _Cont_51554]
    ; jump sumlists_
    jmp sumlists_

_Cont_51554:

_Cont_51554_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51556
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51555
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51555:
    jmp lab51557

lab51556:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51557:
    ; lit x4 <- 1;
    mov r11, 1
    ; x5 <- n0 + x4;
    mov r13, r9
    add r13, r11
    ; substitute (x3 !-> x3)(a4 !-> a4)(n0 !-> n0)(x5 !-> x5);
    ; #move variables
    mov r11, r13
    ; x6 <- n0 * x5;
    mov r13, r9
    imul r13, r11
    ; substitute (n0 !-> n0)(a4 !-> a4)(x3 !-> x3)(x6 !-> x6);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov r11, r13
    ; new a10: _Cont = (a4, x3, x6)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51569
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51570

lab51569:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51567
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51560
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51558
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51559

lab51558:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51559:

lab51560:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51562

lab51561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51562:

lab51563:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51565

lab51564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51565:

lab51566:
    jmp lab51568

lab51567:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51568:

lab51570:
    ; #load tag
    lea rdi, [rel _Cont_51571]
    ; jump factorial_
    jmp factorial_

_Cont_51571:

_Cont_51571_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51573
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51572
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51572:
    jmp lab51574

lab51573:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51574:
    ; x8 <- x6 * x7;
    mov r13, r11
    imul r13, rdx
    ; substitute (x8 !-> x8)(a4 !-> a4)(x3 !-> x3);
    ; #move variables
    mov rdx, r13
    ; lit x9 <- 2;
    mov r11, 2
    ; x10 <- x8 / x9;
    mov rcx, rdx
    mov r13, rax
    mov rax, rdx
    cqo
    idiv r11
    mov rdx, rax
    mov rax, r13
    mov r13, rdx
    mov rdx, rcx
    ; if x3 == x10 \{ ... \}
    cmp r9, r13
    je lab51575
    ; substitute (a4 !-> a4);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a4 False
    add rdx, 5
    jmp rdx

lab51575:
    ; substitute (a4 !-> a4);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a4 True
    add rdx, 0
    jmp rdx

Fun_Unit_List_List_i64_51524:

Fun_Unit_List_List_i64_51524_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51576
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab51577

lab51576:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab51577:
    ; substitute (n !-> n)(a1 !-> a1)(m !-> m);
    ; #erase u
    cmp rax, 0
    je lab51580
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab51578
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab51579

lab51578:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab51579:

lab51580:
    ; #move variables
    mov rdx, r9
    mov r9, r11
    ; new a2: List[List[i64]] = (a1, m)\{ ... \};
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
    je lab51592
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51593

lab51592:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51590
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51583
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51581
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51582

lab51581:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51582:

lab51583:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51586
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51584
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51585

lab51584:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51585:

lab51586:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51589
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51587
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51588

lab51587:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51588:

lab51589:
    jmp lab51591

lab51590:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51591:

lab51593:
    ; #load tag
    lea rdi, [rel List_List_i64_51594]
    ; new a3: List[i64] = (a2)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51606
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51607

lab51606:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51604
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51597
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51595
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51596

lab51595:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51596:

lab51597:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51600
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51598
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51599

lab51598:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51599:

lab51600:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51603
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51601
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51602

lab51601:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51602:

lab51603:
    jmp lab51605

lab51604:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51605:

lab51607:
    ; #load tag
    lea rdi, [rel List_i64_51608]
    ; jump one2n_
    jmp one2n_

List_i64_51608:
    jmp near List_i64_51608_Nil
    jmp near List_i64_51608_Cons

List_i64_51608_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51610
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab51609
    ; ####increment refcount
    add qword [rax + 0], 1

lab51609:
    jmp lab51611

lab51610:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab51611:
    ; let x12: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x12 !-> x12)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump permutations_
    jmp permutations_

List_i64_51608_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51613
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab51612
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51612:
    jmp lab51614

lab51613:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab51614:
    ; substitute (a2 !-> a2)(a12 !-> a12)(as1 !-> as1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let x12: List[i64] = Cons(a12, as1);
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
    je lab51626
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51627

lab51626:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51624
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51616

lab51615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51616:

lab51617:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51619

lab51618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51619:

lab51620:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51622

lab51621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51622:

lab51623:
    jmp lab51625

lab51624:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51625:

lab51627:
    ; #load tag
    mov rdi, 5
    ; substitute (x12 !-> x12)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump permutations_
    jmp permutations_

List_List_i64_51594:
    jmp near List_List_i64_51594_Nil
    jmp near List_List_i64_51594_Cons

List_List_i64_51594_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51629
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab51628
    ; ####increment refcount
    add qword [rax + 0], 1

lab51628:
    jmp lab51630

lab51629:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab51630:
    ; let x11: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (m !-> m)(x11 !-> x11)(a1 !-> a1);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump loop_work_
    jmp loop_work_

List_List_i64_51594_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51632
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab51631
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51631:
    jmp lab51633

lab51632:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab51633:
    ; substitute (m !-> m)(a1 !-> a1)(a11 !-> a11)(as0 !-> as0);
    ; #move variables
    mov r10, rsi
    mov rsi, r8
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x11: List[List[i64]] = Cons(a11, as0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab51645
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51646

lab51645:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51643
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51635

lab51634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51635:

lab51636:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51639
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51637
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51638

lab51637:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51638:

lab51639:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51642
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51640
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51641

lab51640:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51641:

lab51642:
    jmp lab51644

lab51643:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51644:

lab51646:
    ; #load tag
    mov r9, 5
    ; substitute (m !-> m)(x11 !-> x11)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump loop_work_
    jmp loop_work_

main_loop_:
    ; substitute (n0 !-> n)(m0 !-> m)(n !-> n)(a0 !-> a0)(iters !-> iters)(m !-> m);
    ; #move variables
    mov r13, rdx
    mov r15, rdi
    mov rdx, r9
    ; new a2: Bool = (n, a0, iters, m)\{ ... \};
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
    je lab51658
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51659

lab51658:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51656
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51648

lab51647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51648:

lab51649:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51652
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51650
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51651

lab51650:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51651:

lab51652:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51655
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51653
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51654

lab51653:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51654:

lab51655:
    jmp lab51657

lab51656:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51657:

lab51659:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab51671
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51672

lab51671:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51669
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51661

lab51660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51661:

lab51662:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51665
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51663
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51664

lab51663:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51664:

lab51665:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51668
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51666
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51667

lab51666:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51667:

lab51668:
    jmp lab51670

lab51669:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51670:

lab51672:
    ; #load tag
    lea r9, [rel Bool_51673]
    ; substitute (m0 !-> m0)(n0 !-> n0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump perm9_
    jmp perm9_

Bool_51673:
    jmp near Bool_51673_True
    jmp near Bool_51673_False

Bool_51673_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51675
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51674
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51674:
    jmp lab51676

lab51675:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51676:
    ; let res: Bool = True();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(m !-> m)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Bool_51673_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51678
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51677
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51677:
    jmp lab51679

lab51678:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51679:
    ; let res: Bool = False();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(m !-> m)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; lit x0 <- 1;
    mov r15, 1
    ; if iters == x0 \{ ... \}
    cmp rdi, r15
    je lab51680
    ; substitute (a0 !-> a0)(iters !-> iters)(m !-> m)(n !-> n);
    ; #erase res
    cmp r12, 0
    je lab51683
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab51681
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab51682

lab51681:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab51682:

lab51683:
    ; lit x3 <- 1;
    mov r13, 1
    ; x4 <- iters - x3;
    mov r15, rdi
    sub r15, r13
    ; substitute (x4 !-> x4)(m !-> m)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, r15
    ; jump main_loop_
    jmp main_loop_

lab51680:
    ; substitute (a0 !-> a0)(res !-> res);
    ; #move variables
    mov rsi, r12
    mov rdi, r13
    ; switch res \{ ... \};
    lea rcx, [rel Bool_51684]
    add rcx, rdi
    jmp rcx

Bool_51684:
    jmp near Bool_51684_True
    jmp near Bool_51684_False

Bool_51684_True:
    ; lit x1 <- 1;
    mov rdi, 1
    ; println_i64 x1;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rdi, r14
    add rsp, 8
    ; substitute (a0 !-> a0);
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

Bool_51684_False:
    ; lit x2 <- 0;
    mov rdi, 0
    ; println_i64 x2;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call println_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rdi, r14
    add rsp, 8
    ; substitute (a0 !-> a0);
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