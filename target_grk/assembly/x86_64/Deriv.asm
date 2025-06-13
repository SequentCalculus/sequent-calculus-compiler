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
    lea r11, [rel _Cont_51791]
    ; jump main_loop_
    jmp main_loop_

_Cont_51791:

_Cont_51791_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

rev_list_acc_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Expr_51792]
    add rcx, r9
    jmp rcx

List_Expr_51792:
    jmp near List_Expr_51792_Nil
    jmp near List_Expr_51792_Cons

List_Expr_51792_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_Expr_51793]
    add rcx, rdi
    jmp rcx

List_Expr_51793:
    jmp near List_Expr_51793_Nil
    jmp near List_Expr_51793_Cons

List_Expr_51793_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Expr_51793_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51796
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab51794
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51794:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51795
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51795:
    jmp lab51797

lab51796:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51797:
    ; substitute (x1 !-> x1)(xs0 !-> xs0)(a0 !-> a0);
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

List_Expr_51792_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51800
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab51798
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51798:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab51799
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51799:
    jmp lab51801

lab51800:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab51801:
    ; substitute (a0 !-> a0)(xs !-> xs)(x !-> x)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[Expr] = Cons(x, acc);
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
    je lab51813
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51814

lab51813:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51811
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51804
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51802
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51803

lab51802:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51803:

lab51804:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51807
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51805
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51806

lab51805:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51806:

lab51807:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51810
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51808
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51809

lab51808:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51809:

lab51810:
    jmp lab51812

lab51811:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51812:

lab51814:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_list_acc_
    jmp rev_list_acc_

rev_list_:
    ; let x0: List[Expr] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump rev_list_acc_
    jmp rev_list_acc_

map_list_acc_:
    ; substitute (f !-> f)(a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Expr_51815]
    add rcx, r11
    jmp rcx

List_Expr_51815:
    jmp near List_Expr_51815_Nil
    jmp near List_Expr_51815_Cons

List_Expr_51815_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab51818
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab51816
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab51817

lab51816:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab51817:

lab51818:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; jump rev_list_
    jmp rev_list_

List_Expr_51815_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab51821
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab51819
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51819:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab51820
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51820:
    jmp lab51822

lab51821:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab51822:
    ; substitute (f0 !-> f)(x !-> x)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab51823
    ; ####increment refcount
    add qword [rax + 0], 1

lab51823:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a1: Expr = (acc, a0, xs, f)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov [rbx + 32], r12
    mov [rbx + 24], r11
    mov [rbx + 16], r10
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51835
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51836

lab51835:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51833
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51826
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51824
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51825

lab51824:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51825:

lab51826:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51829
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51827
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51828

lab51827:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51828:

lab51829:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51831

lab51830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51831:

lab51832:
    jmp lab51834

lab51833:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51834:

lab51836:
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
    je lab51848
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51849

lab51848:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51846
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51839
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51837
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51838

lab51837:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51838:

lab51839:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51842
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51840
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51841

lab51840:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51841:

lab51842:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51844

lab51843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51844:

lab51845:
    jmp lab51847

lab51846:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51847:

lab51849:
    ; #load tag
    lea r9, [rel Expr_51850]
    ; substitute (x !-> x)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Apply
    jmp r9

Expr_51850:
    jmp near Expr_51850_Add
    jmp near Expr_51850_Sub
    jmp near Expr_51850_Mul
    jmp near Expr_51850_Div
    jmp near Expr_51850_Num
    jmp near Expr_51850_X

Expr_51850_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51855
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51851
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51851:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab51852
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51852:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51853
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51853:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51854
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51854:
    jmp lab51856

lab51855:
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
    mov r8, [r8 + 16]

lab51856:
    ; substitute (f !-> f)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(sums0 !-> sums0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: Expr = Add(sums0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51868
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51869

lab51868:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51866
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51859
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51857
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51858

lab51857:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51858:

lab51859:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51862
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51860
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51861

lab51860:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51861:

lab51862:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51864

lab51863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51864:

lab51865:
    jmp lab51867

lab51866:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51867:

lab51869:
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

Expr_51850_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51874
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51870
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51870:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab51871
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51871:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51872
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51872:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51873
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51873:
    jmp lab51875

lab51874:
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
    mov r8, [r8 + 16]

lab51875:
    ; substitute (f !-> f)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(subs0 !-> subs0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: Expr = Sub(subs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51887
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51888

lab51887:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51885
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51878
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51876
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51877

lab51876:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51877:

lab51878:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51880

lab51879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51880:

lab51881:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51883

lab51882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51883:

lab51884:
    jmp lab51886

lab51885:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51886:

lab51888:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

Expr_51850_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51893
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51889
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51889:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab51890
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51890:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51891
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51891:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51892
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51892:
    jmp lab51894

lab51893:
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
    mov r8, [r8 + 16]

lab51894:
    ; substitute (f !-> f)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(muls0 !-> muls0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: Expr = Mul(muls0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51906
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51907

lab51906:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51904
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51896

lab51895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51896:

lab51897:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51899

lab51898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51899:

lab51900:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51902

lab51901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51902:

lab51903:
    jmp lab51905

lab51904:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51905:

lab51907:
    ; #load tag
    mov r13, 10
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

Expr_51850_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51912
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51908
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51908:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab51909
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51909:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51910
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51910:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51911
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51911:
    jmp lab51913

lab51912:
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
    mov r8, [r8 + 16]

lab51913:
    ; substitute (f !-> f)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(divs0 !-> divs0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: Expr = Div(divs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51925
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51926

lab51925:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51923
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51915

lab51914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51915:

lab51916:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51918

lab51917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51918:

lab51919:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51921

lab51920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51921:

lab51922:
    jmp lab51924

lab51923:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51924:

lab51926:
    ; #load tag
    mov r13, 15
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

Expr_51850_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51931
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51927
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51927:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab51928
    ; ####increment refcount
    add qword [r12 + 0], 1

lab51928:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab51929
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51929:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab51930
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51930:
    jmp lab51932

lab51931:
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
    mov r8, [r8 + 16]

lab51932:
    ; substitute (f !-> f)(acc !-> acc)(a0 !-> a0)(xs !-> xs)(i0 !-> i0);
    ; #move variables
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r12
    ; let x1: Expr = Num(i0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51944
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab51945

lab51944:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51942
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51935
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51933
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51934

lab51933:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51934:

lab51935:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51938
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51936
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51937

lab51936:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51937:

lab51938:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51941
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51939
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51940

lab51939:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51940:

lab51941:
    jmp lab51943

lab51942:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51943:

lab51945:
    ; #load tag
    mov r13, 20
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

Expr_51850_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51950
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab51946
    ; ####increment refcount
    add qword [rax + 0], 1

lab51946:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab51947
    ; ####increment refcount
    add qword [r10 + 0], 1

lab51947:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab51948
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51948:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51949
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51949:
    jmp lab51951

lab51950:
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
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51951:
    ; let x1: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_map_list_acc_0_
    jmp lift_map_list_acc_0_

lift_map_list_acc_0_:
    ; substitute (a0 !-> a0)(xs !-> xs)(f !-> f)(x1 !-> x1)(acc !-> acc);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x0: List[Expr] = Cons(x1, acc);
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
    je lab51963
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51964

lab51963:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51961
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51954
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51952
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51953

lab51952:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51953:

lab51954:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51957
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51955
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51956

lab51955:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51956:

lab51957:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51960
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51958
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51959

lab51958:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51959:

lab51960:
    jmp lab51962

lab51961:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51962:

lab51964:
    ; #load tag
    mov r11, 5
    ; substitute (f !-> f)(xs !-> xs)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump map_list_acc_
    jmp map_list_acc_

map_list_:
    ; let x0: List[Expr] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (f !-> f)(l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump map_list_acc_
    jmp map_list_acc_

map_expr_:
    ; substitute (f !-> f)(a0 !-> a0)(e !-> e);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch e \{ ... \};
    lea rcx, [rel Expr_51965]
    add rcx, r9
    jmp rcx

Expr_51965:
    jmp near Expr_51965_Add
    jmp near Expr_51965_Sub
    jmp near Expr_51965_Mul
    jmp near Expr_51965_Div
    jmp near Expr_51965_Num
    jmp near Expr_51965_X

Expr_51965_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51967
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab51966
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51966:
    jmp lab51968

lab51967:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab51968:
    ; substitute (f !-> f)(sums !-> sums)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: List[Expr] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab51980
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab51981

lab51980:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51978
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51970

lab51969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51970:

lab51971:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51974
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51972
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51973

lab51972:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51973:

lab51974:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51977
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51975
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51976

lab51975:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51976:

lab51977:
    jmp lab51979

lab51978:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51979:

lab51981:
    ; #load tag
    lea r9, [rel List_Expr_51982]
    ; jump map_list_
    jmp map_list_

List_Expr_51982:
    jmp near List_Expr_51982_Nil
    jmp near List_Expr_51982_Cons

List_Expr_51982_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab51984
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab51983
    ; ####increment refcount
    add qword [rax + 0], 1

lab51983:
    jmp lab51985

lab51984:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab51985:
    ; let x0: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

List_Expr_51982_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab51987
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab51986
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51986:
    jmp lab51988

lab51987:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab51988:
    ; substitute (a0 !-> a0)(x6 !-> x6)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Expr] = Cons(x6, xs0);
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
    je lab52000
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52001

lab52000:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51998
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51991
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51989
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51990

lab51989:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51990:

lab51991:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51993

lab51992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51993:

lab51994:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51997
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51995
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51996

lab51995:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51996:

lab51997:
    jmp lab51999

lab51998:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51999:

lab52001:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

Expr_51965_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52003
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52002
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52002:
    jmp lab52004

lab52003:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52004:
    ; substitute (f !-> f)(subs !-> subs)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a2: List[Expr] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52016
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52017

lab52016:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52014
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52007
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52005
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52006

lab52005:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52006:

lab52007:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52010
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52008
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52009

lab52008:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52009:

lab52010:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52012

lab52011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52012:

lab52013:
    jmp lab52015

lab52014:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52015:

lab52017:
    ; #load tag
    lea r9, [rel List_Expr_52018]
    ; jump map_list_
    jmp map_list_

List_Expr_52018:
    jmp near List_Expr_52018_Nil
    jmp near List_Expr_52018_Cons

List_Expr_52018_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52020
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab52019
    ; ####increment refcount
    add qword [rax + 0], 1

lab52019:
    jmp lab52021

lab52020:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab52021:
    ; let x1: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Sub
    add rdi, 5
    jmp rdi

List_Expr_52018_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52023
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52022
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52022:
    jmp lab52024

lab52023:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52024:
    ; substitute (a0 !-> a0)(x7 !-> x7)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x1: List[Expr] = Cons(x7, xs1);
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
    je lab52036
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52037

lab52036:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52034
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52027
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52025
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52026

lab52025:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52026:

lab52027:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52030
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52028
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52029

lab52028:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52029:

lab52030:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52033
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52031
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52032

lab52031:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52032:

lab52033:
    jmp lab52035

lab52034:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52035:

lab52037:
    ; #load tag
    mov rdi, 5
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Sub
    add rdi, 5
    jmp rdi

Expr_51965_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52039
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52038
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52038:
    jmp lab52040

lab52039:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52040:
    ; substitute (f !-> f)(muls !-> muls)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a3: List[Expr] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52052
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52053

lab52052:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52050
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52042

lab52041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52042:

lab52043:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52046
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52044
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52045

lab52044:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52045:

lab52046:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52049
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52047
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52048

lab52047:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52048:

lab52049:
    jmp lab52051

lab52050:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52051:

lab52053:
    ; #load tag
    lea r9, [rel List_Expr_52054]
    ; jump map_list_
    jmp map_list_

List_Expr_52054:
    jmp near List_Expr_52054_Nil
    jmp near List_Expr_52054_Cons

List_Expr_52054_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52056
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab52055
    ; ####increment refcount
    add qword [rax + 0], 1

lab52055:
    jmp lab52057

lab52056:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab52057:
    ; let x2: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Mul
    add rdi, 10
    jmp rdi

List_Expr_52054_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52059
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52058
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52058:
    jmp lab52060

lab52059:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52060:
    ; substitute (a0 !-> a0)(x8 !-> x8)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x2: List[Expr] = Cons(x8, xs2);
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
    je lab52072
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52073

lab52072:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52070
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52063
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52061
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52062

lab52061:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52062:

lab52063:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52066
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52064
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52065

lab52064:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52065:

lab52066:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52069
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52067
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52068

lab52067:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52068:

lab52069:
    jmp lab52071

lab52070:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52071:

lab52073:
    ; #load tag
    mov rdi, 5
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Mul
    add rdi, 10
    jmp rdi

Expr_51965_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52075
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52074
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52074:
    jmp lab52076

lab52075:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52076:
    ; substitute (f !-> f)(divs !-> divs)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a4: List[Expr] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52088
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52089

lab52088:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52086
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52079
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52077
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52078

lab52077:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52078:

lab52079:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52082
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52080
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52081

lab52080:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52081:

lab52082:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52085
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52083
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52084

lab52083:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52084:

lab52085:
    jmp lab52087

lab52086:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52087:

lab52089:
    ; #load tag
    lea r9, [rel List_Expr_52090]
    ; jump map_list_
    jmp map_list_

List_Expr_52090:
    jmp near List_Expr_52090_Nil
    jmp near List_Expr_52090_Cons

List_Expr_52090_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52092
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab52091
    ; ####increment refcount
    add qword [rax + 0], 1

lab52091:
    jmp lab52093

lab52092:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab52093:
    ; let x3: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Div
    add rdi, 15
    jmp rdi

List_Expr_52090_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52095
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52094
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52094:
    jmp lab52096

lab52095:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52096:
    ; substitute (a0 !-> a0)(x9 !-> x9)(xs3 !-> xs3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x3: List[Expr] = Cons(x9, xs3);
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
    je lab52108
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52109

lab52108:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52106
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52099
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52097
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52098

lab52097:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52098:

lab52099:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52102
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52100
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52101

lab52100:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52101:

lab52102:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52105
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52103
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52104

lab52103:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52104:

lab52105:
    jmp lab52107

lab52106:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52107:

lab52109:
    ; #load tag
    mov rdi, 5
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Div
    add rdi, 15
    jmp rdi

Expr_51965_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52110
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52111

lab52110:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52111:
    ; let x4: Expr = Num(i);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52123
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52124

lab52123:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52121
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52114
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52112
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52113

lab52112:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52113:

lab52114:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52117
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52115
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52116

lab52115:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52116:

lab52117:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52120
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52118
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52119

lab52118:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52119:

lab52120:
    jmp lab52122

lab52121:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52122:

lab52124:
    ; #load tag
    mov r9, 20
    ; substitute (x4 !-> x4)(a0 !-> a0)(f !-> f);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f Apply
    jmp r9

Expr_51965_X:
    ; let x5: Expr = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 25
    ; substitute (x5 !-> x5)(a0 !-> a0)(f !-> f);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f Apply
    jmp r9

and_:
    ; substitute (a0 !-> a0)(b2 !-> b2)(b1 !-> b1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch b1 \{ ... \};
    lea rcx, [rel Bool_52125]
    add rcx, r9
    jmp rcx

Bool_52125:
    jmp near Bool_52125_True
    jmp near Bool_52125_False

Bool_52125_True:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_52126]
    add rcx, rdi
    jmp rcx

Bool_52126:
    jmp near Bool_52126_True
    jmp near Bool_52126_False

Bool_52126_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_52126_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_52125_False:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab52129
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52127
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52128

lab52127:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52128:

lab52129:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

equal_list_:
    ; substitute (a0 !-> a0)(l2 !-> l2)(l1 !-> l1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_Expr_52130]
    add rcx, r9
    jmp rcx

List_Expr_52130:
    jmp near List_Expr_52130_Nil
    jmp near List_Expr_52130_Cons

List_Expr_52130_Nil:
    ; switch l2 \{ ... \};
    lea rcx, [rel List_Expr_52131]
    add rcx, rdi
    jmp rcx

List_Expr_52131:
    jmp near List_Expr_52131_Nil
    jmp near List_Expr_52131_Cons

List_Expr_52131_Nil:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_Expr_52131_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52134
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52132
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52132:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52133
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52133:
    jmp lab52135

lab52134:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52135:
    ; substitute (a0 !-> a0);
    ; #erase e
    cmp rsi, 0
    je lab52138
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52136
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52137

lab52136:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52137:

lab52138:
    ; #erase es
    cmp r8, 0
    je lab52141
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52139
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52140

lab52139:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52140:

lab52141:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_Expr_52130_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52144
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab52142
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52142:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab52143
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52143:
    jmp lab52145

lab52144:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab52145:
    ; substitute (a0 !-> a0)(es1 !-> es1)(e1 !-> e1)(l2 !-> l2);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch l2 \{ ... \};
    lea rcx, [rel List_Expr_52146]
    add rcx, r11
    jmp rcx

List_Expr_52146:
    jmp near List_Expr_52146_Nil
    jmp near List_Expr_52146_Cons

List_Expr_52146_Nil:
    ; substitute (a0 !-> a0);
    ; #erase e1
    cmp r8, 0
    je lab52149
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52147
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52148

lab52147:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52148:

lab52149:
    ; #erase es1
    cmp rsi, 0
    je lab52152
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52150
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52151

lab52150:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52151:

lab52152:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_Expr_52146_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab52155
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab52153
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52153:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab52154
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52154:
    jmp lab52156

lab52155:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab52156:
    ; substitute (e2 !-> e2)(e1 !-> e1)(es1 !-> es1)(a0 !-> a0)(es2 !-> es2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: Bool = (es1, a0, es2)\{ ... \};
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
    je lab52168
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52169

lab52168:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52166
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52159
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52157
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52158

lab52157:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52158:

lab52159:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52162
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52160
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52161

lab52160:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52161:

lab52162:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52165
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52163
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52164

lab52163:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52164:

lab52165:
    jmp lab52167

lab52166:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52167:

lab52169:
    ; #load tag
    lea r9, [rel Bool_52170]
    ; substitute (e1 !-> e1)(e2 !-> e2)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump equal_
    jmp equal_

Bool_52170:
    jmp near Bool_52170_True
    jmp near Bool_52170_False

Bool_52170_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52174
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab52171
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52171:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab52172
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52172:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab52173
    ; ####increment refcount
    add qword [rax + 0], 1

lab52173:
    jmp lab52175

lab52174:
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
    mov rax, [rax + 16]

lab52175:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(es1 !-> es1)(es2 !-> es2)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_equal_list_0_
    jmp lift_equal_list_0_

Bool_52170_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52179
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab52176
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52176:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab52177
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52177:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab52178
    ; ####increment refcount
    add qword [rax + 0], 1

lab52178:
    jmp lab52180

lab52179:
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
    mov rax, [rax + 16]

lab52180:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(es1 !-> es1)(es2 !-> es2)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_equal_list_0_
    jmp lift_equal_list_0_

lift_equal_list_0_:
    ; substitute (es2 !-> es2)(es1 !-> es1)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0, x0)\{ ... \};
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
    je lab52192
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52193

lab52192:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52190
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52183
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52181
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52182

lab52181:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52182:

lab52183:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52185

lab52184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52185:

lab52186:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52189
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52187
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52188

lab52187:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52188:

lab52189:
    jmp lab52191

lab52190:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52191:

lab52193:
    ; #load tag
    lea r9, [rel Bool_52194]
    ; substitute (es1 !-> es1)(es2 !-> es2)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump equal_list_
    jmp equal_list_

Bool_52194:
    jmp near Bool_52194_True
    jmp near Bool_52194_False

Bool_52194_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52197
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab52195
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52195:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab52196
    ; ####increment refcount
    add qword [rax + 0], 1

lab52196:
    jmp lab52198

lab52197:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab52198:
    ; let x1: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

Bool_52194_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52201
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab52199
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52199:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab52200
    ; ####increment refcount
    add qword [rax + 0], 1

lab52200:
    jmp lab52202

lab52201:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab52202:
    ; let x1: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

equal_:
    ; substitute (a0 !-> a0)(exp2 !-> exp2)(exp1 !-> exp1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch exp1 \{ ... \};
    lea rcx, [rel Expr_52203]
    add rcx, r9
    jmp rcx

Expr_52203:
    jmp near Expr_52203_Add
    jmp near Expr_52203_Sub
    jmp near Expr_52203_Mul
    jmp near Expr_52203_Div
    jmp near Expr_52203_Num
    jmp near Expr_52203_X

Expr_52203_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52205
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52204
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52204:
    jmp lab52206

lab52205:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52206:
    ; substitute (a0 !-> a0)(sums1 !-> sums1)(exp2 !-> exp2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52207]
    add rcx, r9
    jmp rcx

Expr_52207:
    jmp near Expr_52207_Add
    jmp near Expr_52207_Sub
    jmp near Expr_52207_Mul
    jmp near Expr_52207_Div
    jmp near Expr_52207_Num
    jmp near Expr_52207_X

Expr_52207_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52209
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52208
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52208:
    jmp lab52210

lab52209:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52210:
    ; substitute (sums1 !-> sums1)(sums2 !-> sums2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump equal_list_
    jmp equal_list_

Expr_52207_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52212
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52211
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52211:
    jmp lab52213

lab52212:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52213:
    ; substitute (a0 !-> a0);
    ; #erase subs
    cmp r8, 0
    je lab52216
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52214
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52215

lab52214:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52215:

lab52216:
    ; #erase sums1
    cmp rsi, 0
    je lab52219
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52217
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52218

lab52217:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52218:

lab52219:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52207_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52221
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52220
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52220:
    jmp lab52222

lab52221:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52222:
    ; substitute (a0 !-> a0);
    ; #erase muls
    cmp r8, 0
    je lab52225
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52223
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52224

lab52223:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52224:

lab52225:
    ; #erase sums1
    cmp rsi, 0
    je lab52228
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52226
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52227

lab52226:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52227:

lab52228:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52207_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52230
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52229
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52229:
    jmp lab52231

lab52230:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52231:
    ; substitute (a0 !-> a0);
    ; #erase divs
    cmp r8, 0
    je lab52234
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52232
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52233

lab52232:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52233:

lab52234:
    ; #erase sums1
    cmp rsi, 0
    je lab52237
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52235
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52236

lab52235:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52236:

lab52237:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52207_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52238
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52239

lab52238:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52239:
    ; substitute (a0 !-> a0);
    ; #erase sums1
    cmp rsi, 0
    je lab52242
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52240
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52241

lab52240:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52241:

lab52242:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52207_X:
    ; substitute (a0 !-> a0);
    ; #erase sums1
    cmp rsi, 0
    je lab52245
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52243
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52244

lab52243:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52244:

lab52245:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52203_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52247
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52246
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52246:
    jmp lab52248

lab52247:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52248:
    ; substitute (a0 !-> a0)(subs1 !-> subs1)(exp2 !-> exp2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52249]
    add rcx, r9
    jmp rcx

Expr_52249:
    jmp near Expr_52249_Add
    jmp near Expr_52249_Sub
    jmp near Expr_52249_Mul
    jmp near Expr_52249_Div
    jmp near Expr_52249_Num
    jmp near Expr_52249_X

Expr_52249_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52251
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52250
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52250:
    jmp lab52252

lab52251:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52252:
    ; substitute (a0 !-> a0);
    ; #erase subs1
    cmp rsi, 0
    je lab52255
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52253
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52254

lab52253:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52254:

lab52255:
    ; #erase sums
    cmp r8, 0
    je lab52258
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52256
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52257

lab52256:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52257:

lab52258:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52249_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52260
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52259
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52259:
    jmp lab52261

lab52260:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52261:
    ; substitute (subs1 !-> subs1)(subs2 !-> subs2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump equal_list_
    jmp equal_list_

Expr_52249_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52263
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52262
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52262:
    jmp lab52264

lab52263:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52264:
    ; substitute (a0 !-> a0);
    ; #erase muls
    cmp r8, 0
    je lab52267
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52265
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52266

lab52265:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52266:

lab52267:
    ; #erase subs1
    cmp rsi, 0
    je lab52270
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52268
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52269

lab52268:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52269:

lab52270:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52249_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52272
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52271
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52271:
    jmp lab52273

lab52272:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52273:
    ; substitute (a0 !-> a0);
    ; #erase divs
    cmp r8, 0
    je lab52276
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52274
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52275

lab52274:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52275:

lab52276:
    ; #erase subs1
    cmp rsi, 0
    je lab52279
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52277
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52278

lab52277:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52278:

lab52279:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52249_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52280
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52281

lab52280:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52281:
    ; substitute (a0 !-> a0);
    ; #erase subs1
    cmp rsi, 0
    je lab52284
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52282
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52283

lab52282:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52283:

lab52284:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52249_X:
    ; substitute (a0 !-> a0);
    ; #erase subs1
    cmp rsi, 0
    je lab52287
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52285
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52286

lab52285:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52286:

lab52287:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52203_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52289
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52288
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52288:
    jmp lab52290

lab52289:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52290:
    ; substitute (a0 !-> a0)(muls1 !-> muls1)(exp2 !-> exp2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52291]
    add rcx, r9
    jmp rcx

Expr_52291:
    jmp near Expr_52291_Add
    jmp near Expr_52291_Sub
    jmp near Expr_52291_Mul
    jmp near Expr_52291_Div
    jmp near Expr_52291_Num
    jmp near Expr_52291_X

Expr_52291_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52293
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52292
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52292:
    jmp lab52294

lab52293:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52294:
    ; substitute (a0 !-> a0);
    ; #erase muls1
    cmp rsi, 0
    je lab52297
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52295
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52296

lab52295:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52296:

lab52297:
    ; #erase sums
    cmp r8, 0
    je lab52300
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52298
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52299

lab52298:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52299:

lab52300:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52291_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52302
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52301
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52301:
    jmp lab52303

lab52302:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52303:
    ; substitute (a0 !-> a0);
    ; #erase muls1
    cmp rsi, 0
    je lab52306
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52304
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52305

lab52304:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52305:

lab52306:
    ; #erase subs
    cmp r8, 0
    je lab52309
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52307
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52308

lab52307:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52308:

lab52309:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52291_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52311
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52310
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52310:
    jmp lab52312

lab52311:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52312:
    ; substitute (muls1 !-> muls1)(muls2 !-> muls2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump equal_list_
    jmp equal_list_

Expr_52291_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52314
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52313
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52313:
    jmp lab52315

lab52314:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52315:
    ; substitute (a0 !-> a0);
    ; #erase divs
    cmp r8, 0
    je lab52318
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52316
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52317

lab52316:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52317:

lab52318:
    ; #erase muls1
    cmp rsi, 0
    je lab52321
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52319
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52320

lab52319:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52320:

lab52321:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52291_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52322
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52323

lab52322:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52323:
    ; substitute (a0 !-> a0);
    ; #erase muls1
    cmp rsi, 0
    je lab52326
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52324
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52325

lab52324:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52325:

lab52326:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52291_X:
    ; substitute (a0 !-> a0);
    ; #erase muls1
    cmp rsi, 0
    je lab52329
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52327
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52328

lab52327:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52328:

lab52329:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52203_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52331
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52330
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52330:
    jmp lab52332

lab52331:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52332:
    ; substitute (a0 !-> a0)(divs1 !-> divs1)(exp2 !-> exp2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52333]
    add rcx, r9
    jmp rcx

Expr_52333:
    jmp near Expr_52333_Add
    jmp near Expr_52333_Sub
    jmp near Expr_52333_Mul
    jmp near Expr_52333_Div
    jmp near Expr_52333_Num
    jmp near Expr_52333_X

Expr_52333_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52335
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52334
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52334:
    jmp lab52336

lab52335:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52336:
    ; substitute (a0 !-> a0);
    ; #erase divs1
    cmp rsi, 0
    je lab52339
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52337
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52338

lab52337:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52338:

lab52339:
    ; #erase sums
    cmp r8, 0
    je lab52342
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52340
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52341

lab52340:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52341:

lab52342:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52333_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52344
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52343
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52343:
    jmp lab52345

lab52344:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52345:
    ; substitute (a0 !-> a0);
    ; #erase divs1
    cmp rsi, 0
    je lab52348
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52346
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52347

lab52346:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52347:

lab52348:
    ; #erase subs
    cmp r8, 0
    je lab52351
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52349
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52350

lab52349:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52350:

lab52351:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52333_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52353
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52352
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52352:
    jmp lab52354

lab52353:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52354:
    ; substitute (a0 !-> a0);
    ; #erase divs1
    cmp rsi, 0
    je lab52357
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52355
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52356

lab52355:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52356:

lab52357:
    ; #erase muls
    cmp r8, 0
    je lab52360
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52358
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52359

lab52358:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52359:

lab52360:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52333_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52362
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52361
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52361:
    jmp lab52363

lab52362:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52363:
    ; substitute (divs1 !-> divs1)(divs2 !-> divs2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump equal_list_
    jmp equal_list_

Expr_52333_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52364
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52365

lab52364:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52365:
    ; substitute (a0 !-> a0);
    ; #erase divs1
    cmp rsi, 0
    je lab52368
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52366
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52367

lab52366:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52367:

lab52368:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52333_X:
    ; substitute (a0 !-> a0);
    ; #erase divs1
    cmp rsi, 0
    je lab52371
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52369
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52370

lab52369:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52370:

lab52371:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52203_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52372
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52373

lab52372:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52373:
    ; substitute (a0 !-> a0)(i1 !-> i1)(exp2 !-> exp2);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52374]
    add rcx, r9
    jmp rcx

Expr_52374:
    jmp near Expr_52374_Add
    jmp near Expr_52374_Sub
    jmp near Expr_52374_Mul
    jmp near Expr_52374_Div
    jmp near Expr_52374_Num
    jmp near Expr_52374_X

Expr_52374_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52376
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52375
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52375:
    jmp lab52377

lab52376:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52377:
    ; substitute (a0 !-> a0);
    ; #erase sums
    cmp r8, 0
    je lab52380
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52378
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52379

lab52378:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52379:

lab52380:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52374_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52382
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52381
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52381:
    jmp lab52383

lab52382:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52383:
    ; substitute (a0 !-> a0);
    ; #erase subs
    cmp r8, 0
    je lab52386
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52384
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52385

lab52384:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52385:

lab52386:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52374_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52388
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52387
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52387:
    jmp lab52389

lab52388:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52389:
    ; substitute (a0 !-> a0);
    ; #erase muls
    cmp r8, 0
    je lab52392
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52390
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52391

lab52390:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52391:

lab52392:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52374_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52394
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52393
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52393:
    jmp lab52395

lab52394:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52395:
    ; substitute (a0 !-> a0);
    ; #erase divs
    cmp r8, 0
    je lab52398
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52396
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52397

lab52396:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52397:

lab52398:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52374_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52399
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52400

lab52399:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52400:
    ; if i1 == i2 \{ ... \}
    cmp rdi, r9
    je lab52401
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab52401:
    ; substitute (a0 !-> a0);
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Expr_52374_X:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52203_X:
    ; switch exp2 \{ ... \};
    lea rcx, [rel Expr_52402]
    add rcx, rdi
    jmp rcx

Expr_52402:
    jmp near Expr_52402_Add
    jmp near Expr_52402_Sub
    jmp near Expr_52402_Mul
    jmp near Expr_52402_Div
    jmp near Expr_52402_Num
    jmp near Expr_52402_X

Expr_52402_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52404
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab52403
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52403:
    jmp lab52405

lab52404:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab52405:
    ; substitute (a0 !-> a0);
    ; #erase sums
    cmp rsi, 0
    je lab52408
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52406
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52407

lab52406:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52407:

lab52408:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52402_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52410
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab52409
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52409:
    jmp lab52411

lab52410:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab52411:
    ; substitute (a0 !-> a0);
    ; #erase subs
    cmp rsi, 0
    je lab52414
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52412
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52413

lab52412:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52413:

lab52414:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52402_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52416
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab52415
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52415:
    jmp lab52417

lab52416:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab52417:
    ; substitute (a0 !-> a0);
    ; #erase muls
    cmp rsi, 0
    je lab52420
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52418
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52419

lab52418:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52419:

lab52420:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52402_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52422
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab52421
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52421:
    jmp lab52423

lab52422:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab52423:
    ; substitute (a0 !-> a0);
    ; #erase divs
    cmp rsi, 0
    je lab52426
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52424
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52425

lab52424:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52425:

lab52426:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52402_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52427
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab52428

lab52427:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab52428:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Expr_52402_X:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

deriv_:
    ; substitute (e !-> e)(a0 !-> a0)(e0 !-> e);
    ; #share e
    cmp rax, 0
    je lab52429
    ; ####increment refcount
    add qword [rax + 0], 1

lab52429:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; switch e0 \{ ... \};
    lea rcx, [rel Expr_52430]
    add rcx, r9
    jmp rcx

Expr_52430:
    jmp near Expr_52430_Add
    jmp near Expr_52430_Sub
    jmp near Expr_52430_Mul
    jmp near Expr_52430_Div
    jmp near Expr_52430_Num
    jmp near Expr_52430_X

Expr_52430_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52432
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52431
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52431:
    jmp lab52433

lab52432:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52433:
    ; substitute (sums !-> sums)(a0 !-> a0);
    ; #erase e
    cmp rax, 0
    je lab52436
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab52434
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab52435

lab52434:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab52435:

lab52436:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; new a1: List[Expr] = (a0)\{ ... \};
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
    je lab52448
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52449

lab52448:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52446
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52439
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52437
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52438

lab52437:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52438:

lab52439:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52442
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52440
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52441

lab52440:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52441:

lab52442:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52445
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52443
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52444

lab52443:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52444:

lab52445:
    jmp lab52447

lab52446:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52447:

lab52449:
    ; #load tag
    lea rdi, [rel List_Expr_52450]
    ; new x1: Fun[Expr, Expr] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Expr_Expr_52451]
    ; substitute (x1 !-> x1)(sums !-> sums)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_list_
    jmp map_list_

Fun_Expr_Expr_52451:

Fun_Expr_Expr_52451_Apply:
    ; jump deriv_
    jmp deriv_

List_Expr_52450:
    jmp near List_Expr_52450_Nil
    jmp near List_Expr_52450_Cons

List_Expr_52450_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52453
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab52452
    ; ####increment refcount
    add qword [rax + 0], 1

lab52452:
    jmp lab52454

lab52453:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab52454:
    ; let x0: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

List_Expr_52450_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52456
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52455
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52455:
    jmp lab52457

lab52456:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52457:
    ; substitute (a0 !-> a0)(x34 !-> x34)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Expr] = Cons(x34, xs0);
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
    je lab52469
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52470

lab52469:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52467
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52460
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52458
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52459

lab52458:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52459:

lab52460:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52463
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52461
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52462

lab52461:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52462:

lab52463:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52466
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52464
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52465

lab52464:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52465:

lab52466:
    jmp lab52468

lab52467:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52468:

lab52470:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

Expr_52430_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52472
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52471
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52471:
    jmp lab52473

lab52472:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52473:
    ; substitute (subs !-> subs)(a0 !-> a0);
    ; #erase e
    cmp rax, 0
    je lab52476
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab52474
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab52475

lab52474:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab52475:

lab52476:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; new a3: List[Expr] = (a0)\{ ... \};
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
    je lab52488
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52489

lab52488:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52486
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52479
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52477
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52478

lab52477:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52478:

lab52479:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52481

lab52480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52481:

lab52482:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52484

lab52483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52484:

lab52485:
    jmp lab52487

lab52486:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52487:

lab52489:
    ; #load tag
    lea rdi, [rel List_Expr_52490]
    ; new x3: Fun[Expr, Expr] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Expr_Expr_52491]
    ; substitute (x3 !-> x3)(subs !-> subs)(a3 !-> a3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_list_
    jmp map_list_

Fun_Expr_Expr_52491:

Fun_Expr_Expr_52491_Apply:
    ; jump deriv_
    jmp deriv_

List_Expr_52490:
    jmp near List_Expr_52490_Nil
    jmp near List_Expr_52490_Cons

List_Expr_52490_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52493
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab52492
    ; ####increment refcount
    add qword [rax + 0], 1

lab52492:
    jmp lab52494

lab52493:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab52494:
    ; let x2: List[Expr] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Sub
    add rdi, 5
    jmp rdi

List_Expr_52490_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52496
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52495
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52495:
    jmp lab52497

lab52496:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52497:
    ; substitute (a0 !-> a0)(x35 !-> x35)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x2: List[Expr] = Cons(x35, xs1);
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
    je lab52509
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52510

lab52509:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52507
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52499

lab52498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52499:

lab52500:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52502

lab52501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52502:

lab52503:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52506
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52504
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52505

lab52504:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52505:

lab52506:
    jmp lab52508

lab52507:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52508:

lab52510:
    ; #load tag
    mov rdi, 5
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Sub
    add rdi, 5
    jmp rdi

Expr_52430_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52512
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52511
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52511:
    jmp lab52513

lab52512:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52513:
    ; substitute (muls !-> muls)(a0 !-> a0)(e !-> e);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a5: List[Expr] = (a0, e)\{ ... \};
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
    je lab52525
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52526

lab52525:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52523
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52515

lab52514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52515:

lab52516:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52518

lab52517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52518:

lab52519:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52522
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52520
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52521

lab52520:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52521:

lab52522:
    jmp lab52524

lab52523:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52524:

lab52526:
    ; #load tag
    lea rdi, [rel List_Expr_52527]
    ; new x8: Fun[Expr, Expr] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Expr_Expr_52528]
    ; substitute (x8 !-> x8)(muls !-> muls)(a5 !-> a5);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_list_
    jmp map_list_

Fun_Expr_Expr_52528:

Fun_Expr_Expr_52528_Apply:
    ; substitute (x37 !-> x)(a6 !-> a6)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab52529
    ; ####increment refcount
    add qword [rax + 0], 1

lab52529:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a7: Expr = (a6, x)\{ ... \};
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
    je lab52541
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52542

lab52541:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52539
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52531

lab52530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52531:

lab52532:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52535
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52533
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52534

lab52533:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52534:

lab52535:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52538
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52536
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52537

lab52536:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52537:

lab52538:
    jmp lab52540

lab52539:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52540:

lab52542:
    ; #load tag
    lea rdi, [rel Expr_52543]
    ; jump deriv_
    jmp deriv_

Expr_52543:
    jmp near Expr_52543_Add
    jmp near Expr_52543_Sub
    jmp near Expr_52543_Mul
    jmp near Expr_52543_Div
    jmp near Expr_52543_Num
    jmp near Expr_52543_X

Expr_52543_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52546
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52544
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52544:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52545
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52545:
    jmp lab52547

lab52546:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52547:
    ; substitute (x !-> x)(a6 !-> a6)(sums0 !-> sums0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x10: Expr = Add(sums0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52559
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52560

lab52559:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52557
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52550
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52548
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52549

lab52548:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52549:

lab52550:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52553
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52551
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52552

lab52551:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52552:

lab52553:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52556
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52554
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52555

lab52554:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52555:

lab52556:
    jmp lab52558

lab52557:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52558:

lab52560:
    ; #load tag
    mov r9, 0
    ; substitute (a6 !-> a6)(x !-> x)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

Expr_52543_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52563
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52561
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52561:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52562
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52562:
    jmp lab52564

lab52563:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52564:
    ; substitute (x !-> x)(a6 !-> a6)(subs0 !-> subs0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x10: Expr = Sub(subs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52576
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52577

lab52576:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52574
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52567
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52565
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52566

lab52565:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52566:

lab52567:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52570
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52568
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52569

lab52568:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52569:

lab52570:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52573
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52571
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52572

lab52571:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52572:

lab52573:
    jmp lab52575

lab52574:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52575:

lab52577:
    ; #load tag
    mov r9, 5
    ; substitute (a6 !-> a6)(x !-> x)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

Expr_52543_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52580
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52578
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52578:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52579
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52579:
    jmp lab52581

lab52580:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52581:
    ; substitute (x !-> x)(a6 !-> a6)(muls0 !-> muls0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x10: Expr = Mul(muls0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52593
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52594

lab52593:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52591
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52584
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52582
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52583

lab52582:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52583:

lab52584:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52587
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52585
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52586

lab52585:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52586:

lab52587:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52590
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52588
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52589

lab52588:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52589:

lab52590:
    jmp lab52592

lab52591:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52592:

lab52594:
    ; #load tag
    mov r9, 10
    ; substitute (a6 !-> a6)(x !-> x)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

Expr_52543_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52597
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52595
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52595:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52596
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52596:
    jmp lab52598

lab52597:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52598:
    ; substitute (x !-> x)(a6 !-> a6)(divs0 !-> divs0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x10: Expr = Div(divs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52610
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52611

lab52610:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52608
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52600

lab52599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52600:

lab52601:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52603

lab52602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52603:

lab52604:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52606

lab52605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52606:

lab52607:
    jmp lab52609

lab52608:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52609:

lab52611:
    ; #load tag
    mov r9, 15
    ; substitute (a6 !-> a6)(x !-> x)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

Expr_52543_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52614
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52612
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52612:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52613
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52613:
    jmp lab52615

lab52614:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52615:
    ; substitute (x !-> x)(a6 !-> a6)(i0 !-> i0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x10: Expr = Num(i0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52627
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52628

lab52627:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52625
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52618
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52616
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52617

lab52616:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52617:

lab52618:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52621
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52619
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52620

lab52619:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52620:

lab52621:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52624
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52622
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52623

lab52622:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52623:

lab52624:
    jmp lab52626

lab52625:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52626:

lab52628:
    ; #load tag
    mov r9, 20
    ; substitute (a6 !-> a6)(x !-> x)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

Expr_52543_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52631
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab52629
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52629:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab52630
    ; ####increment refcount
    add qword [rax + 0], 1

lab52630:
    jmp lab52632

lab52631:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab52632:
    ; let x10: Expr = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 25
    ; jump lift_deriv_1_
    jmp lift_deriv_1_

List_Expr_52527:
    jmp near List_Expr_52527_Nil
    jmp near List_Expr_52527_Cons

List_Expr_52527_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52635
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab52633
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52633:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab52634
    ; ####increment refcount
    add qword [rax + 0], 1

lab52634:
    jmp lab52636

lab52635:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab52636:
    ; let x7: List[Expr] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_deriv_0_
    jmp lift_deriv_0_

List_Expr_52527_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52639
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab52637
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52637:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab52638
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52638:
    jmp lab52640

lab52639:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab52640:
    ; substitute (e !-> e)(a0 !-> a0)(x36 !-> x36)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x7: List[Expr] = Cons(x36, xs2);
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
    je lab52652
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52653

lab52652:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52650
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52642

lab52641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52642:

lab52643:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52645

lab52644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52645:

lab52646:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52648

lab52647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52648:

lab52649:
    jmp lab52651

lab52650:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52651:

lab52653:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(e !-> e)(x7 !-> x7);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_0_
    jmp lift_deriv_0_

Expr_52430_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52655
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab52654
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52654:
    jmp lab52656

lab52655:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab52656:
    ; substitute (a0 !-> a0)(divs !-> divs);
    ; #erase e
    cmp rax, 0
    je lab52659
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab52657
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab52658

lab52657:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab52658:

lab52659:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; switch divs \{ ... \};
    lea rcx, [rel List_Expr_52660]
    add rcx, rdi
    jmp rcx

List_Expr_52660:
    jmp near List_Expr_52660_Nil
    jmp near List_Expr_52660_Cons

List_Expr_52660_Nil:
    ; invoke a0 X
    add rdx, 25
    jmp rdx

List_Expr_52660_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52663
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab52661
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52661:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52662
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52662:
    jmp lab52664

lab52663:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab52664:
    ; switch xs \{ ... \};
    lea rcx, [rel List_Expr_52665]
    add rcx, r9
    jmp rcx

List_Expr_52665:
    jmp near List_Expr_52665_Nil
    jmp near List_Expr_52665_Cons

List_Expr_52665_Nil:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab52668
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52666
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52667

lab52666:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52667:

lab52668:
    ; invoke a0 X
    add rdx, 25
    jmp rdx

List_Expr_52665_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52671
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab52669
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52669:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab52670
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52670:
    jmp lab52672

lab52671:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab52672:
    ; switch ys \{ ... \};
    lea rcx, [rel List_Expr_52673]
    add rcx, r11
    jmp rcx

List_Expr_52673:
    jmp near List_Expr_52673_Nil
    jmp near List_Expr_52673_Cons

List_Expr_52673_Nil:
    ; substitute (x38 !-> x)(x !-> x)(y !-> y)(a0 !-> a0);
    ; #share x
    cmp rsi, 0
    je lab52674
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52674:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a8: Expr = (x, y, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab52686
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52687

lab52686:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52684
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52677
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52675
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52676

lab52675:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52676:

lab52677:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52680
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52678
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52679

lab52678:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52679:

lab52680:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52683
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52681
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52682

lab52681:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52682:

lab52683:
    jmp lab52685

lab52684:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52685:

lab52687:
    ; #load tag
    lea rdi, [rel Expr_52688]
    ; jump deriv_
    jmp deriv_

Expr_52688:
    jmp near Expr_52688_Add
    jmp near Expr_52688_Sub
    jmp near Expr_52688_Mul
    jmp near Expr_52688_Div
    jmp near Expr_52688_Num
    jmp near Expr_52688_X

Expr_52688_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52692
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52689
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52689:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52690
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52690:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52691
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52691:
    jmp lab52693

lab52692:
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
    mov rsi, [rsi + 16]

lab52693:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(sums2 !-> sums2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x17: Expr = Add(sums2);
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
    je lab52705
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52706

lab52705:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52703
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52695

lab52694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52695:

lab52696:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52698

lab52697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52698:

lab52699:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52701

lab52700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52701:

lab52702:
    jmp lab52704

lab52703:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52704:

lab52706:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

Expr_52688_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52710
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52707
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52707:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52708
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52708:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52709
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52709:
    jmp lab52711

lab52710:
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
    mov rsi, [rsi + 16]

lab52711:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(subs2 !-> subs2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x17: Expr = Sub(subs2);
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
    je lab52723
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52724

lab52723:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52721
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52714
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52712
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52713

lab52712:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52713:

lab52714:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52717
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52715
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52716

lab52715:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52716:

lab52717:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52720
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52718
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52719

lab52718:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52719:

lab52720:
    jmp lab52722

lab52721:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52722:

lab52724:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

Expr_52688_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52728
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52725
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52725:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52726
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52726:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52727
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52727:
    jmp lab52729

lab52728:
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
    mov rsi, [rsi + 16]

lab52729:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(muls2 !-> muls2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x17: Expr = Mul(muls2);
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
    je lab52741
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52742

lab52741:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52739
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52732
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52730
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52731

lab52730:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52731:

lab52732:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52734

lab52733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52734:

lab52735:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52737

lab52736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52737:

lab52738:
    jmp lab52740

lab52739:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52740:

lab52742:
    ; #load tag
    mov r11, 10
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

Expr_52688_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52746
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52743
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52743:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52744
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52744:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52745
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52745:
    jmp lab52747

lab52746:
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
    mov rsi, [rsi + 16]

lab52747:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(divs2 !-> divs2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x17: Expr = Div(divs2);
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
    je lab52759
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52760

lab52759:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52757
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52750
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52748
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52749

lab52748:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52749:

lab52750:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52753
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52751
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52752

lab52751:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52752:

lab52753:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52756
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52754
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52755

lab52754:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52755:

lab52756:
    jmp lab52758

lab52757:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52758:

lab52760:
    ; #load tag
    mov r11, 15
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

Expr_52688_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52764
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52761
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52761:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52762
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52762:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52763
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52763:
    jmp lab52765

lab52764:
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
    mov rsi, [rsi + 16]

lab52765:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(i2 !-> i2);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r10
    ; let x17: Expr = Num(i2);
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
    je lab52777
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52778

lab52777:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52775
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52768
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52766
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52767

lab52766:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52767:

lab52768:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52771
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52769
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52770

lab52769:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52770:

lab52771:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52774
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52772
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52773

lab52772:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52773:

lab52774:
    jmp lab52776

lab52775:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52776:

lab52778:
    ; #load tag
    mov r11, 20
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

Expr_52688_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52782
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab52779
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52779:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab52780
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52780:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab52781
    ; ####increment refcount
    add qword [rax + 0], 1

lab52781:
    jmp lab52783

lab52782:
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
    mov rax, [rax + 16]

lab52783:
    ; let x17: Expr = X();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 25
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_2_
    jmp lift_deriv_2_

List_Expr_52673_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab52786
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab52784
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52784:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab52785
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52785:
    jmp lab52787

lab52786:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab52787:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab52790
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab52788
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab52789

lab52788:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab52789:

lab52790:
    ; #erase y
    cmp r8, 0
    je lab52793
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab52791
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab52792

lab52791:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab52792:

lab52793:
    ; #erase z
    cmp r10, 0
    je lab52796
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab52794
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab52795

lab52794:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab52795:

lab52796:
    ; #erase zs
    cmp r12, 0
    je lab52799
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab52797
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab52798

lab52797:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab52798:

lab52799:
    ; invoke a0 X
    add rdx, 25
    jmp rdx

Expr_52430_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab52800
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab52801

lab52800:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab52801:
    ; substitute (a0 !-> a0);
    ; #erase e
    cmp rax, 0
    je lab52804
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab52802
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab52803

lab52802:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab52803:

lab52804:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x32 <- 0;
    mov rdi, 0
    ; substitute (x32 !-> x32)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Num
    add rdi, 20
    jmp rdi

Expr_52430_X:
    ; substitute (a0 !-> a0);
    ; #erase e
    cmp rax, 0
    je lab52807
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab52805
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab52806

lab52805:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab52806:

lab52807:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x33 <- 1;
    mov rdi, 1
    ; substitute (x33 !-> x33)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Num
    add rdi, 20
    jmp rdi

lift_deriv_2_:
    ; let x19: List[Expr] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(x !-> x)(x17 !-> x17)(y !-> y)(y0 !-> y)(x19 !-> x19);
    ; #share y
    cmp r10, 0
    je lab52808
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52808:
    ; #move variables
    mov r14, r12
    mov r12, r10
    mov r15, r13
    mov r13, r11
    ; let x18: List[Expr] = Cons(y0, x19);
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
    je lab52820
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52821

lab52820:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52818
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52811
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52809
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52810

lab52809:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52810:

lab52811:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52814
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52812
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52813

lab52812:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52813:

lab52814:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52817
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52815
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52816

lab52815:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52816:

lab52817:
    jmp lab52819

lab52818:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52819:

lab52821:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x17 !-> x17)(x18 !-> x18);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; let x16: List[Expr] = Cons(x17, x18);
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
    je lab52833
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52834

lab52833:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52831
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52824
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52822
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52823

lab52822:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52823:

lab52824:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52827
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52825
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52826

lab52825:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52826:

lab52827:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52830
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52828
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52829

lab52828:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52829:

lab52830:
    jmp lab52832

lab52831:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52832:

lab52834:
    ; #load tag
    mov r11, 5
    ; let x15: Expr = Div(x16);
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
    je lab52846
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab52847

lab52846:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52844
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52837
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52835
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52836

lab52835:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52836:

lab52837:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52840
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52838
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52839

lab52838:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52839:

lab52840:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52843
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52841
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52842

lab52841:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52842:

lab52843:
    jmp lab52845

lab52844:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52845:

lab52847:
    ; #load tag
    mov r11, 15
    ; substitute (y1 !-> y)(x !-> x)(y !-> y)(x15 !-> x15)(a0 !-> a0);
    ; #share y
    cmp r8, 0
    je lab52848
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52848:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rax, r8
    mov rdx, r9
    ; new a9: Expr = (x, y, x15, a0)\{ ... \};
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
    je lab52860
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab52861

lab52860:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52858
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52850

lab52849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52850:

lab52851:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52854
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52852
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52853

lab52852:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52853:

lab52854:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52857
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52855
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52856

lab52855:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52856:

lab52857:
    jmp lab52859

lab52858:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52859:

lab52861:
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
    je lab52873
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab52874

lab52873:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52871
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52863

lab52862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52863:

lab52864:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52866

lab52865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52866:

lab52867:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52870
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52868
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52869

lab52868:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52869:

lab52870:
    jmp lab52872

lab52871:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52872:

lab52874:
    ; #load tag
    lea rdi, [rel Expr_52875]
    ; jump deriv_
    jmp deriv_

Expr_52875:
    jmp near Expr_52875_Add
    jmp near Expr_52875_Sub
    jmp near Expr_52875_Mul
    jmp near Expr_52875_Div
    jmp near Expr_52875_Num
    jmp near Expr_52875_X

Expr_52875_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52880
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52876
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52876:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab52877
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52877:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab52878
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52878:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab52879
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52879:
    jmp lab52881

lab52880:
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
    mov r8, [r8 + 16]

lab52881:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x15 !-> x15)(sums1 !-> sums1);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x28: Expr = Add(sums1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52893
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52894

lab52893:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52891
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52883

lab52882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52883:

lab52884:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52886

lab52885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52886:

lab52887:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52889

lab52888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52889:

lab52890:
    jmp lab52892

lab52891:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52892:

lab52894:
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

Expr_52875_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52899
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52895
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52895:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab52896
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52896:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab52897
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52897:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab52898
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52898:
    jmp lab52900

lab52899:
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
    mov r8, [r8 + 16]

lab52900:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x15 !-> x15)(subs1 !-> subs1);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x28: Expr = Sub(subs1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52912
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52913

lab52912:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52910
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52902

lab52901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52902:

lab52903:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52905

lab52904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52905:

lab52906:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52909
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52907
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52908

lab52907:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52908:

lab52909:
    jmp lab52911

lab52910:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52911:

lab52913:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

Expr_52875_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52918
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52914
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52914:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab52915
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52915:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab52916
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52916:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab52917
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52917:
    jmp lab52919

lab52918:
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
    mov r8, [r8 + 16]

lab52919:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x15 !-> x15)(muls1 !-> muls1);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x28: Expr = Mul(muls1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52931
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52932

lab52931:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52929
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52921

lab52920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52921:

lab52922:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52925
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52923
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52924

lab52923:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52924:

lab52925:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52928
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52926
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52927

lab52926:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52927:

lab52928:
    jmp lab52930

lab52929:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52930:

lab52932:
    ; #load tag
    mov r13, 10
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

Expr_52875_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52937
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52933
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52933:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab52934
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52934:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab52935
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52935:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab52936
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52936:
    jmp lab52938

lab52937:
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
    mov r8, [r8 + 16]

lab52938:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x15 !-> x15)(divs1 !-> divs1);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x28: Expr = Div(divs1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52950
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52951

lab52950:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52948
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52941
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52939
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52940

lab52939:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52940:

lab52941:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52944
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52942
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52943

lab52942:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52943:

lab52944:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52947
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52945
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52946

lab52945:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52946:

lab52947:
    jmp lab52949

lab52948:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52949:

lab52951:
    ; #load tag
    mov r13, 15
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

Expr_52875_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab52956
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab52952
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52952:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab52953
    ; ####increment refcount
    add qword [r12 + 0], 1

lab52953:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab52954
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52954:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab52955
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52955:
    jmp lab52957

lab52956:
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
    mov r8, [r8 + 16]

lab52957:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y)(x15 !-> x15)(i1 !-> i1);
    ; #move variables
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r12
    ; let x28: Expr = Num(i1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab52969
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52970

lab52969:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52967
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52960
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52958
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52959

lab52958:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52959:

lab52960:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52963
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52961
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52962

lab52961:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52962:

lab52963:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52966
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52964
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52965

lab52964:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52965:

lab52966:
    jmp lab52968

lab52967:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52968:

lab52970:
    ; #load tag
    mov r13, 20
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

Expr_52875_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab52975
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab52971
    ; ####increment refcount
    add qword [rax + 0], 1

lab52971:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab52972
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52972:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab52973
    ; ####increment refcount
    add qword [r8 + 0], 1

lab52973:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab52974
    ; ####increment refcount
    add qword [rsi + 0], 1

lab52974:
    jmp lab52976

lab52975:
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
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab52976:
    ; let x28: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(x28 !-> x28)(y !-> y);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_deriv_3_
    jmp lift_deriv_3_

lift_deriv_3_:
    ; let x29: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(y !-> y)(x28 !-> x28)(x29 !-> x29);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; let x27: List[Expr] = Cons(x28, x29);
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
    je lab52988
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab52989

lab52988:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab52986
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52979
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52977
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52978

lab52977:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52978:

lab52979:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52982
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52980
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52981

lab52980:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52981:

lab52982:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52985
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52983
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52984

lab52983:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52984:

lab52985:
    jmp lab52987

lab52986:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab52987:

lab52989:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x !-> x)(x15 !-> x15)(y !-> y)(y0 !-> y)(x27 !-> x27);
    ; #share y
    cmp r10, 0
    je lab52990
    ; ####increment refcount
    add qword [r10 + 0], 1

lab52990:
    ; #move variables
    mov r14, r12
    mov r12, r10
    mov r15, r13
    mov r13, r11
    ; let x26: List[Expr] = Cons(y0, x27);
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
    je lab53002
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53003

lab53002:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53000
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab52993
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52991
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52992

lab52991:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52992:

lab52993:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab52996
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52994
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52995

lab52994:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52995:

lab52996:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab52999
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab52997
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab52998

lab52997:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab52998:

lab52999:
    jmp lab53001

lab53000:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53001:

lab53003:
    ; #load tag
    mov r13, 5
    ; let x25: List[Expr] = Cons(y, x26);
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
    je lab53015
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53016

lab53015:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53013
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53005

lab53004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53005:

lab53006:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53009
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53007
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53008

lab53007:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53008:

lab53009:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53012
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53010
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53011

lab53010:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53011:

lab53012:
    jmp lab53014

lab53013:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53014:

lab53016:
    ; #load tag
    mov r11, 5
    ; let x24: Expr = Mul(x25);
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
    je lab53028
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53029

lab53028:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53026
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53019
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53017
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53018

lab53017:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53018:

lab53019:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53022
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53020
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53021

lab53020:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53021:

lab53022:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53025
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53023
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53024

lab53023:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53024:

lab53025:
    jmp lab53027

lab53026:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53027:

lab53029:
    ; #load tag
    mov r11, 10
    ; let x30: List[Expr] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x23: List[Expr] = Cons(x24, x30);
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
    je lab53041
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53042

lab53041:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53039
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53032
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53030
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53031

lab53030:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53031:

lab53032:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53034

lab53033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53034:

lab53035:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53038
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53036
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53037

lab53036:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53037:

lab53038:
    jmp lab53040

lab53039:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53040:

lab53042:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x15 !-> x15)(x !-> x)(x23 !-> x23);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x22: List[Expr] = Cons(x, x23);
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
    je lab53054
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53055

lab53054:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53052
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53045
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53043
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53044

lab53043:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53044:

lab53045:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53048
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53046
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53047

lab53046:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53047:

lab53048:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53051
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53049
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53050

lab53049:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53050:

lab53051:
    jmp lab53053

lab53052:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53053:

lab53055:
    ; #load tag
    mov r9, 5
    ; let x21: Expr = Div(x22);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53067
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53068

lab53067:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53065
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53058
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53056
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53057

lab53056:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53057:

lab53058:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53061
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53059
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53060

lab53059:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53060:

lab53061:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53064
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53062
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53063

lab53062:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53063:

lab53064:
    jmp lab53066

lab53065:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53066:

lab53068:
    ; #load tag
    mov r9, 15
    ; let x31: List[Expr] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; let x20: List[Expr] = Cons(x21, x31);
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
    je lab53080
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53081

lab53080:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53078
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53071
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53069
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53070

lab53069:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53070:

lab53071:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53073

lab53072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53073:

lab53074:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53077
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53075
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53076

lab53075:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53076:

lab53077:
    jmp lab53079

lab53078:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53079:

lab53081:
    ; #load tag
    mov r9, 5
    ; let x14: List[Expr] = Cons(x15, x20);
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
    je lab53093
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab53094

lab53093:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53091
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53083

lab53082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53083:

lab53084:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53086

lab53085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53086:

lab53087:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53089

lab53088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53089:

lab53090:
    jmp lab53092

lab53091:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53092:

lab53094:
    ; #load tag
    mov rdi, 5
    ; substitute (x14 !-> x14)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Sub
    add rdi, 5
    jmp rdi

lift_deriv_1_:
    ; let x12: List[Expr] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a6 !-> a6)(x10 !-> x10)(x !-> x)(x12 !-> x12);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x11: List[Expr] = Cons(x, x12);
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
    je lab53106
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53107

lab53106:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53104
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53097
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53095
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53096

lab53095:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53096:

lab53097:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53100
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53098
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53099

lab53098:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53099:

lab53100:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53103
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53101
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53102

lab53101:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53102:

lab53103:
    jmp lab53105

lab53104:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53105:

lab53107:
    ; #load tag
    mov r9, 5
    ; let x9: List[Expr] = Cons(x10, x11);
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
    je lab53119
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab53120

lab53119:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53117
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53110
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53108
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53109

lab53108:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53109:

lab53110:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53113
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53111
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53112

lab53111:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53112:

lab53113:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53116
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53114
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53115

lab53114:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53115:

lab53116:
    jmp lab53118

lab53117:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53118:

lab53120:
    ; #load tag
    mov rdi, 5
    ; substitute (x9 !-> x9)(a6 !-> a6);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a6 Div
    add rdi, 15
    jmp rdi

lift_deriv_0_:
    ; let x6: Expr = Add(x7);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53132
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53133

lab53132:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53130
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53123
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53121
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53122

lab53121:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53122:

lab53123:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53126
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53124
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53125

lab53124:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53125:

lab53126:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53129
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53127
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53128

lab53127:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53128:

lab53129:
    jmp lab53131

lab53130:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53131:

lab53133:
    ; #load tag
    mov r9, 0
    ; let x13: List[Expr] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; let x5: List[Expr] = Cons(x6, x13);
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
    je lab53145
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53146

lab53145:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53143
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53136
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53134
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53135

lab53134:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53135:

lab53136:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53139
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53137
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53138

lab53137:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53138:

lab53139:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53142
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53140
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53141

lab53140:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53141:

lab53142:
    jmp lab53144

lab53143:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53144:

lab53146:
    ; #load tag
    mov r9, 5
    ; let x4: List[Expr] = Cons(e, x5);
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
    je lab53158
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab53159

lab53158:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53156
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53149
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53147
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53148

lab53147:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53148:

lab53149:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53152
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53150
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53151

lab53150:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53151:

lab53152:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53155
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53153
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53154

lab53153:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53154:

lab53155:
    jmp lab53157

lab53156:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53157:

lab53159:
    ; #load tag
    mov rdi, 5
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Mul
    add rdi, 10
    jmp rdi

mk_exp_:
    ; lit x4 <- 3;
    mov r11, 3
    ; let x3: Expr = Num(x4);
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
    je lab53171
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53172

lab53171:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53169
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53162
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53160
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53161

lab53160:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53161:

lab53162:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53165
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53163
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53164

lab53163:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53164:

lab53165:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53168
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53166
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53167

lab53166:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53167:

lab53168:
    jmp lab53170

lab53169:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53170:

lab53172:
    ; #load tag
    mov r11, 20
    ; let x6: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x8: Expr = X();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 25
    ; let x9: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x7: List[Expr] = Cons(x8, x9);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53184
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53185

lab53184:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53182
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53175
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53173
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53174

lab53173:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53174:

lab53175:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53178
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53176
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53177

lab53176:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53177:

lab53178:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53181
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53179
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53180

lab53179:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53180:

lab53181:
    jmp lab53183

lab53182:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53183:

lab53185:
    ; #load tag
    mov r15, 5
    ; let x5: List[Expr] = Cons(x6, x7);
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
    je lab53197
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53198

lab53197:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53195
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53188
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53186
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53187

lab53186:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53187:

lab53188:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53191
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53189
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53190

lab53189:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53190:

lab53191:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53194
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53192
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53193

lab53192:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53193:

lab53194:
    jmp lab53196

lab53195:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53196:

lab53198:
    ; #load tag
    mov r13, 5
    ; let x2: List[Expr] = Cons(x3, x5);
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
    je lab53210
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53211

lab53210:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53208
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53201
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53199
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53200

lab53199:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53200:

lab53201:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53204
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53202
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53203

lab53202:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53203:

lab53204:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53207
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53205
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53206

lab53205:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53206:

lab53207:
    jmp lab53209

lab53208:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53209:

lab53211:
    ; #load tag
    mov r11, 5
    ; let x1: Expr = Mul(x2);
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
    je lab53223
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53224

lab53223:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53221
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53214
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53212
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53213

lab53212:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53213:

lab53214:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53217
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53215
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53216

lab53215:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53216:

lab53217:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53219

lab53218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53219:

lab53220:
    jmp lab53222

lab53221:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53222:

lab53224:
    ; #load tag
    mov r11, 10
    ; let x14: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x16: Expr = X();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 25
    ; let x17: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x15: List[Expr] = Cons(x16, x17);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53236
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53237

lab53236:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53234
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53227
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53225
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53226

lab53225:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53226:

lab53227:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53230
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53228
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53229

lab53228:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53229:

lab53230:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53233
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53231
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53232

lab53231:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53232:

lab53233:
    jmp lab53235

lab53234:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53235:

lab53237:
    ; #load tag
    mov r15, 5
    ; let x13: List[Expr] = Cons(x14, x15);
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
    je lab53249
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53250

lab53249:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53247
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53240
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53238
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53239

lab53238:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53239:

lab53240:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53243
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53241
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53242

lab53241:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53242:

lab53243:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53246
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53244
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53245

lab53244:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53245:

lab53246:
    jmp lab53248

lab53247:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53248:

lab53250:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(b !-> b)(a0 !-> a0)(a !-> a)(x13 !-> x13);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x12: List[Expr] = Cons(a, x13);
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
    je lab53262
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53263

lab53262:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53260
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53253
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53251
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53252

lab53251:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53252:

lab53253:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53256
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53254
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53255

lab53254:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53255:

lab53256:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53259
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53257
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53258

lab53257:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53258:

lab53259:
    jmp lab53261

lab53260:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53261:

lab53263:
    ; #load tag
    mov r11, 5
    ; let x11: Expr = Mul(x12);
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
    je lab53275
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53276

lab53275:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53273
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53266
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53264
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53265

lab53264:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53265:

lab53266:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53269
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53267
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53268

lab53267:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53268:

lab53269:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53271

lab53270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53271:

lab53272:
    jmp lab53274

lab53273:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53274:

lab53276:
    ; #load tag
    mov r11, 10
    ; let x22: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x23: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x21: List[Expr] = Cons(x22, x23);
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
    je lab53288
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53289

lab53288:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53286
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53279
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53277
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53278

lab53277:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53278:

lab53279:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53282
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53280
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53281

lab53280:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53281:

lab53282:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53285
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53283
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53284

lab53283:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53284:

lab53285:
    jmp lab53287

lab53286:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53287:

lab53289:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(x11 !-> x11)(a0 !-> a0)(b !-> b)(x21 !-> x21);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x20: List[Expr] = Cons(b, x21);
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
    je lab53301
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53302

lab53301:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53299
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53292
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53290
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53291

lab53290:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53291:

lab53292:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53295
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53293
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53294

lab53293:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53294:

lab53295:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53298
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53296
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53297

lab53296:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53297:

lab53298:
    jmp lab53300

lab53299:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53300:

lab53302:
    ; #load tag
    mov r11, 5
    ; let x19: Expr = Mul(x20);
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
    je lab53314
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53315

lab53314:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53312
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53305
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53303
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53304

lab53303:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53304:

lab53305:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53308
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53306
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53307

lab53306:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53307:

lab53308:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53311
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53309
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53310

lab53309:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53310:

lab53311:
    jmp lab53313

lab53312:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53313:

lab53315:
    ; #load tag
    mov r11, 10
    ; lit x26 <- 5;
    mov r13, 5
    ; let x25: Expr = Num(x26);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53327
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53328

lab53327:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53325
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53318
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53316
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53317

lab53316:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53317:

lab53318:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53321
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53319
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53320

lab53319:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53320:

lab53321:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53323

lab53322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53323:

lab53324:
    jmp lab53326

lab53325:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53326:

lab53328:
    ; #load tag
    mov r13, 20
    ; let x27: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x24: List[Expr] = Cons(x25, x27);
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
    je lab53340
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53341

lab53340:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53338
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53331
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53329
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53330

lab53329:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53330:

lab53331:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53334
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53332
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53333

lab53332:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53333:

lab53334:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53336

lab53335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53336:

lab53337:
    jmp lab53339

lab53338:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53339:

lab53341:
    ; #load tag
    mov r13, 5
    ; let x18: List[Expr] = Cons(x19, x24);
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
    je lab53353
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53354

lab53353:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53351
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53344
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53342
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53343

lab53342:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53343:

lab53344:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53347
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53345
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53346

lab53345:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53346:

lab53347:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53350
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53348
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53349

lab53348:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53349:

lab53350:
    jmp lab53352

lab53351:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53352:

lab53354:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(a0 !-> a0)(x11 !-> x11)(x18 !-> x18);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x10: List[Expr] = Cons(x11, x18);
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
    je lab53366
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab53367

lab53366:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53364
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53357
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53355
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53356

lab53355:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53356:

lab53357:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53360
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53358
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53359

lab53358:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53359:

lab53360:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53363
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53361
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53362

lab53361:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53362:

lab53363:
    jmp lab53365

lab53364:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53365:

lab53367:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(x1 !-> x1)(x10 !-> x10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Expr] = Cons(x1, x10);
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
    je lab53379
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab53380

lab53379:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53377
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53370
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53368
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53369

lab53368:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53369:

lab53370:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53373
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53371
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53372

lab53371:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53372:

lab53373:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53376
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53374
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53375

lab53374:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53375:

lab53376:
    jmp lab53378

lab53377:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53378:

lab53380:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

mk_ans_:
    ; lit x6 <- 3;
    mov r11, 3
    ; let x5: Expr = Num(x6);
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
    je lab53392
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53393

lab53392:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53390
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53383
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53381
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53382

lab53381:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53382:

lab53383:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53385

lab53384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53385:

lab53386:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53388

lab53387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53388:

lab53389:
    jmp lab53391

lab53390:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53391:

lab53393:
    ; #load tag
    mov r11, 20
    ; let x8: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x10: Expr = X();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 25
    ; let x11: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x9: List[Expr] = Cons(x10, x11);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53405
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53406

lab53405:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53403
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53396
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53394
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53395

lab53394:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53395:

lab53396:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53399
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53397
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53398

lab53397:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53398:

lab53399:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53402
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53400
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53401

lab53400:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53401:

lab53402:
    jmp lab53404

lab53403:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53404:

lab53406:
    ; #load tag
    mov r15, 5
    ; let x7: List[Expr] = Cons(x8, x9);
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
    je lab53418
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53419

lab53418:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53416
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53409
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53407
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53408

lab53407:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53408:

lab53409:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53412
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53410
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53411

lab53410:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53411:

lab53412:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53415
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53413
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53414

lab53413:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53414:

lab53415:
    jmp lab53417

lab53416:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53417:

lab53419:
    ; #load tag
    mov r13, 5
    ; let x4: List[Expr] = Cons(x5, x7);
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
    je lab53431
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53432

lab53431:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53429
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53422
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53420
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53421

lab53420:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53421:

lab53422:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53425
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53423
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53424

lab53423:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53424:

lab53425:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53428
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53426
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53427

lab53426:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53427:

lab53428:
    jmp lab53430

lab53429:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53430:

lab53432:
    ; #load tag
    mov r11, 5
    ; let x3: Expr = Mul(x4);
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
    je lab53444
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53445

lab53444:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53442
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53435
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53433
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53434

lab53433:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53434:

lab53435:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53438
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53436
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53437

lab53436:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53437:

lab53438:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53441
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53439
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53440

lab53439:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53440:

lab53441:
    jmp lab53443

lab53442:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53443:

lab53445:
    ; #load tag
    mov r11, 10
    ; lit x18 <- 0;
    mov r13, 0
    ; let x17: Expr = Num(x18);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53457
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53458

lab53457:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53455
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53448
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53446
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53447

lab53446:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53447:

lab53448:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53451
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53449
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53450

lab53449:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53450:

lab53451:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53454
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53452
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53453

lab53452:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53453:

lab53454:
    jmp lab53456

lab53455:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53456:

lab53458:
    ; #load tag
    mov r13, 20
    ; lit x21 <- 3;
    mov r15, 3
    ; let x20: Expr = Num(x21);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53470
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53471

lab53470:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53468
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53461
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53459
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53460

lab53459:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53460:

lab53461:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53464
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53462
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53463

lab53462:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53463:

lab53464:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53467
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53465
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53466

lab53465:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53466:

lab53467:
    jmp lab53469

lab53468:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53469:

lab53471:
    ; #load tag
    mov r15, 20
    ; let x22: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x19: List[Expr] = Cons(x20, x22);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53483
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53484

lab53483:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53481
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53474
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53472
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53473

lab53472:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53473:

lab53474:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53477
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53475
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53476

lab53475:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53476:

lab53477:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53480
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53478
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53479

lab53478:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53479:

lab53480:
    jmp lab53482

lab53481:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53482:

lab53484:
    ; #load tag
    mov r15, 5
    ; let x16: List[Expr] = Cons(x17, x19);
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
    je lab53496
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53497

lab53496:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53494
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53487
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53485
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53486

lab53485:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53486:

lab53487:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53490
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53488
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53489

lab53488:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53489:

lab53490:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53493
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53491
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53492

lab53491:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53492:

lab53493:
    jmp lab53495

lab53494:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53495:

lab53497:
    ; #load tag
    mov r13, 5
    ; let x15: Expr = Div(x16);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53509
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53510

lab53509:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53507
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53499

lab53498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53499:

lab53500:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53502

lab53501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53502:

lab53503:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53506
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53504
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53505

lab53504:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53505:

lab53506:
    jmp lab53508

lab53507:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53508:

lab53510:
    ; #load tag
    mov r13, 15
    ; lit x27 <- 1;
    mov r15, 1
    ; let x26: Expr = Num(x27);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53522
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53523

lab53522:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53520
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53512

lab53511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53512:

lab53513:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53515

lab53514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53515:

lab53516:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53518

lab53517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53518:

lab53519:
    jmp lab53521

lab53520:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53521:

lab53523:
    ; #load tag
    mov r15, 20
    ; let x29: Expr = X();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 25
    ; let x30: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let x28: List[Expr] = Cons(x29, x30);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53535
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53536

lab53535:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53533
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53526
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53524
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53525

lab53524:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53525:

lab53526:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53528

lab53527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53528:

lab53529:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53531

lab53530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53531:

lab53532:
    jmp lab53534

lab53533:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53534:

lab53536:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x25: List[Expr] = Cons(x26, x28);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53548
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53549

lab53548:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53546
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53539
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53537
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53538

lab53537:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53538:

lab53539:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53542
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53540
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53541

lab53540:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53541:

lab53542:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53545
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53543
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53544

lab53543:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53544:

lab53545:
    jmp lab53547

lab53546:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53547:

lab53549:
    ; #load tag
    mov r15, 5
    ; let x24: Expr = Div(x25);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53561
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53562

lab53561:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53559
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53552
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53550
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53551

lab53550:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53551:

lab53552:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53555
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53553
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53554

lab53553:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53554:

lab53555:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53558
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53556
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53557

lab53556:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53557:

lab53558:
    jmp lab53560

lab53559:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53560:

lab53562:
    ; #load tag
    mov r15, 15
    ; lit x35 <- 1;
    mov qword [rsp + 2024], 1
    ; let x34: Expr = Num(x35);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53574
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53575

lab53574:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53572
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53565
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53563
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53564

lab53563:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53564:

lab53565:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53568
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53566
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53567

lab53566:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53567:

lab53568:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53571
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53569
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53570

lab53569:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53570:

lab53571:
    jmp lab53573

lab53572:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53573:

lab53575:
    ; #load tag
    mov qword [rsp + 2024], 20
    ; let x37: Expr = X();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 25
    ; let x38: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    mov qword [rsp + 1992], 0
    ; let x36: List[Expr] = Cons(x37, x38);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53587
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53588

lab53587:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53585
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53578
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53576
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53577

lab53576:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53577:

lab53578:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53581
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53579
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53580

lab53579:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53580:

lab53581:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53584
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53582
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53583

lab53582:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53583:

lab53584:
    jmp lab53586

lab53585:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53586:

lab53588:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; let x33: List[Expr] = Cons(x34, x36);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53600
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53601

lab53600:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53598
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53591
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53589
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53590

lab53589:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53590:

lab53591:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53593

lab53592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53593:

lab53594:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53597
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53595
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53596

lab53595:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53596:

lab53597:
    jmp lab53599

lab53598:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53599:

lab53601:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x32: Expr = Div(x33);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53613
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53614

lab53613:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53611
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53603

lab53602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53603:

lab53604:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53606

lab53605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53606:

lab53607:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53610
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53608
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53609

lab53608:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53609:

lab53610:
    jmp lab53612

lab53611:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53612:

lab53614:
    ; #load tag
    mov qword [rsp + 2024], 15
    ; let x39: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let x31: List[Expr] = Cons(x32, x39);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53626
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53627

lab53626:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53624
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53616

lab53615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53616:

lab53617:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53619

lab53618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53619:

lab53620:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53622

lab53621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53622:

lab53623:
    jmp lab53625

lab53624:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53625:

lab53627:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x23: List[Expr] = Cons(x24, x31);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53639
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53640

lab53639:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53637
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53629

lab53628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53629:

lab53630:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53633
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53631
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53632

lab53631:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53632:

lab53633:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53635

lab53634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53635:

lab53636:
    jmp lab53638

lab53637:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53638:

lab53640:
    ; #load tag
    mov r15, 5
    ; let x14: List[Expr] = Cons(x15, x23);
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
    je lab53652
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53653

lab53652:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53650
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53642

lab53641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53642:

lab53643:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53645

lab53644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53645:

lab53646:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53648

lab53647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53648:

lab53649:
    jmp lab53651

lab53650:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53651:

lab53653:
    ; #load tag
    mov r13, 5
    ; let x13: Expr = Add(x14);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53665
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53666

lab53665:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53663
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53655

lab53654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53655:

lab53656:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53659
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53657
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53658

lab53657:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53658:

lab53659:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53661

lab53660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53661:

lab53662:
    jmp lab53664

lab53663:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53664:

lab53666:
    ; #load tag
    mov r13, 0
    ; let x40: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x12: List[Expr] = Cons(x13, x40);
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
    je lab53678
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53679

lab53678:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53676
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53669
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53667
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53668

lab53667:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53668:

lab53669:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53672
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53670
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53671

lab53670:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53671:

lab53672:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53675
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53673
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53674

lab53673:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53674:

lab53675:
    jmp lab53677

lab53676:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53677:

lab53679:
    ; #load tag
    mov r13, 5
    ; let x2: List[Expr] = Cons(x3, x12);
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
    je lab53691
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53692

lab53691:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53689
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53681

lab53680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53681:

lab53682:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53684

lab53683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53684:

lab53685:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53687

lab53686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53687:

lab53688:
    jmp lab53690

lab53689:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53690:

lab53692:
    ; #load tag
    mov r11, 5
    ; let x1: Expr = Mul(x2);
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
    je lab53704
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53705

lab53704:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53702
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53695
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53693
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53694

lab53693:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53694:

lab53695:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53698
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53696
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53697

lab53696:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53697:

lab53698:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53701
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53699
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53700

lab53699:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53700:

lab53701:
    jmp lab53703

lab53702:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53703:

lab53705:
    ; #load tag
    mov r11, 10
    ; let x47: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x49: Expr = X();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 25
    ; let x50: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x48: List[Expr] = Cons(x49, x50);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53717
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53718

lab53717:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53715
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53708
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53707

lab53706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53707:

lab53708:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53711
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53709
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53710

lab53709:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53710:

lab53711:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53714
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53712
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53713

lab53712:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53713:

lab53714:
    jmp lab53716

lab53715:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53716:

lab53718:
    ; #load tag
    mov r15, 5
    ; let x46: List[Expr] = Cons(x47, x48);
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
    je lab53730
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53731

lab53730:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53728
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53721
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53719
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53720

lab53719:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53720:

lab53721:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53724
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53722
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53723

lab53722:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53723:

lab53724:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53727
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53725
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53726

lab53725:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53726:

lab53727:
    jmp lab53729

lab53728:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53729:

lab53731:
    ; #load tag
    mov r13, 5
    ; substitute (a !-> a)(b !-> b)(a0 !-> a0)(x1 !-> x1)(a1 !-> a)(x46 !-> x46);
    ; #share a
    cmp rax, 0
    je lab53732
    ; ####increment refcount
    add qword [rax + 0], 1

lab53732:
    ; #move variables
    mov r14, r12
    mov r12, rax
    mov r15, r13
    mov r13, rdx
    ; let x45: List[Expr] = Cons(a1, x46);
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
    je lab53744
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53745

lab53744:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53742
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53734

lab53733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53734:

lab53735:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53737

lab53736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53737:

lab53738:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53740

lab53739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53740:

lab53741:
    jmp lab53743

lab53742:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53743:

lab53745:
    ; #load tag
    mov r13, 5
    ; let x44: Expr = Mul(x45);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53757
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53758

lab53757:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53755
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53748
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53746
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53747

lab53746:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53747:

lab53748:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53751
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53749
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53750

lab53749:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53750:

lab53751:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53754
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53752
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53753

lab53752:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53753:

lab53754:
    jmp lab53756

lab53755:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53756:

lab53758:
    ; #load tag
    mov r13, 10
    ; lit x57 <- 0;
    mov r15, 0
    ; let x56: Expr = Num(x57);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53770
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53771

lab53770:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53768
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53760

lab53759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53760:

lab53761:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53763

lab53762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53763:

lab53764:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53767
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53765
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53766

lab53765:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53766:

lab53767:
    jmp lab53769

lab53768:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53769:

lab53771:
    ; #load tag
    mov r15, 20
    ; let x59: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; substitute (x56 !-> x56)(b !-> b)(a0 !-> a0)(x1 !-> x1)(x44 !-> x44)(a !-> a)(x59 !-> x59);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    ; let x58: List[Expr] = Cons(a, x59);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53783
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53784

lab53783:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53781
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53774
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53772
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53773

lab53772:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53773:

lab53774:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53777
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53775
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53776

lab53775:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53776:

lab53777:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53780
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53778
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53779

lab53778:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53779:

lab53780:
    jmp lab53782

lab53781:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53782:

lab53784:
    ; #load tag
    mov r15, 5
    ; substitute (x44 !-> x44)(b !-> b)(a0 !-> a0)(x1 !-> x1)(x56 !-> x56)(x58 !-> x58);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x55: List[Expr] = Cons(x56, x58);
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
    je lab53796
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53797

lab53796:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53794
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53787
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53785
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53786

lab53785:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53786:

lab53787:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53790
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53788
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53789

lab53788:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53789:

lab53790:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53793
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53791
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53792

lab53791:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53792:

lab53793:
    jmp lab53795

lab53794:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53795:

lab53797:
    ; #load tag
    mov r13, 5
    ; let x54: Expr = Div(x55);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53809
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53810

lab53809:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53807
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53800
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53798
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53799

lab53798:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53799:

lab53800:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53803
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53801
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53802

lab53801:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53802:

lab53803:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53806
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53804
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53805

lab53804:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53805:

lab53806:
    jmp lab53808

lab53807:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53808:

lab53810:
    ; #load tag
    mov r13, 15
    ; lit x64 <- 1;
    mov r15, 1
    ; let x63: Expr = Num(x64);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53822
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53823

lab53822:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53820
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53812

lab53811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53812:

lab53813:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53815

lab53814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53815:

lab53816:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53818

lab53817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53818:

lab53819:
    jmp lab53821

lab53820:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53821:

lab53823:
    ; #load tag
    mov r15, 20
    ; let x66: Expr = X();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 25
    ; let x67: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let x65: List[Expr] = Cons(x66, x67);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53835
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53836

lab53835:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53833
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53826
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53824
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53825

lab53824:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53825:

lab53826:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53829
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53827
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53828

lab53827:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53828:

lab53829:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53831

lab53830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53831:

lab53832:
    jmp lab53834

lab53833:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53834:

lab53836:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x62: List[Expr] = Cons(x63, x65);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53848
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53849

lab53848:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53846
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53839
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53837
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53838

lab53837:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53838:

lab53839:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53842
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53840
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53841

lab53840:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53841:

lab53842:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53844

lab53843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53844:

lab53845:
    jmp lab53847

lab53846:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53847:

lab53849:
    ; #load tag
    mov r15, 5
    ; let x61: Expr = Div(x62);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53861
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53862

lab53861:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53859
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53852
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53850
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53851

lab53850:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53851:

lab53852:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53855
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53853
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53854

lab53853:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53854:

lab53855:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53858
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53856
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53857

lab53856:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53857:

lab53858:
    jmp lab53860

lab53859:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53860:

lab53862:
    ; #load tag
    mov r15, 15
    ; lit x72 <- 1;
    mov qword [rsp + 2024], 1
    ; let x71: Expr = Num(x72);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53874
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53875

lab53874:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53872
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53864

lab53863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53864:

lab53865:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53867

lab53866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53867:

lab53868:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53870

lab53869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53870:

lab53871:
    jmp lab53873

lab53872:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53873:

lab53875:
    ; #load tag
    mov qword [rsp + 2024], 20
    ; let x74: Expr = X();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 25
    ; let x75: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    mov qword [rsp + 1992], 0
    ; let x73: List[Expr] = Cons(x74, x75);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53887
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53888

lab53887:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53885
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53878
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53876
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53877

lab53876:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53877:

lab53878:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53880

lab53879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53880:

lab53881:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53883

lab53882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53883:

lab53884:
    jmp lab53886

lab53885:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53886:

lab53888:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; let x70: List[Expr] = Cons(x71, x73);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53900
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53901

lab53900:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53898
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53891
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53889
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53890

lab53889:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53890:

lab53891:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53894
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53892
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53893

lab53892:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53893:

lab53894:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53896

lab53895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53896:

lab53897:
    jmp lab53899

lab53898:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53899:

lab53901:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x69: Expr = Div(x70);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53913
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53914

lab53913:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53911
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53904
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53902
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53903

lab53902:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53903:

lab53904:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53907
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53905
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53906

lab53905:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53906:

lab53907:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53910
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53908
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53909

lab53908:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53909:

lab53910:
    jmp lab53912

lab53911:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53912:

lab53914:
    ; #load tag
    mov qword [rsp + 2024], 15
    ; let x76: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let x68: List[Expr] = Cons(x69, x76);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53926
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab53927

lab53926:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53924
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53917
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53915
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53916

lab53915:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53916:

lab53917:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53920
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53918
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53919

lab53918:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53919:

lab53920:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53923
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53921
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53922

lab53921:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53922:

lab53923:
    jmp lab53925

lab53924:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53925:

lab53927:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x60: List[Expr] = Cons(x61, x68);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53939
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab53940

lab53939:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53937
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53930
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53928
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53929

lab53928:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53929:

lab53930:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53933
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53931
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53932

lab53931:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53932:

lab53933:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53936
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53934
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53935

lab53934:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53935:

lab53936:
    jmp lab53938

lab53937:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53938:

lab53940:
    ; #load tag
    mov r15, 5
    ; let x53: List[Expr] = Cons(x54, x60);
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
    je lab53952
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53953

lab53952:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53950
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53943
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53941
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53942

lab53941:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53942:

lab53943:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53946
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53944
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53945

lab53944:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53945:

lab53946:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53949
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53947
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53948

lab53947:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53948:

lab53949:
    jmp lab53951

lab53950:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53951:

lab53953:
    ; #load tag
    mov r13, 5
    ; let x52: Expr = Add(x53);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab53965
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53966

lab53965:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53963
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53956
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53954
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53955

lab53954:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53955:

lab53956:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53959
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53957
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53958

lab53957:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53958:

lab53959:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53961

lab53960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53961:

lab53962:
    jmp lab53964

lab53963:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53964:

lab53966:
    ; #load tag
    mov r13, 0
    ; let x77: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x51: List[Expr] = Cons(x52, x77);
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
    je lab53978
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab53979

lab53978:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53976
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53969
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53967
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53968

lab53967:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53968:

lab53969:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53972
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53970
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53971

lab53970:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53971:

lab53972:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53975
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53973
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53974

lab53973:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53974:

lab53975:
    jmp lab53977

lab53976:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53977:

lab53979:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(b !-> b)(a0 !-> a0)(x44 !-> x44)(x51 !-> x51);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x43: List[Expr] = Cons(x44, x51);
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
    je lab53991
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab53992

lab53991:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab53989
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53982
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53980
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53981

lab53980:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53981:

lab53982:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53985
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53983
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53984

lab53983:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53984:

lab53985:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab53988
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53986
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53987

lab53986:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53987:

lab53988:
    jmp lab53990

lab53989:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab53990:

lab53992:
    ; #load tag
    mov r11, 5
    ; let x42: Expr = Mul(x43);
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
    je lab54004
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54005

lab54004:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54002
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab53995
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53993
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53994

lab53993:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53994:

lab53995:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab53998
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53996
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab53997

lab53996:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab53997:

lab53998:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54001
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab53999
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54000

lab53999:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54000:

lab54001:
    jmp lab54003

lab54002:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54003:

lab54005:
    ; #load tag
    mov r11, 10
    ; let x84: Expr = X();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 25
    ; let x85: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x83: List[Expr] = Cons(x84, x85);
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
    je lab54017
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54018

lab54017:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54015
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54008
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54006
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54007

lab54006:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54007:

lab54008:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54011
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54009
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54010

lab54009:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54010:

lab54011:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54014
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54012
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54013

lab54012:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54013:

lab54014:
    jmp lab54016

lab54015:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54016:

lab54018:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(b !-> b)(a0 !-> a0)(x42 !-> x42)(b0 !-> b)(x83 !-> x83);
    ; #share b
    cmp rsi, 0
    je lab54019
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54019:
    ; #move variables
    mov r14, r12
    mov r12, rsi
    mov r15, r13
    mov r13, rdi
    ; let x82: List[Expr] = Cons(b0, x83);
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
    je lab54031
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54032

lab54031:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54029
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54022
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54020
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54021

lab54020:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54021:

lab54022:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54025
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54023
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54024

lab54023:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54024:

lab54025:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54028
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54026
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54027

lab54026:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54027:

lab54028:
    jmp lab54030

lab54029:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54030:

lab54032:
    ; #load tag
    mov r13, 5
    ; let x81: Expr = Mul(x82);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54044
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54045

lab54044:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54042
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54034

lab54033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54034:

lab54035:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54038
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54036
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54037

lab54036:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54037:

lab54038:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54041
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54039
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54040

lab54039:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54040:

lab54041:
    jmp lab54043

lab54042:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54043:

lab54045:
    ; #load tag
    mov r13, 10
    ; lit x92 <- 0;
    mov r15, 0
    ; let x91: Expr = Num(x92);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54057
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54058

lab54057:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54055
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54048
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54046
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54047

lab54046:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54047:

lab54048:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54051
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54049
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54050

lab54049:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54050:

lab54051:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54054
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54052
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54053

lab54052:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54053:

lab54054:
    jmp lab54056

lab54055:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54056:

lab54058:
    ; #load tag
    mov r15, 20
    ; let x94: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; substitute (x1 !-> x1)(x91 !-> x91)(a0 !-> a0)(x42 !-> x42)(x81 !-> x81)(b !-> b)(x94 !-> x94);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x93: List[Expr] = Cons(b, x94);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54070
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54071

lab54070:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54068
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54061
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54059
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54060

lab54059:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54060:

lab54061:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54064
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54062
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54063

lab54062:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54063:

lab54064:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54067
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54065
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54066

lab54065:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54066:

lab54067:
    jmp lab54069

lab54068:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54069:

lab54071:
    ; #load tag
    mov r15, 5
    ; substitute (x1 !-> x1)(x81 !-> x81)(a0 !-> a0)(x42 !-> x42)(x91 !-> x91)(x93 !-> x93);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x90: List[Expr] = Cons(x91, x93);
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
    je lab54083
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54084

lab54083:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54081
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54073

lab54072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54073:

lab54074:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54077
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54075
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54076

lab54075:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54076:

lab54077:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54080
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54078
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54079

lab54078:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54079:

lab54080:
    jmp lab54082

lab54081:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54082:

lab54084:
    ; #load tag
    mov r13, 5
    ; let x89: Expr = Div(x90);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54096
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54097

lab54096:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54094
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54086

lab54085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54086:

lab54087:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54089

lab54088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54089:

lab54090:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54093
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54091
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54092

lab54091:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54092:

lab54093:
    jmp lab54095

lab54094:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54095:

lab54097:
    ; #load tag
    mov r13, 15
    ; lit x99 <- 1;
    mov r15, 1
    ; let x98: Expr = Num(x99);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54109
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54110

lab54109:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54107
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54100
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54098
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54099

lab54098:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54099:

lab54100:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54103
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54101
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54102

lab54101:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54102:

lab54103:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54106
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54104
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54105

lab54104:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54105:

lab54106:
    jmp lab54108

lab54107:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54108:

lab54110:
    ; #load tag
    mov r15, 20
    ; let x101: Expr = X();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 25
    ; let x102: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let x100: List[Expr] = Cons(x101, x102);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54122
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab54123

lab54122:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54120
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54113
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54111
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54112

lab54111:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54112:

lab54113:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54116
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54114
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54115

lab54114:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54115:

lab54116:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54119
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54117
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54118

lab54117:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54118:

lab54119:
    jmp lab54121

lab54120:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54121:

lab54123:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; let x97: List[Expr] = Cons(x98, x100);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54135
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54136

lab54135:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54133
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54126
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54124
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54125

lab54124:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54125:

lab54126:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54129
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54127
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54128

lab54127:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54128:

lab54129:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54132
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54130
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54131

lab54130:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54131:

lab54132:
    jmp lab54134

lab54133:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54134:

lab54136:
    ; #load tag
    mov r15, 5
    ; let x96: Expr = Div(x97);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54148
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54149

lab54148:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54146
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54139
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54137
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54138

lab54137:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54138:

lab54139:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54142
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54140
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54141

lab54140:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54141:

lab54142:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54145
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54143
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54144

lab54143:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54144:

lab54145:
    jmp lab54147

lab54146:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54147:

lab54149:
    ; #load tag
    mov r15, 15
    ; let x103: List[Expr] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x95: List[Expr] = Cons(x96, x103);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54161
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54162

lab54161:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54159
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54152
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54150
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54151

lab54150:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54151:

lab54152:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54155
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54153
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54154

lab54153:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54154:

lab54155:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54158
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54156
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54157

lab54156:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54157:

lab54158:
    jmp lab54160

lab54159:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54160:

lab54162:
    ; #load tag
    mov r15, 5
    ; let x88: List[Expr] = Cons(x89, x95);
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
    je lab54174
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54175

lab54174:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54172
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54165
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54163
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54164

lab54163:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54164:

lab54165:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54168
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54166
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54167

lab54166:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54167:

lab54168:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54171
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54169
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54170

lab54169:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54170:

lab54171:
    jmp lab54173

lab54172:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54173:

lab54175:
    ; #load tag
    mov r13, 5
    ; let x87: Expr = Add(x88);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54187
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54188

lab54187:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54185
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54178
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54176
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54177

lab54176:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54177:

lab54178:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54181
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54179
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54180

lab54179:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54180:

lab54181:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54184
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54182
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54183

lab54182:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54183:

lab54184:
    jmp lab54186

lab54185:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54186:

lab54188:
    ; #load tag
    mov r13, 0
    ; let x104: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x86: List[Expr] = Cons(x87, x104);
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
    je lab54200
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54201

lab54200:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54198
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54191
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54189
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54190

lab54189:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54190:

lab54191:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54194
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54192
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54193

lab54192:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54193:

lab54194:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54197
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54195
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54196

lab54195:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54196:

lab54197:
    jmp lab54199

lab54198:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54199:

lab54201:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(x42 !-> x42)(a0 !-> a0)(x81 !-> x81)(x86 !-> x86);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x80: List[Expr] = Cons(x81, x86);
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
    je lab54213
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54214

lab54213:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54211
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54204
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54202
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54203

lab54202:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54203:

lab54204:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54207
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54205
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54206

lab54205:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54206:

lab54207:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54210
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54208
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54209

lab54208:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54209:

lab54210:
    jmp lab54212

lab54211:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54212:

lab54214:
    ; #load tag
    mov r11, 5
    ; let x79: Expr = Mul(x80);
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
    je lab54226
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54227

lab54226:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54224
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54217
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54215
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54216

lab54215:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54216:

lab54217:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54219

lab54218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54219:

lab54220:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54222

lab54221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54222:

lab54223:
    jmp lab54225

lab54224:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54225:

lab54227:
    ; #load tag
    mov r11, 10
    ; lit x107 <- 0;
    mov r13, 0
    ; let x106: Expr = Num(x107);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54239
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54240

lab54239:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54237
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54230
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54228
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54229

lab54228:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54229:

lab54230:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54233
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54231
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54232

lab54231:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54232:

lab54233:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54236
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54234
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54235

lab54234:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54235:

lab54236:
    jmp lab54238

lab54237:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54238:

lab54240:
    ; #load tag
    mov r13, 20
    ; let x108: List[Expr] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x105: List[Expr] = Cons(x106, x108);
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
    je lab54252
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54253

lab54252:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54250
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54243
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54241
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54242

lab54241:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54242:

lab54243:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54246
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54244
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54245

lab54244:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54245:

lab54246:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54249
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54247
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54248

lab54247:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54248:

lab54249:
    jmp lab54251

lab54250:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54251:

lab54253:
    ; #load tag
    mov r13, 5
    ; let x78: List[Expr] = Cons(x79, x105);
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
    je lab54265
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54266

lab54265:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54263
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54256
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54254
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54255

lab54254:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54255:

lab54256:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54259
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54257
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54258

lab54257:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54258:

lab54259:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54262
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54260
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54261

lab54260:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54261:

lab54262:
    jmp lab54264

lab54263:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54264:

lab54266:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(a0 !-> a0)(x42 !-> x42)(x78 !-> x78);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x41: List[Expr] = Cons(x42, x78);
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
    je lab54278
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54279

lab54278:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54276
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54269
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54267
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54268

lab54267:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54268:

lab54269:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54271

lab54270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54271:

lab54272:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54275
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54273
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54274

lab54273:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54274:

lab54275:
    jmp lab54277

lab54276:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54277:

lab54279:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(x1 !-> x1)(x41 !-> x41);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Expr] = Cons(x1, x41);
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
    je lab54291
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54292

lab54291:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54289
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54282
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54280
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54281

lab54280:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54281:

lab54282:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54285
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54283
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54284

lab54283:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54284:

lab54285:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54288
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54286
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54287

lab54286:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54287:

lab54288:
    jmp lab54290

lab54289:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54290:

lab54292:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Add
    add rdi, 0
    jmp rdi

main_loop_:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp rdx, r13
    je lab54293
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x8 <- 1;
    mov r11, 1
    ; x9 <- iters - x8;
    mov r13, rdx
    sub r13, r11
    ; substitute (x9 !-> x9)(n !-> n)(n1 !-> n)(a0 !-> a0);
    ; #move variables
    mov r11, r9
    mov r9, rdi
    mov r10, r8
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab54293:
    ; substitute (m0 !-> m)(n0 !-> n)(m !-> m)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov r13, rdi
    mov rdx, r9
    ; new a1: Expr = (m, a0, n)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab54305
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54306

lab54305:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54303
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54296
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54294
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54295

lab54294:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54295:

lab54296:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54299
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54297
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54298

lab54297:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54298:

lab54299:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54302
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54300
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54301

lab54300:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54301:

lab54302:
    jmp lab54304

lab54303:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54304:

lab54306:
    ; #load tag
    lea r9, [rel Expr_54307]
    ; substitute (m0 !-> m0)(a1 !-> a1)(n0 !-> n0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    ; let x2: Expr = Num(n0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54319
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54320

lab54319:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54317
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54310
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54308
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54309

lab54308:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54309:

lab54310:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54313
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54311
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54312

lab54311:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54312:

lab54313:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54316
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54314
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54315

lab54314:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54315:

lab54316:
    jmp lab54318

lab54317:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54318:

lab54320:
    ; #load tag
    mov r9, 20
    ; substitute (x2 !-> x2)(a1 !-> a1)(m0 !-> m0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x3: Expr = Num(m0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54332
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54333

lab54332:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54330
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54323
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54321
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54322

lab54321:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54322:

lab54323:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54326
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54324
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54325

lab54324:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54325:

lab54326:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54328

lab54327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54328:

lab54329:
    jmp lab54331

lab54330:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54331:

lab54333:
    ; #load tag
    mov r9, 20
    ; substitute (x2 !-> x2)(x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump mk_exp_
    jmp mk_exp_

Expr_54307:
    jmp near Expr_54307_Add
    jmp near Expr_54307_Sub
    jmp near Expr_54307_Mul
    jmp near Expr_54307_Div
    jmp near Expr_54307_Num
    jmp near Expr_54307_X

Expr_54307_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54335
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54334
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54334:
    mov rdi, [rsi + 24]
    jmp lab54336

lab54335:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54336:
    ; substitute (n !-> n)(m !-> m)(a0 !-> a0)(sums2 !-> sums2);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x1: Expr = Add(sums2);
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
    je lab54348
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54349

lab54348:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54346
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54339
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54337
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54338

lab54337:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54338:

lab54339:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54342
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54340
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54341

lab54340:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54341:

lab54342:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54345
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54343
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54344

lab54343:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54344:

lab54345:
    jmp lab54347

lab54346:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54347:

lab54349:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Expr_54307_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54351
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54350
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54350:
    mov rdi, [rsi + 24]
    jmp lab54352

lab54351:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54352:
    ; substitute (n !-> n)(m !-> m)(a0 !-> a0)(subs2 !-> subs2);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x1: Expr = Sub(subs2);
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
    je lab54364
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54365

lab54364:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54362
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54355
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54353
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54354

lab54353:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54354:

lab54355:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54358
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54356
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54357

lab54356:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54357:

lab54358:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54361
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54359
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54360

lab54359:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54360:

lab54361:
    jmp lab54363

lab54362:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54363:

lab54365:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Expr_54307_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54367
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54366
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54366:
    mov rdi, [rsi + 24]
    jmp lab54368

lab54367:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54368:
    ; substitute (n !-> n)(m !-> m)(a0 !-> a0)(muls2 !-> muls2);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x1: Expr = Mul(muls2);
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
    je lab54380
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54381

lab54380:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54378
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54371
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54369
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54370

lab54369:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54370:

lab54371:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54374
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54372
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54373

lab54372:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54373:

lab54374:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54377
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54375
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54376

lab54375:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54376:

lab54377:
    jmp lab54379

lab54378:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54379:

lab54381:
    ; #load tag
    mov r11, 10
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Expr_54307_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54383
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54382
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54382:
    mov rdi, [rsi + 24]
    jmp lab54384

lab54383:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54384:
    ; substitute (n !-> n)(m !-> m)(a0 !-> a0)(divs2 !-> divs2);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x1: Expr = Div(divs2);
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
    je lab54396
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54397

lab54396:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54394
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54387
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54385
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54386

lab54385:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54386:

lab54387:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54390
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54388
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54389

lab54388:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54389:

lab54390:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54393
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54391
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54392

lab54391:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54392:

lab54393:
    jmp lab54395

lab54394:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54395:

lab54397:
    ; #load tag
    mov r11, 15
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Expr_54307_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54399
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54398
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54398:
    mov rdi, [rsi + 24]
    jmp lab54400

lab54399:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab54400:
    ; substitute (n !-> n)(m !-> m)(a0 !-> a0)(i2 !-> i2);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x1: Expr = Num(i2);
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
    je lab54412
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54413

lab54412:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54410
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54403
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54401
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54402

lab54401:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54402:

lab54403:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54406
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54404
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54405

lab54404:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54405:

lab54406:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54409
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54407
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54408

lab54407:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54408:

lab54409:
    jmp lab54411

lab54410:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54411:

lab54413:
    ; #load tag
    mov r11, 20
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Expr_54307_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54415
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab54414
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54414:
    mov rdx, [rax + 24]
    jmp lab54416

lab54415:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab54416:
    ; let x1: Expr = X();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 25
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(x1 !-> x1);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; substitute (x1 !-> x1)(m !-> m)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a3: Expr = (m, n, a0)\{ ... \};
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
    je lab54428
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54429

lab54428:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54426
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54419
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54417
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54418

lab54417:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54418:

lab54419:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54422
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54420
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54421

lab54420:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54421:

lab54422:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54425
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54423
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54424

lab54423:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54424:

lab54425:
    jmp lab54427

lab54426:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54427:

lab54429:
    ; #load tag
    lea rdi, [rel Expr_54430]
    ; jump deriv_
    jmp deriv_

Expr_54430:
    jmp near Expr_54430_Add
    jmp near Expr_54430_Sub
    jmp near Expr_54430_Mul
    jmp near Expr_54430_Div
    jmp near Expr_54430_Num
    jmp near Expr_54430_X

Expr_54430_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54432
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab54431
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54431:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab54433

lab54432:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab54433:
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(sums1 !-> sums1);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: Expr = Add(sums1);
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
    je lab54445
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54446

lab54445:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54443
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54436
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54434
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54435

lab54434:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54435:

lab54436:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54439
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54437
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54438

lab54437:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54438:

lab54439:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54442
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54440
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54441

lab54440:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54441:

lab54442:
    jmp lab54444

lab54443:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54444:

lab54446:
    ; #load tag
    mov r11, 0
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Expr_54430_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54448
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab54447
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54447:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab54449

lab54448:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab54449:
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(subs1 !-> subs1);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: Expr = Sub(subs1);
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
    je lab54461
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54462

lab54461:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54459
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54452
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54450
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54451

lab54450:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54451:

lab54452:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54455
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54453
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54454

lab54453:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54454:

lab54455:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54458
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54456
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54457

lab54456:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54457:

lab54458:
    jmp lab54460

lab54459:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54460:

lab54462:
    ; #load tag
    mov r11, 5
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Expr_54430_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54464
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab54463
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54463:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab54465

lab54464:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab54465:
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(muls1 !-> muls1);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: Expr = Mul(muls1);
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
    je lab54477
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54478

lab54477:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54475
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54468
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54466
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54467

lab54466:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54467:

lab54468:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54471
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54469
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54470

lab54469:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54470:

lab54471:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54474
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54472
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54473

lab54472:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54473:

lab54474:
    jmp lab54476

lab54475:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54476:

lab54478:
    ; #load tag
    mov r11, 10
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Expr_54430_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54480
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab54479
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54479:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab54481

lab54480:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab54481:
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(divs1 !-> divs1);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: Expr = Div(divs1);
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
    je lab54493
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54494

lab54493:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54491
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54484
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54482
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54483

lab54482:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54483:

lab54484:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54487
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54485
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54486

lab54485:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54486:

lab54487:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54490
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54488
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54489

lab54488:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54489:

lab54490:
    jmp lab54492

lab54491:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54492:

lab54494:
    ; #load tag
    mov r11, 15
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Expr_54430_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54496
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab54495
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54495:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab54497

lab54496:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab54497:
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(i1 !-> i1);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r10
    ; let res: Expr = Num(i1);
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
    je lab54509
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54510

lab54509:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54507
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54499

lab54498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54499:

lab54500:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54502

lab54501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54502:

lab54503:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54506
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54504
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54505

lab54504:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54505:

lab54506:
    jmp lab54508

lab54507:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54508:

lab54510:
    ; #load tag
    mov r11, 20
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Expr_54430_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54512
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab54511
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54511:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab54513

lab54512:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab54513:
    ; let res: Expr = X();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 25
    ; substitute (a0 !-> a0)(m !-> m)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lift_main_loop_1_:
    ; substitute (a0 !-> a0)(m !-> m)(res !-> res)(n !-> n);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    mov r8, r10
    ; let x4: Expr = Num(n);
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
    je lab54525
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54526

lab54525:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54523
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54515

lab54514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54515:

lab54516:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54518

lab54517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54518:

lab54519:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54522
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54520
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54521

lab54520:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54521:

lab54522:
    jmp lab54524

lab54523:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54524:

lab54526:
    ; #load tag
    mov r11, 20
    ; substitute (a0 !-> a0)(x4 !-> x4)(res !-> res)(m !-> m);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rsi, r10
    ; let x5: Expr = Num(m);
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
    je lab54538
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54539

lab54538:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54536
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54528

lab54527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54528:

lab54529:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54531

lab54530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54531:

lab54532:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54535
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54533
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54534

lab54533:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54534:

lab54535:
    jmp lab54537

lab54536:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54537:

lab54539:
    ; #load tag
    mov r11, 20
    ; substitute (x5 !-> x5)(x4 !-> x4)(res !-> res)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a4: Expr = (res, a0)\{ ... \};
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
    je lab54551
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54552

lab54551:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54549
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54542
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54540
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54541

lab54540:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54541:

lab54542:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54545
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54543
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54544

lab54543:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54544:

lab54545:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54548
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54546
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54547

lab54546:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54547:

lab54548:
    jmp lab54550

lab54549:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54550:

lab54552:
    ; #load tag
    lea r9, [rel Expr_54553]
    ; substitute (x4 !-> x4)(x5 !-> x5)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump mk_ans_
    jmp mk_ans_

Expr_54553:
    jmp near Expr_54553_Add
    jmp near Expr_54553_Sub
    jmp near Expr_54553_Mul
    jmp near Expr_54553_Div
    jmp near Expr_54553_Num
    jmp near Expr_54553_X

Expr_54553_Add:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54556
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab54554
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54554:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab54555
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54555:
    jmp lab54557

lab54556:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab54557:
    ; substitute (a0 !-> a0)(res !-> res)(sums0 !-> sums0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let expected: Expr = Add(sums0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54569
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54570

lab54569:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54567
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54560
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54558
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54559

lab54558:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54559:

lab54560:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54562

lab54561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54562:

lab54563:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54565

lab54564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54565:

lab54566:
    jmp lab54568

lab54567:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54568:

lab54570:
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Expr_54553_Sub:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54573
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab54571
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54571:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab54572
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54572:
    jmp lab54574

lab54573:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab54574:
    ; substitute (a0 !-> a0)(res !-> res)(subs0 !-> subs0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let expected: Expr = Sub(subs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54586
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54587

lab54586:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54584
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54577
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54575
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54576

lab54575:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54576:

lab54577:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54580
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54578
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54579

lab54578:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54579:

lab54580:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54583
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54581
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54582

lab54581:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54582:

lab54583:
    jmp lab54585

lab54584:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54585:

lab54587:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Expr_54553_Mul:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54590
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab54588
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54588:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab54589
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54589:
    jmp lab54591

lab54590:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab54591:
    ; substitute (a0 !-> a0)(res !-> res)(muls0 !-> muls0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let expected: Expr = Mul(muls0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54603
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54604

lab54603:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54601
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54593

lab54592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54593:

lab54594:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54597
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54595
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54596

lab54595:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54596:

lab54597:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54600
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54598
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54599

lab54598:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54599:

lab54600:
    jmp lab54602

lab54601:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54602:

lab54604:
    ; #load tag
    mov r9, 10
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Expr_54553_Div:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54607
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab54605
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54605:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab54606
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54606:
    jmp lab54608

lab54607:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab54608:
    ; substitute (a0 !-> a0)(res !-> res)(divs0 !-> divs0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let expected: Expr = Div(divs0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54620
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54621

lab54620:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54618
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54611
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54609
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54610

lab54609:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54610:

lab54611:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54614
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54612
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54613

lab54612:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54613:

lab54614:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54616

lab54615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54616:

lab54617:
    jmp lab54619

lab54618:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54619:

lab54621:
    ; #load tag
    mov r9, 15
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Expr_54553_Num:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54624
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab54622
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54622:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab54623
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54623:
    jmp lab54625

lab54624:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab54625:
    ; substitute (a0 !-> a0)(res !-> res)(i0 !-> i0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; let expected: Expr = Num(i0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54637
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54638

lab54637:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54635
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54628
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54626
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54627

lab54626:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54627:

lab54628:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54631
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54629
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54630

lab54629:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54630:

lab54631:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54634
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54632
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54633

lab54632:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54633:

lab54634:
    jmp lab54636

lab54635:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54636:

lab54638:
    ; #load tag
    mov r9, 20
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Expr_54553_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54641
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab54639
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54639:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab54640
    ; ####increment refcount
    add qword [rax + 0], 1

lab54640:
    jmp lab54642

lab54641:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab54642:
    ; let expected: Expr = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 25
    ; substitute (a0 !-> a0)(expected !-> expected)(res !-> res);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

lift_main_loop_2_:
    ; substitute (res !-> res)(expected !-> expected)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a5: Bool = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54654
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54655

lab54654:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54652
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54645
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54643
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54644

lab54643:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54644:

lab54645:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54648
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54646
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54647

lab54646:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54647:

lab54648:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54651
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54649
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54650

lab54649:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54650:

lab54651:
    jmp lab54653

lab54652:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54653:

lab54655:
    ; #load tag
    lea r9, [rel Bool_54656]
    ; substitute (expected !-> expected)(res !-> res)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump equal_
    jmp equal_

Bool_54656:
    jmp near Bool_54656_True
    jmp near Bool_54656_False

Bool_54656_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54658
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab54657
    ; ####increment refcount
    add qword [rax + 0], 1

lab54657:
    jmp lab54659

lab54658:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab54659:
    ; lit x6 <- 1;
    mov rdi, 1
    ; println_i64 x6;
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
    ; lit x10 <- 0;
    mov rdi, 0
    ; substitute (x10 !-> x10)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Bool_54656_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54661
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab54660
    ; ####increment refcount
    add qword [rax + 0], 1

lab54660:
    jmp lab54662

lab54661:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab54662:
    ; lit x7 <- 0;
    mov rdi, 0
    ; println_i64 x7;
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
    ; lit x11 <- 0;
    mov rdi, 0
    ; substitute (x11 !-> x11)(a0 !-> a0);
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