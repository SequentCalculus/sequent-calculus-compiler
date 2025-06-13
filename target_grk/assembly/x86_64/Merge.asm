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
    ; new x2: Fun[i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_i64_i64_62861]
    ; substitute (x2 !-> x2)(n0 !-> n)(n !-> n)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, r9
    mov r9, rdi
    mov rax, r8
    ; new a3: List[i64] = (n, iters)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
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
    je lab62873
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62874

lab62873:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62871
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62863

lab62862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62863:

lab62864:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62866

lab62865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62866:

lab62867:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62870
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62868
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62869

lab62868:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62869:

lab62870:
    jmp lab62872

lab62871:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62872:

lab62874:
    ; #load tag
    lea r9, [rel List_i64_62875]
    ; substitute (n0 !-> n0)(x2 !-> x2)(a3 !-> a3);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump tabulate_
    jmp tabulate_

List_i64_62875:
    jmp near List_i64_62875_Nil
    jmp near List_i64_62875_Cons

List_i64_62875_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62876
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    jmp lab62877

lab62876:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]

lab62877:
    ; let l1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (iters !-> iters)(l1 !-> l1)(n !-> n);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rsi, r8
    ; jump lift_main_0_
    jmp lift_main_0_

List_i64_62875_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62878
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab62879

lab62878:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab62879:
    ; substitute (iters !-> iters)(n !-> n)(a7 !-> a7)(as1 !-> as1);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    ; let l1: List[i64] = Cons(a7, as1);
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
    je lab62891
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62892

lab62891:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62889
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62882
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62880
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62881

lab62880:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62881:

lab62882:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62885
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62883
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62884

lab62883:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62884:

lab62885:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62888
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62886
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62887

lab62886:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62887:

lab62888:
    jmp lab62890

lab62889:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62890:

lab62892:
    ; #load tag
    mov r9, 5
    ; substitute (iters !-> iters)(l1 !-> l1)(n !-> n);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    ; jump lift_main_0_
    jmp lift_main_0_

Fun_i64_i64_62861:

Fun_i64_i64_62861_Apply:
    ; lit x7 <- 2;
    mov r9, 2
    ; x8 <- x7 * x;
    mov r11, r9
    imul r11, rdx
    ; substitute (x8 !-> x8)(a2 !-> a2);
    ; #move variables
    mov rdx, r11
    ; invoke a2 Ret
    jmp rdi

lift_main_0_:
    ; new x3: Fun[i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel Fun_i64_i64_62893]
    ; substitute (x3 !-> x3)(n0 !-> n)(n !-> n)(l1 !-> l1)(iters !-> iters);
    ; #move variables
    mov r13, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    mov rdi, r9
    ; new a4: List[i64] = (n, l1, iters)\{ ... \};
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
    je lab62905
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62906

lab62905:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62903
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62896
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62894
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62895

lab62894:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62895:

lab62896:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62899
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62897
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62898

lab62897:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62898:

lab62899:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62902
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62900
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62901

lab62900:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62901:

lab62902:
    jmp lab62904

lab62903:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62904:

lab62906:
    ; #load tag
    lea r9, [rel List_i64_62907]
    ; substitute (n0 !-> n0)(x3 !-> x3)(a4 !-> a4);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump tabulate_
    jmp tabulate_

List_i64_62907:
    jmp near List_i64_62907_Nil
    jmp near List_i64_62907_Cons

List_i64_62907_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62909
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab62908
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62908:
    mov rdx, [rax + 24]
    jmp lab62910

lab62909:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab62910:
    ; let l2: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (iters !-> iters)(l1 !-> l1)(l2 !-> l2)(n !-> n);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, r10
    ; jump lift_main_1_
    jmp lift_main_1_

List_i64_62907_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62912
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab62911
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62911:
    mov r9, [r8 + 24]
    jmp lab62913

lab62912:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab62913:
    ; substitute (iters !-> iters)(l1 !-> l1)(n !-> n)(a6 !-> a6)(as0 !-> as0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let l2: List[i64] = Cons(a6, as0);
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
    je lab62925
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab62926

lab62925:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62923
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62915

lab62914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62915:

lab62916:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62918

lab62917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62918:

lab62919:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62921

lab62920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62921:

lab62922:
    jmp lab62924

lab62923:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62924:

lab62926:
    ; #load tag
    mov r11, 5
    ; substitute (iters !-> iters)(l1 !-> l1)(l2 !-> l2)(n !-> n);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    mov r8, r10
    ; jump lift_main_1_
    jmp lift_main_1_

Fun_i64_i64_62893:

Fun_i64_i64_62893_Apply:
    ; lit x4 <- 2;
    mov r9, 2
    ; x5 <- x4 * x1;
    mov r11, r9
    imul r11, rdx
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x6 <- 1;
    mov r9, 1
    ; x9 <- x5 + x6;
    mov r11, rdx
    add r11, r9
    ; substitute (x9 !-> x9)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lift_main_1_:
    ; new a5: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    lea r13, [rel _Cont_62927]
    ; substitute (iters !-> iters)(n !-> n)(l1 !-> l1)(l2 !-> l2)(a5 !-> a5);
    ; #move variables
    mov r10, r8
    mov r8, rsi
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump main_loop_
    jmp main_loop_

_Cont_62927:

_Cont_62927_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

rev_loop_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_62928]
    add rcx, r9
    jmp rcx

List_i64_62928:
    jmp near List_i64_62928_Nil
    jmp near List_i64_62928_Cons

List_i64_62928_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_i64_62929]
    add rcx, rdi
    jmp rcx

List_i64_62929:
    jmp near List_i64_62929_Nil
    jmp near List_i64_62929_Cons

List_i64_62929_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_62929_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62931
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab62930
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62930:
    mov rdi, [rsi + 40]
    jmp lab62932

lab62931:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab62932:
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

List_i64_62928_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62934
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab62933
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62933:
    mov r9, [r8 + 40]
    jmp lab62935

lab62934:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab62935:
    ; substitute (a0 !-> a0)(ps !-> ps)(p !-> p)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[i64] = Cons(p, acc);
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
    je lab62947
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62948

lab62947:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62945
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62938
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62936
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62937

lab62936:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62937:

lab62938:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62941
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62939
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62940

lab62939:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62940:

lab62941:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62944
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62942
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62943

lab62942:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62943:

lab62944:
    jmp lab62946

lab62945:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62946:

lab62948:
    ; #load tag
    mov r9, 5
    ; substitute (ps !-> ps)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_loop_
    jmp rev_loop_

rev_:
    ; let x0: List[i64] = Nil();
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
    ; jump rev_loop_
    jmp rev_loop_

tabulate_loop_:
    ; if n < len \{ ... \}
    cmp rdx, rdi
    jl lab62949
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp r8, 0
    je lab62952
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab62950
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab62951

lab62950:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab62951:

lab62952:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    mov rsi, r12
    mov rdi, r13
    ; jump rev_
    jmp rev_

lab62949:
    ; lit x0 <- 1;
    mov r15, 1
    ; x1 <- n + x0;
    mov rcx, rdx
    add rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (n !-> n)(f0 !-> f)(f !-> f)(acc !-> acc)(a0 !-> a0)(len !-> len)(x1 !-> x1);
    ; #share f
    cmp r8, 0
    je lab62953
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62953:
    ; #move variables
    mov r15, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a2: _Cont = (f, acc, a0, len, x1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab62965
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab62966

lab62965:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62963
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62956
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62954
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62955

lab62954:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62955:

lab62956:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62959
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62957
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62958

lab62957:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62958:

lab62959:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62961

lab62960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62961:

lab62962:
    jmp lab62964

lab62963:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62964:

lab62966:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab62978
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62979

lab62978:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62976
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62969
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62967
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62968

lab62967:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62968:

lab62969:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62972
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62970
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62971

lab62970:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62971:

lab62972:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62975
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62973
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62974

lab62973:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62974:

lab62975:
    jmp lab62977

lab62976:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62977:

lab62979:
    ; #load tag
    lea r9, [rel _Cont_62980]
    ; substitute (n !-> n)(a2 !-> a2)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_62980:

_Cont_62980_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62984
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab62981
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62981:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab62982
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62982:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab62983
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62983:
    jmp lab62985

lab62984:
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
    mov rsi, [rsi + 16]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab62985:
    ; substitute (x1 !-> x1)(f !-> f)(len !-> len)(a0 !-> a0)(x3 !-> x3)(acc !-> acc);
    ; #move variables
    mov rcx, r15
    mov r15, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    mov r14, r8
    ; let x2: List[i64] = Cons(x3, acc);
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
    je lab62997
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab62998

lab62997:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62995
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62988
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62986
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62987

lab62986:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62987:

lab62988:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62991
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62989
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62990

lab62989:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62990:

lab62991:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62993

lab62992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62993:

lab62994:
    jmp lab62996

lab62995:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62996:

lab62998:
    ; #load tag
    mov r13, 5
    ; substitute (x1 !-> x1)(len !-> len)(f !-> f)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump tabulate_loop_
    jmp tabulate_loop_

tabulate_:
    ; lit x0 <- 0;
    mov r11, 0
    ; if n < x0 \{ ... \}
    cmp rdx, r11
    jl lab62999
    ; substitute (n !-> n)(f !-> f)(a0 !-> a0);
    ; lit x1 <- 0;
    mov r11, 0
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (x1 !-> x1)(n !-> n)(f !-> f)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, r12
    mov r12, r8
    mov r8, rsi
    ; jump tabulate_loop_
    jmp tabulate_loop_

lab62999:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab63002
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63000
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63001

lab63000:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63001:

lab63002:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

merge_:
    ; substitute (l1 !-> l1)(l2 !-> l2)(a0 !-> a0)(l10 !-> l1);
    ; #share l1
    cmp rax, 0
    je lab63003
    ; ####increment refcount
    add qword [rax + 0], 1

lab63003:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; switch l10 \{ ... \};
    lea rcx, [rel List_i64_63004]
    add rcx, r11
    jmp rcx

List_i64_63004:
    jmp near List_i64_63004_Nil
    jmp near List_i64_63004_Cons

List_i64_63004_Nil:
    ; substitute (a0 !-> a0)(l2 !-> l2);
    ; #erase l1
    cmp rax, 0
    je lab63007
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab63005
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab63006

lab63005:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab63006:

lab63007:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch l2 \{ ... \};
    lea rcx, [rel List_i64_63008]
    add rcx, rdi
    jmp rcx

List_i64_63008:
    jmp near List_i64_63008_Nil
    jmp near List_i64_63008_Cons

List_i64_63008_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63008_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63010
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63009
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63009:
    mov rdi, [rsi + 40]
    jmp lab63011

lab63010:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63011:
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

List_i64_63004_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab63013
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab63012
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63012:
    mov r11, [r10 + 40]
    jmp lab63014

lab63013:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab63014:
    ; substitute (l1 !-> l1)(l2 !-> l2)(a0 !-> a0)(x1 !-> x1)(xs1 !-> xs1)(l20 !-> l2);
    ; #share l2
    cmp rsi, 0
    je lab63015
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63015:
    ; #move variables
    mov r14, rsi
    mov r15, rdi
    ; switch l20 \{ ... \};
    lea rcx, [rel List_i64_63016]
    add rcx, r15
    jmp rcx

List_i64_63016:
    jmp near List_i64_63016_Nil
    jmp near List_i64_63016_Cons

List_i64_63016_Nil:
    ; substitute (a0 !-> a0)(l1 !-> l1);
    ; #erase l2
    cmp rsi, 0
    je lab63019
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63017
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63018

lab63017:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63018:

lab63019:
    ; #erase xs1
    cmp r12, 0
    je lab63022
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab63020
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab63021

lab63020:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab63021:

lab63022:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; switch l1 \{ ... \};
    lea rcx, [rel List_i64_63023]
    add rcx, rdi
    jmp rcx

List_i64_63023:
    jmp near List_i64_63023_Nil
    jmp near List_i64_63023_Cons

List_i64_63023_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63023_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63025
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63024
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63024:
    mov rdi, [rsi + 40]
    jmp lab63026

lab63025:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63026:
    ; substitute (a4 !-> a4)(as1 !-> as1)(a0 !-> a0);
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

List_i64_63016_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab63028
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab63027
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63027:
    mov r15, [r14 + 40]
    jmp lab63029

lab63028:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 40]

lab63029:
    ; if x1 <= x2 \{ ... \}
    cmp r11, r15
    jle lab63030
    ; substitute (l1 !-> l1)(xs2 !-> xs2)(a0 !-> a0)(x2 !-> x2);
    ; #erase l2
    cmp rsi, 0
    je lab63033
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63031
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63032

lab63031:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63032:

lab63033:
    ; #erase xs1
    cmp r12, 0
    je lab63036
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab63034
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab63035

lab63034:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab63035:

lab63036:
    ; #move variables
    mov r11, r15
    mov rsi, [rsp + 2032]
    mov rdi, [rsp + 2024]
    ; new a2: List[i64] = (a0, x2)\{ ... \};
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
    je lab63048
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63049

lab63048:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63046
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63039
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63037
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63038

lab63037:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63038:

lab63039:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63042
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63040
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63041

lab63040:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63041:

lab63042:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63045
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63043
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63044

lab63043:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63044:

lab63045:
    jmp lab63047

lab63046:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63047:

lab63049:
    ; #load tag
    lea r9, [rel List_i64_63050]
    ; jump merge_
    jmp merge_

List_i64_63050:
    jmp near List_i64_63050_Nil
    jmp near List_i64_63050_Cons

List_i64_63050_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63052
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63051
    ; ####increment refcount
    add qword [rax + 0], 1

lab63051:
    jmp lab63053

lab63052:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63053:
    ; let x3: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x3 !-> x3)(a0 !-> a0);
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

List_i64_63050_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63055
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63054
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63054:
    jmp lab63056

lab63055:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63056:
    ; substitute (x2 !-> x2)(a0 !-> a0)(a6 !-> a6)(as3 !-> as3);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x3: List[i64] = Cons(a6, as3);
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
    je lab63068
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63069

lab63068:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63066
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63058

lab63057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63058:

lab63059:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63061

lab63060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63061:

lab63062:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63065
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63063
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63064

lab63063:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63064:

lab63065:
    jmp lab63067

lab63066:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63067:

lab63069:
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x3 !-> x3)(a0 !-> a0);
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

lab63030:
    ; substitute (xs1 !-> xs1)(l2 !-> l2)(a0 !-> a0)(x1 !-> x1);
    ; #erase l1
    cmp rax, 0
    je lab63072
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab63070
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab63071

lab63070:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab63071:

lab63072:
    ; #erase xs2
    cmp qword [rsp + 2032], 0
    je lab63075
    ; ######check refcount
    mov rcx, [rsp + 2032]
    cmp qword [rcx + 0], 0
    je lab63073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63074

lab63073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63074:

lab63075:
    ; #move variables
    mov rax, r12
    mov rdx, r13
    ; new a1: List[i64] = (a0, x1)\{ ... \};
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
    je lab63087
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63088

lab63087:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63085
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63078
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63076
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63077

lab63076:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63077:

lab63078:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63081
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63079
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63080

lab63079:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63080:

lab63081:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63083

lab63082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63083:

lab63084:
    jmp lab63086

lab63085:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63086:

lab63088:
    ; #load tag
    lea r9, [rel List_i64_63089]
    ; jump merge_
    jmp merge_

List_i64_63089:
    jmp near List_i64_63089_Nil
    jmp near List_i64_63089_Cons

List_i64_63089_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63091
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63090
    ; ####increment refcount
    add qword [rax + 0], 1

lab63090:
    jmp lab63092

lab63091:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63092:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(x0 !-> x0)(a0 !-> a0);
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

List_i64_63089_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63094
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63093
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63093:
    jmp lab63095

lab63094:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63095:
    ; substitute (x1 !-> x1)(a0 !-> a0)(a5 !-> a5)(as2 !-> as2);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x0: List[i64] = Cons(a5, as2);
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
    je lab63107
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63108

lab63107:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63105
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63098
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63096
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63097

lab63096:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63097:

lab63098:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63101
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63099
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63100

lab63099:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63100:

lab63101:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63104
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63102
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63103

lab63102:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63103:

lab63104:
    jmp lab63106

lab63105:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63106:

lab63108:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x0 !-> x0)(a0 !-> a0);
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

head_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_63109]
    add rcx, rdi
    jmp rcx

List_i64_63109:
    jmp near List_i64_63109_Nil
    jmp near List_i64_63109_Cons

List_i64_63109_Nil:
    ; lit x0 <- -1;
    mov rdi, -1
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_63109_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63111
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63110
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63110:
    mov rdi, [rsi + 40]
    jmp lab63112

lab63111:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63112:
    ; substitute (x !-> x)(a0 !-> a0);
    ; #erase xs
    cmp r8, 0
    je lab63115
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab63113
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab63114

lab63113:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab63114:

lab63115:
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (l20 !-> l2)(l10 !-> l1)(l1 !-> l1)(l2 !-> l2)(a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #share l1
    cmp r8, 0
    je lab63116
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63116:
    ; #share l2
    cmp r10, 0
    je lab63117
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63117:
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov rsi, r8
    mov rdi, r9
    mov rax, r10
    mov rdx, r11
    ; new a3: List[i64] = (l1, l2, a0, iters, n)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63129
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab63130

lab63129:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63127
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63120
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63118
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63119

lab63118:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63119:

lab63120:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63123
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63121
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63122

lab63121:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63122:

lab63123:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63126
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63124
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63125

lab63124:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63125:

lab63126:
    jmp lab63128

lab63127:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63128:

lab63130:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab63142
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63143

lab63142:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63140
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63133
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63131
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63132

lab63131:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63132:

lab63133:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63136
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63134
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63135

lab63134:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63135:

lab63136:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63139
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63137
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63138

lab63137:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63138:

lab63139:
    jmp lab63141

lab63140:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63141:

lab63143:
    ; #load tag
    lea r9, [rel List_i64_63144]
    ; substitute (l10 !-> l10)(l20 !-> l20)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump merge_
    jmp merge_

List_i64_63144:
    jmp near List_i64_63144_Nil
    jmp near List_i64_63144_Cons

List_i64_63144_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63148
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab63145
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63145:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab63146
    ; ####increment refcount
    add qword [rax + 0], 1

lab63146:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63147
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63147:
    jmp lab63149

lab63148:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab63149:
    ; let res: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(l1 !-> l1)(l2 !-> l2)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_i64_63144_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63153
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab63150
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63150:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63151
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63151:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab63152
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63152:
    jmp lab63154

lab63153:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab63154:
    ; substitute (n !-> n)(iters !-> iters)(l1 !-> l1)(l2 !-> l2)(a0 !-> a0)(a4 !-> a4)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    ; let res: List[i64] = Cons(a4, as0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63166
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab63167

lab63166:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63164
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63156

lab63155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63156:

lab63157:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63159

lab63158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63159:

lab63160:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63163
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63161
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63162

lab63161:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63162:

lab63163:
    jmp lab63165

lab63164:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63165:

lab63167:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(l1 !-> l1)(l2 !-> l2)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r12
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp rdi, [rsp +2024]
    je lab63168
    ; substitute (a0 !-> a0)(iters !-> iters)(l1 !-> l1)(l2 !-> l2)(n !-> n);
    ; #erase res
    cmp r14, 0
    je lab63171
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab63169
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab63170

lab63169:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab63170:

lab63171:
    ; lit x2 <- 1;
    mov r15, 1
    ; x3 <- iters - x2;
    mov rcx, rdi
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x3 !-> x3)(n !-> n)(l1 !-> l1)(l2 !-> l2)(a0 !-> a0);
    ; #move variables
    mov r12, rax
    mov rdi, r13
    mov r13, rdx
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab63168:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #erase l1
    cmp r8, 0
    je lab63174
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab63172
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab63173

lab63172:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab63173:

lab63174:
    ; #erase l2
    cmp r10, 0
    je lab63177
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab63175
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab63176

lab63175:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab63176:

lab63177:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r14
    mov rdx, r15
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
    je lab63189
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab63190

lab63189:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63187
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63180
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63178
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63179

lab63178:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63179:

lab63180:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63183
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63181
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63182

lab63181:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63182:

lab63183:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63185

lab63184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63185:

lab63186:
    jmp lab63188

lab63187:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63188:

lab63190:
    ; #load tag
    lea rdi, [rel _Cont_63191]
    ; jump head_
    jmp head_

_Cont_63191:

_Cont_63191_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63193
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab63192
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63192:
    jmp lab63194

lab63193:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab63194:
    ; println_i64 x1;
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