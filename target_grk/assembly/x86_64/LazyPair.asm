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
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    lea rdx, [rel _Cont_738]
    ; create x2: LazyPair[i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel LazyPair_i64_i64_739]
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump pairSum_
    jmp pairSum_

LazyPair_i64_i64_739:
    jmp near LazyPair_i64_i64_739_Fst
    jmp near LazyPair_i64_i64_739_Snd

LazyPair_i64_i64_739_Fst:
    ; lit x3 <- 1;
    mov rdi, 1
    ; substitute (x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Ret
    jmp rdi

LazyPair_i64_i64_739_Snd:
    ; lit x4 <- 2;
    mov rdi, 2
    ; substitute (x4 !-> x4)(a2 !-> a2);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a2 Ret
    jmp rdi

_Cont_738:

_Cont_738_Ret:
    ; println_i64 x1;
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

swapLazy_:
    ; switch a0 \{ ... \};
    lea rcx, [rel LazyPair_i64_i64_740]
    add rcx, rdi
    jmp rcx

LazyPair_i64_i64_740:
    jmp near LazyPair_i64_i64_740_Fst
    jmp near LazyPair_i64_i64_740_Snd

LazyPair_i64_i64_740_Fst:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab742
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab741
    ; ####increment refcount
    add qword [rsi + 0], 1

lab741:
    jmp lab743

lab742:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab743:
    ; substitute (a1 !-> a1)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Snd
    add rdi, 5
    jmp rdi

LazyPair_i64_i64_740_Snd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab745
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab744
    ; ####increment refcount
    add qword [rsi + 0], 1

lab744:
    jmp lab746

lab745:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab746:
    ; substitute (a2 !-> a2)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Fst
    add rdi, 0
    jmp rdi

toTuple_:
    ; substitute (x2 !-> x)(a0 !-> a0)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab747
    ; ####increment refcount
    add qword [rax + 0], 1

lab747:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; create a1: _Cont = (a0, x)\{ ... \};
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
    je lab759
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab760

lab759:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab757
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab750
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab748
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab749

lab748:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab749:

lab750:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab753
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab751
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab752

lab751:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab752:

lab753:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab756
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab754
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab755

lab754:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab755:

lab756:
    jmp lab758

lab757:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab758:

lab760:
    ; #load tag
    lea rdi, [rel _Cont_761]
    ; substitute (a1 !-> a1)(x2 !-> x2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x2 Fst
    add rdi, 0
    jmp rdi

_Cont_761:

_Cont_761_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab764
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab762
    ; ####increment refcount
    add qword [r8 + 0], 1

lab762:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab763
    ; ####increment refcount
    add qword [rsi + 0], 1

lab763:
    jmp lab765

lab764:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab765:
    ; substitute (x !-> x)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; create a2: _Cont = (a0, x0)\{ ... \};
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
    je lab777
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab778

lab777:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab775
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab768
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab766
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab767

lab766:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab767:

lab768:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab771
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab769
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab770

lab769:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab770:

lab771:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab774
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab772
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab773

lab772:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab773:

lab774:
    jmp lab776

lab775:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab776:

lab778:
    ; #load tag
    lea rdi, [rel _Cont_779]
    ; substitute (a2 !-> a2)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Snd
    add rdi, 5
    jmp rdi

_Cont_779:

_Cont_779_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab781
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab780
    ; ####increment refcount
    add qword [rsi + 0], 1

lab780:
    jmp lab782

lab781:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab782:
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; invoke a0 Tup
    jmp r9

fromTuple_:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_783:

Pair_i64_i64_783_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab784
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab785

lab784:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab785:
    ; substitute (b !-> b)(a !-> a)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch a0 \{ ... \};
    lea rcx, [rel LazyPair_i64_i64_786]
    add rcx, r9
    jmp rcx

LazyPair_i64_i64_786:
    jmp near LazyPair_i64_i64_786_Fst
    jmp near LazyPair_i64_i64_786_Snd

LazyPair_i64_i64_786_Fst:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab788
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab787
    ; ####increment refcount
    add qword [r8 + 0], 1

lab787:
    jmp lab789

lab788:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab789:
    ; substitute (a !-> a)(a1 !-> a1);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a1 Ret
    jmp rdi

LazyPair_i64_i64_786_Snd:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab791
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab790
    ; ####increment refcount
    add qword [r8 + 0], 1

lab790:
    jmp lab792

lab791:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab792:
    ; substitute (b !-> b)(a2 !-> a2);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; invoke a2 Ret
    jmp rdi

pairSum_:
    ; substitute (x3 !-> x)(a0 !-> a0)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab793
    ; ####increment refcount
    add qword [rax + 0], 1

lab793:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; create a1: _Cont = (a0, x)\{ ... \};
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
    je lab805
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab806

lab805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab795

lab794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab795:

lab796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab798

lab797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab798:

lab799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab801

lab800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab801:

lab802:
    jmp lab804

lab803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab804:

lab806:
    ; #load tag
    lea rdi, [rel _Cont_807]
    ; substitute (a1 !-> a1)(x3 !-> x3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x3 Fst
    add rdi, 0
    jmp rdi

_Cont_807:

_Cont_807_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab810
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab808
    ; ####increment refcount
    add qword [r8 + 0], 1

lab808:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab809
    ; ####increment refcount
    add qword [rsi + 0], 1

lab809:
    jmp lab811

lab810:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab811:
    ; substitute (x !-> x)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; create a2: _Cont = (a0, x0)\{ ... \};
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
    je lab823
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab824

lab823:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab821
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab814
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab812
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab813

lab812:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab813:

lab814:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab817
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab815
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab816

lab815:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab816:

lab817:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab820
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab818
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab819

lab818:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab819:

lab820:
    jmp lab822

lab821:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab822:

lab824:
    ; #load tag
    lea rdi, [rel _Cont_825]
    ; substitute (a2 !-> a2)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Snd
    add rdi, 5
    jmp rdi

_Cont_825:

_Cont_825_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab827
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab826
    ; ####increment refcount
    add qword [rsi + 0], 1

lab826:
    jmp lab828

lab827:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab828:
    ; x2 <- x0 + x1;
    mov r11, r9
    add r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
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