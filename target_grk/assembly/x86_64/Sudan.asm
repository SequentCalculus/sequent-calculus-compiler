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
    lea r13, [rel _Cont_51685]
    ; jump main_loop_
    jmp main_loop_

_Cont_51685:

_Cont_51685_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

sudan_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab51686
    ; if y == 0 \{ ... \}
    cmp r9, 0
    je lab51687
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- y - x0;
    mov r15, r9
    sub r15, r13
    ; substitute (n0 !-> n)(x !-> x)(x1 !-> x1)(a0 !-> a0)(y !-> y)(n !-> n);
    ; #move variables
    mov r13, r9
    mov r9, r15
    mov r15, rdx
    ; new a4: _Cont = (a0, y, n)\{ ... \};
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
    je lab51699
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51700

lab51699:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51697
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51690
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51688
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51689

lab51688:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51689:

lab51690:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51693
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51691
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51692

lab51691:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51692:

lab51693:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51695

lab51694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51695:

lab51696:
    jmp lab51698

lab51697:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51698:

lab51700:
    ; #load tag
    lea r11, [rel _Cont_51701]
    ; jump sudan_
    jmp sudan_

_Cont_51701:

_Cont_51701_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51703
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab51702
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51702:
    jmp lab51704

lab51703:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab51704:
    ; lit x2 <- 1;
    mov r13, 1
    ; x3 <- n - x2;
    mov r15, r11
    sub r15, r13
    ; substitute (inner !-> inner)(a0 !-> a0)(y !-> y)(x3 !-> x3);
    ; #move variables
    mov r11, r15
    ; x4 <- inner + y;
    mov r13, rdx
    add r13, r9
    ; substitute (x3 !-> x3)(inner !-> inner)(x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, rsi
    mov r9, r13
    ; jump sudan_
    jmp sudan_

lab51687:
    ; substitute (x !-> x)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

lab51686:
    ; substitute (a0 !-> a0)(x !-> x)(y !-> y);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; x5 <- x + y;
    mov r11, rdi
    add r11, r9
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (y0 !-> y)(n0 !-> n)(x4 !-> x)(y !-> y)(a0 !-> a0)(iters !-> iters)(n !-> n)(x !-> x);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a2: _Cont = (y, a0, iters, n, x)\{ ... \};
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
    je lab51716
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab51717

lab51716:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51714
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51707
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51705
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51706

lab51705:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51706:

lab51707:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51710
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51708
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51709

lab51708:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51709:

lab51710:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51713
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51711
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51712

lab51711:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51712:

lab51713:
    jmp lab51715

lab51714:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51715:

lab51717:
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
    je lab51729
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab51730

lab51729:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51727
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51720
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51718
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51719

lab51718:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51719:

lab51720:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51723
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51721
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51722

lab51721:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51722:

lab51723:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51726
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51724
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51725

lab51724:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51725:

lab51726:
    jmp lab51728

lab51727:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51728:

lab51730:
    ; #load tag
    lea r11, [rel _Cont_51731]
    ; substitute (n0 !-> n0)(x4 !-> x4)(y0 !-> y0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump sudan_
    jmp sudan_

_Cont_51731:

_Cont_51731_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51733
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab51732
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51732:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab51734

lab51733:
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

lab51734:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp r11, [rsp +2024]
    je lab51735
    ; substitute (x !-> x)(y !-> y)(a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #move variables
    mov rdx, r15
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- iters - x1;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x2 !-> x2)(n !-> n)(x !-> x)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov r11, rdi
    mov rdi, r13
    mov r13, r9
    mov r9, rdx
    mov r12, r8
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab51735:
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