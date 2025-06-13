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
    ; new x1: Fun[i64, Unit] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_i64_Unit_40704]
    ; substitute (iters0 !-> iters)(steps0 !-> steps)(x1 !-> x1)(iters !-> iters)(steps !-> steps);
    ; #move variables
    mov r11, rdx
    mov r13, rdi
    ; new a2: _Cont = (iters, steps)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab40716
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40717

lab40716:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40714
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40707
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40705
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40706

lab40705:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40706:

lab40707:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40710
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40708
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40709

lab40708:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40709:

lab40710:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40713
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40711
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40712

lab40711:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40712:

lab40713:
    jmp lab40715

lab40714:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40715:

lab40717:
    ; #load tag
    lea r11, [rel _Cont_40718]
    ; jump go_loop_
    jmp go_loop_

_Cont_40718:

_Cont_40718_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40719
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab40720

lab40719:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab40720:
    ; print_i64 gun_res;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call print_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    add rsp, 8
    ; substitute (steps !-> steps)(iters !-> iters);
    ; #move variables
    mov rdx, r9
    ; new x2: Fun[i64, Unit] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_i64_Unit_40721]
    ; new a3: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel _Cont_40722]
    ; substitute (iters !-> iters)(steps !-> steps)(x2 !-> x2)(a3 !-> a3);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump go_loop_
    jmp go_loop_

_Cont_40722:

_Cont_40722_Ret:
    ; println_i64 shuttle_res;
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
    ; return x0
    mov rax, rdx
    jmp cleanup

Fun_i64_Unit_40721:

Fun_i64_Unit_40721_Apply:
    ; let a0: Fun[i64, Unit] = Apply(x4, a01);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab40734
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab40735

lab40734:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40732
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40725
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40723
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40724

lab40723:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40724:

lab40725:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40727

lab40726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40727:

lab40728:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40731
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40729
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40730

lab40729:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40730:

lab40731:
    jmp lab40733

lab40732:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40733:

lab40735:
    ; #load tag
    mov rdx, 0
    ; jump go_shuttle_
    jmp go_shuttle_

Fun_i64_Unit_40704:

Fun_i64_Unit_40704_Apply:
    ; let a1: Fun[i64, Unit] = Apply(x3, a00);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab40747
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab40748

lab40747:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40745
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40737

lab40736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40737:

lab40738:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40740

lab40739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40740:

lab40741:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40744
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40742
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40743

lab40742:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40743:

lab40744:
    jmp lab40746

lab40745:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40746:

lab40748:
    ; #load tag
    mov rdx, 0
    ; jump go_gun_
    jmp go_gun_

pair_eq_:
    ; substitute (a0 !-> a0)(p2 !-> p2)(p1 !-> p1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch p1 \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_40749:

Pair_i64_i64_40749_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40750
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab40751

lab40750:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab40751:
    ; substitute (a0 !-> a0)(snd1 !-> snd1)(fst1 !-> fst1)(p2 !-> p2);
    ; #move variables
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch p2 \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_40752:

Pair_i64_i64_40752_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab40753
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]
    jmp lab40754

lab40753:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]

lab40754:
    ; if fst1 == fst2 \{ ... \}
    cmp r9, r11
    je lab40755
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab40755:
    ; if snd1 == snd2 \{ ... \}
    cmp rdi, r13
    je lab40756
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab40756:
    ; substitute (a0 !-> a0);
    ; invoke a0 True
    add rdx, 0
    jmp rdx

or_:
    ; substitute (a0 !-> a0)(b2 !-> b2)(b1 !-> b1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch b1 \{ ... \};
    lea rcx, [rel Bool_40757]
    add rcx, r9
    jmp rcx

Bool_40757:
    jmp near Bool_40757_True
    jmp near Bool_40757_False

Bool_40757_True:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab40760
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab40758
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab40759

lab40758:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab40759:

lab40760:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_40757_False:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_40761]
    add rcx, rdi
    jmp rcx

Bool_40761:
    jmp near Bool_40761_True
    jmp near Bool_40761_False

Bool_40761_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_40761_False:
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
    lea rcx, [rel Bool_40762]
    add rcx, rdi
    jmp rcx

Bool_40762:
    jmp near Bool_40762_True
    jmp near Bool_40762_False

Bool_40762_True:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_40762_False:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

fold_:
    ; substitute (a !-> a)(a0 !-> a0)(f !-> f)(xs !-> xs);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch xs \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40763]
    add rcx, r11
    jmp rcx

List_Pair_i64_i64_40763:
    jmp near List_Pair_i64_i64_40763_Nil
    jmp near List_Pair_i64_i64_40763_Cons

List_Pair_i64_i64_40763_Nil:
    ; substitute (a0 !-> a0)(a !-> a);
    ; #erase f
    cmp r8, 0
    je lab40766
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab40764
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab40765

lab40764:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab40765:

lab40766:
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch a \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40767]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_40767:
    jmp near List_Pair_i64_i64_40767_Nil
    jmp near List_Pair_i64_i64_40767_Cons

List_Pair_i64_i64_40767_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_40767_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40770
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40768
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40768:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab40769
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40769:
    jmp lab40771

lab40770:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab40771:
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

List_Pair_i64_i64_40763_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab40774
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab40772
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40772:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab40773
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40773:
    jmp lab40775

lab40774:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab40775:
    ; substitute (a !-> a)(b !-> b)(f0 !-> f)(f !-> f)(x !-> x)(a0 !-> a0);
    ; #share f
    cmp r8, 0
    je lab40776
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40776:
    ; #move variables
    mov r14, rsi
    mov r15, rdi
    mov rsi, r10
    mov r10, r8
    mov rdi, r11
    mov r11, r9
    ; new a1: List[Pair[i64, i64]] = (f, x, a0)\{ ... \};
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
    je lab40788
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40789

lab40788:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40786
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40779
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40777
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40778

lab40777:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40778:

lab40779:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40782
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40780
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40781

lab40780:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40781:

lab40782:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40785
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40783
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40784

lab40783:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40784:

lab40785:
    jmp lab40787

lab40786:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40787:

lab40789:
    ; #load tag
    lea r11, [rel List_Pair_i64_i64_40790]
    ; substitute (a !-> a)(b !-> b)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; invoke f0 Apply2
    jmp r11

List_Pair_i64_i64_40790:
    jmp near List_Pair_i64_i64_40790_Nil
    jmp near List_Pair_i64_i64_40790_Cons

List_Pair_i64_i64_40790_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40794
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40791
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40791:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40792
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40792:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab40793
    ; ####increment refcount
    add qword [rax + 0], 1

lab40793:
    jmp lab40795

lab40794:
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

lab40795:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x0 !-> x0)(x !-> x)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump fold_
    jmp fold_

List_Pair_i64_i64_40790_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40799
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab40796
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40796:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab40797
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40797:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab40798
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40798:
    jmp lab40800

lab40799:
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

lab40800:
    ; substitute (a0 !-> a0)(x !-> x)(f !-> f)(x2 !-> x2)(xs1 !-> xs1);
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
    ; let x0: List[Pair[i64, i64]] = Cons(x2, xs1);
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
    je lab40812
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40813

lab40812:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40810
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40803
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40801
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40802

lab40801:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40802:

lab40803:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40806
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40804
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40805

lab40804:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40805:

lab40806:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40809
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40807
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40808

lab40807:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40808:

lab40809:
    jmp lab40811

lab40810:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40811:

lab40813:
    ; #load tag
    mov r11, 5
    ; substitute (x0 !-> x0)(x !-> x)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump fold_
    jmp fold_

accumulate_:
    ; jump fold_
    jmp fold_

revonto_:
    ; new x0: Fun2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]] = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel Fun2_List_Pair_i64_i64_Pair_i64_i64_List_Pair_i64_i64_40814]
    ; substitute (x !-> x)(y !-> y)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump accumulate_
    jmp accumulate_

Fun2_List_Pair_i64_i64_Pair_i64_i64_List_Pair_i64_i64_40814:

Fun2_List_Pair_i64_i64_Pair_i64_i64_List_Pair_i64_i64_40814_Apply2:
    ; substitute (h !-> h)(t !-> t)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Cons
    add r9, 5
    jmp r9

collect_accum_:
    ; substitute (sofar !-> sofar)(a0 !-> a0)(f !-> f)(xs !-> xs);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch xs \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40815]
    add rcx, r11
    jmp rcx

List_Pair_i64_i64_40815:
    jmp near List_Pair_i64_i64_40815_Nil
    jmp near List_Pair_i64_i64_40815_Cons

List_Pair_i64_i64_40815_Nil:
    ; substitute (a0 !-> a0)(sofar !-> sofar);
    ; #erase f
    cmp r8, 0
    je lab40818
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab40816
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab40817

lab40816:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab40817:

lab40818:
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch sofar \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40819]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_40819:
    jmp near List_Pair_i64_i64_40819_Nil
    jmp near List_Pair_i64_i64_40819_Cons

List_Pair_i64_i64_40819_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_40819_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab40822
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab40820
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40820:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab40821
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40821:
    jmp lab40823

lab40822:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab40823:
    ; substitute (x2 !-> x2)(xs1 !-> xs1)(a0 !-> a0);
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

List_Pair_i64_i64_40815_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab40826
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab40824
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40824:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab40825
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40825:
    jmp lab40827

lab40826:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab40827:
    ; substitute (sofar !-> sofar)(p !-> p)(f0 !-> f)(f !-> f)(xs0 !-> xs0)(a0 !-> a0);
    ; #share f
    cmp r8, 0
    je lab40828
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40828:
    ; #move variables
    mov r14, rsi
    mov r15, rdi
    mov rsi, r10
    mov r10, r8
    mov rdi, r11
    mov r11, r9
    ; new a1: List[Pair[i64, i64]] = (f, xs0, a0)\{ ... \};
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
    je lab40840
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40841

lab40840:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40838
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40831
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40829
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40830

lab40829:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40830:

lab40831:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40834
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40832
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40833

lab40832:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40833:

lab40834:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40837
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40835
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40836

lab40835:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40836:

lab40837:
    jmp lab40839

lab40838:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40839:

lab40841:
    ; #load tag
    lea r11, [rel List_Pair_i64_i64_40842]
    ; substitute (f0 !-> f0)(p !-> p)(sofar !-> sofar)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: List[Pair[i64, i64]] = (sofar, a1)\{ ... \};
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
    je lab40854
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40855

lab40854:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40852
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40844

lab40843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40844:

lab40845:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40848
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40846
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40847

lab40846:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40847:

lab40848:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40850

lab40849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40850:

lab40851:
    jmp lab40853

lab40852:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40853:

lab40855:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_40856]
    ; substitute (p !-> p)(a2 !-> a2)(f0 !-> f0);
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

List_Pair_i64_i64_40856:
    jmp near List_Pair_i64_i64_40856_Nil
    jmp near List_Pair_i64_i64_40856_Cons

List_Pair_i64_i64_40856_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40859
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40857
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40857:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40858
    ; ####increment refcount
    add qword [rax + 0], 1

lab40858:
    jmp lab40860

lab40859:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40860:
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (sofar !-> sofar)(x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump revonto_
    jmp revonto_

List_Pair_i64_i64_40856_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40863
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40861
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40861:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40862
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40862:
    jmp lab40864

lab40863:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40864:
    ; substitute (a1 !-> a1)(sofar !-> sofar)(x4 !-> x4)(xs3 !-> xs3);
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
    ; let x1: List[Pair[i64, i64]] = Cons(x4, xs3);
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
    je lab40876
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40877

lab40876:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40874
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40866

lab40865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40866:

lab40867:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40870
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40868
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40869

lab40868:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40869:

lab40870:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40873
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40871
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40872

lab40871:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40872:

lab40873:
    jmp lab40875

lab40874:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40875:

lab40877:
    ; #load tag
    mov r9, 5
    ; substitute (sofar !-> sofar)(x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump revonto_
    jmp revonto_

List_Pair_i64_i64_40842:
    jmp near List_Pair_i64_i64_40842_Nil
    jmp near List_Pair_i64_i64_40842_Cons

List_Pair_i64_i64_40842_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40881
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40878
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40878:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40879
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40879:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab40880
    ; ####increment refcount
    add qword [rax + 0], 1

lab40880:
    jmp lab40882

lab40881:
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

lab40882:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x0 !-> x0)(xs0 !-> xs0)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump collect_accum_
    jmp collect_accum_

List_Pair_i64_i64_40842_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40886
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab40883
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40883:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab40884
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40884:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab40885
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40885:
    jmp lab40887

lab40886:
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

lab40887:
    ; substitute (a0 !-> a0)(xs0 !-> xs0)(f !-> f)(x3 !-> x3)(xs2 !-> xs2);
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
    ; let x0: List[Pair[i64, i64]] = Cons(x3, xs2);
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
    je lab40899
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40900

lab40899:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40897
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40889

lab40888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40889:

lab40890:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40892

lab40891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40892:

lab40893:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40896
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40894
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40895

lab40894:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40895:

lab40896:
    jmp lab40898

lab40897:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40898:

lab40900:
    ; #load tag
    mov r11, 5
    ; substitute (x0 !-> x0)(xs0 !-> xs0)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump collect_accum_
    jmp collect_accum_

collect_:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x0 !-> x0)(l !-> l)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump collect_accum_
    jmp collect_accum_

exists_:
    ; substitute (a0 !-> a0)(f !-> f)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40901]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_40901:
    jmp near List_Pair_i64_i64_40901_Nil
    jmp near List_Pair_i64_i64_40901_Cons

List_Pair_i64_i64_40901_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab40904
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab40902
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab40903

lab40902:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab40903:

lab40904:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_Pair_i64_i64_40901_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40907
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab40905
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40905:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40906
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40906:
    jmp lab40908

lab40907:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab40908:
    ; substitute (p !-> p)(f0 !-> f)(f !-> f)(ps !-> ps)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab40909
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40909:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rax, r8
    mov r8, rsi
    mov rdx, r9
    mov r9, rdi
    ; new a1: Bool = (f, ps, a0)\{ ... \};
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
    je lab40921
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40922

lab40921:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40919
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40912
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40910
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40911

lab40910:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40911:

lab40912:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40915
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40913
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40914

lab40913:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40914:

lab40915:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40918
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40916
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40917

lab40916:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40917:

lab40918:
    jmp lab40920

lab40919:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40920:

lab40922:
    ; #load tag
    lea r9, [rel Bool_40923]
    ; substitute (p !-> p)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

Bool_40923:
    jmp near Bool_40923_True
    jmp near Bool_40923_False

Bool_40923_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40927
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40924
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40924:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40925
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40925:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab40926
    ; ####increment refcount
    add qword [rax + 0], 1

lab40926:
    jmp lab40928

lab40927:
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

lab40928:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(f !-> f)(ps !-> ps)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_exists_0_
    jmp lift_exists_0_

Bool_40923_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40932
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab40929
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40929:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab40930
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40930:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab40931
    ; ####increment refcount
    add qword [rax + 0], 1

lab40931:
    jmp lab40933

lab40932:
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

lab40933:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(f !-> f)(ps !-> ps)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_exists_0_
    jmp lift_exists_0_

lift_exists_0_:
    ; substitute (ps !-> ps)(f !-> f)(a0 !-> a0)(x0 !-> x0);
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
    je lab40945
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40946

lab40945:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40943
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40936
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40934
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40935

lab40934:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40935:

lab40936:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40939
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40938

lab40937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40938:

lab40939:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40941

lab40940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40941:

lab40942:
    jmp lab40944

lab40943:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40944:

lab40946:
    ; #load tag
    lea r9, [rel Bool_40947]
    ; jump exists_
    jmp exists_

Bool_40947:
    jmp near Bool_40947_True
    jmp near Bool_40947_False

Bool_40947_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40950
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40948
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40948:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40949
    ; ####increment refcount
    add qword [rax + 0], 1

lab40949:
    jmp lab40951

lab40950:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40951:
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
    ; jump or_
    jmp or_

Bool_40947_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab40954
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab40952
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40952:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab40953
    ; ####increment refcount
    add qword [rax + 0], 1

lab40953:
    jmp lab40955

lab40954:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab40955:
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
    ; jump or_
    jmp or_

map_loop_:
    ; substitute (a0 !-> a0)(f !-> f)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_40956]
    add rcx, r11
    jmp rcx

List_Pair_i64_i64_40956:
    jmp near List_Pair_i64_i64_40956_Nil
    jmp near List_Pair_i64_i64_40956_Cons

List_Pair_i64_i64_40956_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab40959
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab40957
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab40958

lab40957:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab40958:

lab40959:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; jump rev_
    jmp rev_

List_Pair_i64_i64_40956_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab40962
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab40960
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40960:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab40961
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40961:
    jmp lab40963

lab40962:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab40963:
    ; substitute (p !-> p)(f0 !-> f)(acc !-> acc)(f !-> f)(ps !-> ps)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab40964
    ; ####increment refcount
    add qword [rsi + 0], 1

lab40964:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a1: Pair[i64, i64] = (acc, f, ps, a0)\{ ... \};
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
    je lab40976
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab40977

lab40976:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40974
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40967
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40965
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40966

lab40965:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40966:

lab40967:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40970
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40968
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40969

lab40968:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40969:

lab40970:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40973
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40971
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40972

lab40971:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40972:

lab40973:
    jmp lab40975

lab40974:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40975:

lab40977:
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
    je lab40989
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab40990

lab40989:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab40987
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab40980
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40978
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40979

lab40978:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40979:

lab40980:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab40983
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40981
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40982

lab40981:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40982:

lab40983:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab40986
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40984
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40985

lab40984:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40985:

lab40986:
    jmp lab40988

lab40987:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab40988:

lab40990:
    ; #load tag
    lea r9, [rel Pair_i64_i64_40991]
    ; substitute (p !-> p)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

Pair_i64_i64_40991:

Pair_i64_i64_40991_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab40996
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab40992
    ; ####increment refcount
    add qword [r8 + 0], 1

lab40992:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab40993
    ; ####increment refcount
    add qword [r14 + 0], 1

lab40993:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab40994
    ; ####increment refcount
    add qword [r12 + 0], 1

lab40994:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab40995
    ; ####increment refcount
    add qword [r10 + 0], 1

lab40995:
    jmp lab40997

lab40996:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab40997:
    ; substitute (a0 !-> a0)(ps !-> ps)(acc !-> acc)(f !-> f)(fst0 !-> fst0)(snd0 !-> snd0);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rsi, r12
    mov rax, r14
    ; let x1: Pair[i64, i64] = Tup(fst0, snd0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab41009
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41010

lab41009:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41007
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab40998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab40999

lab40998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab40999:

lab41000:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41002

lab41001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41002:

lab41003:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41005

lab41004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41005:

lab41006:
    jmp lab41008

lab41007:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41008:

lab41010:
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(ps !-> ps)(f !-> f)(x1 !-> x1)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(x1, acc);
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
    je lab41022
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41023

lab41022:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41020
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41012

lab41011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41012:

lab41013:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41015

lab41014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41015:

lab41016:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41019
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41017
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41018

lab41017:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41018:

lab41019:
    jmp lab41021

lab41020:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41021:

lab41023:
    ; #load tag
    mov r11, 5
    ; substitute (ps !-> ps)(f !-> f)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump map_loop_
    jmp map_loop_

appendRev_:
    ; substitute (a0 !-> a0)(l2 !-> l2)(l1 !-> l1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41024]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_41024:
    jmp near List_Pair_i64_i64_41024_Nil
    jmp near List_Pair_i64_i64_41024_Cons

List_Pair_i64_i64_41024_Nil:
    ; switch l2 \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41025]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_41025:
    jmp near List_Pair_i64_i64_41025_Nil
    jmp near List_Pair_i64_i64_41025_Cons

List_Pair_i64_i64_41025_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_41025_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41028
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab41026
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41026:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab41027
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41027:
    jmp lab41029

lab41028:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab41029:
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

List_Pair_i64_i64_41024_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41032
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab41030
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41030:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab41031
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41031:
    jmp lab41033

lab41032:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab41033:
    ; substitute (a0 !-> a0)(iss !-> iss)(is !-> is)(l2 !-> l2);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(is, l2);
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
    je lab41045
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41046

lab41045:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41043
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41036
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41034
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41035

lab41034:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41035:

lab41036:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41039
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41037
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41038

lab41037:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41038:

lab41039:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41042
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41040
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41041

lab41040:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41041:

lab41042:
    jmp lab41044

lab41043:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41044:

lab41046:
    ; #load tag
    mov r9, 5
    ; substitute (iss !-> iss)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump appendRev_
    jmp appendRev_

rev_:
    ; let x0: List[Pair[i64, i64]] = Nil();
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
    ; jump appendRev_
    jmp appendRev_

append_:
    ; substitute (l1 !-> l1)(a0 !-> a0)(l2 !-> l2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l2 \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41047]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_41047:
    jmp near List_Pair_i64_i64_41047_Nil
    jmp near List_Pair_i64_i64_41047_Cons

List_Pair_i64_i64_41047_Nil:
    ; substitute (a0 !-> a0)(l1 !-> l1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41048]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_41048:
    jmp near List_Pair_i64_i64_41048_Nil
    jmp near List_Pair_i64_i64_41048_Cons

List_Pair_i64_i64_41048_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_41048_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41051
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab41049
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41049:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab41050
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41050:
    jmp lab41052

lab41051:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab41052:
    ; substitute (x2 !-> x2)(xs0 !-> xs0)(a0 !-> a0);
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

List_Pair_i64_i64_41047_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41055
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab41053
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41053:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab41054
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41054:
    jmp lab41056

lab41055:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab41056:
    ; new a1: List[Pair[i64, i64]] = (a0, is, iss)\{ ... \};
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
    je lab41068
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab41069

lab41068:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41066
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41058

lab41057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41058:

lab41059:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41061

lab41060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41061:

lab41062:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41065
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41063
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41064

lab41063:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41064:

lab41065:
    jmp lab41067

lab41066:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41067:

lab41069:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_41070]
    ; jump rev_
    jmp rev_

List_Pair_i64_i64_41070:
    jmp near List_Pair_i64_i64_41070_Nil
    jmp near List_Pair_i64_i64_41070_Cons

List_Pair_i64_i64_41070_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41074
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab41071
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41071:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41072
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41072:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41073
    ; ####increment refcount
    add qword [rax + 0], 1

lab41073:
    jmp lab41075

lab41074:
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

lab41075:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; jump lift_append_0_
    jmp lift_append_0_

List_Pair_i64_i64_41070_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41079
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab41076
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41076:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41077
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41077:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41078
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41078:
    jmp lab41080

lab41079:
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

lab41080:
    ; substitute (iss !-> iss)(is !-> is)(a0 !-> a0)(x3 !-> x3)(xs1 !-> xs1);
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
    ; let x0: List[Pair[i64, i64]] = Cons(x3, xs1);
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
    je lab41092
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41093

lab41092:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41090
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41083
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41081
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41082

lab41081:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41082:

lab41083:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41086
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41084
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41085

lab41084:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41085:

lab41086:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41089
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41087
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41088

lab41087:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41088:

lab41089:
    jmp lab41091

lab41090:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41091:

lab41093:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(is !-> is)(iss !-> iss)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_append_0_
    jmp lift_append_0_

lift_append_0_:
    ; substitute (a0 !-> a0)(x0 !-> x0)(is !-> is)(iss !-> iss);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x1: List[Pair[i64, i64]] = Cons(is, iss);
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
    je lab41105
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41106

lab41105:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41103
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41096
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41094
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41095

lab41094:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41095:

lab41096:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41099
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41097
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41098

lab41097:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41098:

lab41099:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41102
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41100
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41101

lab41100:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41101:

lab41102:
    jmp lab41104

lab41103:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41104:

lab41106:
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
    ; jump appendRev_
    jmp appendRev_

map_:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (l !-> l)(f !-> f)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump map_loop_
    jmp map_loop_

member_:
    ; substitute (l !-> l)(a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new x0: Fun[Pair[i64, i64], Bool] = (p)\{ ... \};
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
    je lab41118
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41119

lab41118:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41116
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41109
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41107
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41108

lab41107:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41108:

lab41109:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41112
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41110
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41111

lab41110:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41111:

lab41112:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41115
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41113
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41114

lab41113:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41114:

lab41115:
    jmp lab41117

lab41116:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41117:

lab41119:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_41120]
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump exists_
    jmp exists_

Fun_Pair_i64_i64_Bool_41120:

Fun_Pair_i64_i64_Bool_41120_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41122
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41121
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41121:
    jmp lab41123

lab41122:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41123:
    ; substitute (p !-> p)(p1 !-> p1)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump pair_eq_
    jmp pair_eq_

len_loop_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41124]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_41124:
    jmp near List_Pair_i64_i64_41124_Nil
    jmp near List_Pair_i64_i64_41124_Cons

List_Pair_i64_i64_41124_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_Pair_i64_i64_41124_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41127
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab41125
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41125:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab41126
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41126:
    jmp lab41128

lab41127:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab41128:
    ; substitute (a0 !-> a0)(acc !-> acc)(ps !-> ps);
    ; #erase p
    cmp r8, 0
    je lab41131
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab41129
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab41130

lab41129:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab41130:

lab41131:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- acc + x0;
    mov r13, rdi
    add r13, r11
    ; substitute (ps !-> ps)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump len_loop_
    jmp len_loop_

len_:
    ; lit x0 <- 0;
    mov r9, 0
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump len_loop_
    jmp len_loop_

filter_loop_:
    ; substitute (a0 !-> a0)(f !-> f)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41132]
    add rcx, r11
    jmp rcx

List_Pair_i64_i64_41132:
    jmp near List_Pair_i64_i64_41132_Nil
    jmp near List_Pair_i64_i64_41132_Cons

List_Pair_i64_i64_41132_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab41135
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab41133
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab41134

lab41133:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab41134:

lab41135:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; jump rev_
    jmp rev_

List_Pair_i64_i64_41132_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab41138
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab41136
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41136:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab41137
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41137:
    jmp lab41139

lab41138:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab41139:
    ; substitute (p !-> p)(f0 !-> f)(acc !-> acc)(f !-> f)(ps !-> ps)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab41140
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41140:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a1: List[Pair[i64, i64]] = (f, ps, a0)\{ ... \};
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
    je lab41152
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41153

lab41152:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41150
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41143
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41141
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41142

lab41141:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41142:

lab41143:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41146
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41144
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41145

lab41144:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41145:

lab41146:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41149
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41147
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41148

lab41147:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41148:

lab41149:
    jmp lab41151

lab41150:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41151:

lab41153:
    ; #load tag
    lea r11, [rel List_Pair_i64_i64_41154]
    ; substitute (p0 !-> p)(f0 !-> f0)(acc !-> acc)(a1 !-> a1)(p !-> p);
    ; #share p
    cmp rax, 0
    je lab41155
    ; ####increment refcount
    add qword [rax + 0], 1

lab41155:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    ; new a2: Bool = (acc, a1, p)\{ ... \};
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
    je lab41167
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41168

lab41167:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41165
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41158
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41156
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41157

lab41156:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41157:

lab41158:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41161
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41159
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41160

lab41159:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41160:

lab41161:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41164
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41162
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41163

lab41162:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41163:

lab41164:
    jmp lab41166

lab41165:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41166:

lab41168:
    ; #load tag
    lea r9, [rel Bool_41169]
    ; substitute (p0 !-> p0)(a2 !-> a2)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

Bool_41169:
    jmp near Bool_41169_True
    jmp near Bool_41169_False

Bool_41169_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41173
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab41170
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41170:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41171
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41171:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41172
    ; ####increment refcount
    add qword [rax + 0], 1

lab41172:
    jmp lab41174

lab41173:
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

lab41174:
    ; substitute (p !-> p)(acc !-> acc)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Cons
    add r9, 5
    jmp r9

Bool_41169_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41178
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab41175
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41175:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41176
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41176:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41177
    ; ####increment refcount
    add qword [rax + 0], 1

lab41177:
    jmp lab41179

lab41178:
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

lab41179:
    ; substitute (a1 !-> a1)(acc !-> acc);
    ; #erase p
    cmp r8, 0
    je lab41182
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab41180
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab41181

lab41180:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab41181:

lab41182:
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch acc \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41183]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_41183:
    jmp near List_Pair_i64_i64_41183_Nil
    jmp near List_Pair_i64_i64_41183_Cons

List_Pair_i64_i64_41183_Nil:
    ; invoke a1 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_41183_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41186
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab41184
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41184:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab41185
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41185:
    jmp lab41187

lab41186:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab41187:
    ; substitute (x2 !-> x2)(xs1 !-> xs1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a1 Cons
    add r9, 5
    jmp r9

List_Pair_i64_i64_41154:
    jmp near List_Pair_i64_i64_41154_Nil
    jmp near List_Pair_i64_i64_41154_Cons

List_Pair_i64_i64_41154_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41191
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab41188
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41188:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41189
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41189:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41190
    ; ####increment refcount
    add qword [rax + 0], 1

lab41190:
    jmp lab41192

lab41191:
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

lab41192:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (ps !-> ps)(f !-> f)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump filter_loop_
    jmp filter_loop_

List_Pair_i64_i64_41154_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41196
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab41193
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41193:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41194
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41194:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41195
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41195:
    jmp lab41197

lab41196:
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

lab41197:
    ; substitute (a0 !-> a0)(ps !-> ps)(f !-> f)(x1 !-> x1)(xs0 !-> xs0);
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
    ; let x0: List[Pair[i64, i64]] = Cons(x1, xs0);
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
    je lab41209
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41210

lab41209:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41207
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41200
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41198
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41199

lab41198:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41199:

lab41200:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41203
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41201
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41202

lab41201:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41202:

lab41203:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41206
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41204
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41205

lab41204:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41205:

lab41206:
    jmp lab41208

lab41207:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41208:

lab41210:
    ; #load tag
    mov r11, 5
    ; substitute (ps !-> ps)(f !-> f)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump filter_loop_
    jmp filter_loop_

filter_:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (l !-> l)(p !-> p)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump filter_loop_
    jmp filter_loop_

lexordset_:
    ; substitute (a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch xs \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41211]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_41211:
    jmp near List_Pair_i64_i64_41211_Nil
    jmp near List_Pair_i64_i64_41211_Cons

List_Pair_i64_i64_41211_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_41211_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41214
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab41212
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41212:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab41213
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41213:
    jmp lab41215

lab41214:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab41215:
    ; substitute (x14 !-> x)(a8 !-> a)(x !-> x)(a0 !-> a0)(a !-> a);
    ; #share a
    cmp rsi, 0
    je lab41216
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41216:
    ; #share x
    cmp r8, 0
    je lab41217
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41217:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov r12, rsi
    mov r13, rdi
    mov rax, r8
    mov rdx, r9
    ; new a1: List[Pair[i64, i64]] = (x, a0, a)\{ ... \};
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
    je lab41229
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41230

lab41229:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41227
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41219

lab41218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41219:

lab41220:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41222

lab41221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41222:

lab41223:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41225

lab41224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41225:

lab41226:
    jmp lab41228

lab41227:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41228:

lab41230:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_41231]
    ; new a2: List[Pair[i64, i64]] = (a1)\{ ... \};
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
    je lab41243
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41244

lab41243:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41241
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41234
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41232
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41233

lab41232:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41233:

lab41234:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41237
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41235
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41236

lab41235:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41236:

lab41237:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41240
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41238
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41239

lab41238:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41239:

lab41240:
    jmp lab41242

lab41241:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41242:

lab41244:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_41245]
    ; substitute (x14 !-> x14)(a2 !-> a2)(a8 !-> a8);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new x2: Fun[Pair[i64, i64], Bool] = (a8)\{ ... \};
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
    je lab41257
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41258

lab41257:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41255
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41248
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41246
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41247

lab41246:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41247:

lab41248:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41251
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41249
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41250

lab41249:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41250:

lab41251:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41254
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41252
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41253

lab41252:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41253:

lab41254:
    jmp lab41256

lab41255:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41256:

lab41258:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_41259]
    ; substitute (x14 !-> x14)(x2 !-> x2)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump filter_
    jmp filter_

Fun_Pair_i64_i64_Bool_41259:

Fun_Pair_i64_i64_Bool_41259_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41261
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41260
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41260:
    jmp lab41262

lab41261:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41262:
    ; substitute (a8 !-> a8)(b !-> b)(a3 !-> a3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexless_
    jmp lexless_

List_Pair_i64_i64_41245:
    jmp near List_Pair_i64_i64_41245_Nil
    jmp near List_Pair_i64_i64_41245_Cons

List_Pair_i64_i64_41245_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41264
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab41263
    ; ####increment refcount
    add qword [rax + 0], 1

lab41263:
    jmp lab41265

lab41264:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab41265:
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexordset_
    jmp lexordset_

List_Pair_i64_i64_41245_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41267
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41266
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41266:
    jmp lab41268

lab41267:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41268:
    ; substitute (a1 !-> a1)(x13 !-> x13)(xs4 !-> xs4);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x1: List[Pair[i64, i64]] = Cons(x13, xs4);
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
    je lab41280
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab41281

lab41280:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41278
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41271
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41269
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41270

lab41269:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41270:

lab41271:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41274
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41272
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41273

lab41272:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41273:

lab41274:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41277
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41275
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41276

lab41275:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41276:

lab41277:
    jmp lab41279

lab41278:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41279:

lab41281:
    ; #load tag
    mov rdi, 5
    ; substitute (x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexordset_
    jmp lexordset_

List_Pair_i64_i64_41231:
    jmp near List_Pair_i64_i64_41231_Nil
    jmp near List_Pair_i64_i64_41231_Cons

List_Pair_i64_i64_41231_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41285
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab41282
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41282:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41283
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41283:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41284
    ; ####increment refcount
    add qword [rax + 0], 1

lab41284:
    jmp lab41286

lab41285:
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

lab41286:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a !-> a)(a0 !-> a0)(x !-> x)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_lexordset_0_
    jmp lift_lexordset_0_

List_Pair_i64_i64_41231_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41290
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab41287
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41287:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41288
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41288:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41289
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41289:
    jmp lab41291

lab41290:
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

lab41291:
    ; substitute (a !-> a)(a0 !-> a0)(x !-> x)(x12 !-> x12)(xs3 !-> xs3);
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
    ; let x0: List[Pair[i64, i64]] = Cons(x12, xs3);
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
    je lab41303
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41304

lab41303:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41301
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41294
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41292
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41293

lab41292:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41293:

lab41294:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41297
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41295
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41296

lab41295:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41296:

lab41297:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41300
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41298
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41299

lab41298:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41299:

lab41300:
    jmp lab41302

lab41301:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41302:

lab41304:
    ; #load tag
    mov r11, 5
    ; jump lift_lexordset_0_
    jmp lift_lexordset_0_

lift_lexordset_0_:
    ; substitute (a !-> a)(x !-> x)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a4: List[Pair[i64, i64]] = (a0, x0)\{ ... \};
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
    je lab41316
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41317

lab41316:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41314
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41307
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41305
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41306

lab41305:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41306:

lab41307:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41310
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41308
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41309

lab41308:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41309:

lab41310:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41313
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41311
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41312

lab41311:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41312:

lab41313:
    jmp lab41315

lab41314:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41315:

lab41317:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_41318]
    ; let x5: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a !-> a)(x !-> x)(a4 !-> a4)(a8 !-> a)(x5 !-> x5);
    ; #share a
    cmp rax, 0
    je lab41319
    ; ####increment refcount
    add qword [rax + 0], 1

lab41319:
    ; #move variables
    mov r12, r10
    mov r10, rax
    mov r13, r11
    mov r11, rdx
    ; let x4: List[Pair[i64, i64]] = Cons(a8, x5);
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
    je lab41331
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41332

lab41331:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41329
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41322
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41320
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41321

lab41320:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41321:

lab41322:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41325
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41323
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41324

lab41323:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41324:

lab41325:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41328
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41326
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41327

lab41326:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41327:

lab41328:
    jmp lab41330

lab41329:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41330:

lab41332:
    ; #load tag
    mov r11, 5
    ; new a5: List[Pair[i64, i64]] = (a4, x4)\{ ... \};
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
    je lab41344
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41345

lab41344:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41342
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41335
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41333
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41334

lab41333:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41334:

lab41335:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41338
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41336
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41337

lab41336:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41337:

lab41338:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41341
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41339
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41340

lab41339:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41340:

lab41341:
    jmp lab41343

lab41342:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41343:

lab41345:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_41346]
    ; new a6: List[Pair[i64, i64]] = (a5)\{ ... \};
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
    je lab41358
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41359

lab41358:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41356
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41349
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41347
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41348

lab41347:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41348:

lab41349:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41352
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41350
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41351

lab41350:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41351:

lab41352:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41355
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41353
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41354

lab41353:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41354:

lab41355:
    jmp lab41357

lab41356:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41357:

lab41359:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_41360]
    ; substitute (a6 !-> a6)(x !-> x)(a !-> a);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new x8: Fun[Pair[i64, i64], Bool] = (a)\{ ... \};
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
    je lab41372
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41373

lab41372:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41370
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41363
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41361
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41362

lab41361:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41362:

lab41363:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41366
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41364
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41365

lab41364:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41365:

lab41366:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41369
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41367
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41368

lab41367:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41368:

lab41369:
    jmp lab41371

lab41370:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41371:

lab41373:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_41374]
    ; substitute (x !-> x)(x8 !-> x8)(a6 !-> a6);
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

Fun_Pair_i64_i64_Bool_41374:

Fun_Pair_i64_i64_Bool_41374_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41376
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41375
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41375:
    jmp lab41377

lab41376:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41377:
    ; substitute (a !-> a)(b0 !-> b0)(a7 !-> a7);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexgreater_
    jmp lexgreater_

List_Pair_i64_i64_41360:
    jmp near List_Pair_i64_i64_41360_Nil
    jmp near List_Pair_i64_i64_41360_Cons

List_Pair_i64_i64_41360_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41379
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab41378
    ; ####increment refcount
    add qword [rax + 0], 1

lab41378:
    jmp lab41380

lab41379:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab41380:
    ; let x7: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x7 !-> x7)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexordset_
    jmp lexordset_

List_Pair_i64_i64_41360_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41382
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41381
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41381:
    jmp lab41383

lab41382:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41383:
    ; substitute (a5 !-> a5)(x11 !-> x11)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x7: List[Pair[i64, i64]] = Cons(x11, xs2);
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
    je lab41395
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab41396

lab41395:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41393
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41385

lab41384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41385:

lab41386:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41388

lab41387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41388:

lab41389:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41392
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41390
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41391

lab41390:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41391:

lab41392:
    jmp lab41394

lab41393:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41394:

lab41396:
    ; #load tag
    mov rdi, 5
    ; substitute (x7 !-> x7)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexordset_
    jmp lexordset_

List_Pair_i64_i64_41346:
    jmp near List_Pair_i64_i64_41346_Nil
    jmp near List_Pair_i64_i64_41346_Cons

List_Pair_i64_i64_41346_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41399
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab41397
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41397:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab41398
    ; ####increment refcount
    add qword [rax + 0], 1

lab41398:
    jmp lab41400

lab41399:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab41400:
    ; let x6: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x4 !-> x4)(x6 !-> x6)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump append_
    jmp append_

List_Pair_i64_i64_41346_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41403
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab41401
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41401:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab41402
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41402:
    jmp lab41404

lab41403:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab41404:
    ; substitute (x4 !-> x4)(a4 !-> a4)(x10 !-> x10)(xs1 !-> xs1);
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
    ; let x6: List[Pair[i64, i64]] = Cons(x10, xs1);
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
    je lab41416
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41417

lab41416:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41414
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41407
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41405
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41406

lab41405:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41406:

lab41407:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41410
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41408
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41409

lab41408:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41409:

lab41410:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41413
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41411
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41412

lab41411:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41412:

lab41413:
    jmp lab41415

lab41414:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41415:

lab41417:
    ; #load tag
    mov r9, 5
    ; substitute (x4 !-> x4)(x6 !-> x6)(a4 !-> a4);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump append_
    jmp append_

List_Pair_i64_i64_41318:
    jmp near List_Pair_i64_i64_41318_Nil
    jmp near List_Pair_i64_i64_41318_Cons

List_Pair_i64_i64_41318_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41420
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab41418
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41418:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab41419
    ; ####increment refcount
    add qword [rax + 0], 1

lab41419:
    jmp lab41421

lab41420:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab41421:
    ; let x3: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump append_
    jmp append_

List_Pair_i64_i64_41318_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41424
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab41422
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41422:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab41423
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41423:
    jmp lab41425

lab41424:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab41425:
    ; substitute (x0 !-> x0)(a0 !-> a0)(x9 !-> x9)(xs0 !-> xs0);
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
    ; let x3: List[Pair[i64, i64]] = Cons(x9, xs0);
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
    je lab41437
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41438

lab41437:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41435
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41428
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41426
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41427

lab41426:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41427:

lab41428:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41431
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41429
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41430

lab41429:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41430:

lab41431:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41434
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41432
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41433

lab41432:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41433:

lab41434:
    jmp lab41436

lab41435:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41436:

lab41438:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump append_
    jmp append_

lexless_:
    ; substitute (a0 !-> a0)(b !-> b)(a !-> a);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch a \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_41439:

Pair_i64_i64_41439_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41440
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab41441

lab41440:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab41441:
    ; substitute (a0 !-> a0)(snd1 !-> snd1)(fst1 !-> fst1)(b !-> b);
    ; #move variables
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch b \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_41442:

Pair_i64_i64_41442_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab41443
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]
    jmp lab41444

lab41443:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]

lab41444:
    ; if fst2 < fst1 \{ ... \}
    cmp r11, r9
    jl lab41445
    ; if fst2 == fst1 \{ ... \}
    cmp r11, r9
    je lab41446
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab41446:
    ; if snd2 < snd1 \{ ... \}
    cmp r13, rdi
    jl lab41447
    ; substitute (a0 !-> a0);
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab41447:
    ; substitute (a0 !-> a0);
    ; invoke a0 True
    add rdx, 0
    jmp rdx

lab41445:
    ; substitute (a0 !-> a0);
    ; invoke a0 True
    add rdx, 0
    jmp rdx

lexgreater_:
    ; substitute (b !-> b)(a !-> a)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lexless_
    jmp lexless_

diff_:
    ; substitute (x !-> x)(a0 !-> a0)(y !-> y);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new x0: Fun[Pair[i64, i64], Bool] = (y)\{ ... \};
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
    je lab41459
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41460

lab41459:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41457
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41450
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41448
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41449

lab41448:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41449:

lab41450:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41453
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41451
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41452

lab41451:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41452:

lab41453:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41456
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41454
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41455

lab41454:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41455:

lab41456:
    jmp lab41458

lab41457:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41458:

lab41460:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_41461]
    ; substitute (x !-> x)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump filter_
    jmp filter_

Fun_Pair_i64_i64_Bool_41461:

Fun_Pair_i64_i64_Bool_41461_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab41463
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab41462
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41462:
    jmp lab41464

lab41463:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab41464:
    ; substitute (p !-> p)(y !-> y)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a2: Bool = (a1)\{ ... \};
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
    je lab41476
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41477

lab41476:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41474
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41467
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41465
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41466

lab41465:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41466:

lab41467:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41470
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41468
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41469

lab41468:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41469:

lab41470:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41473
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41471
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41472

lab41471:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41472:

lab41473:
    jmp lab41475

lab41474:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41475:

lab41477:
    ; #load tag
    lea r9, [rel Bool_41478]
    ; substitute (y !-> y)(p !-> p)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump member_
    jmp member_

Bool_41478:
    jmp near Bool_41478_True
    jmp near Bool_41478_False

Bool_41478_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41480
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab41479
    ; ####increment refcount
    add qword [rax + 0], 1

lab41479:
    jmp lab41481

lab41480:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab41481:
    ; let x1: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_41478_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41483
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab41482
    ; ####increment refcount
    add qword [rax + 0], 1

lab41482:
    jmp lab41484

lab41483:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab41484:
    ; let x1: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

collect_neighbors_:
    ; substitute (xover !-> xover)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1)(a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; switch xs \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41485]
    add rcx, r15
    jmp rcx

List_Pair_i64_i64_41485:
    jmp near List_Pair_i64_i64_41485_Nil
    jmp near List_Pair_i64_i64_41485_Cons

List_Pair_i64_i64_41485_Nil:
    ; substitute (x3 !-> x3)(xover !-> xover)(a0 !-> a0);
    ; #erase x1
    cmp r10, 0
    je lab41488
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab41486
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab41487

lab41486:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab41487:

lab41488:
    ; #erase x2
    cmp r8, 0
    je lab41491
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab41489
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab41490

lab41489:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab41490:

lab41491:
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r12
    mov r9, r13
    ; jump diff_
    jmp diff_

List_Pair_i64_i64_41485_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab41494
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41492
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41492:
    mov r15, [r14 + 40]
    mov r14, [r14 + 32]
    cmp r14, 0
    je lab41493
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41493:
    jmp lab41495

lab41494:
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
    mov r14, [r14 + 32]

lab41495:
    ; substitute (xover0 !-> xover)(a8 !-> a)(x2 !-> x2)(x1 !-> x1)(a0 !-> a0)(a !-> a)(x !-> x)(xover !-> xover)(x3 !-> x3);
    ; #share a
    cmp r14, 0
    je lab41496
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41496:
    ; #share xover
    cmp rax, 0
    je lab41497
    ; ####increment refcount
    add qword [rax + 0], 1

lab41497:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 2000], rsi
    mov [rsp + 1992], rdi
    mov rsi, r14
    mov rdi, r15
    ; new a1: Bool = (x2, x1, a0, a, x, xover, x3)\{ ... \};
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41509
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41510

lab41509:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41507
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41499

lab41498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41499:

lab41500:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41502

lab41501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41502:

lab41503:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41506
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41504
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41505

lab41504:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41505:

lab41506:
    jmp lab41508

lab41507:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41508:

lab41510:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41522
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41523

lab41522:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41520
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41512

lab41511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41512:

lab41513:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41515

lab41514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41515:

lab41516:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41518

lab41517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41518:

lab41519:
    jmp lab41521

lab41520:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41521:

lab41523:
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
    je lab41535
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41536

lab41535:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41533
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41526
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41524
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41525

lab41524:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41525:

lab41526:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41528

lab41527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41528:

lab41529:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41531

lab41530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41531:

lab41532:
    jmp lab41534

lab41533:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41534:

lab41536:
    ; #load tag
    lea r9, [rel Bool_41537]
    ; jump member_
    jmp member_

Bool_41537:
    jmp near Bool_41537_True
    jmp near Bool_41537_False

Bool_41537_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41545
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41538
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41538:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41539
    ; ####increment refcount
    add qword [rax + 0], 1

lab41539:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41540
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41540:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41541
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41541:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41542
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41542:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41543
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41543:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41544
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41544:
    jmp lab41546

lab41545:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41546:
    ; substitute (xover !-> xover)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1)(x !-> x)(a0 !-> a0);
    ; #erase a
    cmp r10, 0
    je lab41549
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab41547
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab41548

lab41547:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab41548:

lab41549:
    ; #move variables
    mov rcx, r14
    mov r14, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov r11, rdi
    mov rsi, [rsp + 2032]
    mov rdi, [rsp + 2024]
    ; jump collect_neighbors_
    jmp collect_neighbors_

Bool_41537_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41557
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41550
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41550:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41551
    ; ####increment refcount
    add qword [rax + 0], 1

lab41551:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41552
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41552:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41553
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41553:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41554
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41554:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41555
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41555:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41556
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41556:
    jmp lab41558

lab41557:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41558:
    ; substitute (x30 !-> x3)(a7 !-> a)(a0 !-> a0)(a !-> a)(x !-> x)(xover !-> xover)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1);
    ; #share a
    cmp r10, 0
    je lab41559
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41559:
    ; #share x3
    cmp qword [rsp + 2032], 0
    je lab41560
    mov rcx, [rsp + 2032]
    add qword [rcx + 0], 1

lab41560:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 2000], rsi
    mov [rsp + 1992], rdi
    mov rsi, r10
    mov rdi, r11
    mov rax, [rsp + 2032]
    mov rdx, [rsp + 2024]
    ; new a2: Bool = (a0, a, x, xover, x3, x2, x1)\{ ... \};
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41572
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41573

lab41572:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41570
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41562

lab41561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41562:

lab41563:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41565

lab41564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41565:

lab41566:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41569
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41567
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41568

lab41567:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41568:

lab41569:
    jmp lab41571

lab41570:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41571:

lab41573:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41585
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41586

lab41585:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41583
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41576
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41574
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41575

lab41574:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41575:

lab41576:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41579
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41577
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41578

lab41577:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41578:

lab41579:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41582
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41580
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41581

lab41580:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41581:

lab41582:
    jmp lab41584

lab41583:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41584:

lab41586:
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
    je lab41598
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41599

lab41598:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41596
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41589
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41587
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41588

lab41587:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41588:

lab41589:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41592
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41590
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41591

lab41590:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41591:

lab41592:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41595
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41593
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41594

lab41593:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41594:

lab41595:
    jmp lab41597

lab41596:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41597:

lab41599:
    ; #load tag
    lea r9, [rel Bool_41600]
    ; jump member_
    jmp member_

Bool_41600:
    jmp near Bool_41600_True
    jmp near Bool_41600_False

Bool_41600_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41608
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41601
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41601:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41602
    ; ####increment refcount
    add qword [rax + 0], 1

lab41602:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41603
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41603:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41604
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41604:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41605
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41605:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41606
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41606:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41607
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41607:
    jmp lab41609

lab41608:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41609:
    ; substitute (a0 !-> a0)(x1 !-> x1)(x !-> x)(x2 !-> x2)(x3 !-> x3)(a !-> a)(xover !-> xover);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], r10
    mov r10, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(a, xover);
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
    je lab41621
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41622

lab41621:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41619
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41612
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41610
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41611

lab41610:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41611:

lab41612:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41615
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41613
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41614

lab41613:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41614:

lab41615:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41618
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41616
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41617

lab41616:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41617:

lab41618:
    jmp lab41620

lab41619:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41620:

lab41622:
    ; #load tag
    mov r15, 5
    ; substitute (x0 !-> x0)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump collect_neighbors_
    jmp collect_neighbors_

Bool_41600_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41630
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41623
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41623:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41624
    ; ####increment refcount
    add qword [rax + 0], 1

lab41624:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41625
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41625:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41626
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41626:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41627
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41627:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41628
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41628:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41629
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41629:
    jmp lab41631

lab41630:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41631:
    ; substitute (x20 !-> x2)(a6 !-> a)(x !-> x)(xover !-> xover)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1)(a0 !-> a0)(a !-> a);
    ; #share a
    cmp rsi, 0
    je lab41632
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41632:
    ; #share x2
    cmp r14, 0
    je lab41633
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41633:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 2000], rsi
    mov [rsp + 1992], rdi
    mov rax, r14
    mov rdx, r15
    ; new a3: Bool = (x, xover, x3, x2, x1, a0, a)\{ ... \};
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41645
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41646

lab41645:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41643
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41635

lab41634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41635:

lab41636:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41639
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41637
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41638

lab41637:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41638:

lab41639:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41642
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41640
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41641

lab41640:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41641:

lab41642:
    jmp lab41644

lab41643:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41644:

lab41646:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41658
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41659

lab41658:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41656
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41648

lab41647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41648:

lab41649:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41652
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41650
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41651

lab41650:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41651:

lab41652:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41655
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41653
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41654

lab41653:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41654:

lab41655:
    jmp lab41657

lab41656:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41657:

lab41659:
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
    je lab41671
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41672

lab41671:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41669
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41661

lab41660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41661:

lab41662:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41665
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41663
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41664

lab41663:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41664:

lab41665:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41668
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41666
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41667

lab41666:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41667:

lab41668:
    jmp lab41670

lab41669:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41670:

lab41672:
    ; #load tag
    lea r9, [rel Bool_41673]
    ; jump member_
    jmp member_

Bool_41673:
    jmp near Bool_41673_True
    jmp near Bool_41673_False

Bool_41673_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41681
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41674
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41674:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41675
    ; ####increment refcount
    add qword [rax + 0], 1

lab41675:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41676
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41676:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41677
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41677:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41678
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41678:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41679
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41679:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41680
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41680:
    jmp lab41682

lab41681:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41682:
    ; substitute (x !-> x)(xover !-> xover)(a0 !-> a0)(x2 !-> x2)(x1 !-> x1)(a !-> a)(x3 !-> x3);
    ; #move variables
    mov rcx, r14
    mov r14, [rsp + 2032]
    mov [rsp + 2032], r8
    mov r8, rcx
    mov rcx, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, rcx
    ; let x4: List[Pair[i64, i64]] = Cons(a, x3);
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
    je lab41694
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41695

lab41694:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41692
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41684

lab41683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41684:

lab41685:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41687

lab41686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41687:

lab41688:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41691
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41689
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41690

lab41689:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41690:

lab41691:
    jmp lab41693

lab41692:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41693:

lab41695:
    ; #load tag
    mov r15, 5
    ; substitute (xover !-> xover)(x4 !-> x4)(x2 !-> x2)(x1 !-> x1)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r14
    mov r14, r8
    mov r8, r10
    mov r10, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r15
    mov r15, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump collect_neighbors_
    jmp collect_neighbors_

Bool_41673_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41703
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41696
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41696:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41697
    ; ####increment refcount
    add qword [rax + 0], 1

lab41697:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41698
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41698:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41699
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41699:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41700
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41700:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41701
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41701:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41702
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41702:
    jmp lab41704

lab41703:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41704:
    ; substitute (a5 !-> a)(x10 !-> x1)(x3 !-> x3)(x2 !-> x2)(x1 !-> x1)(a0 !-> a0)(a !-> a)(x !-> x)(xover !-> xover);
    ; #share a
    cmp qword [rsp + 2032], 0
    je lab41705
    mov rcx, [rsp + 2032]
    add qword [rcx + 0], 1

lab41705:
    ; #share x1
    cmp r12, 0
    je lab41706
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41706:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 2000], rsi
    mov [rsp + 1992], rdi
    mov rsi, r12
    mov rdi, r13
    mov rax, [rsp + 2032]
    mov rdx, [rsp + 2024]
    ; new a4: Bool = (x3, x2, x1, a0, a, x, xover)\{ ... \};
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41718
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41719

lab41718:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41716
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41709
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41707
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41708

lab41707:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41708:

lab41709:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41712
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41710
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41711

lab41710:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41711:

lab41712:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41715
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41713
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41714

lab41713:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41714:

lab41715:
    jmp lab41717

lab41716:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41717:

lab41719:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41731
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41732

lab41731:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41729
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41722
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41720
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41721

lab41720:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41721:

lab41722:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41725
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41723
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41724

lab41723:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41724:

lab41725:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41727

lab41726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41727:

lab41728:
    jmp lab41730

lab41729:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41730:

lab41732:
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
    je lab41744
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41745

lab41744:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41742
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41734

lab41733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41734:

lab41735:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41737

lab41736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41737:

lab41738:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41740

lab41739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41740:

lab41741:
    jmp lab41743

lab41742:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41743:

lab41745:
    ; #load tag
    lea r9, [rel Bool_41746]
    ; substitute (x10 !-> x10)(a5 !-> a5)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump member_
    jmp member_

Bool_41746:
    jmp near Bool_41746_True
    jmp near Bool_41746_False

Bool_41746_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41754
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41747
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41747:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41748
    ; ####increment refcount
    add qword [rax + 0], 1

lab41748:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41749
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41749:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41750
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41750:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41751
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41751:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41752
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41752:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41753
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41753:
    jmp lab41755

lab41754:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41755:
    ; substitute (x3 !-> x3)(xover !-> xover)(x1 !-> x1)(a0 !-> a0)(x !-> x)(a !-> a)(x2 !-> x2);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; let x5: List[Pair[i64, i64]] = Cons(a, x2);
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
    je lab41767
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41768

lab41767:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41765
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41758
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41756
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41757

lab41756:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41757:

lab41758:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41760

lab41759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41760:

lab41761:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41763

lab41762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41763:

lab41764:
    jmp lab41766

lab41765:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41766:

lab41768:
    ; #load tag
    mov r15, 5
    ; substitute (xover !-> xover)(x3 !-> x3)(x5 !-> x5)(x1 !-> x1)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r14
    mov r14, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r15
    mov r15, r11
    mov r11, r9
    mov r9, rcx
    ; jump collect_neighbors_
    jmp collect_neighbors_

Bool_41746_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab41776
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab41769
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41769:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab41770
    ; ####increment refcount
    add qword [rax + 0], 1

lab41770:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab41771
    ; ####increment refcount
    add qword [r10 + 0], 1

lab41771:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab41772
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41772:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab41773
    ; ####increment refcount
    add qword [rcx + 0], 1

lab41773:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab41774
    ; ####increment refcount
    add qword [r14 + 0], 1

lab41774:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab41775
    ; ####increment refcount
    add qword [r12 + 0], 1

lab41775:
    jmp lab41777

lab41776:
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
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab41777:
    ; substitute (x3 !-> x3)(x2 !-> x2)(xover !-> xover)(a0 !-> a0)(x !-> x)(a !-> a)(x1 !-> x1);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], r8
    mov r8, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, rcx
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; let x6: List[Pair[i64, i64]] = Cons(a, x1);
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
    je lab41789
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41790

lab41789:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41787
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41780
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41778
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41779

lab41778:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41779:

lab41780:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41783
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41781
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41782

lab41781:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41782:

lab41783:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41785

lab41784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41785:

lab41786:
    jmp lab41788

lab41787:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41788:

lab41790:
    ; #load tag
    mov r15, 5
    ; substitute (xover !-> xover)(x3 !-> x3)(x2 !-> x2)(x6 !-> x6)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r14
    mov r14, r10
    mov r10, rcx
    mov rcx, r15
    mov r15, r11
    mov r11, rcx
    ; jump collect_neighbors_
    jmp collect_neighbors_

occurs3_:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; let x2: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x3: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (x0 !-> x0)(x1 !-> x1)(x2 !-> x2)(x3 !-> x3)(l !-> l)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, r15
    mov r15, rdi
    mov rdi, rcx
    ; jump collect_neighbors_
    jmp collect_neighbors_

neighbours_:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_41791:

Pair_i64_i64_41791_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41792
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab41793

lab41792:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab41793:
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- fst - x1;
    mov r13, rdi
    sub r13, r11
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x2 !-> x2);
    ; #move variables
    mov r11, r13
    ; lit x3 <- 1;
    mov r13, 1
    ; x4 <- snd - x3;
    mov r15, r9
    sub r15, r13
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x2 !-> x2)(x4 !-> x4);
    ; #move variables
    mov r13, r15
    ; let x0: Pair[i64, i64] = Tup(x2, x4);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab41805
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41806

lab41805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41795

lab41794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41795:

lab41796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41798

lab41797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41798:

lab41799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41801

lab41800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41801:

lab41802:
    jmp lab41804

lab41803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41804:

lab41806:
    ; #load tag
    mov r11, 0
    ; lit x7 <- 1;
    mov r13, 1
    ; x8 <- fst - x7;
    mov r15, rdi
    sub r15, r13
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x8 !-> x8)(snd0 !-> snd);
    ; #move variables
    mov r13, r15
    mov r15, r9
    ; let x6: Pair[i64, i64] = Tup(x8, snd0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab41818
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41819

lab41818:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41816
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41809
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41807
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41808

lab41807:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41808:

lab41809:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41812
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41810
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41811

lab41810:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41811:

lab41812:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41815
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41813
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41814

lab41813:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41814:

lab41815:
    jmp lab41817

lab41816:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41817:

lab41819:
    ; #load tag
    mov r13, 0
    ; lit x11 <- 1;
    mov r15, 1
    ; x12 <- fst - x11;
    mov rcx, rdi
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x12 !-> x12);
    ; #move variables
    mov r15, [rsp + 2024]
    ; lit x13 <- 1;
    mov qword [rsp + 2024], 1
    ; x14 <- snd + x13;
    mov rcx, r9
    add rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x12 !-> x12)(x14 !-> x14);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2024], rcx
    ; let x10: Pair[i64, i64] = Tup(x12, x14);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab41831
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41832

lab41831:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41829
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41822
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41820
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41821

lab41820:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41821:

lab41822:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41825
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41823
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41824

lab41823:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41824:

lab41825:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41827

lab41826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41827:

lab41828:
    jmp lab41830

lab41829:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41830:

lab41832:
    ; #load tag
    mov r15, 0
    ; lit x17 <- 1;
    mov qword [rsp + 2024], 1
    ; x18 <- snd - x17;
    mov rcx, r9
    sub rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(fst0 !-> fst)(x18 !-> x18);
    ; #move variables
    mov [rsp + 2024], rdi
    ; let x16: Pair[i64, i64] = Tup(fst0, x18);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41844
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41845

lab41844:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41842
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41835
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41833
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41834

lab41833:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41834:

lab41835:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41837

lab41836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41837:

lab41838:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41840

lab41839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41840:

lab41841:
    jmp lab41843

lab41842:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41843:

lab41845:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; lit x21 <- 1;
    mov qword [rsp + 2008], 1
    ; x22 <- snd + x21;
    mov rcx, r9
    add rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(fst1 !-> fst)(x22 !-> x22);
    ; #move variables
    mov [rsp + 2008], rdi
    ; let x20: Pair[i64, i64] = Tup(fst1, x22);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41857
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41858

lab41857:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41855
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41848
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41846
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41847

lab41846:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41847:

lab41848:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41850

lab41849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41850:

lab41851:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41854
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41852
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41853

lab41852:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41853:

lab41854:
    jmp lab41856

lab41855:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41856:

lab41858:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; lit x25 <- 1;
    mov qword [rsp + 1992], 1
    ; x26 <- fst + x25;
    mov rcx, rdi
    add rcx, [rsp + 1992]
    mov [rsp + 1976], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x20 !-> x20)(x26 !-> x26);
    ; #move variables
    mov rcx, [rsp + 1976]
    mov [rsp + 1992], rcx
    ; lit x27 <- 1;
    mov qword [rsp + 1976], 1
    ; x28 <- snd - x27;
    mov rcx, r9
    sub rcx, [rsp + 1976]
    mov [rsp + 1960], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x20 !-> x20)(x26 !-> x26)(x28 !-> x28);
    ; #move variables
    mov rcx, [rsp + 1960]
    mov [rsp + 1976], rcx
    ; let x24: Pair[i64, i64] = Tup(x26, x28);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41870
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41871

lab41870:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41868
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41861
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41859
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41860

lab41859:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41860:

lab41861:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41863

lab41862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41863:

lab41864:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41866

lab41865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41866:

lab41867:
    jmp lab41869

lab41868:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41869:

lab41871:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; lit x31 <- 1;
    mov qword [rsp + 1976], 1
    ; x32 <- fst + x31;
    mov rcx, rdi
    add rcx, [rsp + 1976]
    mov [rsp + 1960], rcx
    ; substitute (a0 !-> a0)(fst !-> fst)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x20 !-> x20)(x24 !-> x24)(x32 !-> x32)(snd1 !-> snd);
    ; #move variables
    mov rcx, [rsp + 1960]
    mov [rsp + 1976], rcx
    mov [rsp + 1960], r9
    ; let x30: Pair[i64, i64] = Tup(x32, snd1);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41883
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41884

lab41883:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41881
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41874
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41872
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41873

lab41872:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41873:

lab41874:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41876

lab41875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41876:

lab41877:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41880
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41878
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41879

lab41878:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41879:

lab41880:
    jmp lab41882

lab41881:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41882:

lab41884:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; lit x35 <- 1;
    mov qword [rsp + 1960], 1
    ; x36 <- fst + x35;
    mov rcx, rdi
    add rcx, [rsp + 1960]
    mov [rsp + 1944], rcx
    ; substitute (a0 !-> a0)(x36 !-> x36)(snd !-> snd)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x20 !-> x20)(x24 !-> x24)(x30 !-> x30);
    ; #move variables
    mov rdi, [rsp + 1944]
    ; lit x37 <- 1;
    mov qword [rsp + 1960], 1
    ; x38 <- snd + x37;
    mov rcx, r9
    add rcx, [rsp + 1960]
    mov [rsp + 1944], rcx
    ; substitute (a0 !-> a0)(x30 !-> x30)(x24 !-> x24)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x20 !-> x20)(x36 !-> x36)(x38 !-> x38);
    ; #move variables
    mov r9, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov r8, [rsp + 2000]
    mov rsi, [rsp + 1984]
    mov rdi, [rsp + 1976]
    mov rcx, [rsp + 1944]
    mov [rsp + 1976], rcx
    ; let x34: Pair[i64, i64] = Tup(x36, x38);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41896
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41897

lab41896:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41894
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41886

lab41885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41886:

lab41887:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41889

lab41888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41889:

lab41890:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41892

lab41891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41892:

lab41893:
    jmp lab41895

lab41894:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41895:

lab41897:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; let x39: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1984], 0
    ; #load tag
    mov qword [rsp + 1976], 0
    ; let x33: List[Pair[i64, i64]] = Cons(x34, x39);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab41909
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41910

lab41909:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41907
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41899

lab41898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41899:

lab41900:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41902

lab41901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41902:

lab41903:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41905

lab41904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41905:

lab41906:
    jmp lab41908

lab41907:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41908:

lab41910:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(x20 !-> x20)(x24 !-> x24)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x16 !-> x16)(x30 !-> x30)(x33 !-> x33);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, rcx
    ; let x29: List[Pair[i64, i64]] = Cons(x30, x33);
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
    je lab41922
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41923

lab41922:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41920
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41912

lab41911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41912:

lab41913:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41915

lab41914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41915:

lab41916:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41918

lab41917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41918:

lab41919:
    jmp lab41921

lab41920:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41921:

lab41923:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(x20 !-> x20)(x16 !-> x16)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x24 !-> x24)(x29 !-> x29);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], r8
    mov r8, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, rcx
    ; let x23: List[Pair[i64, i64]] = Cons(x24, x29);
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
    je lab41935
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab41936

lab41935:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41933
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41926
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41924
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41925

lab41924:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41925:

lab41926:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41928

lab41927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41928:

lab41929:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41932
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41930
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41931

lab41930:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41931:

lab41932:
    jmp lab41934

lab41933:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41934:

lab41936:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(x10 !-> x10)(x16 !-> x16)(x0 !-> x0)(x6 !-> x6)(x20 !-> x20)(x23 !-> x23);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x19: List[Pair[i64, i64]] = Cons(x20, x23);
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
    je lab41948
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab41949

lab41948:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41946
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41939
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41938

lab41937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41938:

lab41939:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41941

lab41940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41941:

lab41942:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41945
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41943
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41944

lab41943:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41944:

lab41945:
    jmp lab41947

lab41946:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41947:

lab41949:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x10 !-> x10)(x6 !-> x6)(x0 !-> x0)(x16 !-> x16)(x19 !-> x19);
    ; #move variables
    mov rcx, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; let x15: List[Pair[i64, i64]] = Cons(x16, x19);
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
    je lab41961
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab41962

lab41961:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41959
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41952
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41950
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41951

lab41950:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41951:

lab41952:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41954

lab41953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41954:

lab41955:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41958
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41956
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41957

lab41956:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41957:

lab41958:
    jmp lab41960

lab41959:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41960:

lab41962:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x0 !-> x0)(x6 !-> x6)(x10 !-> x10)(x15 !-> x15);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x9: List[Pair[i64, i64]] = Cons(x10, x15);
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
    je lab41974
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab41975

lab41974:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41972
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41965
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41963
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41964

lab41963:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41964:

lab41965:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41968
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41966
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41967

lab41966:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41967:

lab41968:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41970

lab41969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41970:

lab41971:
    jmp lab41973

lab41972:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41973:

lab41975:
    ; #load tag
    mov r11, 5
    ; let x5: List[Pair[i64, i64]] = Cons(x6, x9);
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
    je lab41987
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab41988

lab41987:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab41985
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab41978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41977

lab41976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41977:

lab41978:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab41981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41980

lab41979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41980:

lab41981:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab41984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41983

lab41982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41983:

lab41984:
    jmp lab41986

lab41985:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab41986:

lab41988:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x5 !-> x5)(a0 !-> a0);
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

alive_:
    ; substitute (a0 !-> a0)(g !-> g);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch g \{ ... \};
    ; #if there is only one clause, we can just fall through

Gen_41989:

Gen_41989_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41991
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab41990
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41990:
    jmp lab41992

lab41991:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab41992:
    ; switch livecoords \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_41993]
    add rcx, rdi
    jmp rcx

List_Pair_i64_i64_41993:
    jmp near List_Pair_i64_i64_41993_Nil
    jmp near List_Pair_i64_i64_41993_Cons

List_Pair_i64_i64_41993_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_41993_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab41996
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab41994
    ; ####increment refcount
    add qword [r8 + 0], 1

lab41994:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab41995
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41995:
    jmp lab41997

lab41996:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab41997:
    ; substitute (x0 !-> x0)(xs0 !-> xs0)(a0 !-> a0);
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

mkgen_:
    ; new a1: List[Pair[i64, i64]] = (a0)\{ ... \};
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
    je lab42009
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42010

lab42009:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42007
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab41998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab41999

lab41998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab41999:

lab42000:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42002

lab42001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42002:

lab42003:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42005

lab42004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42005:

lab42006:
    jmp lab42008

lab42007:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42008:

lab42010:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_42011]
    ; jump lexordset_
    jmp lexordset_

List_Pair_i64_i64_42011:
    jmp near List_Pair_i64_i64_42011_Nil
    jmp near List_Pair_i64_i64_42011_Cons

List_Pair_i64_i64_42011_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42013
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42012
    ; ####increment refcount
    add qword [rax + 0], 1

lab42012:
    jmp lab42014

lab42013:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42014:
    ; let x0: List[Pair[i64, i64]] = Nil();
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
    ; invoke a0 Gen
    jmp rdi

List_Pair_i64_i64_42011_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42016
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42015
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42015:
    jmp lab42017

lab42016:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42017:
    ; substitute (a0 !-> a0)(x1 !-> x1)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(x1, xs0);
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
    je lab42029
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42030

lab42029:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42027
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42020
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42018
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42019

lab42018:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42019:

lab42020:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42023
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42021
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42022

lab42021:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42022:

lab42023:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42026
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42024
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42025

lab42024:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42025:

lab42026:
    jmp lab42028

lab42027:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42028:

lab42030:
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
    ; invoke a0 Gen
    jmp rdi

twoorthree_:
    ; lit x0 <- 2;
    mov r9, 2
    ; if n == x0 \{ ... \}
    cmp rdx, r9
    je lab42031
    ; substitute (n !-> n)(a0 !-> a0);
    ; lit x1 <- 3;
    mov r9, 3
    ; if n == x1 \{ ... \}
    cmp rdx, r9
    je lab42032
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab42032:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

lab42031:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

mk_nextgen_fn_:
    ; new a12: List[Pair[i64, i64]] = (a0)\{ ... \};
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
    je lab42044
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42045

lab42044:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42042
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42034

lab42033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42034:

lab42035:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42038
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42036
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42037

lab42036:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42037:

lab42038:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42041
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42039
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42040

lab42039:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42040:

lab42041:
    jmp lab42043

lab42042:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42043:

lab42045:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_42046]
    ; jump alive_
    jmp alive_

List_Pair_i64_i64_42046:
    jmp near List_Pair_i64_i64_42046_Nil
    jmp near List_Pair_i64_i64_42046_Cons

List_Pair_i64_i64_42046_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42048
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42047
    ; ####increment refcount
    add qword [rax + 0], 1

lab42047:
    jmp lab42049

lab42048:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42049:
    ; let living: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_mk_nextgen_fn_0_
    jmp lift_mk_nextgen_fn_0_

List_Pair_i64_i64_42046_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42051
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42050
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42050:
    jmp lab42052

lab42051:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42052:
    ; substitute (a0 !-> a0)(x16 !-> x16)(xs7 !-> xs7);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let living: List[Pair[i64, i64]] = Cons(x16, xs7);
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
    je lab42064
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42065

lab42064:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42062
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42055
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42053
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42054

lab42053:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42054:

lab42055:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42058
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42056
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42057

lab42056:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42057:

lab42058:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42061
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42059
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42060

lab42059:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42060:

lab42061:
    jmp lab42063

lab42062:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42063:

lab42065:
    ; #load tag
    mov rdi, 5
    ; jump lift_mk_nextgen_fn_0_
    jmp lift_mk_nextgen_fn_0_

lift_mk_nextgen_fn_0_:
    ; substitute (a0 !-> a0)(living0 !-> living)(living !-> living);
    ; #share living
    cmp rsi, 0
    je lab42066
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42066:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    ; new isalive: Fun[Pair[i64, i64], Bool] = (living)\{ ... \};
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
    je lab42078
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42079

lab42078:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42076
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42069
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42067
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42068

lab42067:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42068:

lab42069:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42072
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42070
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42071

lab42070:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42071:

lab42072:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42075
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42074

lab42073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42074:

lab42075:
    jmp lab42077

lab42076:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42077:

lab42079:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_42080]
    ; substitute (a0 !-> a0)(living0 !-> living0)(isalive0 !-> isalive)(isalive !-> isalive);
    ; #share isalive
    cmp r8, 0
    je lab42081
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42081:
    ; #move variables
    mov r10, r8
    mov r11, r9
    ; new liveneighbours: Fun[Pair[i64, i64], i64] = (isalive)\{ ... \};
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
    je lab42093
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42094

lab42093:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42091
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42083

lab42082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42083:

lab42084:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42086

lab42085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42086:

lab42087:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42089

lab42088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42089:

lab42090:
    jmp lab42092

lab42091:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42092:

lab42094:
    ; #load tag
    lea r11, [rel Fun_Pair_i64_i64_i64_42095]
    ; new x2: Fun[Pair[i64, i64], Bool] = (liveneighbours)\{ ... \};
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
    je lab42107
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42108

lab42107:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42105
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42098
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42096
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42097

lab42096:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42097:

lab42098:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42101
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42099
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42100

lab42099:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42100:

lab42101:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42104
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42102
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42103

lab42102:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42103:

lab42104:
    jmp lab42106

lab42105:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42106:

lab42108:
    ; #load tag
    lea r11, [rel Fun_Pair_i64_i64_Bool_42109]
    ; substitute (x2 !-> x2)(living00 !-> living0)(isalive0 !-> isalive0)(living0 !-> living0)(a0 !-> a0);
    ; #share living0
    cmp rsi, 0
    je lab42110
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42110:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a13: List[Pair[i64, i64]] = (isalive0, living0, a0)\{ ... \};
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
    je lab42122
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42123

lab42122:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42120
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42113
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42111
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42112

lab42111:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42112:

lab42113:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42116
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42114
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42115

lab42114:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42115:

lab42116:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42119
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42117
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42118

lab42117:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42118:

lab42119:
    jmp lab42121

lab42120:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42121:

lab42123:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_42124]
    ; substitute (living00 !-> living00)(x2 !-> x2)(a13 !-> a13);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump filter_
    jmp filter_

List_Pair_i64_i64_42124:
    jmp near List_Pair_i64_i64_42124_Nil
    jmp near List_Pair_i64_i64_42124_Cons

List_Pair_i64_i64_42124_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42128
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab42125
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42125:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab42126
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42126:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab42127
    ; ####increment refcount
    add qword [rax + 0], 1

lab42127:
    jmp lab42129

lab42128:
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

lab42129:
    ; let survivors: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(isalive0 !-> isalive0)(living0 !-> living0)(survivors !-> survivors);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_mk_nextgen_fn_1_
    jmp lift_mk_nextgen_fn_1_

List_Pair_i64_i64_42124_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42133
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab42130
    ; ####increment refcount
    add qword [r12 + 0], 1

lab42130:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab42131
    ; ####increment refcount
    add qword [r10 + 0], 1

lab42131:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab42132
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42132:
    jmp lab42134

lab42133:
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

lab42134:
    ; substitute (a0 !-> a0)(living0 !-> living0)(isalive0 !-> isalive0)(x15 !-> x15)(xs6 !-> xs6);
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
    ; let survivors: List[Pair[i64, i64]] = Cons(x15, xs6);
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
    je lab42146
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42147

lab42146:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42144
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42137
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42135
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42136

lab42135:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42136:

lab42137:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42140
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42138
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42139

lab42138:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42139:

lab42140:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42143
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42141
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42142

lab42141:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42142:

lab42143:
    jmp lab42145

lab42144:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42145:

lab42147:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(isalive0 !-> isalive0)(living0 !-> living0)(survivors !-> survivors);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_mk_nextgen_fn_1_
    jmp lift_mk_nextgen_fn_1_

Fun_Pair_i64_i64_Bool_42109:

Fun_Pair_i64_i64_Bool_42109_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42149
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42148
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42148:
    jmp lab42150

lab42149:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42150:
    ; substitute (p1 !-> p1)(liveneighbours !-> liveneighbours)(a6 !-> a6);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a7: _Cont = (a6)\{ ... \};
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
    je lab42162
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42163

lab42162:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42160
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42153
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42151
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42152

lab42151:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42152:

lab42153:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42156
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42154
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42155

lab42154:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42155:

lab42156:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42159
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42157
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42158

lab42157:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42158:

lab42159:
    jmp lab42161

lab42160:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42161:

lab42163:
    ; #load tag
    lea r9, [rel _Cont_42164]
    ; substitute (p1 !-> p1)(a7 !-> a7)(liveneighbours !-> liveneighbours);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke liveneighbours Apply
    jmp r9

_Cont_42164:

_Cont_42164_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab42166
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab42165
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42165:
    jmp lab42167

lab42166:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab42167:
    ; jump twoorthree_
    jmp twoorthree_

Fun_Pair_i64_i64_i64_42095:

Fun_Pair_i64_i64_i64_42095_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42169
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42168
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42168:
    jmp lab42170

lab42169:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42170:
    ; substitute (p0 !-> p0)(isalive !-> isalive)(a8 !-> a8);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a9: List[Pair[i64, i64]] = (a8)\{ ... \};
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
    je lab42182
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42183

lab42182:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42180
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42173
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42171
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42172

lab42171:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42172:

lab42173:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42176
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42174
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42175

lab42174:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42175:

lab42176:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42179
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42177
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42178

lab42177:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42178:

lab42179:
    jmp lab42181

lab42180:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42181:

lab42183:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_42184]
    ; new a10: List[Pair[i64, i64]] = (isalive, a9)\{ ... \};
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
    je lab42196
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42197

lab42196:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42194
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42187
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42185
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42186

lab42185:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42186:

lab42187:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42190
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42188
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42189

lab42188:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42189:

lab42190:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42193
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42191
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42192

lab42191:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42192:

lab42193:
    jmp lab42195

lab42194:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42195:

lab42197:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_42198]
    ; jump neighbours_
    jmp neighbours_

List_Pair_i64_i64_42198:
    jmp near List_Pair_i64_i64_42198_Nil
    jmp near List_Pair_i64_i64_42198_Cons

List_Pair_i64_i64_42198_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42201
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab42199
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42199:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab42200
    ; ####increment refcount
    add qword [rax + 0], 1

lab42200:
    jmp lab42202

lab42201:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab42202:
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(isalive !-> isalive)(a9 !-> a9);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump filter_
    jmp filter_

List_Pair_i64_i64_42198_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42205
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab42203
    ; ####increment refcount
    add qword [r10 + 0], 1

lab42203:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab42204
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42204:
    jmp lab42206

lab42205:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab42206:
    ; substitute (a9 !-> a9)(isalive !-> isalive)(x10 !-> x10)(xs1 !-> xs1);
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
    ; let x1: List[Pair[i64, i64]] = Cons(x10, xs1);
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
    je lab42218
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42219

lab42218:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42216
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42209
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42207
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42208

lab42207:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42208:

lab42209:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42212
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42210
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42211

lab42210:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42211:

lab42212:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42215
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42213
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42214

lab42213:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42214:

lab42215:
    jmp lab42217

lab42216:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42217:

lab42219:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(isalive !-> isalive)(a9 !-> a9);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump filter_
    jmp filter_

List_Pair_i64_i64_42184:
    jmp near List_Pair_i64_i64_42184_Nil
    jmp near List_Pair_i64_i64_42184_Cons

List_Pair_i64_i64_42184_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42221
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42220
    ; ####increment refcount
    add qword [rax + 0], 1

lab42220:
    jmp lab42222

lab42221:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42222:
    ; let x0: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x0 !-> x0)(a8 !-> a8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump len_
    jmp len_

List_Pair_i64_i64_42184_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42224
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42223
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42223:
    jmp lab42225

lab42224:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42225:
    ; substitute (a8 !-> a8)(x9 !-> x9)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(x9, xs0);
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
    je lab42237
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42238

lab42237:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42235
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42228
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42226
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42227

lab42226:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42227:

lab42228:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42231
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42229
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42230

lab42229:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42230:

lab42231:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42234
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42232
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42233

lab42232:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42233:

lab42234:
    jmp lab42236

lab42235:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42236:

lab42238:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a8 !-> a8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump len_
    jmp len_

Fun_Pair_i64_i64_Bool_42080:

Fun_Pair_i64_i64_Bool_42080_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42240
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42239
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42239:
    jmp lab42241

lab42240:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42241:
    ; substitute (living !-> living)(p !-> p)(a11 !-> a11);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump member_
    jmp member_

lift_mk_nextgen_fn_1_:
    ; substitute (a0 !-> a0)(survivors !-> survivors)(living !-> living)(isalive !-> isalive);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new x3: Fun[Pair[i64, i64], List[Pair[i64, i64]]] = (isalive)\{ ... \};
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
    je lab42253
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42254

lab42253:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42251
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42244
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42242
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42243

lab42242:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42243:

lab42244:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42247
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42245
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42246

lab42245:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42246:

lab42247:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42250
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42248
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42249

lab42248:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42249:

lab42250:
    jmp lab42252

lab42251:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42252:

lab42254:
    ; #load tag
    lea r11, [rel Fun_Pair_i64_i64_List_Pair_i64_i64_42255]
    ; substitute (x3 !-> x3)(living !-> living)(survivors !-> survivors)(a0 !-> a0);
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
    ; new a14: List[Pair[i64, i64]] = (survivors, a0)\{ ... \};
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
    je lab42267
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42268

lab42267:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42265
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42258
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42256
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42257

lab42256:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42257:

lab42258:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42261
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42259
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42260

lab42259:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42260:

lab42261:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42264
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42262
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42263

lab42262:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42263:

lab42264:
    jmp lab42266

lab42265:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42266:

lab42268:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_42269]
    ; substitute (living !-> living)(x3 !-> x3)(a14 !-> a14);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump collect_
    jmp collect_

List_Pair_i64_i64_42269:
    jmp near List_Pair_i64_i64_42269_Nil
    jmp near List_Pair_i64_i64_42269_Cons

List_Pair_i64_i64_42269_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42272
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab42270
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42270:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab42271
    ; ####increment refcount
    add qword [rax + 0], 1

lab42271:
    jmp lab42273

lab42272:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab42273:
    ; let newnbrlist: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(newnbrlist !-> newnbrlist)(survivors !-> survivors);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_mk_nextgen_fn_3_
    jmp lift_mk_nextgen_fn_3_

List_Pair_i64_i64_42269_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42276
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab42274
    ; ####increment refcount
    add qword [r10 + 0], 1

lab42274:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab42275
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42275:
    jmp lab42277

lab42276:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab42277:
    ; substitute (a0 !-> a0)(survivors !-> survivors)(x14 !-> x14)(xs5 !-> xs5);
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
    ; let newnbrlist: List[Pair[i64, i64]] = Cons(x14, xs5);
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
    je lab42289
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42290

lab42289:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42287
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42280
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42278
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42279

lab42278:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42279:

lab42280:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42283
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42281
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42282

lab42281:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42282:

lab42283:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42286
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42284
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42285

lab42284:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42285:

lab42286:
    jmp lab42288

lab42287:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42288:

lab42290:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(newnbrlist !-> newnbrlist)(survivors !-> survivors);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_mk_nextgen_fn_3_
    jmp lift_mk_nextgen_fn_3_

Fun_Pair_i64_i64_List_Pair_i64_i64_42255:

Fun_Pair_i64_i64_List_Pair_i64_i64_42255_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42292
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42291
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42291:
    jmp lab42293

lab42292:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42293:
    ; new a3: List[Pair[i64, i64]] = (a2, isalive)\{ ... \};
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
    je lab42305
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42306

lab42305:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42303
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42296
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42294
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42295

lab42294:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42295:

lab42296:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42299
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42297
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42298

lab42297:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42298:

lab42299:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42302
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42300
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42301

lab42300:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42301:

lab42302:
    jmp lab42304

lab42303:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42304:

lab42306:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_42307]
    ; jump neighbours_
    jmp neighbours_

List_Pair_i64_i64_42307:
    jmp near List_Pair_i64_i64_42307_Nil
    jmp near List_Pair_i64_i64_42307_Cons

List_Pair_i64_i64_42307_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42310
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab42308
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42308:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab42309
    ; ####increment refcount
    add qword [rax + 0], 1

lab42309:
    jmp lab42311

lab42310:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab42311:
    ; let x5: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_mk_nextgen_fn_2_
    jmp lift_mk_nextgen_fn_2_

List_Pair_i64_i64_42307_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42314
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab42312
    ; ####increment refcount
    add qword [r10 + 0], 1

lab42312:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab42313
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42313:
    jmp lab42315

lab42314:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab42315:
    ; substitute (isalive !-> isalive)(a2 !-> a2)(x11 !-> x11)(xs2 !-> xs2);
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
    ; let x5: List[Pair[i64, i64]] = Cons(x11, xs2);
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
    je lab42327
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42328

lab42327:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42325
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42318
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42316
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42317

lab42316:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42317:

lab42318:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42321
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42319
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42320

lab42319:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42320:

lab42321:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42323

lab42322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42323:

lab42324:
    jmp lab42326

lab42325:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42326:

lab42328:
    ; #load tag
    mov r9, 5
    ; substitute (a2 !-> a2)(isalive !-> isalive)(x5 !-> x5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_mk_nextgen_fn_2_
    jmp lift_mk_nextgen_fn_2_

lift_mk_nextgen_fn_3_:
    ; substitute (newnbrlist !-> newnbrlist)(a0 !-> a0)(survivors !-> survivors);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a15: List[Pair[i64, i64]] = (a0, survivors)\{ ... \};
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
    je lab42340
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42341

lab42340:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42338
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42331
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42329
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42330

lab42329:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42330:

lab42331:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42334
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42332
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42333

lab42332:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42333:

lab42334:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42336

lab42335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42336:

lab42337:
    jmp lab42339

lab42338:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42339:

lab42341:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_42342]
    ; jump occurs3_
    jmp occurs3_

List_Pair_i64_i64_42342:
    jmp near List_Pair_i64_i64_42342_Nil
    jmp near List_Pair_i64_i64_42342_Cons

List_Pair_i64_i64_42342_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42345
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab42343
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42343:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab42344
    ; ####increment refcount
    add qword [rax + 0], 1

lab42344:
    jmp lab42346

lab42345:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab42346:
    ; let newborn: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(newborn !-> newborn)(survivors !-> survivors);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_mk_nextgen_fn_4_
    jmp lift_mk_nextgen_fn_4_

List_Pair_i64_i64_42342_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42349
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab42347
    ; ####increment refcount
    add qword [r10 + 0], 1

lab42347:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab42348
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42348:
    jmp lab42350

lab42349:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab42350:
    ; substitute (survivors !-> survivors)(a0 !-> a0)(x13 !-> x13)(xs4 !-> xs4);
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
    ; let newborn: List[Pair[i64, i64]] = Cons(x13, xs4);
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
    je lab42362
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42363

lab42362:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42360
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42353
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42351
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42352

lab42351:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42352:

lab42353:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42355

lab42354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42355:

lab42356:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42359
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42357
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42358

lab42357:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42358:

lab42359:
    jmp lab42361

lab42360:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42361:

lab42363:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(newborn !-> newborn)(survivors !-> survivors);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_mk_nextgen_fn_4_
    jmp lift_mk_nextgen_fn_4_

lift_mk_nextgen_fn_4_:
    ; substitute (survivors !-> survivors)(newborn !-> newborn)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[Pair[i64, i64]] = (a0)\{ ... \};
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
    je lab42375
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42376

lab42375:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42373
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42366
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42364
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42365

lab42364:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42365:

lab42366:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42369
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42367
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42368

lab42367:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42368:

lab42369:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42372
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42370
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42371

lab42370:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42371:

lab42372:
    jmp lab42374

lab42373:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42374:

lab42376:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_42377]
    ; jump append_
    jmp append_

List_Pair_i64_i64_42377:
    jmp near List_Pair_i64_i64_42377_Nil
    jmp near List_Pair_i64_i64_42377_Cons

List_Pair_i64_i64_42377_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42379
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42378
    ; ####increment refcount
    add qword [rax + 0], 1

lab42378:
    jmp lab42380

lab42379:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42380:
    ; let x4: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump mkgen_
    jmp mkgen_

List_Pair_i64_i64_42377_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42382
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42381
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42381:
    jmp lab42383

lab42382:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42383:
    ; substitute (a0 !-> a0)(x12 !-> x12)(xs3 !-> xs3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x4: List[Pair[i64, i64]] = Cons(x12, xs3);
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
    je lab42395
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42396

lab42395:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42393
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42385

lab42384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42385:

lab42386:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42388

lab42387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42388:

lab42389:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42392
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42390
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42391

lab42390:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42391:

lab42392:
    jmp lab42394

lab42393:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42394:

lab42396:
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
    ; jump mkgen_
    jmp mkgen_

lift_mk_nextgen_fn_2_:
    ; substitute (a2 !-> a2)(x5 !-> x5)(isalive !-> isalive);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new x6: Fun[Pair[i64, i64], Bool] = (isalive)\{ ... \};
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
    je lab42408
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42409

lab42408:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42406
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42399
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42397
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42398

lab42397:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42398:

lab42399:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42402
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42400
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42401

lab42400:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42401:

lab42402:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42405
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42403
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42404

lab42403:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42404:

lab42405:
    jmp lab42407

lab42406:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42407:

lab42409:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Bool_42410]
    ; substitute (x5 !-> x5)(x6 !-> x6)(a2 !-> a2);
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

Fun_Pair_i64_i64_Bool_42410:

Fun_Pair_i64_i64_Bool_42410_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab42412
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab42411
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42411:
    jmp lab42413

lab42412:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab42413:
    ; substitute (n !-> n)(isalive !-> isalive)(a4 !-> a4);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a5: Bool = (a4)\{ ... \};
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
    je lab42425
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42426

lab42425:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42423
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42416
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42414
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42415

lab42414:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42415:

lab42416:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42419
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42417
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42418

lab42417:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42418:

lab42419:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42422
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42420
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42421

lab42420:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42421:

lab42422:
    jmp lab42424

lab42423:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42424:

lab42426:
    ; #load tag
    lea r9, [rel Bool_42427]
    ; substitute (n !-> n)(a5 !-> a5)(isalive !-> isalive);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke isalive Apply
    jmp r9

Bool_42427:
    jmp near Bool_42427_True
    jmp near Bool_42427_False

Bool_42427_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42429
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42428
    ; ####increment refcount
    add qword [rax + 0], 1

lab42428:
    jmp lab42430

lab42429:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42430:
    ; let x7: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x7 !-> x7)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_42427_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab42432
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab42431
    ; ####increment refcount
    add qword [rax + 0], 1

lab42431:
    jmp lab42433

lab42432:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab42433:
    ; let x7: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x7 !-> x7)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

nthgen_:
    ; if i == 0 \{ ... \}
    cmp rdi, 0
    je lab42434
    ; new a1: Gen = (i, a0)\{ ... \};
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
    je lab42446
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42447

lab42446:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42444
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42437
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42435
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42436

lab42435:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42436:

lab42437:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42440
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42438
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42439

lab42438:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42439:

lab42440:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42443
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42441
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42442

lab42441:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42442:

lab42443:
    jmp lab42445

lab42444:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42445:

lab42447:
    ; #load tag
    lea rdi, [rel Gen_42448]
    ; jump mk_nextgen_fn_
    jmp mk_nextgen_fn_

Gen_42448:

Gen_42448_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab42450
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab42449
    ; ####increment refcount
    add qword [r8 + 0], 1

lab42449:
    mov rdi, [rsi + 40]
    jmp lab42451

lab42450:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab42451:
    ; substitute (a0 !-> a0)(i !-> i)(coordslist1 !-> coordslist1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x0: Gen = Gen(coordslist1);
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
    je lab42463
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42464

lab42463:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42461
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42454
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42452
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42453

lab42452:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42453:

lab42454:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42457
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42455
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42456

lab42455:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42456:

lab42457:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42460
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42458
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42459

lab42458:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42459:

lab42460:
    jmp lab42462

lab42461:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42462:

lab42464:
    ; #load tag
    mov r9, 0
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- i - x1;
    mov r13, rdi
    sub r13, r11
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump nthgen_
    jmp nthgen_

lab42434:
    ; substitute (a0 !-> a0)(g !-> g);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; switch g \{ ... \};
    ; #if there is only one clause, we can just fall through

Gen_42465:

Gen_42465_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab42467
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab42466
    ; ####increment refcount
    add qword [rsi + 0], 1

lab42466:
    jmp lab42468

lab42467:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab42468:
    ; substitute (coordslist0 !-> coordslist0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Gen
    jmp rdi

gun_:
    ; lit x1 <- 9;
    mov rdi, 9
    ; lit x2 <- 29;
    mov r9, 29
    ; let x0: Pair[i64, i64] = Tup(x1, x2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
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
    je lab42480
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42481

lab42480:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42478
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42471
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42469
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42470

lab42469:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42470:

lab42471:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42474
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42472
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42473

lab42472:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42473:

lab42474:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42477
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42475
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42476

lab42475:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42476:

lab42477:
    jmp lab42479

lab42478:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42479:

lab42481:
    ; #load tag
    mov rdi, 0
    ; lit x5 <- 9;
    mov r9, 9
    ; lit x6 <- 30;
    mov r11, 30
    ; let x4: Pair[i64, i64] = Tup(x5, x6);
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
    je lab42493
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42494

lab42493:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42491
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42484
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42482
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42483

lab42482:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42483:

lab42484:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42487
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42485
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42486

lab42485:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42486:

lab42487:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42490
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42488
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42489

lab42488:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42489:

lab42490:
    jmp lab42492

lab42491:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42492:

lab42494:
    ; #load tag
    mov r9, 0
    ; lit x9 <- 9;
    mov r11, 9
    ; lit x10 <- 31;
    mov r13, 31
    ; let x8: Pair[i64, i64] = Tup(x9, x10);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab42506
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42507

lab42506:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42504
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42497
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42495
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42496

lab42495:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42496:

lab42497:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42499

lab42498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42499:

lab42500:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42502

lab42501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42502:

lab42503:
    jmp lab42505

lab42504:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42505:

lab42507:
    ; #load tag
    mov r11, 0
    ; lit x13 <- 9;
    mov r13, 9
    ; lit x14 <- 32;
    mov r15, 32
    ; let x12: Pair[i64, i64] = Tup(x13, x14);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab42519
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42520

lab42519:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42517
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42510
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42508
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42509

lab42508:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42509:

lab42510:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42512

lab42511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42512:

lab42513:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42515

lab42514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42515:

lab42516:
    jmp lab42518

lab42517:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42518:

lab42520:
    ; #load tag
    mov r13, 0
    ; let x15: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x11: List[Pair[i64, i64]] = Cons(x12, x15);
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
    je lab42532
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42533

lab42532:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42530
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42523
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42521
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42522

lab42521:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42522:

lab42523:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42526
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42524
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42525

lab42524:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42525:

lab42526:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42528

lab42527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42528:

lab42529:
    jmp lab42531

lab42530:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42531:

lab42533:
    ; #load tag
    mov r13, 5
    ; let x7: List[Pair[i64, i64]] = Cons(x8, x11);
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
    je lab42545
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42546

lab42545:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42543
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42536
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42534
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42535

lab42534:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42535:

lab42536:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42539
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42537
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42538

lab42537:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42538:

lab42539:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42542
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42540
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42541

lab42540:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42541:

lab42542:
    jmp lab42544

lab42543:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42544:

lab42546:
    ; #load tag
    mov r11, 5
    ; let x3: List[Pair[i64, i64]] = Cons(x4, x7);
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
    je lab42558
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42559

lab42558:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42556
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42549
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42547
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42548

lab42547:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42548:

lab42549:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42552
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42550
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42551

lab42550:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42551:

lab42552:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42555
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42553
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42554

lab42553:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42554:

lab42555:
    jmp lab42557

lab42556:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42557:

lab42559:
    ; #load tag
    mov r9, 5
    ; let r9: List[Pair[i64, i64]] = Cons(x0, x3);
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
    je lab42571
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42572

lab42571:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42569
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42562
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42560
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42561

lab42560:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42561:

lab42562:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42565
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42563
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42564

lab42563:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42564:

lab42565:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42568
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42566
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42567

lab42566:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42567:

lab42568:
    jmp lab42570

lab42569:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42570:

lab42572:
    ; #load tag
    mov rdi, 5
    ; lit x17 <- 8;
    mov r9, 8
    ; lit x18 <- 20;
    mov r11, 20
    ; let x16: Pair[i64, i64] = Tup(x17, x18);
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
    je lab42584
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42585

lab42584:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42582
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42575
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42573
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42574

lab42573:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42574:

lab42575:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42578
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42576
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42577

lab42576:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42577:

lab42578:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42581
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42579
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42580

lab42579:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42580:

lab42581:
    jmp lab42583

lab42582:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42583:

lab42585:
    ; #load tag
    mov r9, 0
    ; lit x21 <- 8;
    mov r11, 8
    ; lit x22 <- 28;
    mov r13, 28
    ; let x20: Pair[i64, i64] = Tup(x21, x22);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab42597
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42598

lab42597:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42595
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42588
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42586
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42587

lab42586:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42587:

lab42588:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42591
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42589
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42590

lab42589:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42590:

lab42591:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42593

lab42592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42593:

lab42594:
    jmp lab42596

lab42595:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42596:

lab42598:
    ; #load tag
    mov r11, 0
    ; lit x25 <- 8;
    mov r13, 8
    ; lit x26 <- 29;
    mov r15, 29
    ; let x24: Pair[i64, i64] = Tup(x25, x26);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab42610
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42611

lab42610:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42608
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42600

lab42599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42600:

lab42601:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42603

lab42602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42603:

lab42604:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42606

lab42605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42606:

lab42607:
    jmp lab42609

lab42608:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42609:

lab42611:
    ; #load tag
    mov r13, 0
    ; lit x29 <- 8;
    mov r15, 8
    ; lit x30 <- 30;
    mov qword [rsp + 2024], 30
    ; let x28: Pair[i64, i64] = Tup(x29, x30);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab42623
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab42624

lab42623:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42621
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42614
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42612
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42613

lab42612:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42613:

lab42614:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42616

lab42615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42616:

lab42617:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42619

lab42618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42619:

lab42620:
    jmp lab42622

lab42621:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42622:

lab42624:
    ; #load tag
    mov r15, 0
    ; lit x33 <- 8;
    mov qword [rsp + 2024], 8
    ; lit x34 <- 31;
    mov qword [rsp + 2008], 31
    ; let x32: Pair[i64, i64] = Tup(x33, x34);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42636
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42637

lab42636:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42634
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42627
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42625
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42626

lab42625:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42626:

lab42627:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42629

lab42628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42629:

lab42630:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42633
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42631
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42632

lab42631:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42632:

lab42633:
    jmp lab42635

lab42634:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42635:

lab42637:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; lit x37 <- 8;
    mov qword [rsp + 2008], 8
    ; lit x38 <- 40;
    mov qword [rsp + 1992], 40
    ; let x36: Pair[i64, i64] = Tup(x37, x38);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42649
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42650

lab42649:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42647
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42640
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42638
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42639

lab42638:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42639:

lab42640:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42642

lab42641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42642:

lab42643:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42645

lab42644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42645:

lab42646:
    jmp lab42648

lab42647:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42648:

lab42650:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; lit x41 <- 8;
    mov qword [rsp + 1992], 8
    ; lit x42 <- 41;
    mov qword [rsp + 1976], 41
    ; let x40: Pair[i64, i64] = Tup(x41, x42);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42662
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42663

lab42662:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42660
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42653
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42651
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42652

lab42651:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42652:

lab42653:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42655

lab42654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42655:

lab42656:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42659
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42657
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42658

lab42657:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42658:

lab42659:
    jmp lab42661

lab42660:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42661:

lab42663:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a0 !-> a0)(x36 !-> x36)(x16 !-> x16)(x20 !-> x20)(x24 !-> x24)(x28 !-> x28)(x32 !-> x32)(x40 !-> x40)(r9 !-> r9);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2000]
    mov [rsp + 2016], rcx
    mov [rsp + 2000], rsi
    mov rsi, [rsp + 2040]
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 2008], rcx
    mov [rsp + 1992], rdi
    mov rdi, [rsp + 2040]
    ; let x39: List[Pair[i64, i64]] = Cons(x40, r9);
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
    je lab42675
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42676

lab42675:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42673
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42666
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42664
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42665

lab42664:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42665:

lab42666:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42669
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42667
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42668

lab42667:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42668:

lab42669:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42672
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42670
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42671

lab42670:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42671:

lab42672:
    jmp lab42674

lab42673:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42674:

lab42676:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(x32 !-> x32)(x16 !-> x16)(x20 !-> x20)(x24 !-> x24)(x28 !-> x28)(x36 !-> x36)(x39 !-> x39);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    ; let x35: List[Pair[i64, i64]] = Cons(x36, x39);
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
    je lab42688
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42689

lab42688:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42686
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42679
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42677
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42678

lab42677:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42678:

lab42679:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42681

lab42680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42681:

lab42682:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42684

lab42683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42684:

lab42685:
    jmp lab42687

lab42686:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42687:

lab42689:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(x28 !-> x28)(x16 !-> x16)(x20 !-> x20)(x24 !-> x24)(x32 !-> x32)(x35 !-> x35);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x31: List[Pair[i64, i64]] = Cons(x32, x35);
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
    je lab42701
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab42702

lab42701:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42699
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42692
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42690
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42691

lab42690:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42691:

lab42692:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42695
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42693
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42694

lab42693:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42694:

lab42695:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42698
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42696
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42697

lab42696:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42697:

lab42698:
    jmp lab42700

lab42699:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42700:

lab42702:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x24 !-> x24)(x16 !-> x16)(x20 !-> x20)(x28 !-> x28)(x31 !-> x31);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x27: List[Pair[i64, i64]] = Cons(x28, x31);
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
    je lab42714
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42715

lab42714:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42712
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42705
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42703
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42704

lab42703:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42704:

lab42705:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42708
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42707

lab42706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42707:

lab42708:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42711
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42709
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42710

lab42709:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42710:

lab42711:
    jmp lab42713

lab42712:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42713:

lab42715:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x20 !-> x20)(x16 !-> x16)(x24 !-> x24)(x27 !-> x27);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x23: List[Pair[i64, i64]] = Cons(x24, x27);
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
    je lab42727
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42728

lab42727:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42725
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42718
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42716
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42717

lab42716:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42717:

lab42718:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42721
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42719
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42720

lab42719:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42720:

lab42721:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42724
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42722
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42723

lab42722:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42723:

lab42724:
    jmp lab42726

lab42725:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42726:

lab42728:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x16 !-> x16)(x20 !-> x20)(x23 !-> x23);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x19: List[Pair[i64, i64]] = Cons(x20, x23);
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
    je lab42740
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42741

lab42740:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42738
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42731
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42729
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42730

lab42729:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42730:

lab42731:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42734
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42732
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42733

lab42732:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42733:

lab42734:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42737
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42735
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42736

lab42735:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42736:

lab42737:
    jmp lab42739

lab42738:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42739:

lab42741:
    ; #load tag
    mov r9, 5
    ; let r8: List[Pair[i64, i64]] = Cons(x16, x19);
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
    je lab42753
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42754

lab42753:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42751
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42744
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42742
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42743

lab42742:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42743:

lab42744:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42747
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42745
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42746

lab42745:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42746:

lab42747:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42750
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42748
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42749

lab42748:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42749:

lab42750:
    jmp lab42752

lab42751:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42752:

lab42754:
    ; #load tag
    mov rdi, 5
    ; lit x44 <- 7;
    mov r9, 7
    ; lit x45 <- 19;
    mov r11, 19
    ; let x43: Pair[i64, i64] = Tup(x44, x45);
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
    je lab42766
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42767

lab42766:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42764
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42757
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42755
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42756

lab42755:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42756:

lab42757:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42760
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42758
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42759

lab42758:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42759:

lab42760:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42763
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42761
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42762

lab42761:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42762:

lab42763:
    jmp lab42765

lab42764:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42765:

lab42767:
    ; #load tag
    mov r9, 0
    ; lit x48 <- 7;
    mov r11, 7
    ; lit x49 <- 21;
    mov r13, 21
    ; let x47: Pair[i64, i64] = Tup(x48, x49);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab42779
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42780

lab42779:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42777
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42770
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42768
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42769

lab42768:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42769:

lab42770:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42773
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42771
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42772

lab42771:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42772:

lab42773:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42776
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42774
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42775

lab42774:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42775:

lab42776:
    jmp lab42778

lab42777:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42778:

lab42780:
    ; #load tag
    mov r11, 0
    ; lit x52 <- 7;
    mov r13, 7
    ; lit x53 <- 28;
    mov r15, 28
    ; let x51: Pair[i64, i64] = Tup(x52, x53);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab42792
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42793

lab42792:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42790
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42783
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42781
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42782

lab42781:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42782:

lab42783:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42785

lab42784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42785:

lab42786:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42788

lab42787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42788:

lab42789:
    jmp lab42791

lab42790:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42791:

lab42793:
    ; #load tag
    mov r13, 0
    ; lit x56 <- 7;
    mov r15, 7
    ; lit x57 <- 31;
    mov qword [rsp + 2024], 31
    ; let x55: Pair[i64, i64] = Tup(x56, x57);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab42805
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab42806

lab42805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42795

lab42794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42795:

lab42796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42798

lab42797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42798:

lab42799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42801

lab42800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42801:

lab42802:
    jmp lab42804

lab42803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42804:

lab42806:
    ; #load tag
    mov r15, 0
    ; lit x60 <- 7;
    mov qword [rsp + 2024], 7
    ; lit x61 <- 40;
    mov qword [rsp + 2008], 40
    ; let x59: Pair[i64, i64] = Tup(x60, x61);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42818
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42819

lab42818:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42816
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42809
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42807
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42808

lab42807:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42808:

lab42809:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42812
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42810
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42811

lab42810:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42811:

lab42812:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42815
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42813
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42814

lab42813:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42814:

lab42815:
    jmp lab42817

lab42816:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42817:

lab42819:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; lit x64 <- 7;
    mov qword [rsp + 2008], 7
    ; lit x65 <- 41;
    mov qword [rsp + 1992], 41
    ; let x63: Pair[i64, i64] = Tup(x64, x65);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42831
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42832

lab42831:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42829
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42822
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42820
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42821

lab42820:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42821:

lab42822:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42825
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42823
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42824

lab42823:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42824:

lab42825:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42827

lab42826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42827:

lab42828:
    jmp lab42830

lab42829:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42830:

lab42832:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (a0 !-> a0)(x59 !-> x59)(x43 !-> x43)(x47 !-> x47)(x51 !-> x51)(x55 !-> x55)(x63 !-> x63)(r8 !-> r8);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 2032], rcx
    mov [rsp + 2016], rsi
    mov rsi, [rsp + 2040]
    mov rcx, [rsp + 2024]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2024], rcx
    mov [rsp + 2008], rdi
    mov rdi, [rsp + 2040]
    ; let x62: List[Pair[i64, i64]] = Cons(x63, r8);
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
    je lab42844
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42845

lab42844:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42842
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42835
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42833
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42834

lab42833:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42834:

lab42835:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42837

lab42836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42837:

lab42838:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42840

lab42839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42840:

lab42841:
    jmp lab42843

lab42842:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42843:

lab42845:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(x55 !-> x55)(x43 !-> x43)(x47 !-> x47)(x51 !-> x51)(x59 !-> x59)(x62 !-> x62);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x58: List[Pair[i64, i64]] = Cons(x59, x62);
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
    je lab42857
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab42858

lab42857:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42855
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42848
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42846
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42847

lab42846:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42847:

lab42848:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42850

lab42849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42850:

lab42851:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42854
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42852
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42853

lab42852:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42853:

lab42854:
    jmp lab42856

lab42855:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42856:

lab42858:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x51 !-> x51)(x43 !-> x43)(x47 !-> x47)(x55 !-> x55)(x58 !-> x58);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x54: List[Pair[i64, i64]] = Cons(x55, x58);
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
    je lab42870
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42871

lab42870:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42868
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42861
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42859
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42860

lab42859:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42860:

lab42861:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42863

lab42862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42863:

lab42864:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42866

lab42865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42866:

lab42867:
    jmp lab42869

lab42868:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42869:

lab42871:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x47 !-> x47)(x43 !-> x43)(x51 !-> x51)(x54 !-> x54);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x50: List[Pair[i64, i64]] = Cons(x51, x54);
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
    je lab42883
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42884

lab42883:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42881
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42874
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42872
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42873

lab42872:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42873:

lab42874:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42876

lab42875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42876:

lab42877:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42880
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42878
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42879

lab42878:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42879:

lab42880:
    jmp lab42882

lab42881:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42882:

lab42884:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x43 !-> x43)(x47 !-> x47)(x50 !-> x50);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x46: List[Pair[i64, i64]] = Cons(x47, x50);
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
    je lab42896
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42897

lab42896:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42894
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42886

lab42885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42886:

lab42887:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42889

lab42888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42889:

lab42890:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42892

lab42891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42892:

lab42893:
    jmp lab42895

lab42894:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42895:

lab42897:
    ; #load tag
    mov r9, 5
    ; let r7: List[Pair[i64, i64]] = Cons(x43, x46);
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
    je lab42909
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab42910

lab42909:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42907
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42899

lab42898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42899:

lab42900:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42902

lab42901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42902:

lab42903:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42905

lab42904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42905:

lab42906:
    jmp lab42908

lab42907:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42908:

lab42910:
    ; #load tag
    mov rdi, 5
    ; lit x67 <- 6;
    mov r9, 6
    ; lit x68 <- 7;
    mov r11, 7
    ; let x66: Pair[i64, i64] = Tup(x67, x68);
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
    je lab42922
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab42923

lab42922:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42920
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42912

lab42911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42912:

lab42913:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42915

lab42914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42915:

lab42916:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42918

lab42917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42918:

lab42919:
    jmp lab42921

lab42920:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42921:

lab42923:
    ; #load tag
    mov r9, 0
    ; lit x71 <- 6;
    mov r11, 6
    ; lit x72 <- 8;
    mov r13, 8
    ; let x70: Pair[i64, i64] = Tup(x71, x72);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab42935
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab42936

lab42935:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42933
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42926
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42924
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42925

lab42924:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42925:

lab42926:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42928

lab42927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42928:

lab42929:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42932
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42930
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42931

lab42930:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42931:

lab42932:
    jmp lab42934

lab42933:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42934:

lab42936:
    ; #load tag
    mov r11, 0
    ; lit x75 <- 6;
    mov r13, 6
    ; lit x76 <- 18;
    mov r15, 18
    ; let x74: Pair[i64, i64] = Tup(x75, x76);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab42948
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab42949

lab42948:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42946
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42939
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42938

lab42937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42938:

lab42939:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42941

lab42940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42941:

lab42942:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42945
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42943
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42944

lab42943:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42944:

lab42945:
    jmp lab42947

lab42946:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42947:

lab42949:
    ; #load tag
    mov r13, 0
    ; lit x79 <- 6;
    mov r15, 6
    ; lit x80 <- 22;
    mov qword [rsp + 2024], 22
    ; let x78: Pair[i64, i64] = Tup(x79, x80);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab42961
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab42962

lab42961:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42959
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42952
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42950
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42951

lab42950:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42951:

lab42952:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42954

lab42953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42954:

lab42955:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42958
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42956
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42957

lab42956:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42957:

lab42958:
    jmp lab42960

lab42959:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42960:

lab42962:
    ; #load tag
    mov r15, 0
    ; lit x83 <- 6;
    mov qword [rsp + 2024], 6
    ; lit x84 <- 23;
    mov qword [rsp + 2008], 23
    ; let x82: Pair[i64, i64] = Tup(x83, x84);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42974
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42975

lab42974:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42972
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42965
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42963
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42964

lab42963:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42964:

lab42965:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42968
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42966
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42967

lab42966:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42967:

lab42968:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42970

lab42969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42970:

lab42971:
    jmp lab42973

lab42972:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42973:

lab42975:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; lit x87 <- 6;
    mov qword [rsp + 2008], 6
    ; lit x88 <- 28;
    mov qword [rsp + 1992], 28
    ; let x86: Pair[i64, i64] = Tup(x87, x88);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab42987
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab42988

lab42987:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42985
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42977

lab42976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42977:

lab42978:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42980

lab42979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42980:

lab42981:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42983

lab42982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42983:

lab42984:
    jmp lab42986

lab42985:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42986:

lab42988:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; lit x91 <- 6;
    mov qword [rsp + 1992], 6
    ; lit x92 <- 29;
    mov qword [rsp + 1976], 29
    ; let x90: Pair[i64, i64] = Tup(x91, x92);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43000
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43001

lab43000:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab42998
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab42991
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42989
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42990

lab42989:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42990:

lab42991:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab42994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42993

lab42992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42993:

lab42994:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab42997
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab42995
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab42996

lab42995:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab42996:

lab42997:
    jmp lab42999

lab42998:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab42999:

lab43001:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; lit x95 <- 6;
    mov qword [rsp + 1976], 6
    ; lit x96 <- 30;
    mov qword [rsp + 1960], 30
    ; let x94: Pair[i64, i64] = Tup(x95, x96);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43013
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43014

lab43013:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43011
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43004
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43002
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43003

lab43002:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43003:

lab43004:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43007
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43005
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43006

lab43005:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43006:

lab43007:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43010
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43008
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43009

lab43008:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43009:

lab43010:
    jmp lab43012

lab43011:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43012:

lab43014:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; lit x99 <- 6;
    mov qword [rsp + 1960], 6
    ; lit x100 <- 31;
    mov qword [rsp + 1944], 31
    ; let x98: Pair[i64, i64] = Tup(x99, x100);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43026
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43027

lab43026:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43024
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43017
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43015
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43016

lab43015:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43016:

lab43017:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43020
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43018
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43019

lab43018:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43019:

lab43020:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43023
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43021
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43022

lab43021:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43022:

lab43023:
    jmp lab43025

lab43024:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43025:

lab43027:
    ; #load tag
    mov qword [rsp + 1960], 0
    ; lit x103 <- 6;
    mov qword [rsp + 1944], 6
    ; lit x104 <- 36;
    mov qword [rsp + 1928], 36
    ; let x102: Pair[i64, i64] = Tup(x103, x104);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43039
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43040

lab43039:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43037
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43030
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43028
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43029

lab43028:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43029:

lab43030:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43033
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43031
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43032

lab43031:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43032:

lab43033:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43036
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43034
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43035

lab43034:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43035:

lab43036:
    jmp lab43038

lab43037:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43038:

lab43040:
    ; #load tag
    mov qword [rsp + 1944], 0
    ; substitute (a0 !-> a0)(x98 !-> x98)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x78 !-> x78)(x82 !-> x82)(x86 !-> x86)(x90 !-> x90)(x94 !-> x94)(x102 !-> x102)(r7 !-> r7);
    ; #move variables
    mov rcx, [rsp + 1968]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1952]
    mov [rsp + 1968], rcx
    mov [rsp + 1952], rsi
    mov rsi, [rsp + 2040]
    mov rcx, [rsp + 1960]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1944]
    mov [rsp + 1960], rcx
    mov [rsp + 1944], rdi
    mov rdi, [rsp + 2040]
    ; let x101: List[Pair[i64, i64]] = Cons(x102, r7);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43052
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43053

lab43052:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43050
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43042

lab43041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43042:

lab43043:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43046
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43044
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43045

lab43044:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43045:

lab43046:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43049
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43047
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43048

lab43047:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43048:

lab43049:
    jmp lab43051

lab43050:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43051:

lab43053:
    ; #load tag
    mov qword [rsp + 1960], 5
    ; substitute (a0 !-> a0)(x94 !-> x94)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x78 !-> x78)(x82 !-> x82)(x86 !-> x86)(x90 !-> x90)(x98 !-> x98)(x101 !-> x101);
    ; #move variables
    mov rcx, [rsp + 1984]
    mov [rsp + 1984], rsi
    mov rsi, rcx
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], rdi
    mov rdi, rcx
    ; let x97: List[Pair[i64, i64]] = Cons(x98, x101);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43065
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43066

lab43065:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43063
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43056
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43054
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43055

lab43054:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43055:

lab43056:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43058

lab43057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43058:

lab43059:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43061

lab43060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43061:

lab43062:
    jmp lab43064

lab43063:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43064:

lab43066:
    ; #load tag
    mov qword [rsp + 1976], 5
    ; substitute (a0 !-> a0)(x90 !-> x90)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x78 !-> x78)(x82 !-> x82)(x86 !-> x86)(x94 !-> x94)(x97 !-> x97);
    ; #move variables
    mov rcx, [rsp + 2000]
    mov [rsp + 2000], rsi
    mov rsi, rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, rcx
    ; let x93: List[Pair[i64, i64]] = Cons(x94, x97);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43078
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43079

lab43078:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43076
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43069
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43067
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43068

lab43067:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43068:

lab43069:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43072
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43070
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43071

lab43070:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43071:

lab43072:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43075
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43074

lab43073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43074:

lab43075:
    jmp lab43077

lab43076:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43077:

lab43079:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(x86 !-> x86)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x78 !-> x78)(x82 !-> x82)(x90 !-> x90)(x93 !-> x93);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, rcx
    ; let x89: List[Pair[i64, i64]] = Cons(x90, x93);
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
    je lab43091
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43092

lab43091:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43089
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43082
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43080
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43081

lab43080:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43081:

lab43082:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43085
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43083
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43084

lab43083:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43084:

lab43085:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43088
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43086
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43087

lab43086:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43087:

lab43088:
    jmp lab43090

lab43089:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43090:

lab43092:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(x82 !-> x82)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x78 !-> x78)(x86 !-> x86)(x89 !-> x89);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    ; let x85: List[Pair[i64, i64]] = Cons(x86, x89);
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
    je lab43104
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43105

lab43104:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43102
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43095
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43093
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43094

lab43093:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43094:

lab43095:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43098
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43096
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43097

lab43096:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43097:

lab43098:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43101
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43099
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43100

lab43099:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43100:

lab43101:
    jmp lab43103

lab43102:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43103:

lab43105:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(x78 !-> x78)(x66 !-> x66)(x70 !-> x70)(x74 !-> x74)(x82 !-> x82)(x85 !-> x85);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x81: List[Pair[i64, i64]] = Cons(x82, x85);
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
    je lab43117
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab43118

lab43117:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43115
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43108
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43106
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43107

lab43106:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43107:

lab43108:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43111
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43109
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43110

lab43109:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43110:

lab43111:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43114
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43112
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43113

lab43112:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43113:

lab43114:
    jmp lab43116

lab43115:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43116:

lab43118:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x74 !-> x74)(x66 !-> x66)(x70 !-> x70)(x78 !-> x78)(x81 !-> x81);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x77: List[Pair[i64, i64]] = Cons(x78, x81);
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
    je lab43130
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43131

lab43130:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43128
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43121
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43119
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43120

lab43119:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43120:

lab43121:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43124
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43122
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43123

lab43122:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43123:

lab43124:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43127
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43125
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43126

lab43125:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43126:

lab43127:
    jmp lab43129

lab43128:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43129:

lab43131:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x70 !-> x70)(x66 !-> x66)(x74 !-> x74)(x77 !-> x77);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x73: List[Pair[i64, i64]] = Cons(x74, x77);
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
    je lab43143
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43144

lab43143:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43141
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43134
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43132
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43133

lab43132:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43133:

lab43134:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43137
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43135
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43136

lab43135:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43136:

lab43137:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43140
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43138
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43139

lab43138:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43139:

lab43140:
    jmp lab43142

lab43141:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43142:

lab43144:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x66 !-> x66)(x70 !-> x70)(x73 !-> x73);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x69: List[Pair[i64, i64]] = Cons(x70, x73);
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
    je lab43156
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43157

lab43156:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43154
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43147
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43145
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43146

lab43145:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43146:

lab43147:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43150
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43148
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43149

lab43148:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43149:

lab43150:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43153
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43151
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43152

lab43151:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43152:

lab43153:
    jmp lab43155

lab43154:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43155:

lab43157:
    ; #load tag
    mov r9, 5
    ; let r6: List[Pair[i64, i64]] = Cons(x66, x69);
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
    je lab43169
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43170

lab43169:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43167
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43159

lab43158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43159:

lab43160:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43163
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43161
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43162

lab43161:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43162:

lab43163:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43166
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43164
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43165

lab43164:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43165:

lab43166:
    jmp lab43168

lab43167:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43168:

lab43170:
    ; #load tag
    mov rdi, 5
    ; lit x106 <- 5;
    mov r9, 5
    ; lit x107 <- 7;
    mov r11, 7
    ; let x105: Pair[i64, i64] = Tup(x106, x107);
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
    je lab43182
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43183

lab43182:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43180
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43173
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43171
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43172

lab43171:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43172:

lab43173:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43176
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43174
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43175

lab43174:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43175:

lab43176:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43179
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43177
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43178

lab43177:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43178:

lab43179:
    jmp lab43181

lab43180:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43181:

lab43183:
    ; #load tag
    mov r9, 0
    ; lit x110 <- 5;
    mov r11, 5
    ; lit x111 <- 8;
    mov r13, 8
    ; let x109: Pair[i64, i64] = Tup(x110, x111);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43195
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43196

lab43195:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43193
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43185

lab43184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43185:

lab43186:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43189
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43187
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43188

lab43187:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43188:

lab43189:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43192
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43190
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43191

lab43190:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43191:

lab43192:
    jmp lab43194

lab43193:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43194:

lab43196:
    ; #load tag
    mov r11, 0
    ; lit x114 <- 5;
    mov r13, 5
    ; lit x115 <- 18;
    mov r15, 18
    ; let x113: Pair[i64, i64] = Tup(x114, x115);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab43208
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43209

lab43208:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43206
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43199
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43197
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43198

lab43197:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43198:

lab43199:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43202
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43200
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43201

lab43200:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43201:

lab43202:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43205
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43203
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43204

lab43203:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43204:

lab43205:
    jmp lab43207

lab43206:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43207:

lab43209:
    ; #load tag
    mov r13, 0
    ; lit x118 <- 5;
    mov r15, 5
    ; lit x119 <- 22;
    mov qword [rsp + 2024], 22
    ; let x117: Pair[i64, i64] = Tup(x118, x119);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab43221
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab43222

lab43221:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43219
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43212
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43210
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43211

lab43210:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43211:

lab43212:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43215
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43213
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43214

lab43213:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43214:

lab43215:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43218
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43216
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43217

lab43216:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43217:

lab43218:
    jmp lab43220

lab43219:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43220:

lab43222:
    ; #load tag
    mov r15, 0
    ; lit x122 <- 5;
    mov qword [rsp + 2024], 5
    ; lit x123 <- 23;
    mov qword [rsp + 2008], 23
    ; let x121: Pair[i64, i64] = Tup(x122, x123);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43234
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43235

lab43234:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43232
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43225
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43223
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43224

lab43223:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43224:

lab43225:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43228
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43226
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43227

lab43226:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43227:

lab43228:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43231
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43229
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43230

lab43229:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43230:

lab43231:
    jmp lab43233

lab43232:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43233:

lab43235:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; lit x126 <- 5;
    mov qword [rsp + 2008], 5
    ; lit x127 <- 29;
    mov qword [rsp + 1992], 29
    ; let x125: Pair[i64, i64] = Tup(x126, x127);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43247
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43248

lab43247:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43245
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43238
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43236
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43237

lab43236:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43237:

lab43238:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43241
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43239
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43240

lab43239:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43240:

lab43241:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43244
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43242
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43243

lab43242:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43243:

lab43244:
    jmp lab43246

lab43245:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43246:

lab43248:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; lit x130 <- 5;
    mov qword [rsp + 1992], 5
    ; lit x131 <- 30;
    mov qword [rsp + 1976], 30
    ; let x129: Pair[i64, i64] = Tup(x130, x131);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43260
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43261

lab43260:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43258
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43251
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43249
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43250

lab43249:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43250:

lab43251:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43254
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43252
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43253

lab43252:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43253:

lab43254:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43257
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43255
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43256

lab43255:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43256:

lab43257:
    jmp lab43259

lab43258:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43259:

lab43261:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; lit x134 <- 5;
    mov qword [rsp + 1976], 5
    ; lit x135 <- 31;
    mov qword [rsp + 1960], 31
    ; let x133: Pair[i64, i64] = Tup(x134, x135);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43273
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43274

lab43273:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43271
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43264
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43262
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43263

lab43262:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43263:

lab43264:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43267
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43265
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43266

lab43265:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43266:

lab43267:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43270
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43268
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43269

lab43268:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43269:

lab43270:
    jmp lab43272

lab43271:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43272:

lab43274:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; lit x138 <- 5;
    mov qword [rsp + 1960], 5
    ; lit x139 <- 32;
    mov qword [rsp + 1944], 32
    ; let x137: Pair[i64, i64] = Tup(x138, x139);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43286
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43287

lab43286:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43284
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43277
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43275
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43276

lab43275:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43276:

lab43277:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43280
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43278
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43279

lab43278:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43279:

lab43280:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43283
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43281
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43282

lab43281:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43282:

lab43283:
    jmp lab43285

lab43284:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43285:

lab43287:
    ; #load tag
    mov qword [rsp + 1960], 0
    ; lit x142 <- 5;
    mov qword [rsp + 1944], 5
    ; lit x143 <- 36;
    mov qword [rsp + 1928], 36
    ; let x141: Pair[i64, i64] = Tup(x142, x143);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43299
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43300

lab43299:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43297
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43290
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43288
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43289

lab43288:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43289:

lab43290:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43293
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43291
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43292

lab43291:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43292:

lab43293:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43296
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43294
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43295

lab43294:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43295:

lab43296:
    jmp lab43298

lab43297:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43298:

lab43300:
    ; #load tag
    mov qword [rsp + 1944], 0
    ; substitute (a0 !-> a0)(x137 !-> x137)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x117 !-> x117)(x121 !-> x121)(x125 !-> x125)(x129 !-> x129)(x133 !-> x133)(x141 !-> x141)(r6 !-> r6);
    ; #move variables
    mov rcx, [rsp + 1968]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1952]
    mov [rsp + 1968], rcx
    mov [rsp + 1952], rsi
    mov rsi, [rsp + 2040]
    mov rcx, [rsp + 1960]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1944]
    mov [rsp + 1960], rcx
    mov [rsp + 1944], rdi
    mov rdi, [rsp + 2040]
    ; let x140: List[Pair[i64, i64]] = Cons(x141, r6);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43312
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43313

lab43312:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43310
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43303
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43301
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43302

lab43301:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43302:

lab43303:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43306
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43304
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43305

lab43304:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43305:

lab43306:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43309
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43307
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43308

lab43307:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43308:

lab43309:
    jmp lab43311

lab43310:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43311:

lab43313:
    ; #load tag
    mov qword [rsp + 1960], 5
    ; substitute (a0 !-> a0)(x133 !-> x133)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x117 !-> x117)(x121 !-> x121)(x125 !-> x125)(x129 !-> x129)(x137 !-> x137)(x140 !-> x140);
    ; #move variables
    mov rcx, [rsp + 1984]
    mov [rsp + 1984], rsi
    mov rsi, rcx
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], rdi
    mov rdi, rcx
    ; let x136: List[Pair[i64, i64]] = Cons(x137, x140);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43325
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43326

lab43325:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43323
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43316
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43314
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43315

lab43314:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43315:

lab43316:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43319
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43317
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43318

lab43317:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43318:

lab43319:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43322
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43320
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43321

lab43320:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43321:

lab43322:
    jmp lab43324

lab43323:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43324:

lab43326:
    ; #load tag
    mov qword [rsp + 1976], 5
    ; substitute (a0 !-> a0)(x129 !-> x129)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x117 !-> x117)(x121 !-> x121)(x125 !-> x125)(x133 !-> x133)(x136 !-> x136);
    ; #move variables
    mov rcx, [rsp + 2000]
    mov [rsp + 2000], rsi
    mov rsi, rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, rcx
    ; let x132: List[Pair[i64, i64]] = Cons(x133, x136);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43338
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43339

lab43338:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43336
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43328

lab43327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43328:

lab43329:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43332
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43330
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43331

lab43330:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43331:

lab43332:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43335
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43333
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43334

lab43333:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43334:

lab43335:
    jmp lab43337

lab43336:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43337:

lab43339:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(x125 !-> x125)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x117 !-> x117)(x121 !-> x121)(x129 !-> x129)(x132 !-> x132);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, rcx
    ; let x128: List[Pair[i64, i64]] = Cons(x129, x132);
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
    je lab43351
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43352

lab43351:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43349
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43342
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43340
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43341

lab43340:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43341:

lab43342:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43345
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43343
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43344

lab43343:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43344:

lab43345:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43348
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43346
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43347

lab43346:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43347:

lab43348:
    jmp lab43350

lab43349:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43350:

lab43352:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(x121 !-> x121)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x117 !-> x117)(x125 !-> x125)(x128 !-> x128);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    ; let x124: List[Pair[i64, i64]] = Cons(x125, x128);
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
    je lab43364
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab43365

lab43364:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43362
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43355
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43353
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43354

lab43353:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43354:

lab43355:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43358
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43356
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43357

lab43356:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43357:

lab43358:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43361
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43359
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43360

lab43359:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43360:

lab43361:
    jmp lab43363

lab43362:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43363:

lab43365:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(x117 !-> x117)(x105 !-> x105)(x109 !-> x109)(x113 !-> x113)(x121 !-> x121)(x124 !-> x124);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x120: List[Pair[i64, i64]] = Cons(x121, x124);
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
    je lab43377
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab43378

lab43377:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43375
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43368
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43366
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43367

lab43366:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43367:

lab43368:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43371
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43369
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43370

lab43369:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43370:

lab43371:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43374
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43372
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43373

lab43372:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43373:

lab43374:
    jmp lab43376

lab43375:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43376:

lab43378:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x113 !-> x113)(x105 !-> x105)(x109 !-> x109)(x117 !-> x117)(x120 !-> x120);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x116: List[Pair[i64, i64]] = Cons(x117, x120);
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
    je lab43390
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43391

lab43390:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43388
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43381
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43379
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43380

lab43379:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43380:

lab43381:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43384
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43382
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43383

lab43382:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43383:

lab43384:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43387
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43385
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43386

lab43385:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43386:

lab43387:
    jmp lab43389

lab43388:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43389:

lab43391:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x109 !-> x109)(x105 !-> x105)(x113 !-> x113)(x116 !-> x116);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x112: List[Pair[i64, i64]] = Cons(x113, x116);
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
    je lab43403
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43404

lab43403:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43401
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43394
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43392
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43393

lab43392:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43393:

lab43394:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43397
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43395
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43396

lab43395:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43396:

lab43397:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43400
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43398
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43399

lab43398:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43399:

lab43400:
    jmp lab43402

lab43401:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43402:

lab43404:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x105 !-> x105)(x109 !-> x109)(x112 !-> x112);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x108: List[Pair[i64, i64]] = Cons(x109, x112);
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
    je lab43416
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43417

lab43416:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43414
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43407
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43405
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43406

lab43405:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43406:

lab43407:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43410
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43408
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43409

lab43408:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43409:

lab43410:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43413
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43411
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43412

lab43411:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43412:

lab43413:
    jmp lab43415

lab43414:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43415:

lab43417:
    ; #load tag
    mov r9, 5
    ; let r5: List[Pair[i64, i64]] = Cons(x105, x108);
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
    je lab43429
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43430

lab43429:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43427
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43420
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43418
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43419

lab43418:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43419:

lab43420:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43423
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43421
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43422

lab43421:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43422:

lab43423:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43426
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43424
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43425

lab43424:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43425:

lab43426:
    jmp lab43428

lab43427:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43428:

lab43430:
    ; #load tag
    mov rdi, 5
    ; lit x145 <- 4;
    mov r9, 4
    ; lit x146 <- 18;
    mov r11, 18
    ; let x144: Pair[i64, i64] = Tup(x145, x146);
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
    je lab43442
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43443

lab43442:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43440
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43433
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43431
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43432

lab43431:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43432:

lab43433:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43436
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43434
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43435

lab43434:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43435:

lab43436:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43439
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43437
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43438

lab43437:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43438:

lab43439:
    jmp lab43441

lab43440:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43441:

lab43443:
    ; #load tag
    mov r9, 0
    ; lit x149 <- 4;
    mov r11, 4
    ; lit x150 <- 22;
    mov r13, 22
    ; let x148: Pair[i64, i64] = Tup(x149, x150);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43455
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43456

lab43455:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43453
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43446
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43444
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43445

lab43444:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43445:

lab43446:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43449
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43447
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43448

lab43447:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43448:

lab43449:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43452
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43450
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43451

lab43450:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43451:

lab43452:
    jmp lab43454

lab43453:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43454:

lab43456:
    ; #load tag
    mov r11, 0
    ; lit x153 <- 4;
    mov r13, 4
    ; lit x154 <- 23;
    mov r15, 23
    ; let x152: Pair[i64, i64] = Tup(x153, x154);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab43468
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43469

lab43468:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43466
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43459
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43457
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43458

lab43457:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43458:

lab43459:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43462
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43460
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43461

lab43460:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43461:

lab43462:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43465
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43463
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43464

lab43463:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43464:

lab43465:
    jmp lab43467

lab43466:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43467:

lab43469:
    ; #load tag
    mov r13, 0
    ; lit x157 <- 4;
    mov r15, 4
    ; lit x158 <- 32;
    mov qword [rsp + 2024], 32
    ; let x156: Pair[i64, i64] = Tup(x157, x158);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab43481
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab43482

lab43481:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43479
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43472
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43470
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43471

lab43470:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43471:

lab43472:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43475
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43473
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43474

lab43473:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43474:

lab43475:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43478
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43476
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43477

lab43476:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43477:

lab43478:
    jmp lab43480

lab43479:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43480:

lab43482:
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(x152 !-> x152)(x144 !-> x144)(x148 !-> x148)(x156 !-> x156)(r5 !-> r5);
    ; #move variables
    mov rcx, r12
    mov r12, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, r15
    mov r15, rdi
    mov rdi, rcx
    ; let x155: List[Pair[i64, i64]] = Cons(x156, r5);
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
    je lab43494
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43495

lab43494:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43492
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43484

lab43483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43484:

lab43485:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43488
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43486
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43487

lab43486:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43487:

lab43488:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43491
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43489
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43490

lab43489:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43490:

lab43491:
    jmp lab43493

lab43492:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43493:

lab43495:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x148 !-> x148)(x144 !-> x144)(x152 !-> x152)(x155 !-> x155);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x151: List[Pair[i64, i64]] = Cons(x152, x155);
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
    je lab43507
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43508

lab43507:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43505
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43498
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43496
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43497

lab43496:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43497:

lab43498:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43501
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43499
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43500

lab43499:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43500:

lab43501:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43504
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43502
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43503

lab43502:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43503:

lab43504:
    jmp lab43506

lab43505:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43506:

lab43508:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x144 !-> x144)(x148 !-> x148)(x151 !-> x151);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x147: List[Pair[i64, i64]] = Cons(x148, x151);
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
    je lab43520
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43521

lab43520:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43518
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43510

lab43509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43510:

lab43511:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43514
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43512
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43513

lab43512:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43513:

lab43514:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43517
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43515
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43516

lab43515:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43516:

lab43517:
    jmp lab43519

lab43518:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43519:

lab43521:
    ; #load tag
    mov r9, 5
    ; let r4: List[Pair[i64, i64]] = Cons(x144, x147);
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
    je lab43533
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43534

lab43533:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43531
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43524
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43522
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43523

lab43522:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43523:

lab43524:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43527
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43525
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43526

lab43525:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43526:

lab43527:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43530
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43528
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43529

lab43528:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43529:

lab43530:
    jmp lab43532

lab43531:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43532:

lab43534:
    ; #load tag
    mov rdi, 5
    ; lit x160 <- 3;
    mov r9, 3
    ; lit x161 <- 19;
    mov r11, 19
    ; let x159: Pair[i64, i64] = Tup(x160, x161);
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
    je lab43546
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43547

lab43546:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43544
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43537
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43535
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43536

lab43535:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43536:

lab43537:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43540
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43538
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43539

lab43538:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43539:

lab43540:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43543
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43541
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43542

lab43541:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43542:

lab43543:
    jmp lab43545

lab43544:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43545:

lab43547:
    ; #load tag
    mov r9, 0
    ; lit x164 <- 3;
    mov r11, 3
    ; lit x165 <- 21;
    mov r13, 21
    ; let x163: Pair[i64, i64] = Tup(x164, x165);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43559
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43560

lab43559:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43557
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43550
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43548
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43549

lab43548:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43549:

lab43550:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43553
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43551
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43552

lab43551:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43552:

lab43553:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43556
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43554
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43555

lab43554:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43555:

lab43556:
    jmp lab43558

lab43557:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43558:

lab43560:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(x159 !-> x159)(x163 !-> x163)(r4 !-> r4);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x162: List[Pair[i64, i64]] = Cons(x163, r4);
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
    je lab43572
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43573

lab43572:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43570
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43562

lab43561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43562:

lab43563:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43565

lab43564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43565:

lab43566:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43569
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43567
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43568

lab43567:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43568:

lab43569:
    jmp lab43571

lab43570:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43571:

lab43573:
    ; #load tag
    mov r9, 5
    ; let r3: List[Pair[i64, i64]] = Cons(x159, x162);
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
    je lab43585
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43586

lab43585:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43583
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43576
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43574
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43575

lab43574:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43575:

lab43576:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43579
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43577
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43578

lab43577:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43578:

lab43579:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43582
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43580
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43581

lab43580:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43581:

lab43582:
    jmp lab43584

lab43583:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43584:

lab43586:
    ; #load tag
    mov rdi, 5
    ; lit x167 <- 2;
    mov r9, 2
    ; lit x168 <- 20;
    mov r11, 20
    ; let x166: Pair[i64, i64] = Tup(x167, x168);
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
    je lab43598
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43599

lab43598:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43596
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43589
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43587
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43588

lab43587:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43588:

lab43589:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43592
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43590
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43591

lab43590:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43591:

lab43592:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43595
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43593
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43594

lab43593:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43594:

lab43595:
    jmp lab43597

lab43596:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43597:

lab43599:
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(x166 !-> x166)(r3 !-> r3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let r2: List[Pair[i64, i64]] = Cons(x166, r3);
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
    je lab43611
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43612

lab43611:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43609
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43602
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43600
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43601

lab43600:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43601:

lab43602:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43605
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43603
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43604

lab43603:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43604:

lab43605:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43608
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43606
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43607

lab43606:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43607:

lab43608:
    jmp lab43610

lab43609:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43610:

lab43612:
    ; #load tag
    mov rdi, 5
    ; substitute (r2 !-> r2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump mkgen_
    jmp mkgen_

go_gun_:
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Unit_43613:

Fun_i64_Unit_43613_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab43615
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab43614
    ; ####increment refcount
    add qword [rsi + 0], 1

lab43614:
    mov rdx, [rax + 40]
    jmp lab43616

lab43615:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab43616:
    ; new a2: Gen = (steps, a1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab43628
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab43629

lab43628:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43626
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43619
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43617
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43618

lab43617:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43618:

lab43619:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43622
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43620
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43621

lab43620:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43621:

lab43622:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43625
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43623
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43624

lab43623:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43624:

lab43625:
    jmp lab43627

lab43626:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43627:

lab43629:
    ; #load tag
    lea rdx, [rel Gen_43630]
    ; jump gun_
    jmp gun_

Gen_43630:

Gen_43630_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab43632
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab43631
    ; ####increment refcount
    add qword [r8 + 0], 1

lab43631:
    mov rdi, [rsi + 40]
    jmp lab43633

lab43632:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab43633:
    ; substitute (a1 !-> a1)(steps !-> steps)(coordslist1 !-> coordslist1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x0: Gen = Gen(coordslist1);
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
    je lab43645
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43646

lab43645:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43643
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43635

lab43634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43635:

lab43636:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43639
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43637
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43638

lab43637:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43638:

lab43639:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43642
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43640
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43641

lab43640:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43641:

lab43642:
    jmp lab43644

lab43643:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43644:

lab43646:
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(steps !-> steps)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a3: Gen = (a1)\{ ... \};
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
    je lab43658
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43659

lab43658:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43656
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43648

lab43647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43648:

lab43649:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43652
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43650
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43651

lab43650:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43651:

lab43652:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43655
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43653
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43654

lab43653:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43654:

lab43655:
    jmp lab43657

lab43656:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43657:

lab43659:
    ; #load tag
    lea r9, [rel Gen_43660]
    ; jump nthgen_
    jmp nthgen_

Gen_43660:

Gen_43660_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab43662
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab43661
    ; ####increment refcount
    add qword [rsi + 0], 1

lab43661:
    jmp lab43663

lab43662:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab43663:
    ; substitute (a1 !-> a1)(coordslist0 !-> coordslist0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let gen: Gen = Gen(coordslist0);
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
    je lab43675
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43676

lab43675:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43673
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43666
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43664
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43665

lab43664:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43665:

lab43666:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43669
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43667
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43668

lab43667:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43668:

lab43669:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43672
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43670
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43671

lab43670:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43671:

lab43672:
    jmp lab43674

lab43673:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43674:

lab43676:
    ; #load tag
    mov rdi, 0
    ; substitute (a1 !-> a1);
    ; #erase gen
    cmp rsi, 0
    je lab43679
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab43677
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab43678

lab43677:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab43678:

lab43679:
    ; invoke a1 Unit
    jmp rdx

centerLine_:
    ; lit x0 <- 5;
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

bail_:
    ; lit x1 <- 0;
    mov rdi, 0
    ; lit x2 <- 0;
    mov r9, 0
    ; let x0: Pair[i64, i64] = Tup(x1, x2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
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
    je lab43691
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43692

lab43691:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43689
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43681

lab43680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43681:

lab43682:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43684

lab43683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43684:

lab43685:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43687

lab43686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43687:

lab43688:
    jmp lab43690

lab43689:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43690:

lab43692:
    ; #load tag
    mov rdi, 0
    ; lit x5 <- 0;
    mov r9, 0
    ; lit x6 <- 1;
    mov r11, 1
    ; let x4: Pair[i64, i64] = Tup(x5, x6);
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
    je lab43704
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43705

lab43704:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43702
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43695
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43693
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43694

lab43693:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43694:

lab43695:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43698
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43696
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43697

lab43696:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43697:

lab43698:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43701
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43699
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43700

lab43699:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43700:

lab43701:
    jmp lab43703

lab43702:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43703:

lab43705:
    ; #load tag
    mov r9, 0
    ; lit x9 <- 1;
    mov r11, 1
    ; lit x10 <- 0;
    mov r13, 0
    ; let x8: Pair[i64, i64] = Tup(x9, x10);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43717
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43718

lab43717:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43715
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43708
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43707

lab43706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43707:

lab43708:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43711
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43709
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43710

lab43709:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43710:

lab43711:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43714
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43712
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43713

lab43712:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43713:

lab43714:
    jmp lab43716

lab43715:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43716:

lab43718:
    ; #load tag
    mov r11, 0
    ; lit x13 <- 1;
    mov r13, 1
    ; lit x14 <- 1;
    mov r15, 1
    ; let x12: Pair[i64, i64] = Tup(x13, x14);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab43730
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43731

lab43730:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43728
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43721
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43719
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43720

lab43719:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43720:

lab43721:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43724
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43722
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43723

lab43722:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43723:

lab43724:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43727
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43725
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43726

lab43725:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43726:

lab43727:
    jmp lab43729

lab43728:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43729:

lab43731:
    ; #load tag
    mov r13, 0
    ; let x15: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x11: List[Pair[i64, i64]] = Cons(x12, x15);
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
    je lab43743
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43744

lab43743:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43741
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43734
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43732
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43733

lab43732:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43733:

lab43734:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43737
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43735
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43736

lab43735:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43736:

lab43737:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43740
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43738
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43739

lab43738:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43739:

lab43740:
    jmp lab43742

lab43741:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43742:

lab43744:
    ; #load tag
    mov r13, 5
    ; let x7: List[Pair[i64, i64]] = Cons(x8, x11);
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
    je lab43756
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43757

lab43756:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43754
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43747
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43745
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43746

lab43745:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43746:

lab43747:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43750
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43748
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43749

lab43748:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43749:

lab43750:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43753
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43751
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43752

lab43751:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43752:

lab43753:
    jmp lab43755

lab43754:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43755:

lab43757:
    ; #load tag
    mov r11, 5
    ; let x3: List[Pair[i64, i64]] = Cons(x4, x7);
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
    je lab43769
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43770

lab43769:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43767
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43760
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43758
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43759

lab43758:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43759:

lab43760:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43763
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43761
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43762

lab43761:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43762:

lab43763:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43766
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43764
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43765

lab43764:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43765:

lab43766:
    jmp lab43768

lab43767:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43768:

lab43770:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x3 !-> x3)(a0 !-> a0);
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

shuttle_:
    ; lit x1 <- 4;
    mov rdi, 4
    ; lit x2 <- 1;
    mov r9, 1
    ; let x0: Pair[i64, i64] = Tup(x1, x2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
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
    je lab43782
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43783

lab43782:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43780
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43773
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43771
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43772

lab43771:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43772:

lab43773:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43776
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43774
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43775

lab43774:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43775:

lab43776:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43779
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43777
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43778

lab43777:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43778:

lab43779:
    jmp lab43781

lab43780:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43781:

lab43783:
    ; #load tag
    mov rdi, 0
    ; lit x5 <- 4;
    mov r9, 4
    ; lit x6 <- 0;
    mov r11, 0
    ; let x4: Pair[i64, i64] = Tup(x5, x6);
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
    je lab43795
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43796

lab43795:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43793
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43785

lab43784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43785:

lab43786:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43788

lab43787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43788:

lab43789:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43792
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43790
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43791

lab43790:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43791:

lab43792:
    jmp lab43794

lab43793:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43794:

lab43796:
    ; #load tag
    mov r9, 0
    ; lit x9 <- 4;
    mov r11, 4
    ; lit x10 <- 5;
    mov r13, 5
    ; let x8: Pair[i64, i64] = Tup(x9, x10);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43808
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43809

lab43808:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43806
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43798

lab43797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43798:

lab43799:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43801

lab43800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43801:

lab43802:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43805
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43803
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43804

lab43803:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43804:

lab43805:
    jmp lab43807

lab43806:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43807:

lab43809:
    ; #load tag
    mov r11, 0
    ; lit x13 <- 4;
    mov r13, 4
    ; lit x14 <- 6;
    mov r15, 6
    ; let x12: Pair[i64, i64] = Tup(x13, x14);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab43821
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43822

lab43821:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43819
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43812
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43810
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43811

lab43810:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43811:

lab43812:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43815
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43813
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43814

lab43813:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43814:

lab43815:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43818
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43816
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43817

lab43816:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43817:

lab43818:
    jmp lab43820

lab43819:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43820:

lab43822:
    ; #load tag
    mov r13, 0
    ; let x15: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x11: List[Pair[i64, i64]] = Cons(x12, x15);
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
    je lab43834
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43835

lab43834:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43832
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43825
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43823
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43824

lab43823:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43824:

lab43825:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43827

lab43826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43827:

lab43828:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43831
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43829
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43830

lab43829:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43830:

lab43831:
    jmp lab43833

lab43832:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43833:

lab43835:
    ; #load tag
    mov r13, 5
    ; let x7: List[Pair[i64, i64]] = Cons(x8, x11);
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
    je lab43847
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43848

lab43847:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43845
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43837

lab43836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43837:

lab43838:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43840

lab43839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43840:

lab43841:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43844
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43842
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43843

lab43842:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43843:

lab43844:
    jmp lab43846

lab43845:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43846:

lab43848:
    ; #load tag
    mov r11, 5
    ; let x3: List[Pair[i64, i64]] = Cons(x4, x7);
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
    je lab43860
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43861

lab43860:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43858
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43850

lab43849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43850:

lab43851:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43854
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43852
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43853

lab43852:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43853:

lab43854:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43857
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43855
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43856

lab43855:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43856:

lab43857:
    jmp lab43859

lab43858:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43859:

lab43861:
    ; #load tag
    mov r9, 5
    ; let r4: List[Pair[i64, i64]] = Cons(x0, x3);
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
    je lab43873
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43874

lab43873:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43871
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43863

lab43862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43863:

lab43864:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43866

lab43865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43866:

lab43867:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43870
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43868
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43869

lab43868:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43869:

lab43870:
    jmp lab43872

lab43871:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43872:

lab43874:
    ; #load tag
    mov rdi, 5
    ; lit x17 <- 3;
    mov r9, 3
    ; lit x18 <- 2;
    mov r11, 2
    ; let x16: Pair[i64, i64] = Tup(x17, x18);
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
    je lab43886
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43887

lab43886:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43884
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43876

lab43875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43876:

lab43877:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43880
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43878
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43879

lab43878:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43879:

lab43880:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43883
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43881
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43882

lab43881:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43882:

lab43883:
    jmp lab43885

lab43884:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43885:

lab43887:
    ; #load tag
    mov r9, 0
    ; lit x21 <- 3;
    mov r11, 3
    ; lit x22 <- 3;
    mov r13, 3
    ; let x20: Pair[i64, i64] = Tup(x21, x22);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43899
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43900

lab43899:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43897
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43889

lab43888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43889:

lab43890:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43892

lab43891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43892:

lab43893:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43896
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43894
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43895

lab43894:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43895:

lab43896:
    jmp lab43898

lab43897:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43898:

lab43900:
    ; #load tag
    mov r11, 0
    ; lit x25 <- 3;
    mov r13, 3
    ; lit x26 <- 4;
    mov r15, 4
    ; let x24: Pair[i64, i64] = Tup(x25, x26);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab43912
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab43913

lab43912:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43910
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43902

lab43901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43902:

lab43903:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43905

lab43904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43905:

lab43906:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43909
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43907
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43908

lab43907:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43908:

lab43909:
    jmp lab43911

lab43910:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43911:

lab43913:
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(x20 !-> x20)(x16 !-> x16)(x24 !-> x24)(r4 !-> r4);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x23: List[Pair[i64, i64]] = Cons(x24, r4);
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
    je lab43925
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43926

lab43925:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43923
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43915

lab43914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43915:

lab43916:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43918

lab43917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43918:

lab43919:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43921

lab43920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43921:

lab43922:
    jmp lab43924

lab43923:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43924:

lab43926:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x16 !-> x16)(x20 !-> x20)(x23 !-> x23);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x19: List[Pair[i64, i64]] = Cons(x20, x23);
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
    je lab43938
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43939

lab43938:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43936
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43928

lab43927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43928:

lab43929:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43932
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43930
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43931

lab43930:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43931:

lab43932:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43935
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43933
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43934

lab43933:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43934:

lab43935:
    jmp lab43937

lab43936:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43937:

lab43939:
    ; #load tag
    mov r9, 5
    ; let r3: List[Pair[i64, i64]] = Cons(x16, x19);
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
    je lab43951
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab43952

lab43951:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43949
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43941

lab43940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43941:

lab43942:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43945
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43943
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43944

lab43943:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43944:

lab43945:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43948
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43946
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43947

lab43946:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43947:

lab43948:
    jmp lab43950

lab43949:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43950:

lab43952:
    ; #load tag
    mov rdi, 5
    ; lit x28 <- 2;
    mov r9, 2
    ; lit x29 <- 1;
    mov r11, 1
    ; let x27: Pair[i64, i64] = Tup(x28, x29);
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
    je lab43964
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43965

lab43964:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43962
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43954

lab43953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43954:

lab43955:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43958
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43956
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43957

lab43956:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43957:

lab43958:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43961
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43959
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43960

lab43959:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43960:

lab43961:
    jmp lab43963

lab43962:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43963:

lab43965:
    ; #load tag
    mov r9, 0
    ; lit x32 <- 2;
    mov r11, 2
    ; lit x33 <- 5;
    mov r13, 5
    ; let x31: Pair[i64, i64] = Tup(x32, x33);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab43977
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab43978

lab43977:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43975
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43968
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43966
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43967

lab43966:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43967:

lab43968:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43970

lab43969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43970:

lab43971:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43974
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43972
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43973

lab43972:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43973:

lab43974:
    jmp lab43976

lab43975:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43976:

lab43978:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(x27 !-> x27)(x31 !-> x31)(r3 !-> r3);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x30: List[Pair[i64, i64]] = Cons(x31, r3);
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
    je lab43990
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab43991

lab43990:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab43988
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43980

lab43979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43980:

lab43981:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43983

lab43982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43983:

lab43984:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab43987
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43985
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43986

lab43985:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43986:

lab43987:
    jmp lab43989

lab43988:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab43989:

lab43991:
    ; #load tag
    mov r9, 5
    ; let r2: List[Pair[i64, i64]] = Cons(x27, x30);
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
    je lab44003
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44004

lab44003:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44001
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab43994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43993

lab43992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43993:

lab43994:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab43997
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43995
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43996

lab43995:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43996:

lab43997:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab43998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab43999

lab43998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab43999:

lab44000:
    jmp lab44002

lab44001:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44002:

lab44004:
    ; #load tag
    mov rdi, 5
    ; lit x35 <- 1;
    mov r9, 1
    ; lit x36 <- 2;
    mov r11, 2
    ; let x34: Pair[i64, i64] = Tup(x35, x36);
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
    je lab44016
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44017

lab44016:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44014
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44007
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44005
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44006

lab44005:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44006:

lab44007:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44010
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44008
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44009

lab44008:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44009:

lab44010:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44012

lab44011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44012:

lab44013:
    jmp lab44015

lab44014:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44015:

lab44017:
    ; #load tag
    mov r9, 0
    ; lit x39 <- 1;
    mov r11, 1
    ; lit x40 <- 4;
    mov r13, 4
    ; let x38: Pair[i64, i64] = Tup(x39, x40);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab44029
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44030

lab44029:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44027
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44020
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44018
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44019

lab44018:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44019:

lab44020:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44023
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44021
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44022

lab44021:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44022:

lab44023:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44026
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44024
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44025

lab44024:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44025:

lab44026:
    jmp lab44028

lab44027:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44028:

lab44030:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(x34 !-> x34)(x38 !-> x38)(r2 !-> r2);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x37: List[Pair[i64, i64]] = Cons(x38, r2);
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
    je lab44042
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44043

lab44042:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44040
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44033
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44031
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44032

lab44031:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44032:

lab44033:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44036
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44034
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44035

lab44034:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44035:

lab44036:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44039
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44037
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44038

lab44037:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44038:

lab44039:
    jmp lab44041

lab44040:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44041:

lab44043:
    ; #load tag
    mov r9, 5
    ; let r1: List[Pair[i64, i64]] = Cons(x34, x37);
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
    je lab44055
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44056

lab44055:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44053
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44046
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44044
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44045

lab44044:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44045:

lab44046:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44049
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44047
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44048

lab44047:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44048:

lab44049:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44052
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44050
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44051

lab44050:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44051:

lab44052:
    jmp lab44054

lab44053:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44054:

lab44056:
    ; #load tag
    mov rdi, 5
    ; lit x42 <- 0;
    mov r9, 0
    ; lit x43 <- 3;
    mov r11, 3
    ; let x41: Pair[i64, i64] = Tup(x42, x43);
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
    je lab44068
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44069

lab44068:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44066
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44058

lab44057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44058:

lab44059:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44061

lab44060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44061:

lab44062:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44065
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44063
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44064

lab44063:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44064:

lab44065:
    jmp lab44067

lab44066:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44067:

lab44069:
    ; #load tag
    mov r9, 0
    ; substitute (x41 !-> x41)(r1 !-> r1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Cons
    add r9, 5
    jmp r9

at_pos_:
    ; substitute (coordlist !-> coordlist)(a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new move: Fun[Pair[i64, i64], Pair[i64, i64]] = (p)\{ ... \};
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
    je lab44081
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44082

lab44081:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44079
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44072
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44070
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44071

lab44070:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44071:

lab44072:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44075
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44074

lab44073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44074:

lab44075:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44078
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44076
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44077

lab44076:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44077:

lab44078:
    jmp lab44080

lab44079:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44080:

lab44082:
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_Pair_i64_i64_44083]
    ; substitute (coordlist !-> coordlist)(move !-> move)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump map_
    jmp map_

Fun_Pair_i64_i64_Pair_i64_i64_44083:

Fun_Pair_i64_i64_Pair_i64_i64_44083_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44085
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44084
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44084:
    jmp lab44086

lab44085:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44086:
    ; substitute (p !-> p)(a1 !-> a1)(a !-> a);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch a \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_44087:

Pair_i64_i64_44087_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44088
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab44089

lab44088:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab44089:
    ; substitute (snd1 !-> snd1)(a1 !-> a1)(fst1 !-> fst1)(p !-> p);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_44090:

Pair_i64_i64_44090_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab44091
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]
    jmp lab44092

lab44091:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r11, [r10 + 40]

lab44092:
    ; x0 <- fst1 + fst2;
    mov r15, r9
    add r15, r11
    ; substitute (snd1 !-> snd1)(a1 !-> a1)(x0 !-> x0)(snd2 !-> snd2);
    ; #move variables
    mov r11, r13
    mov r9, r15
    ; x1 <- snd1 + snd2;
    mov r13, rdx
    add r13, r11
    ; substitute (x0 !-> x0)(x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov r8, rsi
    mov rdx, r9
    mov r9, rdi
    mov rdi, r13
    ; invoke a1 Tup
    jmp r9

non_steady_:
    ; new a1: List[Pair[i64, i64]] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44104
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44105

lab44104:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44102
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44095
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44093
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44094

lab44093:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44094:

lab44095:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44098
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44096
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44097

lab44096:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44097:

lab44098:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44101
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44099
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44100

lab44099:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44100:

lab44101:
    jmp lab44103

lab44102:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44103:

lab44105:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44106]
    ; new a2: List[Pair[i64, i64]] = (a1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44118
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44119

lab44118:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44116
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44109
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44107
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44108

lab44107:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44108:

lab44109:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44112
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44110
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44111

lab44110:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44111:

lab44112:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44115
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44113
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44114

lab44113:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44114:

lab44115:
    jmp lab44117

lab44116:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44117:

lab44119:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44120]
    ; new a3: List[Pair[i64, i64]] = (a2)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44132
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44133

lab44132:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44130
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44123
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44121
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44122

lab44121:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44122:

lab44123:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44126
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44124
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44125

lab44124:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44125:

lab44126:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44129
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44127
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44128

lab44127:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44128:

lab44129:
    jmp lab44131

lab44130:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44131:

lab44133:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44134]
    ; new a4: List[Pair[i64, i64]] = (a3)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44146
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44147

lab44146:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44144
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44137
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44135
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44136

lab44135:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44136:

lab44137:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44140
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44138
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44139

lab44138:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44139:

lab44140:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44143
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44141
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44142

lab44141:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44142:

lab44143:
    jmp lab44145

lab44144:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44145:

lab44147:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44148]
    ; jump bail_
    jmp bail_

List_Pair_i64_i64_44148:
    jmp near List_Pair_i64_i64_44148_Nil
    jmp near List_Pair_i64_i64_44148_Cons

List_Pair_i64_i64_44148_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44150
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44149
    ; ####increment refcount
    add qword [rax + 0], 1

lab44149:
    jmp lab44151

lab44150:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44151:
    ; let x3: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_non_steady_4_
    jmp lift_non_steady_4_

List_Pair_i64_i64_44148_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44153
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44152
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44152:
    jmp lab44154

lab44153:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44154:
    ; substitute (a3 !-> a3)(x26 !-> x26)(xs7 !-> xs7);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x3: List[Pair[i64, i64]] = Cons(x26, xs7);
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
    je lab44166
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44167

lab44166:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44164
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44156

lab44155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44156:

lab44157:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44159

lab44158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44159:

lab44160:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44163
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44161
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44162

lab44161:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44162:

lab44163:
    jmp lab44165

lab44164:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44165:

lab44167:
    ; #load tag
    mov rdi, 5
    ; jump lift_non_steady_4_
    jmp lift_non_steady_4_

List_Pair_i64_i64_44134:
    jmp near List_Pair_i64_i64_44134_Nil
    jmp near List_Pair_i64_i64_44134_Cons

List_Pair_i64_i64_44134_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44169
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44168
    ; ####increment refcount
    add qword [rax + 0], 1

lab44168:
    jmp lab44170

lab44169:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44170:
    ; let x2: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_non_steady_2_
    jmp lift_non_steady_2_

List_Pair_i64_i64_44134_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44172
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44171
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44171:
    jmp lab44173

lab44172:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44173:
    ; substitute (a2 !-> a2)(x25 !-> x25)(xs6 !-> xs6);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x2: List[Pair[i64, i64]] = Cons(x25, xs6);
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
    je lab44185
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44186

lab44185:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44183
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44176
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44174
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44175

lab44174:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44175:

lab44176:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44179
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44177
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44178

lab44177:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44178:

lab44179:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44182
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44180
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44181

lab44180:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44181:

lab44182:
    jmp lab44184

lab44183:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44184:

lab44186:
    ; #load tag
    mov rdi, 5
    ; jump lift_non_steady_2_
    jmp lift_non_steady_2_

List_Pair_i64_i64_44120:
    jmp near List_Pair_i64_i64_44120_Nil
    jmp near List_Pair_i64_i64_44120_Cons

List_Pair_i64_i64_44120_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44188
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44187
    ; ####increment refcount
    add qword [rax + 0], 1

lab44187:
    jmp lab44189

lab44188:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44189:
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_non_steady_0_
    jmp lift_non_steady_0_

List_Pair_i64_i64_44120_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44191
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44190
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44190:
    jmp lab44192

lab44191:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44192:
    ; substitute (a1 !-> a1)(x22 !-> x22)(xs3 !-> xs3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x1: List[Pair[i64, i64]] = Cons(x22, xs3);
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
    je lab44204
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44205

lab44204:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44202
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44195
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44193
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44194

lab44193:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44194:

lab44195:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44198
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44196
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44197

lab44196:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44197:

lab44198:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44201
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44199
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44200

lab44199:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44200:

lab44201:
    jmp lab44203

lab44202:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44203:

lab44205:
    ; #load tag
    mov rdi, 5
    ; jump lift_non_steady_0_
    jmp lift_non_steady_0_

List_Pair_i64_i64_44106:
    jmp near List_Pair_i64_i64_44106_Nil
    jmp near List_Pair_i64_i64_44106_Cons

List_Pair_i64_i64_44106_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44207
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44206
    ; ####increment refcount
    add qword [rax + 0], 1

lab44206:
    jmp lab44208

lab44207:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44208:
    ; let x0: List[Pair[i64, i64]] = Nil();
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
    ; jump mkgen_
    jmp mkgen_

List_Pair_i64_i64_44106_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44210
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44209
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44209:
    jmp lab44211

lab44210:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44211:
    ; substitute (a0 !-> a0)(x19 !-> x19)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[Pair[i64, i64]] = Cons(x19, xs0);
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
    je lab44223
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44224

lab44223:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44221
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44214
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44212
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44213

lab44212:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44213:

lab44214:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44217
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44215
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44216

lab44215:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44216:

lab44217:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44219

lab44218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44219:

lab44220:
    jmp lab44222

lab44221:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44222:

lab44224:
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
    ; jump mkgen_
    jmp mkgen_

lift_non_steady_4_:
    ; lit x5 <- 1;
    mov r9, 1
    ; new a5: _Cont = (a3, x3, x5)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    mov [rbx + 24], rdx
    mov [rbx + 16], rax
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44236
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44237

lab44236:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44234
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44227
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44225
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44226

lab44225:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44226:

lab44227:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44230
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44228
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44229

lab44228:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44229:

lab44230:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44233
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44231
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44232

lab44231:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44232:

lab44233:
    jmp lab44235

lab44234:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44235:

lab44237:
    ; #load tag
    lea rdx, [rel _Cont_44238]
    ; jump centerLine_
    jmp centerLine_

_Cont_44238:

_Cont_44238_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44241
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44239
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44239:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab44240
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44240:
    jmp lab44242

lab44241:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab44242:
    ; substitute (x3 !-> x3)(a3 !-> a3)(x5 !-> x5)(x6 !-> x6);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x4: Pair[i64, i64] = Tup(x5, x6);
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
    je lab44254
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44255

lab44254:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44252
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44245
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44243
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44244

lab44243:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44244:

lab44245:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44248
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44246
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44247

lab44246:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44247:

lab44248:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44251
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44249
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44250

lab44249:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44250:

lab44251:
    jmp lab44253

lab44252:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44253:

lab44255:
    ; #load tag
    mov r9, 0
    ; substitute (x3 !-> x3)(x4 !-> x4)(a3 !-> a3);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump at_pos_
    jmp at_pos_

lift_non_steady_2_:
    ; new a6: List[Pair[i64, i64]] = (a2, x2)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44267
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44268

lab44267:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44265
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44258
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44256
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44257

lab44256:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44257:

lab44258:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44261
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44259
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44260

lab44259:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44260:

lab44261:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44264
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44262
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44263

lab44262:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44263:

lab44264:
    jmp lab44266

lab44265:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44266:

lab44268:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44269]
    ; new a7: List[Pair[i64, i64]] = (a6)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44281
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44282

lab44281:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44279
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44271

lab44270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44271:

lab44272:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44275
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44273
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44274

lab44273:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44274:

lab44275:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44278
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44276
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44277

lab44276:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44277:

lab44278:
    jmp lab44280

lab44279:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44280:

lab44282:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44283]
    ; jump bail_
    jmp bail_

List_Pair_i64_i64_44283:
    jmp near List_Pair_i64_i64_44283_Nil
    jmp near List_Pair_i64_i64_44283_Cons

List_Pair_i64_i64_44283_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44285
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44284
    ; ####increment refcount
    add qword [rax + 0], 1

lab44284:
    jmp lab44286

lab44285:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44286:
    ; let x8: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_non_steady_3_
    jmp lift_non_steady_3_

List_Pair_i64_i64_44283_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44288
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44287
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44287:
    jmp lab44289

lab44288:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44289:
    ; substitute (a6 !-> a6)(x24 !-> x24)(xs5 !-> xs5);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x8: List[Pair[i64, i64]] = Cons(x24, xs5);
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
    je lab44301
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44302

lab44301:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44299
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44292
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44290
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44291

lab44290:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44291:

lab44292:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44295
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44293
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44294

lab44293:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44294:

lab44295:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44298
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44296
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44297

lab44296:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44297:

lab44298:
    jmp lab44300

lab44299:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44300:

lab44302:
    ; #load tag
    mov rdi, 5
    ; jump lift_non_steady_3_
    jmp lift_non_steady_3_

List_Pair_i64_i64_44269:
    jmp near List_Pair_i64_i64_44269_Nil
    jmp near List_Pair_i64_i64_44269_Cons

List_Pair_i64_i64_44269_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44305
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab44303
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44303:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab44304
    ; ####increment refcount
    add qword [rax + 0], 1

lab44304:
    jmp lab44306

lab44305:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab44306:
    ; let x7: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x7 !-> x7)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump append_
    jmp append_

List_Pair_i64_i64_44269_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44309
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab44307
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44307:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44308
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44308:
    jmp lab44310

lab44309:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab44310:
    ; substitute (x2 !-> x2)(a2 !-> a2)(x23 !-> x23)(xs4 !-> xs4);
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
    ; let x7: List[Pair[i64, i64]] = Cons(x23, xs4);
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
    je lab44322
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44323

lab44322:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44320
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44313
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44311
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44312

lab44311:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44312:

lab44313:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44316
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44314
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44315

lab44314:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44315:

lab44316:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44319
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44317
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44318

lab44317:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44318:

lab44319:
    jmp lab44321

lab44320:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44321:

lab44323:
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x7 !-> x7)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump append_
    jmp append_

lift_non_steady_3_:
    ; lit x10 <- 21;
    mov r9, 21
    ; new a8: _Cont = (a6, x8, x10)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    mov [rbx + 24], rdx
    mov [rbx + 16], rax
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44335
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44336

lab44335:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44333
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44326
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44324
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44325

lab44324:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44325:

lab44326:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44328

lab44327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44328:

lab44329:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44332
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44330
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44331

lab44330:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44331:

lab44332:
    jmp lab44334

lab44333:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44334:

lab44336:
    ; #load tag
    lea rdx, [rel _Cont_44337]
    ; jump centerLine_
    jmp centerLine_

_Cont_44337:

_Cont_44337_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44340
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44338
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44338:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab44339
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44339:
    jmp lab44341

lab44340:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab44341:
    ; substitute (x8 !-> x8)(a6 !-> a6)(x10 !-> x10)(x11 !-> x11);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x9: Pair[i64, i64] = Tup(x10, x11);
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
    je lab44353
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44354

lab44353:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44351
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44344
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44342
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44343

lab44342:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44343:

lab44344:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44347
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44345
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44346

lab44345:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44346:

lab44347:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44350
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44348
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44349

lab44348:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44349:

lab44350:
    jmp lab44352

lab44351:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44352:

lab44354:
    ; #load tag
    mov r9, 0
    ; substitute (x8 !-> x8)(x9 !-> x9)(a6 !-> a6);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump at_pos_
    jmp at_pos_

lift_non_steady_0_:
    ; new a9: List[Pair[i64, i64]] = (a1, x1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov [rbx + 32], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44366
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44367

lab44366:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44364
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44357
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44355
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44356

lab44355:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44356:

lab44357:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44360
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44358
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44359

lab44358:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44359:

lab44360:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44363
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44361
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44362

lab44361:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44362:

lab44363:
    jmp lab44365

lab44364:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44365:

lab44367:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44368]
    ; new a10: List[Pair[i64, i64]] = (a9)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44380
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44381

lab44380:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44378
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44371
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44369
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44370

lab44369:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44370:

lab44371:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44374
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44372
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44373

lab44372:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44373:

lab44374:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44377
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44375
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44376

lab44375:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44376:

lab44377:
    jmp lab44379

lab44378:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44379:

lab44381:
    ; #load tag
    lea rdx, [rel List_Pair_i64_i64_44382]
    ; jump shuttle_
    jmp shuttle_

List_Pair_i64_i64_44382:
    jmp near List_Pair_i64_i64_44382_Nil
    jmp near List_Pair_i64_i64_44382_Cons

List_Pair_i64_i64_44382_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44384
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44383
    ; ####increment refcount
    add qword [rax + 0], 1

lab44383:
    jmp lab44385

lab44384:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44385:
    ; let x13: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_non_steady_1_
    jmp lift_non_steady_1_

List_Pair_i64_i64_44382_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44387
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab44386
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44386:
    jmp lab44388

lab44387:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab44388:
    ; substitute (a9 !-> a9)(x21 !-> x21)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x13: List[Pair[i64, i64]] = Cons(x21, xs2);
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
    je lab44400
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44401

lab44400:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44398
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44391
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44389
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44390

lab44389:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44390:

lab44391:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44394
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44392
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44393

lab44392:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44393:

lab44394:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44397
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44395
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44396

lab44395:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44396:

lab44397:
    jmp lab44399

lab44398:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44399:

lab44401:
    ; #load tag
    mov rdi, 5
    ; jump lift_non_steady_1_
    jmp lift_non_steady_1_

List_Pair_i64_i64_44368:
    jmp near List_Pair_i64_i64_44368_Nil
    jmp near List_Pair_i64_i64_44368_Cons

List_Pair_i64_i64_44368_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44404
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab44402
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44402:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab44403
    ; ####increment refcount
    add qword [rax + 0], 1

lab44403:
    jmp lab44405

lab44404:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab44405:
    ; let x12: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(x12 !-> x12)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump append_
    jmp append_

List_Pair_i64_i64_44368_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44408
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab44406
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44406:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44407
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44407:
    jmp lab44409

lab44408:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab44409:
    ; substitute (x1 !-> x1)(a1 !-> a1)(x20 !-> x20)(xs1 !-> xs1);
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
    ; let x12: List[Pair[i64, i64]] = Cons(x20, xs1);
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
    je lab44421
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44422

lab44421:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44419
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44412
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44410
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44411

lab44410:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44411:

lab44412:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44415
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44413
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44414

lab44413:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44414:

lab44415:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44418
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44416
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44417

lab44416:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44417:

lab44418:
    jmp lab44420

lab44419:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44420:

lab44422:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x12 !-> x12)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump append_
    jmp append_

lift_non_steady_1_:
    ; lit x15 <- 6;
    mov r9, 6
    ; new a12: _Cont = (a9, x13, x15)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    mov [rbx + 24], rdx
    mov [rbx + 16], rax
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44434
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44435

lab44434:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44432
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44425
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44423
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44424

lab44423:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44424:

lab44425:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44428
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44426
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44427

lab44426:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44427:

lab44428:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44431
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44429
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44430

lab44429:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44430:

lab44431:
    jmp lab44433

lab44432:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44433:

lab44435:
    ; #load tag
    lea rdx, [rel _Cont_44436]
    ; jump centerLine_
    jmp centerLine_

_Cont_44436:

_Cont_44436_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44439
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44437
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44437:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab44438
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44438:
    jmp lab44440

lab44439:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab44440:
    ; lit x17 <- 2;
    mov r13, 2
    ; x18 <- x16 - x17;
    mov r15, rdx
    sub r15, r13
    ; substitute (x13 !-> x13)(a9 !-> a9)(x15 !-> x15)(x18 !-> x18);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    mov r9, r11
    mov r11, r15
    ; let x14: Pair[i64, i64] = Tup(x15, x18);
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
    je lab44452
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44453

lab44452:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44450
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44443
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44441
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44442

lab44441:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44442:

lab44443:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44446
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44444
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44445

lab44444:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44445:

lab44446:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44449
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44447
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44448

lab44447:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44448:

lab44449:
    jmp lab44451

lab44450:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44451:

lab44453:
    ; #load tag
    mov r9, 0
    ; substitute (x13 !-> x13)(x14 !-> x14)(a9 !-> a9);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump at_pos_
    jmp at_pos_

go_shuttle_:
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Unit_44454:

Fun_i64_Unit_44454_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44456
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab44455
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44455:
    mov rdx, [rax + 40]
    jmp lab44457

lab44456:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab44457:
    ; new a2: Gen = (steps, a1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov [rbx + 48], rsi
    mov [rbx + 40], rdx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab44469
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab44470

lab44469:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44467
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44460
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44458
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44459

lab44458:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44459:

lab44460:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44463
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44461
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44462

lab44461:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44462:

lab44463:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44466
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44464
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44465

lab44464:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44465:

lab44466:
    jmp lab44468

lab44467:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44468:

lab44470:
    ; #load tag
    lea rdx, [rel Gen_44471]
    ; jump non_steady_
    jmp non_steady_

Gen_44471:

Gen_44471_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44473
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44472
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44472:
    mov rdi, [rsi + 40]
    jmp lab44474

lab44473:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab44474:
    ; substitute (a1 !-> a1)(steps !-> steps)(coordslist1 !-> coordslist1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let x0: Gen = Gen(coordslist1);
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
    je lab44486
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44487

lab44486:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44484
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44477
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44475
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44476

lab44475:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44476:

lab44477:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44480
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44478
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44479

lab44478:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44479:

lab44480:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44483
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44481
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44482

lab44481:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44482:

lab44483:
    jmp lab44485

lab44484:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44485:

lab44487:
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(steps !-> steps)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a3: Gen = (a1)\{ ... \};
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
    je lab44499
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44500

lab44499:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44497
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44490
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44488
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44489

lab44488:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44489:

lab44490:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44493
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44491
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44492

lab44491:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44492:

lab44493:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44496
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44494
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44495

lab44494:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44495:

lab44496:
    jmp lab44498

lab44497:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44498:

lab44500:
    ; #load tag
    lea r9, [rel Gen_44501]
    ; jump nthgen_
    jmp nthgen_

Gen_44501:

Gen_44501_Gen:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44503
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab44502
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44502:
    jmp lab44504

lab44503:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab44504:
    ; substitute (a1 !-> a1)(coordslist0 !-> coordslist0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let gen: Gen = Gen(coordslist0);
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
    je lab44516
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44517

lab44516:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44514
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44507
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44505
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44506

lab44505:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44506:

lab44507:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44510
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44508
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44509

lab44508:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44509:

lab44510:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44512

lab44511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44512:

lab44513:
    jmp lab44515

lab44514:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44515:

lab44517:
    ; #load tag
    mov rdi, 0
    ; substitute (a1 !-> a1);
    ; #erase gen
    cmp rsi, 0
    je lab44520
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44518
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44519

lab44518:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44519:

lab44520:
    ; invoke a1 Unit
    jmp rdx

go_loop_:
    ; if iters == 0 \{ ... \}
    cmp rdx, 0
    je lab44521
    ; substitute (go0 !-> go)(steps0 !-> steps)(go !-> go)(a0 !-> a0)(iters !-> iters)(steps !-> steps);
    ; #share go
    cmp r8, 0
    je lab44522
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44522:
    ; #move variables
    mov r13, rdx
    mov r15, rdi
    mov rax, r8
    mov rdx, r9
    ; new a2: Unit = (go, a0, iters, steps)\{ ... \};
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
    je lab44534
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44535

lab44534:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44532
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44525
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44523
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44524

lab44523:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44524:

lab44525:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44528
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44526
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44527

lab44526:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44527:

lab44528:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44531
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44529
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44530

lab44529:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44530:

lab44531:
    jmp lab44533

lab44532:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44533:

lab44535:
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
    je lab44547
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44548

lab44547:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44545
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44538
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44536
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44537

lab44536:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44537:

lab44538:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44541
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44539
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44540

lab44539:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44540:

lab44541:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44544
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44542
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44543

lab44542:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44543:

lab44544:
    jmp lab44546

lab44545:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44546:

lab44548:
    ; #load tag
    lea r9, [rel Unit_44549]
    ; substitute (steps0 !-> steps0)(a2 !-> a2)(go0 !-> go0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke go0 Apply
    jmp r9

Unit_44549:

Unit_44549_Unit:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44552
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab44550
    ; ####increment refcount
    add qword [rax + 0], 1

lab44550:
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab44551
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44551:
    jmp lab44553

lab44552:
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
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab44553:
    ; let res: Unit = Unit();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (go !-> go)(a0 !-> a0)(iters !-> iters)(steps !-> steps);
    ; #erase res
    cmp r12, 0
    je lab44556
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab44554
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab44555

lab44554:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab44555:

lab44556:
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- iters - x0;
    mov r15, r9
    sub r15, r13
    ; substitute (x1 !-> x1)(steps !-> steps)(go !-> go)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov r9, rdx
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rdx, r15
    ; jump go_loop_
    jmp go_loop_

lab44521:
    ; substitute (a0 !-> a0);
    ; #erase go
    cmp r8, 0
    je lab44559
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab44557
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab44558

lab44557:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab44558:

lab44559:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; lit x2 <- 0;
    mov rdi, 0
    ; substitute (x2 !-> x2)(a0 !-> a0);
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