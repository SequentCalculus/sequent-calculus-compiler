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
    lea r9, [rel _Cont_62709]
    ; jump main_loop_
    jmp main_loop_

_Cont_62709:

_Cont_62709_Ret:
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
    lea rcx, [rel Bool_62710]
    add rcx, r9
    jmp rcx

Bool_62710:
    jmp near Bool_62710_True
    jmp near Bool_62710_False

Bool_62710_True:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_62711]
    add rcx, rdi
    jmp rcx

Bool_62711:
    jmp near Bool_62711_True
    jmp near Bool_62711_False

Bool_62711_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_62711_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_62710_False:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab62714
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab62712
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab62713

lab62712:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab62713:

lab62714:
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
    lea rcx, [rel Bool_62715]
    add rcx, rdi
    jmp rcx

Bool_62715:
    jmp near Bool_62715_True
    jmp near Bool_62715_False

Bool_62715_True:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_62715_False:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

abs_i_:
    ; lit x0 <- 0;
    mov r9, 0
    ; if i < x0 \{ ... \}
    cmp rdx, r9
    jl lab62716
    ; substitute (i !-> i)(a0 !-> a0);
    ; invoke a0 Ret
    jmp rdi

lab62716:
    ; substitute (i !-> i)(a0 !-> a0);
    ; lit x1 <- -1;
    mov r9, -1
    ; x2 <- x1 * i;
    mov r11, r9
    imul r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

even_abs_:
    ; if i == 0 \{ ... \}
    cmp rdx, 0
    je lab62717
    ; lit x0 <- 1;
    mov r9, 1
    ; x1 <- i - x0;
    mov r11, rdx
    sub r11, r9
    ; substitute (x1 !-> x1)(a0 !-> a0)(a00 !-> a0);
    ; #share a0
    cmp rsi, 0
    je lab62718
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62718:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    mov rdx, r11
    ; jump odd_abs_
    jmp odd_abs_

lab62717:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

odd_abs_:
    ; if i == 0 \{ ... \}
    cmp rdx, 0
    je lab62719
    ; substitute (i !-> i)(k !-> k);
    ; #erase a0
    cmp r8, 0
    je lab62722
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab62720
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab62721

lab62720:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab62721:

lab62722:
    ; lit x0 <- 1;
    mov r9, 1
    ; x1 <- i - x0;
    mov r11, rdx
    sub r11, r9
    ; substitute (x1 !-> x1)(k !-> k);
    ; #move variables
    mov rdx, r11
    ; jump even_abs_
    jmp even_abs_

lab62719:
    ; substitute (k !-> k);
    ; #erase a0
    cmp r8, 0
    je lab62725
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab62723
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab62724

lab62723:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab62724:

lab62725:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke k False
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
    je lab62737
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62738

lab62737:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62735
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62727

lab62726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62727:

lab62728:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62731
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62729
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62730

lab62729:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62730:

lab62731:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62734
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62732
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62733

lab62732:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62733:

lab62734:
    jmp lab62736

lab62735:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62736:

lab62738:
    ; #load tag
    lea rdi, [rel _Cont_62739]
    ; jump abs_i_
    jmp abs_i_

_Cont_62739:

_Cont_62739_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62741
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab62740
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62740:
    jmp lab62742

lab62741:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab62742:
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
    je lab62754
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62755

lab62754:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62752
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62745
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62743
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62744

lab62743:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62744:

lab62745:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62748
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62746
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62747

lab62746:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62747:

lab62748:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62751
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62749
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62750

lab62749:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62750:

lab62751:
    jmp lab62753

lab62752:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62753:

lab62755:
    ; #load tag
    lea rdi, [rel _Cont_62756]
    ; jump abs_i_
    jmp abs_i_

_Cont_62756:

_Cont_62756_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62758
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab62757
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62757:
    jmp lab62759

lab62758:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab62759:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a00 !-> a0);
    ; #share a0
    cmp rsi, 0
    je lab62760
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62760:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
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
    je lab62772
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62773

lab62772:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62770
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62763
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62761
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62762

lab62761:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62762:

lab62763:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62766
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62764
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62765

lab62764:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62765:

lab62766:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62769
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62767
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62768

lab62767:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62768:

lab62769:
    jmp lab62771

lab62770:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62771:

lab62773:
    ; #load tag
    lea rdi, [rel Bool_62774]
    ; jump even_
    jmp even_

Bool_62774:
    jmp near Bool_62774_True
    jmp near Bool_62774_False

Bool_62774_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62776
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab62775
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62775:
    mov rdx, [rax + 24]
    jmp lab62777

lab62776:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab62777:
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

Bool_62774_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62779
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab62778
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62778:
    mov rdx, [rax + 24]
    jmp lab62780

lab62779:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab62780:
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
    je lab62792
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62793

lab62792:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62790
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62783
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62781
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62782

lab62781:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62782:

lab62783:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62785

lab62784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62785:

lab62786:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62788

lab62787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62788:

lab62789:
    jmp lab62791

lab62790:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62791:

lab62793:
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
    je lab62805
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62806

lab62805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62795

lab62794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62795:

lab62796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62798

lab62797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62798:

lab62799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62801

lab62800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62801:

lab62802:
    jmp lab62804

lab62803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62804:

lab62806:
    ; #load tag
    lea rdi, [rel Bool_62807]
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
    je lab62819
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62820

lab62819:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62817
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62810
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62808
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62809

lab62808:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62809:

lab62810:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62812

lab62811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62812:

lab62813:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62815

lab62814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62815:

lab62816:
    jmp lab62818

lab62817:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62818:

lab62820:
    ; #load tag
    lea rdi, [rel Bool_62821]
    ; jump odd_
    jmp odd_

Bool_62821:
    jmp near Bool_62821_True
    jmp near Bool_62821_False

Bool_62821_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62823
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab62822
    ; ####increment refcount
    add qword [rax + 0], 1

lab62822:
    jmp lab62824

lab62823:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab62824:
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

Bool_62821_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62826
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab62825
    ; ####increment refcount
    add qword [rax + 0], 1

lab62825:
    jmp lab62827

lab62826:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab62827:
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

Bool_62807:
    jmp near Bool_62807_True
    jmp near Bool_62807_False

Bool_62807_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62830
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
    je lab62828
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62828:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab62829
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62829:
    mov rdi, [rsi + 24]
    jmp lab62831

lab62830:
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

lab62831:
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

Bool_62807_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62834
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
    je lab62832
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62832:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab62833
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62833:
    mov rdi, [rsi + 24]
    jmp lab62835

lab62834:
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

lab62835:
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
    je lab62847
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62848

lab62847:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62845
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62837

lab62836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62837:

lab62838:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62840

lab62839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62840:

lab62841:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62844
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62842
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62843

lab62842:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62843:

lab62844:
    jmp lab62846

lab62845:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62846:

lab62848:
    ; #load tag
    lea r9, [rel Bool_62849]
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

Bool_62849:
    jmp near Bool_62849_True
    jmp near Bool_62849_False

Bool_62849_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62851
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab62850
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62850:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab62852

lab62851:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab62852:
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

Bool_62849_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62854
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab62853
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62853:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab62855

lab62854:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab62855:
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
    je lab62856
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab62859
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab62857
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab62858

lab62857:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab62858:

lab62859:
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

lab62856:
    ; substitute (a0 !-> a0)(res !-> res);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
    ; switch res \{ ... \};
    lea rcx, [rel Bool_62860]
    add rcx, rdi
    jmp rcx

Bool_62860:
    jmp near Bool_62860_True
    jmp near Bool_62860_False

Bool_62860_True:
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

Bool_62860_False:
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