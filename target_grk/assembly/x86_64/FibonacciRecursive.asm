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
    lea r9, [rel _Cont_51736]
    ; jump main_loop_
    jmp main_loop_

_Cont_51736:

_Cont_51736_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

fibonacci_:
    ; if i == 0 \{ ... \}
    cmp rdx, 0
    je lab51737
    ; lit x0 <- 1;
    mov r9, 1
    ; if i == x0 \{ ... \}
    cmp rdx, r9
    je lab51738
    ; substitute (i0 !-> i)(a0 !-> a0)(i !-> i);
    ; #move variables
    mov r9, rdx
    ; new a1: _Cont = (a0, i)\{ ... \};
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
    je lab51750
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51751

lab51750:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51748
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51740

lab51739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51740:

lab51741:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51744
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51742
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51743

lab51742:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51743:

lab51744:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51747
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51745
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51746

lab51745:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51746:

lab51747:
    jmp lab51749

lab51748:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51749:

lab51751:
    ; #load tag
    lea rdi, [rel _Cont_51752]
    ; lit x2 <- 1;
    mov r9, 1
    ; x3 <- i0 - x2;
    mov r11, rdx
    sub r11, r9
    ; substitute (x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump fibonacci_
    jmp fibonacci_

_Cont_51752:

_Cont_51752_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51754
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51753
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51753:
    jmp lab51755

lab51754:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51755:
    ; substitute (i !-> i)(a0 !-> a0)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a3: _Cont = (a0, x1)\{ ... \};
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
    je lab51767
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51768

lab51767:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51765
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51758
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51756
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51757

lab51756:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51757:

lab51758:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51760

lab51759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51760:

lab51761:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51763

lab51762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51763:

lab51764:
    jmp lab51766

lab51765:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51766:

lab51768:
    ; #load tag
    lea rdi, [rel _Cont_51769]
    ; lit x5 <- 2;
    mov r9, 2
    ; x6 <- i - x5;
    mov r11, rdx
    sub r11, r9
    ; substitute (x6 !-> x6)(a3 !-> a3);
    ; #move variables
    mov rdx, r11
    ; jump fibonacci_
    jmp fibonacci_

_Cont_51769:

_Cont_51769_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51771
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab51770
    ; ####increment refcount
    add qword [rsi + 0], 1

lab51770:
    jmp lab51772

lab51771:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab51772:
    ; x7 <- x1 + x4;
    mov r11, r9
    add r11, rdx
    ; substitute (x7 !-> x7)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab51738:
    ; substitute (i !-> i)(a0 !-> a0);
    ; invoke a0 Ret
    jmp rdi

lab51737:
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: _Cont = (n, a0, iters)\{ ... \};
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
    je lab51784
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51785

lab51784:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab51782
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab51775
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51773
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51774

lab51773:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51774:

lab51775:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab51778
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51776
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51777

lab51776:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51777:

lab51778:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab51781
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab51779
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab51780

lab51779:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab51780:

lab51781:
    jmp lab51783

lab51782:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab51783:

lab51785:
    ; #load tag
    lea rdi, [rel _Cont_51786]
    ; jump fibonacci_
    jmp fibonacci_

_Cont_51786:

_Cont_51786_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab51788
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab51787
    ; ####increment refcount
    add qword [r8 + 0], 1

lab51787:
    mov rdi, [rsi + 24]
    jmp lab51789

lab51788:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab51789:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab51790
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- iters - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab51790:
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