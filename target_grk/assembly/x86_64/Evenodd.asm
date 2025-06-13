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
    lea r9, [rel _Cont_54829]
    ; jump main_loop_
    jmp main_loop_

_Cont_54829:

_Cont_54829_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

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
    lea rcx, [rel Bool_54830]
    add rcx, r9
    jmp rcx

Bool_54830:
    jmp near Bool_54830_True
    jmp near Bool_54830_False

Bool_54830_True:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_54831]
    add rcx, rdi
    jmp rcx

Bool_54831:
    jmp near Bool_54831_True
    jmp near Bool_54831_False

Bool_54831_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_54831_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_54830_False:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab54834
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab54832
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab54833

lab54832:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab54833:

lab54834:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

not_:
    ; substitute (a0 !-> a0)(b !-> b);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch b \{ ... \};
    lea rcx, [rel Bool_54835]
    add rcx, rdi
    jmp rcx

Bool_54835:
    jmp near Bool_54835_True
    jmp near Bool_54835_False

Bool_54835_True:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_54835_False:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

abs_i_:
    ; lit x0 <- 0;
    mov r9, 0
    ; if n < x0 \{ ... \}
    cmp rdx, r9
    jl lab54836
    ; substitute (n !-> n)(a0 !-> a0);
    ; invoke a0 Ret
    jmp rdi

lab54836:
    ; substitute (n !-> n)(a0 !-> a0);
    ; lit x1 <- -1;
    mov r9, -1
    ; x2 <- x1 * n;
    mov r11, r9
    imul r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

even_abs_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab54837
    ; lit x0 <- 1;
    mov r9, 1
    ; x1 <- n - x0;
    mov r11, rdx
    sub r11, r9
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; jump odd_abs_
    jmp odd_abs_

lab54837:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

odd_abs_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab54838
    ; lit x0 <- 1;
    mov r9, 1
    ; x1 <- n - x0;
    mov r11, rdx
    sub r11, r9
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; jump even_abs_
    jmp even_abs_

lab54838:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

even_:
    ; new a1: _Cont = (a0)\{ ... \};
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
    je lab54850
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54851

lab54850:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54848
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54840

lab54839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54840:

lab54841:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54844
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54842
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54843

lab54842:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54843:

lab54844:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54847
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54845
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54846

lab54845:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54846:

lab54847:
    jmp lab54849

lab54848:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54849:

lab54851:
    ; #load tag
    lea rdi, [rel _Cont_54852]
    ; jump abs_i_
    jmp abs_i_

_Cont_54852:

_Cont_54852_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54854
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab54853
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54853:
    jmp lab54855

lab54854:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab54855:
    ; jump even_abs_
    jmp even_abs_

odd_:
    ; new a1: _Cont = (a0)\{ ... \};
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
    je lab54867
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54868

lab54867:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54865
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54858
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54856
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54857

lab54856:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54857:

lab54858:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54861
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54859
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54860

lab54859:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54860:

lab54861:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54863

lab54862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54863:

lab54864:
    jmp lab54866

lab54865:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54866:

lab54868:
    ; #load tag
    lea rdi, [rel _Cont_54869]
    ; jump abs_i_
    jmp abs_i_

_Cont_54869:

_Cont_54869_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54871
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab54870
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54870:
    jmp lab54872

lab54871:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab54872:
    ; jump odd_abs_
    jmp odd_abs_

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: Bool = (n, a0, iters)\{ ... \};
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
    je lab54884
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54885

lab54884:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54882
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54875
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54873
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54874

lab54873:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54874:

lab54875:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54878
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54876
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54877

lab54876:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54877:

lab54878:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54880

lab54879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54880:

lab54881:
    jmp lab54883

lab54882:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54883:

lab54885:
    ; #load tag
    lea rdi, [rel Bool_54886]
    ; jump even_
    jmp even_

Bool_54886:
    jmp near Bool_54886_True
    jmp near Bool_54886_False

Bool_54886_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54888
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab54887
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54887:
    mov rdx, [rax + 24]
    jmp lab54889

lab54888:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab54889:
    ; let x0: Bool = True();
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

Bool_54886_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54891
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab54890
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54890:
    mov rdx, [rax + 24]
    jmp lab54892

lab54891:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab54892:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; substitute (n0 !-> n)(iters !-> iters)(n !-> n)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rdx, r9
    ; new a3: Bool = (iters, n, x0, a0)\{ ... \};
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
    je lab54904
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54905

lab54904:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54902
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54895
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54893
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54894

lab54893:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54894:

lab54895:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54898
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54896
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54897

lab54896:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54897:

lab54898:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54901
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54899
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54900

lab54899:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54900:

lab54901:
    jmp lab54903

lab54902:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54903:

lab54905:
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
    je lab54917
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54918

lab54917:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54915
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54908
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54906
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54907

lab54906:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54907:

lab54908:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54911
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54909
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54910

lab54909:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54910:

lab54911:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54914
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54912
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54913

lab54912:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54913:

lab54914:
    jmp lab54916

lab54915:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54916:

lab54918:
    ; #load tag
    lea rdi, [rel Bool_54919]
    ; new a4: Bool = (a3)\{ ... \};
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
    je lab54931
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab54932

lab54931:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54929
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54921

lab54920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54921:

lab54922:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54925
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54923
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54924

lab54923:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54924:

lab54925:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54928
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54926
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54927

lab54926:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54927:

lab54928:
    jmp lab54930

lab54929:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54930:

lab54932:
    ; #load tag
    lea rdi, [rel Bool_54933]
    ; jump odd_
    jmp odd_

Bool_54933:
    jmp near Bool_54933_True
    jmp near Bool_54933_False

Bool_54933_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54935
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab54934
    ; ####increment refcount
    add qword [rax + 0], 1

lab54934:
    jmp lab54936

lab54935:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab54936:
    ; let x2: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x2 !-> x2)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_54933_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54938
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab54937
    ; ####increment refcount
    add qword [rax + 0], 1

lab54937:
    jmp lab54939

lab54938:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab54939:
    ; let x2: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x2 !-> x2)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_54919:
    jmp near Bool_54919_True
    jmp near Bool_54919_False

Bool_54919_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54942
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
    je lab54940
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54940:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54941
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54941:
    mov rdi, [rsi + 24]
    jmp lab54943

lab54942:
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

lab54943:
    ; let x1: Bool = True();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0)(x1 !-> x1);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, r8
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

Bool_54919_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54946
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
    je lab54944
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54944:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54945
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54945:
    mov rdi, [rsi + 24]
    jmp lab54947

lab54946:
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

lab54947:
    ; let x1: Bool = False();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0)(x1 !-> x1);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, r8
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lift_main_loop_1_:
    ; substitute (x1 !-> x1)(x0 !-> x0)(n !-> n)(iters !-> iters)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rsi, r10
    ; new a5: Bool = (n, iters, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54959
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54960

lab54959:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54957
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54950
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54948
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54949

lab54948:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54949:

lab54950:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54953
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54951
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54952

lab54951:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54952:

lab54953:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54956
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54954
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54955

lab54954:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54955:

lab54956:
    jmp lab54958

lab54957:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54958:

lab54960:
    ; #load tag
    lea r9, [rel Bool_54961]
    ; substitute (x0 !-> x0)(x1 !-> x1)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

Bool_54961:
    jmp near Bool_54961_True
    jmp near Bool_54961_False

Bool_54961_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54963
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab54962
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54962:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab54964

lab54963:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab54964:
    ; let res: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

Bool_54961_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab54966
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab54965
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54965:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab54967

lab54966:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab54967:
    ; let res: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

lift_main_loop_2_:
    ; lit x3 <- 1;
    mov r13, 1
    ; if iters == x3 \{ ... \}
    cmp rdi, r13
    je lab54968
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab54971
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab54969
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab54970

lab54969:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab54970:

lab54971:
    ; lit x6 <- 1;
    mov r11, 1
    ; x7 <- iters - x6;
    mov r13, rdi
    sub r13, r11
    ; substitute (x7 !-> x7)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab54968:
    ; substitute (a0 !-> a0)(res !-> res);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
    ; switch res \{ ... \};
    lea rcx, [rel Bool_54972]
    add rcx, rdi
    jmp rcx

Bool_54972:
    jmp near Bool_54972_True
    jmp near Bool_54972_False

Bool_54972_True:
    ; lit x4 <- 1;
    mov rdi, 1
    ; println_i64 x4;
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
    ; lit x8 <- 0;
    mov rdi, 0
    ; substitute (x8 !-> x8)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Bool_54972_False:
    ; lit x5 <- 0;
    mov rdi, 0
    ; println_i64 x5;
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
    ; lit x9 <- 0;
    mov rdi, 0
    ; substitute (x9 !-> x9)(a0 !-> a0);
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