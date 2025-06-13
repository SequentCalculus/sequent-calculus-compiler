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
    lea rdx, [rel _Cont_865]
    ; lit x2 <- 2;
    mov rdi, 2
    ; lit x3 <- 0;
    mov r9, 0
    ; lit x4 <- 3;
    mov r11, 3
    ; lit x5 <- 3;
    mov r13, 3
    ; let x6: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x7: List[i64] = Cons(x5, x6);
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
    je lab877
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab878

lab877:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab875
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab867

lab866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab867:

lab868:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab870

lab869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab870:

lab871:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab874
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab872
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab873

lab872:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab873:

lab874:
    jmp lab876

lab875:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab876:

lab878:
    ; #load tag
    mov r13, 5
    ; let x8: List[i64] = Cons(x4, x7);
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
    je lab890
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab891

lab890:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab888
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab880

lab879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab880:

lab881:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab883

lab882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab883:

lab884:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab886

lab885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab886:

lab887:
    jmp lab889

lab888:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab889:

lab891:
    ; #load tag
    mov r11, 5
    ; let x9: List[i64] = Cons(x3, x8);
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
    je lab903
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab904

lab903:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab901
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab894
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab892
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab893

lab892:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab893:

lab894:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab896

lab895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab896:

lab897:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab899

lab898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab899:

lab900:
    jmp lab902

lab901:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab902:

lab904:
    ; #load tag
    mov r9, 5
    ; let x10: List[i64] = Cons(x2, x9);
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
    je lab916
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab917

lab916:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab914
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab907
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab905
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab906

lab905:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab906:

lab907:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab910
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab908
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab909

lab908:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab909:

lab910:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab912

lab911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab912:

lab913:
    jmp lab915

lab914:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab915:

lab917:
    ; #load tag
    mov rdi, 5
    ; substitute (x10 !-> x10)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump fmult_
    jmp fmult_

_Cont_865:

_Cont_865_Ret:
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

fmult_:
    ; substitute (l !-> l)(a0 !-> a0)(a00 !-> a0);
    ; #share a0
    cmp rsi, 0
    je lab918
    ; ####increment refcount
    add qword [rsi + 0], 1

lab918:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    ; jump mult_
    jmp mult_

mult_:
    ; substitute (a0 !-> a0)(a !-> a)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_919]
    add rcx, r9
    jmp rcx

List_i64_919:
    jmp near List_i64_919_Nil
    jmp near List_i64_919_Cons

List_i64_919_Nil:
    ; substitute (a0 !-> a0);
    ; #erase a
    cmp rsi, 0
    je lab922
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab920
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab921

lab920:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab921:

lab922:
    ; lit x0 <- -12;
    mov rdi, -12
    ; print_i64 x0;
    ; #save caller-save registers
    mov r12, rax
    mov r13, rdx
    mov r14, rdi
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdi
    call print_i64
    ; #restore caller-save registers
    mov rax, r12
    mov rdx, r13
    mov rdi, r14
    add rsp, 8
    ; substitute (a0 !-> a0);
    ; lit x1 <- 21;
    mov rdi, 21
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
    ; lit x5 <- 1;
    mov rdi, 1
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_919_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab924
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab923
    ; ####increment refcount
    add qword [r10 + 0], 1

lab923:
    mov r9, [r8 + 40]
    jmp lab925

lab924:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab925:
    ; lit x2 <- -24;
    mov r13, -24
    ; print_i64 x2;
    ; #save caller-save registers
    mov r14, rax
    mov r15, rdx
    push rsi
    push rdi
    push r9
    push r10
    push r11
    ; #move argument into place
    mov rdi, r13
    call print_i64
    ; #restore caller-save registers
    mov rax, r14
    mov rdx, r15
    pop r11
    pop r10
    pop r9
    pop rdi
    pop rsi
    ; substitute (a0 !-> a0)(a !-> a)(x !-> x)(xs !-> xs);
    ; lit x3 <- 42;
    mov r13, 42
    ; println_i64 x3;
    ; #save caller-save registers
    mov r14, rax
    mov r15, rdx
    push rsi
    push rdi
    push r9
    push r10
    push r11
    ; #move argument into place
    mov rdi, r13
    call println_i64
    ; #restore caller-save registers
    mov rax, r14
    mov rdx, r15
    pop r11
    pop r10
    pop r9
    pop rdi
    pop rsi
    ; if x == 0 \{ ... \}
    cmp r9, 0
    je lab926
    ; substitute (xs !-> xs)(a !-> a)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; create a1: _Cont = (x, a0)\{ ... \};
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
    je lab938
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab939

lab938:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab936
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab928

lab927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab928:

lab929:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab932
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab930
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab931

lab930:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab931:

lab932:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab935
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab933
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab934

lab933:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab934:

lab935:
    jmp lab937

lab936:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab937:

lab939:
    ; #load tag
    lea r9, [rel _Cont_940]
    ; jump mult_
    jmp mult_

_Cont_940:

_Cont_940_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab942
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab941
    ; ####increment refcount
    add qword [r8 + 0], 1

lab941:
    mov rdi, [rsi + 40]
    jmp lab943

lab942:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab943:
    ; x7 <- x * x4;
    mov r11, rdi
    imul r11, rdx
    ; substitute (x7 !-> x7)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab926:
    ; substitute (a !-> a);
    ; #erase a0
    cmp rax, 0
    je lab946
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab944
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab945

lab944:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab945:

lab946:
    ; #erase xs
    cmp r10, 0
    je lab949
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab947
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab948

lab947:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab948:

lab949:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x6 <- 0;
    mov rdi, 0
    ; substitute (x6 !-> x6)(a !-> a);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a Ret
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