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
    lea r9, [rel _Cont_39751]
    ; jump main_loop_
    jmp main_loop_

_Cont_39751:

_Cont_39751_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

eq_:
    ; if i1 == i2 \{ ... \}
    cmp rdx, rdi
    je lab39752
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab39752:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

expand_:
    ; lit x0 <- 10;
    mov qword [rsp + 2008], 10
    ; x1 <- e * x0;
    mov rcx, r13
    imul rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    ; substitute (a !-> a)(b !-> b)(c !-> c)(d !-> d)(x1 !-> x1)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov r13, [rsp + 1992]
    ; x2 <- f + x1;
    mov rcx, r15
    add rcx, r13
    mov [rsp + 2008], rcx
    ; substitute (a !-> a)(b !-> b)(c !-> c)(d !-> d)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov r14, [rsp + 2032]
    mov r15, [rsp + 2024]
    mov r13, [rsp + 2008]
    ; lit x3 <- 100;
    mov qword [rsp + 2024], 100
    ; x4 <- d * x3;
    mov rcx, r11
    imul rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (a !-> a)(b !-> b)(c !-> c)(x4 !-> x4)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov r11, [rsp + 2008]
    ; x5 <- x2 + x4;
    mov rcx, r13
    add rcx, r11
    mov [rsp + 2024], rcx
    ; substitute (a !-> a)(b !-> b)(c !-> c)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov r12, r14
    mov r13, r15
    mov r11, [rsp + 2024]
    ; lit x6 <- 1000;
    mov r15, 1000
    ; x7 <- c * x6;
    mov rcx, r9
    imul rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (a !-> a)(b !-> b)(x7 !-> x7)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov r9, [rsp + 2024]
    ; x8 <- x5 + x7;
    mov r15, r11
    add r15, r9
    ; substitute (a !-> a)(b !-> b)(x8 !-> x8)(a0 !-> a0);
    ; #move variables
    mov r10, r12
    mov r11, r13
    mov r9, r15
    ; lit x9 <- 10000;
    mov r13, 10000
    ; x10 <- b * x9;
    mov r15, rdi
    imul r15, r13
    ; substitute (a !-> a)(x10 !-> x10)(x8 !-> x8)(a0 !-> a0);
    ; #move variables
    mov rdi, r15
    ; x11 <- x8 + x10;
    mov r13, r9
    add r13, rdi
    ; substitute (a !-> a)(x11 !-> x11)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r9, r11
    mov rdi, r13
    ; lit x12 <- 100000;
    mov r11, 100000
    ; x13 <- a * x12;
    mov r13, rdx
    imul r13, r11
    ; substitute (x13 !-> x13)(x11 !-> x11)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; x14 <- x11 + x13;
    mov r11, rdi
    add r11, rdx
    ; substitute (x14 !-> x14)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

condition_:
    ; substitute (a0 !-> a0)(thirywelvn !-> thirywelvn);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch thirywelvn \{ ... \};
    lea rcx, [rel List_i64_39753]
    add rcx, rdi
    jmp rcx

List_i64_39753:
    jmp near List_i64_39753_Nil
    jmp near List_i64_39753_Cons

List_i64_39753_Nil:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39753_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab39755
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab39754
    ; ####increment refcount
    add qword [r8 + 0], 1

lab39754:
    mov rdi, [rsi + 40]
    jmp lab39756

lab39755:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab39756:
    ; switch ts \{ ... \};
    lea rcx, [rel List_i64_39757]
    add rcx, r9
    jmp rcx

List_i64_39757:
    jmp near List_i64_39757_Nil
    jmp near List_i64_39757_Cons

List_i64_39757_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39757_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab39759
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab39758
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39758:
    mov r9, [r8 + 40]
    jmp lab39760

lab39759:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab39760:
    ; switch hs \{ ... \};
    lea rcx, [rel List_i64_39761]
    add rcx, r11
    jmp rcx

List_i64_39761:
    jmp near List_i64_39761_Nil
    jmp near List_i64_39761_Cons

List_i64_39761_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39761_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab39763
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab39762
    ; ####increment refcount
    add qword [r12 + 0], 1

lab39762:
    mov r11, [r10 + 40]
    jmp lab39764

lab39763:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab39764:
    ; switch is \{ ... \};
    lea rcx, [rel List_i64_39765]
    add rcx, r13
    jmp rcx

List_i64_39765:
    jmp near List_i64_39765_Nil
    jmp near List_i64_39765_Cons

List_i64_39765_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39765_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab39767
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    cmp r14, 0
    je lab39766
    ; ####increment refcount
    add qword [r14 + 0], 1

lab39766:
    mov r13, [r12 + 40]
    jmp lab39768

lab39767:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    mov r13, [r12 + 40]

lab39768:
    ; switch rs \{ ... \};
    lea rcx, [rel List_i64_39769]
    add rcx, r15
    jmp rcx

List_i64_39769:
    jmp near List_i64_39769_Nil
    jmp near List_i64_39769_Cons

List_i64_39769_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39769_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab39771
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab39770
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39770:
    mov r15, [r14 + 40]
    jmp lab39772

lab39771:
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

lab39772:
    ; switch ys \{ ... \};
    lea rcx, [rel List_i64_39773]
    add rcx, [rsp + 2024]
    jmp rcx

List_i64_39773:
    jmp near List_i64_39773_Nil
    jmp near List_i64_39773_Cons

List_i64_39773_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39773_Cons:
    ; #load from memory
    mov rcx, [rsp + 2032]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39775
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab39774
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39774:
    mov rcx, [rax + 40]
    mov [rsp + 2024], rcx
    mov rax, [rsp + 2040]
    jmp lab39776

lab39775:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2016], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2024], rcx
    mov rax, [rsp + 2040]

lab39776:
    ; switch ws \{ ... \};
    lea rcx, [rel List_i64_39777]
    add rcx, [rsp + 2008]
    jmp rcx

List_i64_39777:
    jmp near List_i64_39777_Nil
    jmp near List_i64_39777_Cons

List_i64_39777_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39777_Cons:
    ; #load from memory
    mov rcx, [rsp + 2016]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39779
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab39778
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39778:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]
    jmp lab39780

lab39779:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]

lab39780:
    ; switch es \{ ... \};
    lea rcx, [rel List_i64_39781]
    add rcx, [rsp + 1992]
    jmp rcx

List_i64_39781:
    jmp near List_i64_39781_Nil
    jmp near List_i64_39781_Cons

List_i64_39781_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39781_Cons:
    ; #load from memory
    mov rcx, [rsp + 2000]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39783
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 2000]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab39782
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39782:
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rax, [rsp + 2040]
    jmp lab39784

lab39783:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rax, [rsp + 2040]

lab39784:
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_39785]
    add rcx, [rsp + 1976]
    jmp rcx

List_i64_39785:
    jmp near List_i64_39785_Nil
    jmp near List_i64_39785_Cons

List_i64_39785_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39785_Cons:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39787
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    cmp rcx, 0
    je lab39786
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39786:
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]
    jmp lab39788

lab39787:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]

lab39788:
    ; switch vs \{ ... \};
    lea rcx, [rel List_i64_39789]
    add rcx, [rsp + 1960]
    jmp rcx

List_i64_39789:
    jmp near List_i64_39789_Nil
    jmp near List_i64_39789_Cons

List_i64_39789_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_39789_Cons:
    ; #load from memory
    mov rcx, [rsp + 1968]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39791
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1968]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1952], rcx
    cmp rcx, 0
    je lab39790
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39790:
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rax, [rsp + 2040]
    jmp lab39792

lab39791:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 1968]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1952], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rax, [rsp + 2040]

lab39792:
    ; switch ns \{ ... \};
    lea rcx, [rel List_i64_39793]
    add rcx, [rsp + 1944]
    jmp rcx

List_i64_39793:
    jmp near List_i64_39793_Nil
    jmp near List_i64_39793_Cons

List_i64_39793_Nil:
    ; substitute (y0 !-> y)(t1 !-> t)(h !-> h)(i1 !-> i)(r !-> r)(y !-> y)(w !-> w)(e !-> e)(l !-> l)(v !-> v)(n !-> n)(a0 !-> a0)(t !-> t)(i !-> i);
    ; #move variables
    mov [rsp + 1952], rax
    mov [rsp + 1944], rdx
    mov [rsp + 1928], rdi
    mov [rsp + 1912], r11
    mov rdx, r15
    ; new a2: _Cont = (y, w, e, l, v, n, a0, t, i)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1912]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1928]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39805
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39806

lab39805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39795

lab39794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39795:

lab39796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39798

lab39797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39798:

lab39799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39801

lab39800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39801:

lab39802:
    jmp lab39804

lab39803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39804:

lab39806:
    ; ##store link to previous block
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39818
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39819

lab39818:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39816
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39809
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39807
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39808

lab39807:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39808:

lab39809:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39812
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39810
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39811

lab39810:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39811:

lab39812:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39815
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39813
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39814

lab39813:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39814:

lab39815:
    jmp lab39817

lab39816:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39817:

lab39819:
    ; ##store link to previous block
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39831
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39832

lab39831:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39829
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39822
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39820
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39821

lab39820:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39821:

lab39822:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39825
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39823
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39824

lab39823:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39824:

lab39825:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39827

lab39826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39827:

lab39828:
    jmp lab39830

lab39829:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39830:

lab39832:
    ; ##store link to previous block
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab39844
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab39845

lab39844:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39842
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39835
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39833
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39834

lab39833:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39834:

lab39835:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39837

lab39836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39837:

lab39838:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39840

lab39839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39840:

lab39841:
    jmp lab39843

lab39842:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39843:

lab39845:
    ; #load tag
    lea r15, [rel _Cont_39846]
    ; substitute (t1 !-> t1)(h !-> h)(i1 !-> i1)(r !-> r)(t10 !-> t1)(y0 !-> y0)(a2 !-> a2);
    ; #move variables
    mov [rsp + 2024], r15
    mov r15, rdx
    mov rdx, rdi
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    mov [rsp + 2032], r14
    ; jump expand_
    jmp expand_

_Cont_39846:

_Cont_39846_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab39848
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab39847
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39847:
    mov rax, [rsp + 2040]
    jmp lab39849

lab39848:
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
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab39849:
    ; lit x1 <- 5;
    mov qword [rsp + 1960], 5
    ; substitute (t0 !-> t)(v !-> v)(w !-> w)(e0 !-> e)(l !-> l)(e !-> e)(n !-> n)(a0 !-> a0)(t !-> t)(i !-> i)(x1 !-> x1)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov [rsp + 1944], rdx
    mov [rsp + 1928], rdi
    mov rdi, r15
    mov r15, r11
    mov rdx, [rsp + 1992]
    ; new a4: _Cont = (e, n, a0, t, i, x1, x0, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39861
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39862

lab39861:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39859
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39852
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39850
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39851

lab39850:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39851:

lab39852:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39855
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39853
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39854

lab39853:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39854:

lab39855:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39858
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39856
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39857

lab39856:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39857:

lab39858:
    jmp lab39860

lab39859:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39860:

lab39862:
    ; ##store link to previous block
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39874
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39875

lab39874:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39872
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39864

lab39863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39864:

lab39865:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39867

lab39866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39867:

lab39868:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39870

lab39869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39870:

lab39871:
    jmp lab39873

lab39872:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39873:

lab39875:
    ; ##store link to previous block
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab39887
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab39888

lab39887:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39885
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39878
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39876
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39877

lab39876:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39877:

lab39878:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39880

lab39879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39880:

lab39881:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39883

lab39882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39883:

lab39884:
    jmp lab39886

lab39885:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39886:

lab39888:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab39900
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab39901

lab39900:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39898
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39891
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39889
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39890

lab39889:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39890:

lab39891:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39894
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39892
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39893

lab39892:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39893:

lab39894:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39896

lab39895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39896:

lab39897:
    jmp lab39899

lab39898:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39899:

lab39901:
    ; #load tag
    lea r15, [rel _Cont_39902]
    ; substitute (t0 !-> t0)(w !-> w)(e0 !-> e0)(l !-> l)(v !-> v)(e00 !-> e0)(a4 !-> a4);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov [rsp + 2024], r15
    mov r15, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    mov [rsp + 2032], r14
    ; jump expand_
    jmp expand_

_Cont_39902:

_Cont_39902_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab39904
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab39903
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39903:
    mov r9, [r8 + 24]
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rax, [rsp + 2040]
    jmp lab39905

lab39904:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rax, [rsp + 2040]

lab39905:
    ; x3 <- x1 * x2;
    mov rcx, [rsp + 2024]
    imul rcx, rdx
    mov [rsp + 1976], rcx
    ; substitute (x3 !-> x3)(e !-> e)(n !-> n)(a0 !-> a0)(t !-> t)(i !-> i)(y !-> y)(x0 !-> x0);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 2024], rcx
    mov rdx, [rsp + 1976]
    ; x4 <- x0 + x3;
    mov rcx, [rsp + 2008]
    add rcx, rdx
    mov [rsp + 1992], rcx
    ; substitute (y !-> y)(e !-> e)(n !-> n)(i !-> i)(t !-> t)(a0 !-> a0)(x4 !-> x4);
    ; #move variables
    mov r14, r10
    mov rcx, r15
    mov r15, r11
    mov r11, rcx
    mov rdx, [rsp + 2024]
    mov rcx, [rsp + 1992]
    mov [rsp + 2024], rcx
    ; new a5: _Cont = (a0, x4)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab39917
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab39918

lab39917:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39915
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39908
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39906
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39907

lab39906:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39907:

lab39908:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39911
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39909
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39910

lab39909:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39910:

lab39911:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39914
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39912
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39913

lab39912:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39913:

lab39914:
    jmp lab39916

lab39915:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39916:

lab39918:
    ; #load tag
    lea r15, [rel _Cont_39919]
    ; substitute (n !-> n)(i !-> i)(n0 !-> n)(e !-> e)(t !-> t)(y !-> y)(a5 !-> a5);
    ; #move variables
    mov [rsp + 2024], r15
    mov r15, rdx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rdx, r9
    mov [rsp + 2032], r14
    ; jump expand_
    jmp expand_

_Cont_39919:

_Cont_39919_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab39921
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab39920
    ; ####increment refcount
    add qword [rsi + 0], 1

lab39920:
    jmp lab39922

lab39921:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab39922:
    ; substitute (x4 !-> x4)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump eq_
    jmp eq_

List_i64_39793_Cons:
    ; #load from memory
    mov rcx, [rsp + 1952]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab39924
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 1952]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1936], rcx
    cmp rcx, 0
    je lab39923
    ; ####increment refcount
    add qword [rcx + 0], 1

lab39923:
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rax, [rsp + 2040]
    jmp lab39925

lab39924:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 1952]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1936], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rax, [rsp + 2040]

lab39925:
    ; substitute (a0 !-> a0);
    ; #erase is0
    cmp qword [rsp + 1936], 0
    je lab39928
    ; ######check refcount
    mov rcx, [rsp + 1936]
    cmp qword [rcx + 0], 0
    je lab39926
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39927

lab39926:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39927:

lab39928:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

add_lscomp_:
    ; substitute (a0 !-> a0)(k !-> k)(p1 !-> p1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch p1 \{ ... \};
    lea rcx, [rel List_List_i64_39929]
    add rcx, r9
    jmp rcx

List_List_i64_39929:
    jmp near List_List_i64_39929_Nil
    jmp near List_List_i64_39929_Cons

List_List_i64_39929_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_39929_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab39932
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab39930
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39930:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab39931
    ; ####increment refcount
    add qword [r8 + 0], 1

lab39931:
    jmp lab39933

lab39932:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab39933:
    ; substitute (a0 !-> a0)(k !-> k)(t1 !-> t1)(k0 !-> k)(h1 !-> h1);
    ; #move variables
    mov r13, r9
    mov r9, r11
    mov r11, rdi
    mov r12, r8
    mov r8, r10
    ; let x0: List[i64] = Cons(k0, h1);
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
    je lab39945
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab39946

lab39945:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39943
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39936
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39934
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39935

lab39934:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39935:

lab39936:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39939
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39938

lab39937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39938:

lab39939:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39941

lab39940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39941:

lab39942:
    jmp lab39944

lab39943:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39944:

lab39946:
    ; #load tag
    mov r11, 5
    ; substitute (t1 !-> t1)(k !-> k)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[List[i64]] = (a0, x0)\{ ... \};
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
    je lab39958
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab39959

lab39958:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39956
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39949
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39947
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39948

lab39947:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39948:

lab39949:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39952
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39950
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39951

lab39950:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39951:

lab39952:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39954

lab39953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39954:

lab39955:
    jmp lab39957

lab39956:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39957:

lab39959:
    ; #load tag
    lea r9, [rel List_List_i64_39960]
    ; jump add_lscomp_
    jmp add_lscomp_

List_List_i64_39960:
    jmp near List_List_i64_39960_Nil
    jmp near List_List_i64_39960_Cons

List_List_i64_39960_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab39963
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab39961
    ; ####increment refcount
    add qword [rsi + 0], 1

lab39961:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab39962
    ; ####increment refcount
    add qword [rax + 0], 1

lab39962:
    jmp lab39964

lab39963:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab39964:
    ; let x1: List[List[i64]] = Nil();
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
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_List_i64_39960_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab39967
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab39965
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39965:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab39966
    ; ####increment refcount
    add qword [r8 + 0], 1

lab39966:
    jmp lab39968

lab39967:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab39968:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a2 !-> a2)(as0 !-> as0);
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
    ; let x1: List[List[i64]] = Cons(a2, as0);
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
    je lab39980
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab39981

lab39980:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39978
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39970

lab39969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39970:

lab39971:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39974
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39972
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39973

lab39972:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39973:

lab39974:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39977
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39975
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39976

lab39975:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39976:

lab39977:
    jmp lab39979

lab39978:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39979:

lab39981:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
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

addj_:
    ; substitute (j !-> j)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_39982]
    add rcx, r9
    jmp rcx

List_i64_39982:
    jmp near List_i64_39982_Nil
    jmp near List_i64_39982_Cons

List_i64_39982_Nil:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(j !-> j)(x1 !-> x1);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; let x0: List[i64] = Cons(j, x1);
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
    je lab39994
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab39995

lab39994:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab39992
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab39985
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39983
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39984

lab39983:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39984:

lab39985:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab39988
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39986
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39987

lab39986:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39987:

lab39988:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab39991
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab39989
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab39990

lab39989:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab39990:

lab39991:
    jmp lab39993

lab39992:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab39993:

lab39995:
    ; #load tag
    mov rdi, 5
    ; let x2: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
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

List_i64_39982_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab39997
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab39996
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39996:
    mov r9, [r8 + 40]
    jmp lab39998

lab39997:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab39998:
    ; substitute (j !-> j)(a0 !-> a0)(k !-> k)(ks !-> ks)(k0 !-> k)(ks0 !-> ks);
    ; #share ks
    cmp r10, 0
    je lab39999
    ; ####increment refcount
    add qword [r10 + 0], 1

lab39999:
    ; #move variables
    mov r13, r9
    mov r14, r10
    mov r15, r11
    ; let x4: List[i64] = Cons(k0, ks0);
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
    je lab40011
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab40012

lab40011:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40009
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40002
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40000
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40001

lab40000:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40001:

lab40002:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40005
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40003
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40004

lab40003:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40004:

lab40005:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40008
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40006
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40007

lab40006:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40007:

lab40008:
    jmp lab40010

lab40009:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40010:

lab40012:
    ; #load tag
    mov r13, 5
    ; substitute (j !-> j)(a0 !-> a0)(k !-> k)(ks !-> ks)(j0 !-> j)(x4 !-> x4);
    ; #move variables
    mov r15, r13
    mov r13, rdx
    mov r14, r12
    ; let x3: List[i64] = Cons(j0, x4);
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
    je lab40024
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab40025

lab40024:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40022
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40015
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40013
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40014

lab40013:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40014:

lab40015:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40018
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40016
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40017

lab40016:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40017:

lab40018:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40021
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40019
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40020

lab40019:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40020:

lab40021:
    jmp lab40023

lab40022:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40023:

lab40025:
    ; #load tag
    mov r13, 5
    ; substitute (j !-> j)(ks !-> ks)(k !-> k)(a0 !-> a0)(x3 !-> x3);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a1: List[List[i64]] = (a0, x3)\{ ... \};
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
    je lab40037
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40038

lab40037:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40035
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40028
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40026
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40027

lab40026:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40027:

lab40028:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40031
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40029
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40030

lab40029:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40030:

lab40031:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40034
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40032
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40033

lab40032:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40033:

lab40034:
    jmp lab40036

lab40035:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40036:

lab40038:
    ; #load tag
    lea r11, [rel List_List_i64_40039]
    ; new a2: List[List[i64]] = (k, a1)\{ ... \};
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
    je lab40051
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40052

lab40051:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40049
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40042
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40040
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40041

lab40040:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40041:

lab40042:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40045
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40043
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40044

lab40043:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40044:

lab40045:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40048
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40046
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40047

lab40046:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40047:

lab40048:
    jmp lab40050

lab40049:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40050:

lab40052:
    ; #load tag
    lea r9, [rel List_List_i64_40053]
    ; jump addj_
    jmp addj_

List_List_i64_40053:
    jmp near List_List_i64_40053_Nil
    jmp near List_List_i64_40053_Cons

List_List_i64_40053_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40055
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40054
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40054:
    mov rdx, [rax + 40]
    jmp lab40056

lab40055:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab40056:
    ; let x6: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x6 !-> x6)(k !-> k)(a1 !-> a1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump add_lscomp_
    jmp add_lscomp_

List_List_i64_40053_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40058
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40057
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40057:
    mov r9, [r8 + 40]
    jmp lab40059

lab40058:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab40059:
    ; substitute (a1 !-> a1)(k !-> k)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x6: List[List[i64]] = Cons(a4, as1);
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
    je lab40071
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40072

lab40071:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40069
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40061

lab40060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40061:

lab40062:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40065
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40063
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40064

lab40063:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40064:

lab40065:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40068
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40066
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40067

lab40066:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40067:

lab40068:
    jmp lab40070

lab40069:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40070:

lab40072:
    ; #load tag
    mov r9, 5
    ; substitute (x6 !-> x6)(k !-> k)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump add_lscomp_
    jmp add_lscomp_

List_List_i64_40039:
    jmp near List_List_i64_40039_Nil
    jmp near List_List_i64_40039_Cons

List_List_i64_40039_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40075
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40073
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40073:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40074
    ; ####increment refcount
    add qword [rax + 0], 1

lab40074:
    jmp lab40076

lab40075:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40076:
    ; let x5: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x3 !-> x3)(x5 !-> x5)(a0 !-> a0);
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

List_List_i64_40039_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40079
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40077
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40077:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40078
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40078:
    jmp lab40080

lab40079:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40080:
    ; substitute (x3 !-> x3)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
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
    ; let x5: List[List[i64]] = Cons(a3, as0);
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
    je lab40092
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40093

lab40092:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40090
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40083
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40081
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40082

lab40081:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40082:

lab40083:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40086
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40084
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40085

lab40084:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40085:

lab40086:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40089
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40087
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40088

lab40087:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40088:

lab40089:
    jmp lab40091

lab40090:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40091:

lab40093:
    ; #load tag
    mov r9, 5
    ; substitute (x3 !-> x3)(x5 !-> x5)(a0 !-> a0);
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

perm_lscomp2_:
    ; substitute (a0 !-> a0)(t1 !-> t1)(j !-> j)(p2 !-> p2);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch p2 \{ ... \};
    lea rcx, [rel List_List_i64_40094]
    add rcx, r11
    jmp rcx

List_List_i64_40094:
    jmp near List_List_i64_40094_Nil
    jmp near List_List_i64_40094_Cons

List_List_i64_40094_Nil:
    ; substitute (t1 !-> t1)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump perm_lscomp1_
    jmp perm_lscomp1_

List_List_i64_40094_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab40097
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab40095
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40095:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab40096
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40096:
    jmp lab40098

lab40097:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab40098:
    ; substitute (t2 !-> t2)(t1 !-> t1)(j !-> j)(r !-> r)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; new a1: List[List[i64]] = (r, a0)\{ ... \};
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
    je lab40110
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40111

lab40110:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40108
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40101
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40099
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40100

lab40099:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40100:

lab40101:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40104
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40102
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40103

lab40102:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40103:

lab40104:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40107
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40105
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40106

lab40105:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40106:

lab40107:
    jmp lab40109

lab40108:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40109:

lab40111:
    ; #load tag
    lea r11, [rel List_List_i64_40112]
    ; jump perm_lscomp2_
    jmp perm_lscomp2_

List_List_i64_40112:
    jmp near List_List_i64_40112_Nil
    jmp near List_List_i64_40112_Cons

List_List_i64_40112_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40115
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40113
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40113:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40114
    ; ####increment refcount
    add qword [rax + 0], 1

lab40114:
    jmp lab40116

lab40115:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40116:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (r !-> r)(x0 !-> x0)(a0 !-> a0);
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

List_List_i64_40112_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40119
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40117
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40117:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40118
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40118:
    jmp lab40120

lab40119:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40120:
    ; substitute (a0 !-> a0)(r !-> r)(a2 !-> a2)(as0 !-> as0);
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
    ; let x0: List[List[i64]] = Cons(a2, as0);
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
    je lab40132
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40133

lab40132:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40130
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40123
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40121
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40122

lab40121:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40122:

lab40123:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40126
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40124
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40125

lab40124:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40125:

lab40126:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40129
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40127
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40128

lab40127:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40128:

lab40129:
    jmp lab40131

lab40130:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40131:

lab40133:
    ; #load tag
    mov r9, 5
    ; substitute (r !-> r)(x0 !-> x0)(a0 !-> a0);
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

perm_lscomp1_:
    ; substitute (a0 !-> a0)(j !-> j)(p1 !-> p1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch p1 \{ ... \};
    lea rcx, [rel List_List_i64_40134]
    add rcx, r9
    jmp rcx

List_List_i64_40134:
    jmp near List_List_i64_40134_Nil
    jmp near List_List_i64_40134_Cons

List_List_i64_40134_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_40134_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40137
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40135
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40135:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40136
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40136:
    jmp lab40138

lab40137:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40138:
    ; substitute (pjs !-> pjs)(j0 !-> j)(j !-> j)(t1 !-> t1)(a0 !-> a0);
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rdx, r9
    mov r9, rdi
    mov rax, r8
    ; new a1: List[List[i64]] = (j, t1, a0)\{ ... \};
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
    je lab40150
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40151

lab40150:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40148
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40140

lab40139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40140:

lab40141:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40144
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40142
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40143

lab40142:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40143:

lab40144:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40147
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40145
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40146

lab40145:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40146:

lab40147:
    jmp lab40149

lab40148:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40149:

lab40151:
    ; #load tag
    lea r9, [rel List_List_i64_40152]
    ; substitute (j0 !-> j0)(pjs !-> pjs)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump addj_
    jmp addj_

List_List_i64_40152:
    jmp near List_List_i64_40152_Nil
    jmp near List_List_i64_40152_Cons

List_List_i64_40152_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40155
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40153
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40153:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40154
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40154:
    mov rdx, [rax + 24]
    jmp lab40156

lab40155:
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

lab40156:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x0 !-> x0)(t1 !-> t1)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, r8
    ; jump perm_lscomp2_
    jmp perm_lscomp2_

List_List_i64_40152_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40159
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab40157
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40157:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab40158
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40158:
    mov r9, [r8 + 24]
    jmp lab40160

lab40159:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab40160:
    ; substitute (a0 !-> a0)(t1 !-> t1)(j !-> j)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a2, as0);
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
    je lab40172
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40173

lab40172:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40170
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40163
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40161
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40162

lab40161:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40162:

lab40163:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40166
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40164
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40165

lab40164:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40165:

lab40166:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40169
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40167
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40168

lab40167:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40168:

lab40169:
    jmp lab40171

lab40170:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40171:

lab40173:
    ; #load tag
    mov r11, 5
    ; substitute (x0 !-> x0)(t1 !-> t1)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump perm_lscomp2_
    jmp perm_lscomp2_

permutations_:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_40174]
    add rcx, rdi
    jmp rcx

List_i64_40174:
    jmp near List_i64_40174_Nil
    jmp near List_i64_40174_Cons

List_i64_40174_Nil:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; let x1: List[List[i64]] = Nil();
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
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_i64_40174_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40176
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40175
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40175:
    mov rdi, [rsi + 40]
    jmp lab40177

lab40176:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab40177:
    ; substitute (js !-> js)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[List[i64]] = (j, a0)\{ ... \};
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
    je lab40189
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40190

lab40189:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40187
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40180
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40178
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40179

lab40178:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40179:

lab40180:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40183
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40181
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40182

lab40181:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40182:

lab40183:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40185

lab40184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40185:

lab40186:
    jmp lab40188

lab40187:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40188:

lab40190:
    ; #load tag
    lea rdi, [rel List_List_i64_40191]
    ; jump permutations_
    jmp permutations_

List_List_i64_40191:
    jmp near List_List_i64_40191_Nil
    jmp near List_List_i64_40191_Cons

List_List_i64_40191_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40193
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40192
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40192:
    mov rdx, [rax + 40]
    jmp lab40194

lab40193:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab40194:
    ; let x2: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump perm_lscomp1_
    jmp perm_lscomp1_

List_List_i64_40191_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40196
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40195
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40195:
    mov r9, [r8 + 40]
    jmp lab40197

lab40196:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab40197:
    ; substitute (a0 !-> a0)(j !-> j)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x2: List[List[i64]] = Cons(a2, as0);
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
    je lab40209
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40210

lab40209:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40207
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40200
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40198
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40199

lab40198:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40199:

lab40200:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40203
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40201
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40202

lab40201:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40202:

lab40203:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40206
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40204
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40205

lab40204:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40205:

lab40206:
    jmp lab40208

lab40207:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40208:

lab40210:
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(j !-> j)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump perm_lscomp1_
    jmp perm_lscomp1_

enum_from_to_:
    ; if from <= t \{ ... \}
    cmp rdx, rdi
    jle lab40211
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab40211:
    ; substitute (from0 !-> from)(t !-> t)(a0 !-> a0)(from !-> from);
    ; #move variables
    mov r11, rdx
    ; new a1: List[i64] = (a0, from)\{ ... \};
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
    je lab40223
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40224

lab40223:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40221
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40214
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40212
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40213

lab40212:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40213:

lab40214:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40217
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40215
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40216

lab40215:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40216:

lab40217:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40219

lab40218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40219:

lab40220:
    jmp lab40222

lab40221:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40222:

lab40224:
    ; #load tag
    lea r9, [rel List_i64_40225]
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- from0 + x1;
    mov r13, rdx
    add r13, r11
    ; substitute (x2 !-> x2)(t !-> t)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_40225:
    jmp near List_i64_40225_Nil
    jmp near List_i64_40225_Cons

List_i64_40225_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40227
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40226
    ; ####increment refcount
    add qword [rax + 0], 1

lab40226:
    jmp lab40228

lab40227:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40228:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (from !-> from)(x0 !-> x0)(a0 !-> a0);
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

List_i64_40225_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40230
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40229
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40229:
    jmp lab40231

lab40230:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40231:
    ; substitute (from !-> from)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
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
    je lab40243
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40244

lab40243:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40241
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40234
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40232
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40233

lab40232:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40233:

lab40234:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40237
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40235
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40236

lab40235:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40236:

lab40237:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40240
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40238
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40239

lab40238:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40239:

lab40240:
    jmp lab40242

lab40241:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40242:

lab40244:
    ; #load tag
    mov r9, 5
    ; substitute (from !-> from)(x0 !-> x0)(a0 !-> a0);
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

take_:
    ; substitute (n !-> n)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_40245]
    add rcx, r9
    jmp rcx

List_i64_40245:
    jmp near List_i64_40245_Nil
    jmp near List_i64_40245_Cons

List_i64_40245_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_40245_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40247
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40246
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40246:
    mov r9, [r8 + 40]
    jmp lab40248

lab40247:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab40248:
    ; lit x0 <- 0;
    mov r13, 0
    ; if n <= x0 \{ ... \}
    cmp rdx, r13
    jle lab40249
    ; substitute (n !-> n)(is !-> is)(i !-> i)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a1: List[i64] = (i, a0)\{ ... \};
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
    je lab40261
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40262

lab40261:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40259
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40252
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40250
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40251

lab40250:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40251:

lab40252:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40255
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40253
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40254

lab40253:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40254:

lab40255:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40258
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40256
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40257

lab40256:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40257:

lab40258:
    jmp lab40260

lab40259:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40260:

lab40262:
    ; #load tag
    lea r9, [rel List_i64_40263]
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- n - x2;
    mov r13, rdx
    sub r13, r11
    ; substitute (x3 !-> x3)(is !-> is)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump take_
    jmp take_

List_i64_40263:
    jmp near List_i64_40263_Nil
    jmp near List_i64_40263_Cons

List_i64_40263_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40265
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40264
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40264:
    mov rdx, [rax + 40]
    jmp lab40266

lab40265:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab40266:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (i !-> i)(x1 !-> x1)(a0 !-> a0);
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

List_i64_40263_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40268
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40267
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40267:
    mov r9, [r8 + 40]
    jmp lab40269

lab40268:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab40269:
    ; substitute (a0 !-> a0)(i !-> i)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x1: List[i64] = Cons(a3, as0);
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
    je lab40281
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40282

lab40281:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40279
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40271

lab40270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40271:

lab40272:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40275
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40273
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40274

lab40273:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40274:

lab40275:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40278
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40276
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40277

lab40276:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40277:

lab40278:
    jmp lab40280

lab40279:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40280:

lab40282:
    ; #load tag
    mov r9, 5
    ; substitute (i !-> i)(x1 !-> x1)(a0 !-> a0);
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

lab40249:
    ; substitute (a0 !-> a0);
    ; #erase is
    cmp r10, 0
    je lab40285
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab40283
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab40284

lab40283:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab40284:

lab40285:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

filter_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_40286]
    add rcx, r9
    jmp rcx

List_List_i64_40286:
    jmp near List_List_i64_40286_Nil
    jmp near List_List_i64_40286_Cons

List_List_i64_40286_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab40289
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab40287
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab40288

lab40287:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab40288:

lab40289:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_40286_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40292
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40290
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40290:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40291
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40291:
    jmp lab40293

lab40292:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40293:
    ; substitute (f0 !-> f)(l00 !-> l0)(l0 !-> l0)(ls !-> ls)(f !-> f)(a0 !-> a0);
    ; #share f
    cmp rax, 0
    je lab40294
    ; ####increment refcount
    add qword [rax + 0], 1

lab40294:
    ; #share l0
    cmp r8, 0
    je lab40295
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40295:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a2: Bool = (l0, ls, f, a0)\{ ... \};
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
    je lab40307
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40308

lab40307:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40305
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40298
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40296
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40297

lab40296:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40297:

lab40298:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40301
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40299
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40300

lab40299:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40300:

lab40301:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40304
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40302
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40303

lab40302:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40303:

lab40304:
    jmp lab40306

lab40305:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40306:

lab40308:
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
    je lab40320
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40321

lab40320:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40318
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40311
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40309
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40310

lab40309:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40310:

lab40311:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40314
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40312
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40313

lab40312:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40313:

lab40314:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40317
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40315
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40316

lab40315:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40316:

lab40317:
    jmp lab40319

lab40318:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40319:

lab40321:
    ; #load tag
    lea r9, [rel Bool_40322]
    ; substitute (l00 !-> l00)(a2 !-> a2)(f0 !-> f0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Ap
    jmp r9

Bool_40322:
    jmp near Bool_40322_True
    jmp near Bool_40322_False

Bool_40322_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40327
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40323
    ; ####increment refcount
    add qword [rax + 0], 1

lab40323:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab40324
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40324:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab40325
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40325:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab40326
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40326:
    jmp lab40328

lab40327:
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

lab40328:
    ; substitute (f !-> f)(ls !-> ls)(l0 !-> l0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[List[i64]] = (l0, a0)\{ ... \};
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
    je lab40340
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40341

lab40340:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40338
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40331
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40329
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40330

lab40329:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40330:

lab40331:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40334
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40332
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40333

lab40332:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40333:

lab40334:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40336

lab40335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40336:

lab40337:
    jmp lab40339

lab40338:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40339:

lab40341:
    ; #load tag
    lea r9, [rel List_List_i64_40342]
    ; jump filter_
    jmp filter_

List_List_i64_40342:
    jmp near List_List_i64_40342_Nil
    jmp near List_List_i64_40342_Cons

List_List_i64_40342_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40345
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40343
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40343:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40344
    ; ####increment refcount
    add qword [rax + 0], 1

lab40344:
    jmp lab40346

lab40345:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40346:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (l0 !-> l0)(x0 !-> x0)(a0 !-> a0);
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

List_List_i64_40342_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40349
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40347
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40347:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40348
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40348:
    jmp lab40350

lab40349:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40350:
    ; substitute (a0 !-> a0)(l0 !-> l0)(a3 !-> a3)(as0 !-> as0);
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
    ; let x0: List[List[i64]] = Cons(a3, as0);
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
    je lab40362
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40363

lab40362:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40360
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40353
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40351
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40352

lab40351:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40352:

lab40353:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40355

lab40354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40355:

lab40356:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40359
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40357
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40358

lab40357:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40358:

lab40359:
    jmp lab40361

lab40360:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40361:

lab40363:
    ; #load tag
    mov r9, 5
    ; substitute (l0 !-> l0)(x0 !-> x0)(a0 !-> a0);
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

Bool_40322_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40368
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40364
    ; ####increment refcount
    add qword [rax + 0], 1

lab40364:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab40365
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40365:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab40366
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40366:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab40367
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40367:
    jmp lab40369

lab40368:
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

lab40369:
    ; substitute (f !-> f)(ls !-> ls)(a0 !-> a0);
    ; #erase l0
    cmp rax, 0
    je lab40372
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab40370
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab40371

lab40370:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab40371:

lab40372:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    mov r8, r10
    mov r9, r11
    ; jump filter_
    jmp filter_

map_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_40373]
    add rcx, r9
    jmp rcx

List_i64_40373:
    jmp near List_i64_40373_Nil
    jmp near List_i64_40373_Cons

List_i64_40373_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab40376
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab40374
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab40375

lab40374:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab40375:

lab40376:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_40373_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40378
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40377
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40377:
    mov r9, [r8 + 40]
    jmp lab40379

lab40378:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab40379:
    ; substitute (f0 !-> f)(i !-> i)(a0 !-> a0)(is !-> is)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab40380
    ; ####increment refcount
    add qword [rax + 0], 1

lab40380:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: List[List[i64]] = (a0, is, f)\{ ... \};
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
    je lab40392
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40393

lab40392:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40390
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40383
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40381
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40382

lab40381:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40382:

lab40383:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40385

lab40384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40385:

lab40386:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40388

lab40387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40388:

lab40389:
    jmp lab40391

lab40390:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40391:

lab40393:
    ; #load tag
    lea r9, [rel List_List_i64_40394]
    ; substitute (i !-> i)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Ap
    jmp r9

List_List_i64_40394:
    jmp near List_List_i64_40394_Nil
    jmp near List_List_i64_40394_Cons

List_List_i64_40394_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40398
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40395
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40395:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40396
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40396:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab40397
    ; ####increment refcount
    add qword [rax + 0], 1

lab40397:
    jmp lab40399

lab40398:
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

lab40399:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(f !-> f)(is !-> is)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_map_0_
    jmp lift_map_0_

List_List_i64_40394_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40403
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab40400
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40400:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab40401
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40401:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab40402
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40402:
    jmp lab40404

lab40403:
    ; ##... or release blocks onto linear free list when loading
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

lab40404:
    ; substitute (f !-> f)(is !-> is)(a0 !-> a0)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a4, as1);
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
    je lab40416
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40417

lab40416:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40414
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40407
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40405
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40406

lab40405:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40406:

lab40407:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40410
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40408
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40409

lab40408:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40409:

lab40410:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40413
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40411
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40412

lab40411:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40412:

lab40413:
    jmp lab40415

lab40414:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40415:

lab40417:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(f !-> f)(is !-> is)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_map_0_
    jmp lift_map_0_

lift_map_0_:
    ; substitute (is !-> is)(f !-> f)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: List[List[List[i64]]] = (a0, x0)\{ ... \};
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
    je lab40429
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40430

lab40429:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40427
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40420
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40418
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40419

lab40418:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40419:

lab40420:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40423
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40421
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40422

lab40421:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40422:

lab40423:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40426
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40424
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40425

lab40424:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40425:

lab40426:
    jmp lab40428

lab40427:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40428:

lab40430:
    ; #load tag
    lea r9, [rel List_List_List_i64_40431]
    ; substitute (f !-> f)(is !-> is)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_
    jmp map_

List_List_List_i64_40431:
    jmp near List_List_List_i64_40431_Nil
    jmp near List_List_List_i64_40431_Cons

List_List_List_i64_40431_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40434
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40432
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40432:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40433
    ; ####increment refcount
    add qword [rax + 0], 1

lab40433:
    jmp lab40435

lab40434:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40435:
    ; let x1: List[List[List[i64]]] = Nil();
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
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_List_List_i64_40431_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40438
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40436
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40436:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40437
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40437:
    jmp lab40439

lab40438:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40439:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
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
    ; let x1: List[List[List[i64]]] = Cons(a3, as0);
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
    je lab40451
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40452

lab40451:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40449
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40442
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40440
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40441

lab40440:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40441:

lab40442:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40445
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40443
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40444

lab40443:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40444:

lab40445:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40448
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40446
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40447

lab40446:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40447:

lab40448:
    jmp lab40450

lab40449:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40450:

lab40452:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
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

test_cryptarithm_nofib_:
    ; new x0: Fun[i64, List[List[i64]]] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_i64_List_List_i64_40453]
    ; new a6: List[i64] = (a0, x0)\{ ... \};
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
    je lab40465
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40466

lab40465:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40463
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40456
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40454
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40455

lab40454:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40455:

lab40456:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40459
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40457
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40458

lab40457:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40458:

lab40459:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40462
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40460
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40461

lab40460:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40461:

lab40462:
    jmp lab40464

lab40463:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40464:

lab40466:
    ; #load tag
    lea rdi, [rel List_i64_40467]
    ; lit x2 <- 1;
    mov r9, 1
    ; substitute (x2 !-> x2)(n !-> n)(a6 !-> a6);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_40467:
    jmp near List_i64_40467_Nil
    jmp near List_i64_40467_Cons

List_i64_40467_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40470
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40468
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40468:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40469
    ; ####increment refcount
    add qword [rax + 0], 1

lab40469:
    jmp lab40471

lab40470:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40471:
    ; let x1: List[i64] = Nil();
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
    ; jump map_
    jmp map_

List_i64_40467_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40474
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40472
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40472:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40473
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40473:
    jmp lab40475

lab40474:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40475:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a11 !-> a11)(as3 !-> as3);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x1: List[i64] = Cons(a11, as3);
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
    je lab40487
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40488

lab40487:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40485
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40478
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40476
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40477

lab40476:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40477:

lab40478:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40481
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40479
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40480

lab40479:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40480:

lab40481:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40484
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40482
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40483

lab40482:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40483:

lab40484:
    jmp lab40486

lab40485:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40486:

lab40488:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump map_
    jmp map_

Fun_i64_List_List_i64_40453:

Fun_i64_List_List_i64_40453_Ap:
    ; lit x3 <- 10;
    mov r9, 10
    ; new a4: List[i64] = (a1, x3)\{ ... \};
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
    je lab40500
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40501

lab40500:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40498
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40491
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40489
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40490

lab40489:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40490:

lab40491:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40494
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40492
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40493

lab40492:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40493:

lab40494:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40497
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40495
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40496

lab40495:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40496:

lab40497:
    jmp lab40499

lab40498:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40499:

lab40501:
    ; #load tag
    lea rdi, [rel List_i64_40502]
    ; lit x5 <- 0;
    mov r9, 0
    ; lit x6 <- 9;
    mov r11, 9
    ; x7 <- x6 + i;
    mov r13, r11
    add r13, rdx
    ; substitute (x5 !-> x5)(x7 !-> x7)(a4 !-> a4);
    ; #move variables
    mov r8, rsi
    mov rdx, r9
    mov r9, rdi
    mov rdi, r13
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_40502:
    jmp near List_i64_40502_Nil
    jmp near List_i64_40502_Cons

List_i64_40502_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40504
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40503
    ; ####increment refcount
    add qword [rax + 0], 1

lab40503:
    jmp lab40505

lab40504:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40505:
    ; let x4: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_test_cryptarithm_nofib_0_
    jmp lift_test_cryptarithm_nofib_0_

List_i64_40502_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40507
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40506
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40506:
    jmp lab40508

lab40507:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40508:
    ; substitute (x3 !-> x3)(a1 !-> a1)(a10 !-> a10)(as2 !-> as2);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x4: List[i64] = Cons(a10, as2);
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
    je lab40520
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40521

lab40520:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40518
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40510

lab40509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40510:

lab40511:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40514
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40512
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40513

lab40512:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40513:

lab40514:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40517
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40515
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40516

lab40515:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40516:

lab40517:
    jmp lab40519

lab40518:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40519:

lab40521:
    ; #load tag
    mov r9, 5
    ; substitute (a1 !-> a1)(x3 !-> x3)(x4 !-> x4);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_test_cryptarithm_nofib_0_
    jmp lift_test_cryptarithm_nofib_0_

lift_test_cryptarithm_nofib_0_:
    ; substitute (x4 !-> x4)(x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a7: List[i64] = (a1)\{ ... \};
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
    je lab40533
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40534

lab40533:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40531
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40524
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40522
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40523

lab40522:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40523:

lab40524:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40527
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40525
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40526

lab40525:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40526:

lab40527:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40530
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40528
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40529

lab40528:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40529:

lab40530:
    jmp lab40532

lab40531:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40532:

lab40534:
    ; #load tag
    lea r9, [rel List_i64_40535]
    ; substitute (x3 !-> x3)(x4 !-> x4)(a7 !-> a7);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump take_
    jmp take_

List_i64_40535:
    jmp near List_i64_40535_Nil
    jmp near List_i64_40535_Cons

List_i64_40535_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40537
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab40536
    ; ####increment refcount
    add qword [rax + 0], 1

lab40536:
    jmp lab40538

lab40537:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab40538:
    ; let p0: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_test_cryptarithm_nofib_1_
    jmp lift_test_cryptarithm_nofib_1_

List_i64_40535_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40540
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab40539
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40539:
    jmp lab40541

lab40540:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab40541:
    ; substitute (a1 !-> a1)(a9 !-> a9)(as1 !-> as1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let p0: List[i64] = Cons(a9, as1);
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
    je lab40553
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40554

lab40553:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40551
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40544
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40542
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40543

lab40542:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40543:

lab40544:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40547
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40545
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40546

lab40545:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40546:

lab40547:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40550
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40548
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40549

lab40548:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40549:

lab40550:
    jmp lab40552

lab40551:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40552:

lab40554:
    ; #load tag
    mov rdi, 5
    ; jump lift_test_cryptarithm_nofib_1_
    jmp lift_test_cryptarithm_nofib_1_

lift_test_cryptarithm_nofib_1_:
    ; new x8: Fun[List[i64], Bool] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_List_i64_Bool_40555]
    ; substitute (p0 !-> p0)(a1 !-> a1)(x8 !-> x8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a3: List[List[i64]] = (a1, x8)\{ ... \};
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
    je lab40567
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40568

lab40567:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40565
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40558
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40556
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40557

lab40556:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40557:

lab40558:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40561
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40559
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40560

lab40559:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40560:

lab40561:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40564
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40562
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40563

lab40562:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40563:

lab40564:
    jmp lab40566

lab40565:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40566:

lab40568:
    ; #load tag
    lea rdi, [rel List_List_i64_40569]
    ; jump permutations_
    jmp permutations_

List_List_i64_40569:
    jmp near List_List_i64_40569_Nil
    jmp near List_List_i64_40569_Cons

List_List_i64_40569_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40572
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40570
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40570:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40571
    ; ####increment refcount
    add qword [rax + 0], 1

lab40571:
    jmp lab40573

lab40572:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40573:
    ; let x9: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x8 !-> x8)(x9 !-> x9)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump filter_
    jmp filter_

List_List_i64_40569_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40576
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40574
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40574:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40575
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40575:
    jmp lab40577

lab40576:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40577:
    ; substitute (x8 !-> x8)(a1 !-> a1)(a8 !-> a8)(as0 !-> as0);
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
    ; let x9: List[List[i64]] = Cons(a8, as0);
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
    je lab40589
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40590

lab40589:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40587
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40580
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40578
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40579

lab40578:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40579:

lab40580:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40583
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40581
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40582

lab40581:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40582:

lab40583:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40586
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40584
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40585

lab40584:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40585:

lab40586:
    jmp lab40588

lab40587:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40588:

lab40590:
    ; #load tag
    mov r9, 5
    ; substitute (x8 !-> x8)(x9 !-> x9)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump filter_
    jmp filter_

Fun_List_i64_Bool_40555:

Fun_List_i64_Bool_40555_Ap:
    ; jump condition_
    jmp condition_

first_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_List_i64_40591]
    add rcx, rdi
    jmp rcx

List_List_List_i64_40591:
    jmp near List_List_List_i64_40591_Nil
    jmp near List_List_List_i64_40591_Cons

List_List_List_i64_40591_Nil:
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

List_List_List_i64_40591_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40594
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40592
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40592:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab40593
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40593:
    jmp lab40595

lab40594:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab40595:
    ; substitute (a0 !-> a0)(i !-> i);
    ; #erase is
    cmp r8, 0
    je lab40598
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab40596
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab40597

lab40596:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab40597:

lab40598:
    ; switch i \{ ... \};
    lea rcx, [rel List_List_i64_40599]
    add rcx, rdi
    jmp rcx

List_List_i64_40599:
    jmp near List_List_i64_40599_Nil
    jmp near List_List_i64_40599_Cons

List_List_i64_40599_Nil:
    ; lit x1 <- -1;
    mov rdi, -1
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_List_i64_40599_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40602
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40600
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40600:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab40601
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40601:
    jmp lab40603

lab40602:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab40603:
    ; substitute (a0 !-> a0)(i0 !-> i0);
    ; #erase is0
    cmp r8, 0
    je lab40606
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab40604
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab40605

lab40604:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab40605:

lab40606:
    ; switch i0 \{ ... \};
    lea rcx, [rel List_i64_40607]
    add rcx, rdi
    jmp rcx

List_i64_40607:
    jmp near List_i64_40607_Nil
    jmp near List_i64_40607_Cons

List_i64_40607_Nil:
    ; lit x2 <- -1;
    mov rdi, -1
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_40607_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40609
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40608
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40608:
    mov rdi, [rsi + 40]
    jmp lab40610

lab40609:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab40610:
    ; substitute (i1 !-> i1)(a0 !-> a0);
    ; #erase is1
    cmp r8, 0
    je lab40613
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab40611
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab40612

lab40611:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab40612:

lab40613:
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; lit x0 <- 1;
    mov r11, 1
    ; if iters == x0 \{ ... \}
    cmp rdx, r11
    je lab40614
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a4: List[List[List[i64]]] = (n, a0, iters)\{ ... \};
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
    je lab40626
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40627

lab40626:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40624
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40616

lab40615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40616:

lab40617:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40619

lab40618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40619:

lab40620:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40622

lab40621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40622:

lab40623:
    jmp lab40625

lab40624:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40625:

lab40627:
    ; #load tag
    lea rdi, [rel List_List_List_i64_40628]
    ; jump test_cryptarithm_nofib_
    jmp test_cryptarithm_nofib_

List_List_List_i64_40628:
    jmp near List_List_List_i64_40628_Nil
    jmp near List_List_List_i64_40628_Cons

List_List_List_i64_40628_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40630
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40629
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40629:
    mov rdx, [rax + 24]
    jmp lab40631

lab40630:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab40631:
    ; let res: List[List[List[i64]]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab40634
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab40632
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab40633

lab40632:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab40633:

lab40634:
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_List_List_i64_40628_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40636
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab40635
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40635:
    mov r9, [r8 + 24]
    jmp lab40637

lab40636:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab40637:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(a6 !-> a6)(as1 !-> as1);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: List[List[List[i64]]] = Cons(a6, as1);
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
    je lab40649
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40650

lab40649:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40647
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40640
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40638
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40639

lab40638:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40639:

lab40640:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40642

lab40641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40642:

lab40643:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40645

lab40644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40645:

lab40646:
    jmp lab40648

lab40647:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40648:

lab40650:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab40653
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab40651
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab40652

lab40651:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab40652:

lab40653:
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lab40614:
    ; substitute (n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a3: List[List[List[i64]]] = (a0)\{ ... \};
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
    je lab40665
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40666

lab40665:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40663
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40655

lab40654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40655:

lab40656:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40659
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40657
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40658

lab40657:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40658:

lab40659:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40661

lab40660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40661:

lab40662:
    jmp lab40664

lab40663:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40664:

lab40666:
    ; #load tag
    lea rdi, [rel List_List_List_i64_40667]
    ; jump test_cryptarithm_nofib_
    jmp test_cryptarithm_nofib_

List_List_List_i64_40667:
    jmp near List_List_List_i64_40667_Nil
    jmp near List_List_List_i64_40667_Cons

List_List_List_i64_40667_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40669
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab40668
    ; ####increment refcount
    add qword [rax + 0], 1

lab40668:
    jmp lab40670

lab40669:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab40670:
    ; let res: List[List[List[i64]]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_List_List_i64_40667_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40672
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab40671
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40671:
    jmp lab40673

lab40672:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab40673:
    ; substitute (a0 !-> a0)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let res: List[List[List[i64]]] = Cons(a5, as0);
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
    je lab40685
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40686

lab40685:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40683
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40676
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40674
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40675

lab40674:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40675:

lab40676:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40679
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40677
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40678

lab40677:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40678:

lab40679:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40681

lab40680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40681:

lab40682:
    jmp lab40684

lab40683:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40684:

lab40686:
    ; #load tag
    mov rdi, 5
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_1_:
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- iters - x2;
    mov r13, rdi
    sub r13, r11
    ; substitute (x3 !-> x3)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lift_main_loop_0_:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
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
    je lab40698
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab40699

lab40698:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40696
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40689
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40687
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40688

lab40687:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40688:

lab40689:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40692
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40690
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40691

lab40690:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40691:

lab40692:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40695
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40693
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40694

lab40693:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40694:

lab40695:
    jmp lab40697

lab40696:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40697:

lab40699:
    ; #load tag
    lea rdi, [rel _Cont_40700]
    ; jump first_
    jmp first_

_Cont_40700:

_Cont_40700_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40702
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab40701
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40701:
    jmp lab40703

lab40702:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab40703:
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