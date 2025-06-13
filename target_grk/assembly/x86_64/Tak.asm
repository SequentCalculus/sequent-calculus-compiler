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
    mov r11, r8
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
    mov r12, 0
    ; #load tag
    lea r13, [rel _Cont_65779]
    ; jump main_loop_
    jmp main_loop_

_Cont_65779:

_Cont_65779_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

tak_:
    ; if y < x \{ ... \}
    cmp rdi, rdx
    jl lab65780
    ; substitute (z !-> z)(a0 !-> a0);
    ; #move variables
    mov rdx, r9
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

lab65780:
    ; substitute (x10 !-> x)(y1 !-> y)(z1 !-> z)(a0 !-> a0)(x !-> x)(y !-> y)(z !-> z);
    ; #move variables
    mov r13, rdx
    mov r15, rdi
    mov [rsp + 2024], r9
    ; new a1: _Cont = (a0, x, y, z)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65792
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65793

lab65792:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65790
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65783
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65781
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65782

lab65781:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65782:

lab65783:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65785

lab65784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65785:

lab65786:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65788

lab65787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65788:

lab65789:
    jmp lab65791

lab65790:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65791:

lab65793:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab65805
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65806

lab65805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65795

lab65794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65795:

lab65796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65798

lab65797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65798:

lab65799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65801

lab65800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65801:

lab65802:
    jmp lab65804

lab65803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65804:

lab65806:
    ; #load tag
    lea r11, [rel _Cont_65807]
    ; lit x1 <- 1;
    mov r13, 1
    ; x2 <- x10 - x1;
    mov r15, rdx
    sub r15, r13
    ; substitute (x2 !-> x2)(y1 !-> y1)(z1 !-> z1)(a1 !-> a1);
    ; #move variables
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_65807:

_Cont_65807_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65809
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65808
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65808:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab65810

lab65809:
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
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab65810:
    ; substitute (z0 !-> z)(y0 !-> y)(x9 !-> x)(y !-> y)(z !-> z)(x0 !-> x0)(a0 !-> a0)(x !-> x);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2032], rsi
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdi, r11
    mov rdx, r13
    ; new a3: _Cont = (y, z, x0, a0, x)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    mov [rbx + 24], r15
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65822
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab65823

lab65822:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65820
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65812

lab65811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65812:

lab65813:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65815

lab65814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65815:

lab65816:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65818

lab65817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65818:

lab65819:
    jmp lab65821

lab65820:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65821:

lab65823:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65835
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65836

lab65835:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65833
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65826
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65824
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65825

lab65824:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65825:

lab65826:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65829
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65827
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65828

lab65827:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65828:

lab65829:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65831

lab65830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65831:

lab65832:
    jmp lab65834

lab65833:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65834:

lab65836:
    ; #load tag
    lea r11, [rel _Cont_65837]
    ; lit x4 <- 1;
    mov r13, 1
    ; x5 <- y0 - x4;
    mov r15, rdi
    sub r15, r13
    ; substitute (x5 !-> x5)(z0 !-> z0)(x9 !-> x9)(a3 !-> a3);
    ; #move variables
    mov rdi, rdx
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_65837:

_Cont_65837_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65839
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab65838
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65838:
    mov r11, [r10 + 24]
    jmp lab65840

lab65839:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]

lab65840:
    ; substitute (x !-> x)(y !-> y)(z !-> z)(x0 !-> x0)(a0 !-> a0)(x3 !-> x3);
    ; #move variables
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    ; new a5: _Cont = (x0, a0, x3)\{ ... \};
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
    je lab65852
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65853

lab65852:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65850
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65843
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65841
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65842

lab65841:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65842:

lab65843:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65846
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65844
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65845

lab65844:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65845:

lab65846:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65849
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65847
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65848

lab65847:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65848:

lab65849:
    jmp lab65851

lab65850:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65851:

lab65853:
    ; #load tag
    lea r11, [rel _Cont_65854]
    ; lit x7 <- 1;
    mov r13, 1
    ; x8 <- z - x7;
    mov r15, r9
    sub r15, r13
    ; substitute (x8 !-> x8)(x !-> x)(y !-> y)(a5 !-> a5);
    ; #move variables
    mov r9, rdi
    mov rdi, rdx
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_65854:

_Cont_65854_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65856
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab65855
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65855:
    mov rdi, [rsi + 24]
    jmp lab65857

lab65856:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab65857:
    ; substitute (x0 !-> x0)(x3 !-> x3)(x6 !-> x6)(a0 !-> a0);
    ; #move variables
    mov rcx, rdi
    mov rdi, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, r8
    ; jump tak_
    jmp tak_

main_loop_:
    ; substitute (z0 !-> z)(x4 !-> x)(y0 !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x)(y !-> y);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a2: _Cont = (z, a0, iters, x, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov [rbx + 24], r15
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65869
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab65870

lab65869:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65867
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65860
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65858
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65859

lab65858:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65859:

lab65860:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65863
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65861
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65862

lab65861:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65862:

lab65863:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65866
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65864
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65865

lab65864:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65865:

lab65866:
    jmp lab65868

lab65867:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65868:

lab65870:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab65882
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65883

lab65882:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65880
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65873
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65871
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65872

lab65871:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65872:

lab65873:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65876
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65874
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65875

lab65874:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65875:

lab65876:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65879
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65877
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65878

lab65877:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65878:

lab65879:
    jmp lab65881

lab65880:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65881:

lab65883:
    ; #load tag
    lea r11, [rel _Cont_65884]
    ; substitute (x4 !-> x4)(y0 !-> y0)(z0 !-> z0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump tak_
    jmp tak_

_Cont_65884:

_Cont_65884_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65886
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab65885
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65885:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab65887

lab65886:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]

lab65887:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp r11, [rsp +2024]
    je lab65888
    ; substitute (y !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x);
    ; #move variables
    mov rdx, r15
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- iters - x1;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x2 !-> x2)(x !-> x)(y !-> y)(z !-> z)(a0 !-> a0);
    ; #move variables
    mov r11, rdi
    mov rdi, r13
    mov r13, r9
    mov r9, rdx
    mov r12, r8
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab65888:
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