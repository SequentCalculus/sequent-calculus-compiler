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
    lea r13, [rel _Cont_54663]
    ; jump main_loop_
    jmp main_loop_

_Cont_54663:

_Cont_54663_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

tak_:
    ; if y < x \{ ... \}
    cmp rdi, rdx
    jl lab54664
    ; substitute (z !-> z)(k !-> k);
    ; #erase a0
    cmp r12, 0
    je lab54667
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab54665
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab54666

lab54665:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab54666:

lab54667:
    ; #move variables
    mov rdx, r9
    mov rsi, r10
    mov rdi, r11
    ; invoke k Ret
    jmp rdi

lab54664:
    ; substitute (x10 !-> x)(y1 !-> y)(z1 !-> z)(k !-> k)(a0 !-> a0)(x !-> x)(y !-> y)(z !-> z);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    ; new a: _Cont = (k, a0, x, y, z)\{ ... \};
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
    je lab54679
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54680

lab54679:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54677
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54670
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54668
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54669

lab54668:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54669:

lab54670:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54673
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54671
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54672

lab54671:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54672:

lab54673:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54676
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54674
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54675

lab54674:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54675:

lab54676:
    jmp lab54678

lab54677:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54678:

lab54680:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab54692
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54693

lab54692:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54690
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54683
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54681
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54682

lab54681:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54682:

lab54683:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54686
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54684
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54685

lab54684:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54685:

lab54686:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54689
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54687
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54688

lab54687:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54688:

lab54689:
    jmp lab54691

lab54690:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54691:

lab54693:
    ; #load tag
    lea r11, [rel _Cont_54694]
    ; lit x1 <- 1;
    mov r13, 1
    ; x2 <- x10 - x1;
    mov r15, rdx
    sub r15, r13
    ; substitute (x2 !-> x2)(y1 !-> y1)(z1 !-> z1)(a !-> a)(a4 !-> a);
    ; #share a
    cmp r10, 0
    je lab54695
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54695:
    ; #move variables
    mov r12, r10
    mov r13, r11
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_54694:

_Cont_54694_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54698
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54696
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54696:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab54697
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54697:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab54699

lab54698:
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

lab54699:
    ; substitute (z0 !-> z)(y0 !-> y)(x9 !-> x)(x !-> x)(y !-> y)(z !-> z)(x0 !-> x0)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2024], rdx
    mov [rsp + 2016], rsi
    mov [rsp + 2008], rdi
    mov [rsp + 2000], r8
    mov [rsp + 1992], r9
    mov r9, r11
    mov rdi, r13
    mov rdx, r15
    ; new b: _Cont = (x, y, z, x0, k, a0)\{ ... \};
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
    je lab54711
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab54712

lab54711:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54709
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54701

lab54700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54701:

lab54702:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54705
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54703
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54704

lab54703:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54704:

lab54705:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54708
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54707

lab54706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54707:

lab54708:
    jmp lab54710

lab54709:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54710:

lab54712:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab54724
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54725

lab54724:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54722
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54715
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54713
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54714

lab54713:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54714:

lab54715:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54718
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54716
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54717

lab54716:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54717:

lab54718:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54721
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54719
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54720

lab54719:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54720:

lab54721:
    jmp lab54723

lab54722:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54723:

lab54725:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab54737
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54738

lab54737:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54735
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54727

lab54726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54727:

lab54728:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54731
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54729
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54730

lab54729:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54730:

lab54731:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54734
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54732
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54733

lab54732:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54733:

lab54734:
    jmp lab54736

lab54735:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54736:

lab54738:
    ; #load tag
    lea r11, [rel _Cont_54739]
    ; lit x4 <- 1;
    mov r13, 1
    ; x5 <- y0 - x4;
    mov r15, rdi
    sub r15, r13
    ; substitute (x5 !-> x5)(z0 !-> z0)(x9 !-> x9)(b !-> b)(b0 !-> b);
    ; #share b
    cmp r10, 0
    je lab54740
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54740:
    ; #move variables
    mov rdi, rdx
    mov r12, r10
    mov r13, r11
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_54739:

_Cont_54739_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54743
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
    mov r9, [r8 + 24]
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab54741
    ; ####increment refcount
    add qword [rcx + 0], 1

lab54741:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab54742
    ; ####increment refcount
    add qword [r14 + 0], 1

lab54742:
    mov r13, [r12 + 24]
    jmp lab54744

lab54743:
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
    mov r9, [r8 + 24]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]

lab54744:
    ; substitute (z !-> z)(x !-> x)(y !-> y)(x3 !-> x3)(x0 !-> x0)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new c: _Cont = (x3, x0, k, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab54756
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab54757

lab54756:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54754
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54747
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54745
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54746

lab54745:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54746:

lab54747:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54750
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54748
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54749

lab54748:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54749:

lab54750:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54753
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54751
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54752

lab54751:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54752:

lab54753:
    jmp lab54755

lab54754:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54755:

lab54757:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab54769
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54770

lab54769:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54767
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54760
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54758
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54759

lab54758:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54759:

lab54760:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54763
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54761
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54762

lab54761:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54762:

lab54763:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54766
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54764
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54765

lab54764:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54765:

lab54766:
    jmp lab54768

lab54767:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54768:

lab54770:
    ; #load tag
    lea r11, [rel _Cont_54771]
    ; lit x7 <- 1;
    mov r13, 1
    ; x8 <- z - x7;
    mov r15, rdx
    sub r15, r13
    ; substitute (x8 !-> x8)(x !-> x)(y !-> y)(c !-> c)(c0 !-> c);
    ; #share c
    cmp r10, 0
    je lab54772
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54772:
    ; #move variables
    mov r12, r10
    mov r13, r11
    mov rdx, r15
    ; jump tak_
    jmp tak_

_Cont_54771:

_Cont_54771_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54775
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab54773
    ; ####increment refcount
    add qword [r12 + 0], 1

lab54773:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab54774
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54774:
    mov r9, [r8 + 24]
    jmp lab54776

lab54775:
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
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab54776:
    ; substitute (x0 !-> x0)(x3 !-> x3)(x6 !-> x6)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump tak_
    jmp tak_

main_loop_:
    ; substitute (z0 !-> z)(x4 !-> x)(y0 !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x)(y !-> y);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a: _Cont = (z, a0, iters, x, y)\{ ... \};
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
    je lab54788
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab54789

lab54788:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54786
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54779
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54777
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54778

lab54777:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54778:

lab54779:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54782
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54780
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54781

lab54780:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54781:

lab54782:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54785
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54783
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54784

lab54783:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54784:

lab54785:
    jmp lab54787

lab54786:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54787:

lab54789:
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
    je lab54801
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab54802

lab54801:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54799
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54792
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54790
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54791

lab54790:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54791:

lab54792:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54795
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54793
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54794

lab54793:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54794:

lab54795:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54798
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54796
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54797

lab54796:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54797:

lab54798:
    jmp lab54800

lab54799:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54800:

lab54802:
    ; #load tag
    lea r11, [rel _Cont_54803]
    ; substitute (x4 !-> x4)(y0 !-> y0)(z0 !-> z0)(a !-> a)(a2 !-> a);
    ; #share a
    cmp r10, 0
    je lab54804
    ; ####increment refcount
    add qword [r10 + 0], 1

lab54804:
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r12, r10
    mov r13, r11
    ; jump tak_
    jmp tak_

_Cont_54803:

_Cont_54803_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54806
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54805
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54805:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab54807

lab54806:
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

lab54807:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp r11, [rsp +2024]
    je lab54808
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

lab54808:
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