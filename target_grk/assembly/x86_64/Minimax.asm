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
    mov rdx, rsi
    ; actual code

main_:
    ; new a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel _Cont_44765]
    ; jump main_loop_
    jmp main_loop_

_Cont_44765:

_Cont_44765_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

mk_leaf_:
    ; let x0: List[RoseTree[Pair[List[Option[Player]], i64]]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (p !-> p)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke a0 Rose
    jmp r9

top_:
    ; substitute (a0 !-> a0)(t !-> t);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch t \{ ... \};
    ; #if there is only one clause, we can just fall through

RoseTree_Pair_List_Option_Player_i64_44766:

RoseTree_Pair_List_Option_Player_i64_44766_Rose:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44769
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44767
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44767:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44768
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44768:
    jmp lab44770

lab44769:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44770:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #erase ps
    cmp r8, 0
    je lab44773
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab44771
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab44772

lab44771:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab44772:

lab44773:
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_List_Option_Player_i64_44774:

Pair_List_Option_Player_i64_44774_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44776
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44775
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44775:
    jmp lab44777

lab44776:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44777:
    ; substitute (a1 !-> a1)(b0 !-> b0)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; invoke a0 Tup
    jmp r9

snd_:
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

Pair_List_Option_Player_i64_44778:

Pair_List_Option_Player_i64_44778_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44780
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44779
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44779:
    jmp lab44781

lab44780:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44781:
    ; substitute (b !-> b)(a0 !-> a0);
    ; #erase a
    cmp rsi, 0
    je lab44784
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44782
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44783

lab44782:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44783:

lab44784:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

player_eq_:
    ; substitute (a0 !-> a0)(p2 !-> p2)(p1 !-> p1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch p1 \{ ... \};
    lea rcx, [rel Player_44785]
    add rcx, r9
    jmp rcx

Player_44785:
    jmp near Player_44785_X
    jmp near Player_44785_O

Player_44785_X:
    ; switch p2 \{ ... \};
    lea rcx, [rel Player_44786]
    add rcx, rdi
    jmp rcx

Player_44786:
    jmp near Player_44786_X
    jmp near Player_44786_O

Player_44786_X:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Player_44786_O:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Player_44785_O:
    ; switch p2 \{ ... \};
    lea rcx, [rel Player_44787]
    add rcx, rdi
    jmp rcx

Player_44787:
    jmp near Player_44787_X
    jmp near Player_44787_O

Player_44787_X:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Player_44787_O:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

other_:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    lea rcx, [rel Player_44788]
    add rcx, rdi
    jmp rcx

Player_44788:
    jmp near Player_44788_X
    jmp near Player_44788_O

Player_44788_X:
    ; invoke a0 O
    add rdx, 5
    jmp rdx

Player_44788_O:
    ; invoke a0 X
    add rdx, 0
    jmp rdx

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
    lea rcx, [rel Bool_44789]
    add rcx, r9
    jmp rcx

Bool_44789:
    jmp near Bool_44789_True
    jmp near Bool_44789_False

Bool_44789_True:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_44790]
    add rcx, rdi
    jmp rcx

Bool_44790:
    jmp near Bool_44790_True
    jmp near Bool_44790_False

Bool_44790_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_44790_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_44789_False:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab44793
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44791
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44792

lab44791:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44792:

lab44793:
    ; invoke a0 False
    add rdx, 5
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
    lea rcx, [rel Bool_44794]
    add rcx, r9
    jmp rcx

Bool_44794:
    jmp near Bool_44794_True
    jmp near Bool_44794_False

Bool_44794_True:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab44797
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44795
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44796

lab44795:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44796:

lab44797:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_44794_False:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_44798]
    add rcx, rdi
    jmp rcx

Bool_44798:
    jmp near Bool_44798_True
    jmp near Bool_44798_False

Bool_44798_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_44798_False:
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
    lea rcx, [rel Bool_44799]
    add rcx, rdi
    jmp rcx

Bool_44799:
    jmp near Bool_44799_True
    jmp near Bool_44799_False

Bool_44799_True:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_44799_False:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

is_some_:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    lea rcx, [rel Option_Player_44800]
    add rcx, rdi
    jmp rcx

Option_Player_44800:
    jmp near Option_Player_44800_None
    jmp near Option_Player_44800_Some

Option_Player_44800_None:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Option_Player_44800_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44802
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab44801
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44801:
    jmp lab44803

lab44802:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab44803:
    ; substitute (a0 !-> a0);
    ; #erase p0
    cmp rsi, 0
    je lab44806
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44804
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44805

lab44804:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44805:

lab44806:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

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
    lea rcx, [rel List_Option_Player_44807]
    add rcx, rdi
    jmp rcx

List_Option_Player_44807:
    jmp near List_Option_Player_44807_Nil
    jmp near List_Option_Player_44807_Cons

List_Option_Player_44807_Nil:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

List_Option_Player_44807_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44810
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44808
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44808:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44809
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44809:
    jmp lab44811

lab44810:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44811:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #erase ps
    cmp r8, 0
    je lab44814
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab44812
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab44813

lab44812:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab44813:

lab44814:
    ; switch p \{ ... \};
    lea rcx, [rel Option_Player_44815]
    add rcx, rdi
    jmp rcx

Option_Player_44815:
    jmp near Option_Player_44815_None
    jmp near Option_Player_44815_Some

Option_Player_44815_None:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

Option_Player_44815_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44817
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab44816
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44816:
    jmp lab44818

lab44817:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab44818:
    ; substitute (a1 !-> a1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

tail_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Option_Player_44819]
    add rcx, rdi
    jmp rcx

List_Option_Player_44819:
    jmp near List_Option_Player_44819_Nil
    jmp near List_Option_Player_44819_Cons

List_Option_Player_44819_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Option_Player_44819_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44822
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44820
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44820:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44821
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44821:
    jmp lab44823

lab44822:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44823:
    ; substitute (a0 !-> a0)(ps !-> ps);
    ; #erase p
    cmp rsi, 0
    je lab44826
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44824
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44825

lab44824:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44825:

lab44826:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch ps \{ ... \};
    lea rcx, [rel List_Option_Player_44827]
    add rcx, rdi
    jmp rcx

List_Option_Player_44827:
    jmp near List_Option_Player_44827_Nil
    jmp near List_Option_Player_44827_Cons

List_Option_Player_44827_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Option_Player_44827_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44830
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44828
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44828:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44829
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44829:
    jmp lab44831

lab44830:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44831:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

rev_acc_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_44832]
    add rcx, r9
    jmp rcx

List_i64_44832:
    jmp near List_i64_44832_Nil
    jmp near List_i64_44832_Cons

List_i64_44832_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_i64_44833]
    add rcx, rdi
    jmp rcx

List_i64_44833:
    jmp near List_i64_44833_Nil
    jmp near List_i64_44833_Cons

List_i64_44833_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_44833_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44835
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44834
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44834:
    mov rdi, [rsi + 40]
    jmp lab44836

lab44835:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab44836:
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

List_i64_44832_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44838
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab44837
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44837:
    mov r9, [r8 + 40]
    jmp lab44839

lab44838:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab44839:
    ; substitute (a0 !-> a0)(xs !-> xs)(x !-> x)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[i64] = Cons(x, acc);
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
    je lab44851
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44852

lab44851:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44849
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44842
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44840
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44841

lab44840:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44841:

lab44842:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44844

lab44843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44844:

lab44845:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44848
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44846
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44847

lab44846:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44847:

lab44848:
    jmp lab44850

lab44849:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44850:

lab44852:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_acc_
    jmp rev_acc_

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
    ; jump rev_acc_
    jmp rev_acc_

rev_i_board_acc_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_Option_Player_44853]
    add rcx, r9
    jmp rcx

List_List_Option_Player_44853:
    jmp near List_List_Option_Player_44853_Nil
    jmp near List_List_Option_Player_44853_Cons

List_List_Option_Player_44853_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_List_Option_Player_44854]
    add rcx, rdi
    jmp rcx

List_List_Option_Player_44854:
    jmp near List_List_Option_Player_44854_Nil
    jmp near List_List_Option_Player_44854_Cons

List_List_Option_Player_44854_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_Option_Player_44854_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44857
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44855
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44855:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44856
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44856:
    jmp lab44858

lab44857:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44858:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

List_List_Option_Player_44853_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44861
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab44859
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44859:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44860
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44860:
    jmp lab44862

lab44861:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab44862:
    ; substitute (a0 !-> a0)(xs !-> xs)(x !-> x)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[List[Option[Player]]] = Cons(x, acc);
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
    je lab44874
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44875

lab44874:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44872
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44864

lab44863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44864:

lab44865:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44867

lab44866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44867:

lab44868:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44870

lab44869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44870:

lab44871:
    jmp lab44873

lab44872:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44873:

lab44875:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_i_board_acc_
    jmp rev_i_board_acc_

rev_i_board_:
    ; let x0: List[List[Option[Player]]] = Nil();
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
    ; jump rev_i_board_acc_
    jmp rev_i_board_acc_

rev_board_tree_acc_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_RoseTree_Pair_List_Option_Player_i64_44876]
    add rcx, r9
    jmp rcx

List_RoseTree_Pair_List_Option_Player_i64_44876:
    jmp near List_RoseTree_Pair_List_Option_Player_i64_44876_Nil
    jmp near List_RoseTree_Pair_List_Option_Player_i64_44876_Cons

List_RoseTree_Pair_List_Option_Player_i64_44876_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_RoseTree_Pair_List_Option_Player_i64_44877]
    add rcx, rdi
    jmp rcx

List_RoseTree_Pair_List_Option_Player_i64_44877:
    jmp near List_RoseTree_Pair_List_Option_Player_i64_44877_Nil
    jmp near List_RoseTree_Pair_List_Option_Player_i64_44877_Cons

List_RoseTree_Pair_List_Option_Player_i64_44877_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_RoseTree_Pair_List_Option_Player_i64_44877_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44880
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab44878
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44878:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab44879
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44879:
    jmp lab44881

lab44880:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab44881:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

List_RoseTree_Pair_List_Option_Player_i64_44876_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44884
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab44882
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44882:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44883
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44883:
    jmp lab44885

lab44884:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab44885:
    ; substitute (a0 !-> a0)(xs !-> xs)(x !-> x)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[RoseTree[Pair[List[Option[Player]], i64]]] = Cons(x, acc);
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
    je lab44897
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44898

lab44897:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44895
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44888
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44886
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44887

lab44886:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44887:

lab44888:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44891
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44889
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44890

lab44889:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44890:

lab44891:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44894
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44892
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44893

lab44892:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44893:

lab44894:
    jmp lab44896

lab44895:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44896:

lab44898:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_board_tree_acc_
    jmp rev_board_tree_acc_

rev_board_tree_:
    ; let x0: List[RoseTree[Pair[List[Option[Player]], i64]]] = Nil();
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
    ; jump rev_board_tree_acc_
    jmp rev_board_tree_acc_

map_i_board_acc_:
    ; substitute (a0 !-> a0)(f !-> f)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_44899]
    add rcx, r11
    jmp rcx

List_i64_44899:
    jmp near List_i64_44899_Nil
    jmp near List_i64_44899_Cons

List_i64_44899_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab44902
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44900
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44901

lab44900:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44901:

lab44902:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; jump rev_i_board_
    jmp rev_i_board_

List_i64_44899_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab44904
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab44903
    ; ####increment refcount
    add qword [r12 + 0], 1

lab44903:
    mov r11, [r10 + 40]
    jmp lab44905

lab44904:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab44905:
    ; substitute (x !-> x)(f0 !-> f)(acc !-> acc)(f !-> f)(xs !-> xs)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab44906
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44906:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a1: List[Option[Player]] = (acc, f, xs, a0)\{ ... \};
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
    je lab44918
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44919

lab44918:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44916
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44909
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44907
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44908

lab44907:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44908:

lab44909:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44912
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44910
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44911

lab44910:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44911:

lab44912:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44915
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44913
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44914

lab44913:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44914:

lab44915:
    jmp lab44917

lab44916:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44917:

lab44919:
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
    je lab44931
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab44932

lab44931:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44929
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44921

lab44920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44921:

lab44922:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44925
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44923
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44924

lab44923:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44924:

lab44925:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44928
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44926
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44927

lab44926:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44927:

lab44928:
    jmp lab44930

lab44929:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44930:

lab44932:
    ; #load tag
    lea r9, [rel List_Option_Player_44933]
    ; substitute (x !-> x)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

List_Option_Player_44933:
    jmp near List_Option_Player_44933_Nil
    jmp near List_Option_Player_44933_Cons

List_Option_Player_44933_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44938
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab44934
    ; ####increment refcount
    add qword [rax + 0], 1

lab44934:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab44935
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44935:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44936
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44936:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab44937
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44937:
    jmp lab44939

lab44938:
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

lab44939:
    ; let x1: List[Option[Player]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_map_i_board_acc_0_
    jmp lift_map_i_board_acc_0_

List_Option_Player_44933_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab44944
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab44940
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44940:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab44941
    ; ####increment refcount
    add qword [r14 + 0], 1

lab44941:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab44942
    ; ####increment refcount
    add qword [r12 + 0], 1

lab44942:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab44943
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44943:
    jmp lab44945

lab44944:
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

lab44945:
    ; substitute (a0 !-> a0)(xs !-> xs)(acc !-> acc)(f !-> f)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: List[Option[Player]] = Cons(a2, as0);
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
    je lab44957
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab44958

lab44957:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44955
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44948
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44946
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44947

lab44946:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44947:

lab44948:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44951
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44949
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44950

lab44949:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44950:

lab44951:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44954
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44952
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44953

lab44952:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44953:

lab44954:
    jmp lab44956

lab44955:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44956:

lab44958:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(acc !-> acc)(f !-> f)(x1 !-> x1)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    ; jump lift_map_i_board_acc_0_
    jmp lift_map_i_board_acc_0_

lift_map_i_board_acc_0_:
    ; substitute (a0 !-> a0)(xs !-> xs)(f !-> f)(x1 !-> x1)(acc !-> acc);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; let x0: List[List[Option[Player]]] = Cons(x1, acc);
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
    je lab44970
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44971

lab44970:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44968
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44961
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44959
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44960

lab44959:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44960:

lab44961:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44964
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44962
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44963

lab44962:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44963:

lab44964:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44967
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44965
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44966

lab44965:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44966:

lab44967:
    jmp lab44969

lab44968:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44969:

lab44971:
    ; #load tag
    mov r11, 5
    ; substitute (xs !-> xs)(f !-> f)(x0 !-> x0)(a0 !-> a0);
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
    ; jump map_i_board_acc_
    jmp map_i_board_acc_

map_i_board_:
    ; let x0: List[List[Option[Player]]] = Nil();
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
    ; jump map_i_board_acc_
    jmp map_i_board_acc_

map_board_tree_acc_:
    ; substitute (a0 !-> a0)(f !-> f)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_Option_Player_44972]
    add rcx, r11
    jmp rcx

List_List_Option_Player_44972:
    jmp near List_List_Option_Player_44972_Nil
    jmp near List_List_Option_Player_44972_Cons

List_List_Option_Player_44972_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab44975
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab44973
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab44974

lab44973:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab44974:

lab44975:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; jump rev_board_tree_
    jmp rev_board_tree_

List_List_Option_Player_44972_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab44978
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab44976
    ; ####increment refcount
    add qword [r12 + 0], 1

lab44976:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab44977
    ; ####increment refcount
    add qword [r10 + 0], 1

lab44977:
    jmp lab44979

lab44978:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab44979:
    ; substitute (x !-> x)(f0 !-> f)(acc !-> acc)(f !-> f)(xs !-> xs)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab44980
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44980:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a1: RoseTree[Pair[List[Option[Player]], i64]] = (acc, f, xs, a0)\{ ... \};
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
    je lab44992
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab44993

lab44992:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44990
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44983
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44981
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44982

lab44981:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44982:

lab44983:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44986
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44984
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44985

lab44984:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44985:

lab44986:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44989
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44987
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44988

lab44987:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44988:

lab44989:
    jmp lab44991

lab44990:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44991:

lab44993:
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
    je lab45005
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45006

lab45005:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45003
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44996
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44994
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44995

lab44994:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44995:

lab44996:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44999
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44997
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44998

lab44997:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44998:

lab44999:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45002
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45000
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45001

lab45000:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45001:

lab45002:
    jmp lab45004

lab45003:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45004:

lab45006:
    ; #load tag
    lea r9, [rel RoseTree_Pair_List_Option_Player_i64_45007]
    ; substitute (x !-> x)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

RoseTree_Pair_List_Option_Player_i64_45007:

RoseTree_Pair_List_Option_Player_i64_45007_Rose:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45012
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45008
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45008:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab45009
    ; ####increment refcount
    add qword [r14 + 0], 1

lab45009:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab45010
    ; ####increment refcount
    add qword [r12 + 0], 1

lab45010:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab45011
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45011:
    jmp lab45013

lab45012:
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

lab45013:
    ; substitute (a0 !-> a0)(xs !-> xs)(acc !-> acc)(f !-> f)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, r14
    mov r14, rsi
    mov rsi, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: RoseTree[Pair[List[Option[Player]], i64]] = Rose(a2, as0);
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
    je lab45025
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45026

lab45025:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45023
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45015

lab45014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45015:

lab45016:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45019
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45017
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45018

lab45017:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45018:

lab45019:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45022
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45020
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45021

lab45020:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45021:

lab45022:
    jmp lab45024

lab45023:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45024:

lab45026:
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(xs !-> xs)(f !-> f)(x1 !-> x1)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rcx
    ; let x0: List[RoseTree[Pair[List[Option[Player]], i64]]] = Cons(x1, acc);
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
    je lab45038
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45039

lab45038:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45036
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45029
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45027
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45028

lab45027:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45028:

lab45029:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45032
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45030
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45031

lab45030:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45031:

lab45032:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45034

lab45033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45034:

lab45035:
    jmp lab45037

lab45036:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45037:

lab45039:
    ; #load tag
    mov r11, 5
    ; substitute (xs !-> xs)(f !-> f)(x0 !-> x0)(a0 !-> a0);
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
    ; jump map_board_tree_acc_
    jmp map_board_tree_acc_

map_board_tree_:
    ; let x0: List[RoseTree[Pair[List[Option[Player]], i64]]] = Nil();
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
    ; jump map_board_tree_acc_
    jmp map_board_tree_acc_

map_tree_i_acc_:
    ; substitute (a0 !-> a0)(f !-> f)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_RoseTree_Pair_List_Option_Player_i64_45040]
    add rcx, r11
    jmp rcx

List_RoseTree_Pair_List_Option_Player_i64_45040:
    jmp near List_RoseTree_Pair_List_Option_Player_i64_45040_Nil
    jmp near List_RoseTree_Pair_List_Option_Player_i64_45040_Cons

List_RoseTree_Pair_List_Option_Player_i64_45040_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab45043
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab45041
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab45042

lab45041:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab45042:

lab45043:
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
    ; jump rev_
    jmp rev_

List_RoseTree_Pair_List_Option_Player_i64_45040_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab45046
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab45044
    ; ####increment refcount
    add qword [r12 + 0], 1

lab45044:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab45045
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45045:
    jmp lab45047

lab45046:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab45047:
    ; substitute (x !-> x)(f0 !-> f)(acc !-> acc)(f !-> f)(xs !-> xs)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab45048
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45048:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r10
    mov r10, rsi
    mov rdx, r11
    mov r11, rdi
    ; new a1: _Cont = (acc, f, xs, a0)\{ ... \};
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
    je lab45060
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45061

lab45060:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45058
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45051
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45049
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45050

lab45049:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45050:

lab45051:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45054
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45052
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45053

lab45052:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45053:

lab45054:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45057
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45055
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45056

lab45055:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45056:

lab45057:
    jmp lab45059

lab45058:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45059:

lab45061:
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
    je lab45073
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45074

lab45073:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45071
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45064
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45062
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45063

lab45062:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45063:

lab45064:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45067
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45065
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45066

lab45065:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45066:

lab45067:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45070
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45068
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45069

lab45068:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45069:

lab45070:
    jmp lab45072

lab45071:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45072:

lab45074:
    ; #load tag
    lea r9, [rel _Cont_45075]
    ; substitute (x !-> x)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_45075:

_Cont_45075_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45080
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab45076
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45076:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab45077
    ; ####increment refcount
    add qword [r12 + 0], 1

lab45077:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab45078
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45078:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab45079
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45079:
    jmp lab45081

lab45080:
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
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab45081:
    ; substitute (a0 !-> a0)(xs !-> xs)(f !-> f)(x1 !-> x1)(acc !-> acc);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x0: List[i64] = Cons(x1, acc);
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
    je lab45093
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45094

lab45093:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45091
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45083

lab45082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45083:

lab45084:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45086

lab45085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45086:

lab45087:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45089

lab45088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45089:

lab45090:
    jmp lab45092

lab45091:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45092:

lab45094:
    ; #load tag
    mov r11, 5
    ; substitute (xs !-> xs)(f !-> f)(x0 !-> x0)(a0 !-> a0);
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
    ; jump map_tree_i_acc_
    jmp map_tree_i_acc_

map_tree_i_:
    ; let x0: List[i64] = Nil();
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
    ; jump map_tree_i_acc_
    jmp map_tree_i_acc_

tabulate_loop_:
    ; if n == len \{ ... \}
    cmp rdx, rdi
    je lab45095
    ; substitute (f0 !-> f)(len !-> len)(f !-> f)(a0 !-> a0)(n !-> n);
    ; #share f
    cmp r8, 0
    je lab45096
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45096:
    ; #move variables
    mov r13, rdx
    mov rax, r8
    mov rdx, r9
    ; new a1: Option[Player] = (len, f, a0, n)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab45108
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45109

lab45108:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45106
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45099
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45097
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45098

lab45097:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45098:

lab45099:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45102
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45100
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45101

lab45100:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45101:

lab45102:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45105
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45103
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45104

lab45103:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45104:

lab45105:
    jmp lab45107

lab45106:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45107:

lab45109:
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
    je lab45121
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45122

lab45121:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45119
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45112
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45110
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45111

lab45110:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45111:

lab45112:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45115
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45113
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45114

lab45113:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45114:

lab45115:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45118
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45116
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45117

lab45116:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45117:

lab45118:
    jmp lab45120

lab45119:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45120:

lab45122:
    ; #load tag
    lea rdi, [rel Option_Player_45123]
    ; let x1: Unit = Unit();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Apply
    jmp r9

Option_Player_45123:
    jmp near Option_Player_45123_None
    jmp near Option_Player_45123_Some

Option_Player_45123_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45126
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab45124
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45124:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab45125
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45125:
    jmp lab45127

lab45126:
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
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab45127:
    ; let x0: Option[Player] = None();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(f !-> f)(len !-> len)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_tabulate_loop_0_
    jmp lift_tabulate_loop_0_

Option_Player_45123_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45130
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab45128
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45128:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab45129
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45129:
    jmp lab45131

lab45130:
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
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab45131:
    ; substitute (n !-> n)(len !-> len)(f !-> f)(a0 !-> a0)(a5 !-> a5);
    ; #move variables
    mov r12, rax
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x0: Option[Player] = Some(a5);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab45143
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45144

lab45143:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45141
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45134
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45132
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45133

lab45132:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45133:

lab45134:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45137
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45135
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45136

lab45135:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45136:

lab45137:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45140
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45138
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45139

lab45138:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45139:

lab45140:
    jmp lab45142

lab45141:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45142:

lab45144:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(f !-> f)(len !-> len)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    mov rax, r10
    ; jump lift_tabulate_loop_0_
    jmp lift_tabulate_loop_0_

lab45095:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp r8, 0
    je lab45147
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab45145
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab45146

lab45145:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab45146:

lab45147:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lift_tabulate_loop_0_:
    ; substitute (n !-> n)(f !-> f)(len !-> len)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a2: List[Option[Player]] = (a0, x0)\{ ... \};
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
    je lab45159
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45160

lab45159:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45157
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45150
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45148
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45149

lab45148:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45149:

lab45150:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45153
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45151
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45152

lab45151:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45152:

lab45153:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45156
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45154
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45155

lab45154:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45155:

lab45156:
    jmp lab45158

lab45157:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45158:

lab45160:
    ; #load tag
    lea r11, [rel List_Option_Player_45161]
    ; lit x3 <- 1;
    mov r13, 1
    ; x4 <- n + x3;
    mov r15, rdx
    add r15, r13
    ; substitute (x4 !-> x4)(len !-> len)(f !-> f)(a2 !-> a2);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r15
    ; jump tabulate_loop_
    jmp tabulate_loop_

List_Option_Player_45161:
    jmp near List_Option_Player_45161_Nil
    jmp near List_Option_Player_45161_Cons

List_Option_Player_45161_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45164
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45162
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45162:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45163
    ; ####increment refcount
    add qword [rax + 0], 1

lab45163:
    jmp lab45165

lab45164:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45165:
    ; let x2: List[Option[Player]] = Nil();
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

List_Option_Player_45161_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45168
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45166
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45166:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45167
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45167:
    jmp lab45169

lab45168:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab45169:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a4 !-> a4)(as0 !-> as0);
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
    ; let x2: List[Option[Player]] = Cons(a4, as0);
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
    je lab45181
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45182

lab45181:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45179
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45172
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45170
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45171

lab45170:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45171:

lab45172:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45175
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45173
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45174

lab45173:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45174:

lab45175:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45178
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45176
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45177

lab45176:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45177:

lab45178:
    jmp lab45180

lab45179:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45180:

lab45182:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
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

tabulate_:
    ; lit x0 <- 0;
    mov r11, 0
    ; if len < x0 \{ ... \}
    cmp rdx, r11
    jl lab45183
    ; substitute (len !-> len)(f !-> f)(a0 !-> a0);
    ; lit x1 <- 0;
    mov r11, 0
    ; substitute (x1 !-> x1)(len !-> len)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, r8
    mov r8, rsi
    ; jump tabulate_loop_
    jmp tabulate_loop_

lab45183:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab45186
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab45184
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab45185

lab45184:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab45185:

lab45186:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

push_:
    ; substitute (a0 !-> a0)(i !-> i)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_45187]
    add rcx, r9
    jmp rcx

List_i64_45187:
    jmp near List_i64_45187_Nil
    jmp near List_i64_45187_Cons

List_i64_45187_Nil:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (i !-> i)(x0 !-> x0)(a0 !-> a0);
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

List_i64_45187_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45189
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45188
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45188:
    mov r9, [r8 + 40]
    jmp lab45190

lab45189:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab45190:
    ; substitute (is !-> is)(i !-> i)(i1 !-> i1)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a1: List[i64] = (i1, a0)\{ ... \};
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
    je lab45202
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45203

lab45202:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45200
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45193
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45191
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45192

lab45191:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45192:

lab45193:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45196
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45194
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45195

lab45194:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45195:

lab45196:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45199
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45197
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45198

lab45197:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45198:

lab45199:
    jmp lab45201

lab45200:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45201:

lab45203:
    ; #load tag
    lea r9, [rel List_i64_45204]
    ; jump push_
    jmp push_

List_i64_45204:
    jmp near List_i64_45204_Nil
    jmp near List_i64_45204_Cons

List_i64_45204_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45206
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45205
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45205:
    mov rdx, [rax + 40]
    jmp lab45207

lab45206:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab45207:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (i1 !-> i1)(x1 !-> x1)(a0 !-> a0);
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

List_i64_45204_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45209
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45208
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45208:
    mov r9, [r8 + 40]
    jmp lab45210

lab45209:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab45210:
    ; substitute (a0 !-> a0)(i1 !-> i1)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let x1: List[i64] = Cons(a2, as0);
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
    je lab45222
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45223

lab45222:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45220
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45213
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45211
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45212

lab45211:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45212:

lab45213:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45216
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45214
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45215

lab45214:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45215:

lab45216:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45219
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45217
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45218

lab45217:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45218:

lab45219:
    jmp lab45221

lab45220:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45221:

lab45223:
    ; #load tag
    mov r9, 5
    ; substitute (i1 !-> i1)(x1 !-> x1)(a0 !-> a0);
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

nth_:
    ; substitute (a0 !-> a0)(i !-> i)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Option_Player_45224]
    add rcx, r9
    jmp rcx

List_Option_Player_45224:
    jmp near List_Option_Player_45224_Nil
    jmp near List_Option_Player_45224_Cons

List_Option_Player_45224_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 None
    add rdx, 0
    jmp rdx

List_Option_Player_45224_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45227
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45225
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45225:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45226
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45226:
    jmp lab45228

lab45227:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab45228:
    ; if i == 0 \{ ... \}
    cmp rdi, 0
    je lab45229
    ; substitute (a0 !-> a0)(i !-> i)(ps !-> ps);
    ; #erase p
    cmp r8, 0
    je lab45232
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab45230
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab45231

lab45230:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab45231:

lab45232:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- i - x0;
    mov r13, rdi
    sub r13, r11
    ; substitute (ps !-> ps)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump nth_
    jmp nth_

lab45229:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #erase ps
    cmp r10, 0
    je lab45235
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab45233
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab45234

lab45233:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab45234:

lab45235:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch p \{ ... \};
    lea rcx, [rel Option_Player_45236]
    add rcx, rdi
    jmp rcx

Option_Player_45236:
    jmp near Option_Player_45236_None
    jmp near Option_Player_45236_Some

Option_Player_45236_None:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

Option_Player_45236_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45238
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab45237
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45237:
    jmp lab45239

lab45238:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab45239:
    ; substitute (a2 !-> a2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

find_:
    ; substitute (a0 !-> a0)(i !-> i)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Option_Player_45240]
    add rcx, r9
    jmp rcx

List_Option_Player_45240:
    jmp near List_Option_Player_45240_Nil
    jmp near List_Option_Player_45240_Cons

List_Option_Player_45240_Nil:
    ; substitute (a0 !-> a0);
    ; invoke a0 None
    add rdx, 0
    jmp rdx

List_Option_Player_45240_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45243
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45241
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45241:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45242
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45242:
    jmp lab45244

lab45243:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab45244:
    ; if i == 0 \{ ... \}
    cmp rdi, 0
    je lab45245
    ; substitute (a0 !-> a0)(i !-> i)(ps !-> ps);
    ; #erase p
    cmp r8, 0
    je lab45248
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab45246
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab45247

lab45246:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab45247:

lab45248:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- i - x0;
    mov r13, rdi
    sub r13, r11
    ; substitute (ps !-> ps)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump find_
    jmp find_

lab45245:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #erase ps
    cmp r10, 0
    je lab45251
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab45249
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab45250

lab45249:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab45250:

lab45251:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch p \{ ... \};
    lea rcx, [rel Option_Player_45252]
    add rcx, rdi
    jmp rcx

Option_Player_45252:
    jmp near Option_Player_45252_None
    jmp near Option_Player_45252_Some

Option_Player_45252_None:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

Option_Player_45252_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45254
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab45253
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45253:
    jmp lab45255

lab45254:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab45255:
    ; substitute (a2 !-> a2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

exists_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_45256]
    add rcx, r9
    jmp rcx

List_List_i64_45256:
    jmp near List_List_i64_45256_Nil
    jmp near List_List_i64_45256_Cons

List_List_i64_45256_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab45259
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab45257
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab45258

lab45257:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab45258:

lab45259:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_List_i64_45256_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45262
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45260
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45260:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45261
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45261:
    jmp lab45263

lab45262:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab45263:
    ; substitute (f0 !-> f)(is !-> is)(a0 !-> a0)(iss !-> iss)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab45264
    ; ####increment refcount
    add qword [rax + 0], 1

lab45264:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: Bool = (a0, iss, f)\{ ... \};
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
    je lab45276
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45277

lab45276:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45274
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45267
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45265
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45266

lab45265:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45266:

lab45267:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45270
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45268
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45269

lab45268:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45269:

lab45270:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45273
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45271
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45272

lab45271:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45272:

lab45273:
    jmp lab45275

lab45274:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45275:

lab45277:
    ; #load tag
    lea r9, [rel Bool_45278]
    ; substitute (is !-> is)(a1 !-> a1)(f0 !-> f0);
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

Bool_45278:
    jmp near Bool_45278_True
    jmp near Bool_45278_False

Bool_45278_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45282
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45279
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45279:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45280
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45280:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45281
    ; ####increment refcount
    add qword [rax + 0], 1

lab45281:
    jmp lab45283

lab45282:
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

lab45283:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp r8, 0
    je lab45286
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab45284
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab45285

lab45284:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab45285:

lab45286:
    ; #erase iss
    cmp rsi, 0
    je lab45289
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab45287
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab45288

lab45287:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab45288:

lab45289:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_45278_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45293
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45290
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45290:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45291
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45291:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45292
    ; ####increment refcount
    add qword [rax + 0], 1

lab45292:
    jmp lab45294

lab45293:
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

lab45294:
    ; substitute (f !-> f)(iss !-> iss)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump exists_
    jmp exists_

all_i_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_45295]
    add rcx, r9
    jmp rcx

List_i64_45295:
    jmp near List_i64_45295_Nil
    jmp near List_i64_45295_Cons

List_i64_45295_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab45298
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab45296
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab45297

lab45296:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab45297:

lab45298:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_i64_45295_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45300
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45299
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45299:
    mov r9, [r8 + 40]
    jmp lab45301

lab45300:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab45301:
    ; substitute (f0 !-> f)(i !-> i)(a0 !-> a0)(is !-> is)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab45302
    ; ####increment refcount
    add qword [rax + 0], 1

lab45302:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: Bool = (a0, is, f)\{ ... \};
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
    je lab45314
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45315

lab45314:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45312
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45305
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45303
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45304

lab45303:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45304:

lab45305:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45308
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45306
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45307

lab45306:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45307:

lab45308:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45311
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45309
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45310

lab45309:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45310:

lab45311:
    jmp lab45313

lab45312:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45313:

lab45315:
    ; #load tag
    lea r9, [rel Bool_45316]
    ; substitute (i !-> i)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Apply
    jmp r9

Bool_45316:
    jmp near Bool_45316_True
    jmp near Bool_45316_False

Bool_45316_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45320
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45317
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45317:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45318
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45318:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45319
    ; ####increment refcount
    add qword [rax + 0], 1

lab45319:
    jmp lab45321

lab45320:
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

lab45321:
    ; let x0: Bool = True();
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
    ; jump lift_all_i_0_
    jmp lift_all_i_0_

Bool_45316_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45325
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45322
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45322:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45323
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45323:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45324
    ; ####increment refcount
    add qword [rax + 0], 1

lab45324:
    jmp lab45326

lab45325:
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

lab45326:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(f !-> f)(is !-> is)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_all_i_0_
    jmp lift_all_i_0_

lift_all_i_0_:
    ; substitute (is !-> is)(f !-> f)(a0 !-> a0)(x0 !-> x0);
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
    je lab45338
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45339

lab45338:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45336
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45328

lab45327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45328:

lab45329:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45332
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45330
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45331

lab45330:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45331:

lab45332:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45335
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45333
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45334

lab45333:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45334:

lab45335:
    jmp lab45337

lab45336:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45337:

lab45339:
    ; #load tag
    lea r9, [rel Bool_45340]
    ; substitute (f !-> f)(is !-> is)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump all_i_
    jmp all_i_

Bool_45340:
    jmp near Bool_45340_True
    jmp near Bool_45340_False

Bool_45340_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45343
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45341
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45341:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45342
    ; ####increment refcount
    add qword [rax + 0], 1

lab45342:
    jmp lab45344

lab45343:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45344:
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
    ; jump and_
    jmp and_

Bool_45340_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45347
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45345
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45345:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45346
    ; ####increment refcount
    add qword [rax + 0], 1

lab45346:
    jmp lab45348

lab45347:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45348:
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
    ; jump and_
    jmp and_

empty_:
    ; lit x0 <- 9;
    mov rdi, 9
    ; new x1: Fun[Unit, Option[Player]] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Unit_Option_Player_45349]
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump tabulate_
    jmp tabulate_

Fun_Unit_Option_Player_45349:

Fun_Unit_Option_Player_45349_Apply:
    ; substitute (a1 !-> a1);
    ; #erase u
    cmp rax, 0
    je lab45352
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab45350
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab45351

lab45350:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab45351:

lab45352:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a1 None
    add rdx, 0
    jmp rdx

all_board_:
    ; substitute (a0 !-> a0)(f !-> f)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Option_Player_45353]
    add rcx, r9
    jmp rcx

List_Option_Player_45353:
    jmp near List_Option_Player_45353_Nil
    jmp near List_Option_Player_45353_Cons

List_Option_Player_45353_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab45356
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab45354
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab45355

lab45354:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab45355:

lab45356:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_Option_Player_45353_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45359
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45357
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45357:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab45358
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45358:
    jmp lab45360

lab45359:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab45360:
    ; substitute (p !-> p)(f0 !-> f)(f !-> f)(ps !-> ps)(a0 !-> a0);
    ; #share f
    cmp rsi, 0
    je lab45361
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45361:
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
    je lab45373
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45374

lab45373:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45371
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45364
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45362
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45363

lab45362:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45363:

lab45364:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45367
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45365
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45366

lab45365:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45366:

lab45367:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45370
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45368
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45369

lab45368:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45369:

lab45370:
    jmp lab45372

lab45371:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45372:

lab45374:
    ; #load tag
    lea r9, [rel Bool_45375]
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

Bool_45375:
    jmp near Bool_45375_True
    jmp near Bool_45375_False

Bool_45375_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45379
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45376
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45376:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45377
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45377:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45378
    ; ####increment refcount
    add qword [rax + 0], 1

lab45378:
    jmp lab45380

lab45379:
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

lab45380:
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
    ; jump lift_all_board_0_
    jmp lift_all_board_0_

Bool_45375_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45384
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab45381
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45381:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab45382
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45382:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab45383
    ; ####increment refcount
    add qword [rax + 0], 1

lab45383:
    jmp lab45385

lab45384:
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

lab45385:
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
    ; jump lift_all_board_0_
    jmp lift_all_board_0_

lift_all_board_0_:
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
    je lab45397
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45398

lab45397:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45395
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45388
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45386
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45387

lab45386:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45387:

lab45388:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45391
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45389
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45390

lab45389:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45390:

lab45391:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45394
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45392
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45393

lab45392:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45393:

lab45394:
    jmp lab45396

lab45395:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45396:

lab45398:
    ; #load tag
    lea r9, [rel Bool_45399]
    ; jump all_board_
    jmp all_board_

Bool_45399:
    jmp near Bool_45399_True
    jmp near Bool_45399_False

Bool_45399_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45402
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45400
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45400:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45401
    ; ####increment refcount
    add qword [rax + 0], 1

lab45401:
    jmp lab45403

lab45402:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45403:
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
    ; jump and_
    jmp and_

Bool_45399_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45406
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45404
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45404:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45405
    ; ####increment refcount
    add qword [rax + 0], 1

lab45405:
    jmp lab45407

lab45406:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45407:
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
    ; jump and_
    jmp and_

is_full_:
    ; new x0: Fun[Option[Player], Bool] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Option_Player_Bool_45408]
    ; substitute (board !-> board)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump all_board_
    jmp all_board_

Fun_Option_Player_Bool_45408:

Fun_Option_Player_Bool_45408_Apply:
    ; jump is_some_
    jmp is_some_

is_cat_:
    ; substitute (board0 !-> board)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab45409
    ; ####increment refcount
    add qword [rax + 0], 1

lab45409:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a1: Bool = (a0, board)\{ ... \};
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
    je lab45421
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45422

lab45421:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45419
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45412
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45410
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45411

lab45410:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45411:

lab45412:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45415
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45413
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45414

lab45413:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45414:

lab45415:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45418
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45416
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45417

lab45416:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45417:

lab45418:
    jmp lab45420

lab45419:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45420:

lab45422:
    ; #load tag
    lea rdi, [rel Bool_45423]
    ; jump is_full_
    jmp is_full_

Bool_45423:
    jmp near Bool_45423_True
    jmp near Bool_45423_False

Bool_45423_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45426
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45424
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45424:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45425
    ; ####increment refcount
    add qword [rax + 0], 1

lab45425:
    jmp lab45427

lab45426:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45427:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_is_cat_0_
    jmp lift_is_cat_0_

Bool_45423_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45430
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45428
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45428:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45429
    ; ####increment refcount
    add qword [rax + 0], 1

lab45429:
    jmp lab45431

lab45430:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45431:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; jump lift_is_cat_0_
    jmp lift_is_cat_0_

lift_is_cat_0_:
    ; substitute (board !-> board)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0, x0)\{ ... \};
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
    je lab45443
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45444

lab45443:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45441
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45434
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45432
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45433

lab45432:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45433:

lab45434:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45437
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45435
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45436

lab45435:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45436:

lab45437:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45440
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45438
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45439

lab45438:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45439:

lab45440:
    jmp lab45442

lab45441:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45442:

lab45444:
    ; #load tag
    lea rdi, [rel Bool_45445]
    ; substitute (board0 !-> board)(a2 !-> a2)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab45446
    ; ####increment refcount
    add qword [rax + 0], 1

lab45446:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a3: Bool = (a2, board)\{ ... \};
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
    je lab45458
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45459

lab45458:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45456
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45449
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45447
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45448

lab45447:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45448:

lab45449:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45452
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45450
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45451

lab45450:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45451:

lab45452:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45455
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45453
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45454

lab45453:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45454:

lab45455:
    jmp lab45457

lab45456:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45457:

lab45459:
    ; #load tag
    lea rdi, [rel Bool_45460]
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
    je lab45472
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45473

lab45472:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45470
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45463
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45461
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45462

lab45461:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45462:

lab45463:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45466
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45464
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45465

lab45464:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45465:

lab45466:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45469
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45467
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45468

lab45467:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45468:

lab45469:
    jmp lab45471

lab45470:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45471:

lab45473:
    ; #load tag
    lea rdi, [rel Bool_45474]
    ; let x4: Player = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (board0 !-> board0)(x4 !-> x4)(a4 !-> a4);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump is_win_for_
    jmp is_win_for_

Bool_45474:
    jmp near Bool_45474_True
    jmp near Bool_45474_False

Bool_45474_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45476
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab45475
    ; ####increment refcount
    add qword [rax + 0], 1

lab45475:
    jmp lab45477

lab45476:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab45477:
    ; let x3: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x3 !-> x3)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_45474_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45479
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab45478
    ; ####increment refcount
    add qword [rax + 0], 1

lab45478:
    jmp lab45480

lab45479:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab45480:
    ; let x3: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x3 !-> x3)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_45460:
    jmp near Bool_45460_True
    jmp near Bool_45460_False

Bool_45460_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45483
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45481
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45481:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45482
    ; ####increment refcount
    add qword [rax + 0], 1

lab45482:
    jmp lab45484

lab45483:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45484:
    ; let x2: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_is_cat_1_
    jmp lift_is_cat_1_

Bool_45460_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45487
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45485
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45485:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45486
    ; ####increment refcount
    add qword [rax + 0], 1

lab45486:
    jmp lab45488

lab45487:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45488:
    ; let x2: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; jump lift_is_cat_1_
    jmp lift_is_cat_1_

Bool_45445:
    jmp near Bool_45445_True
    jmp near Bool_45445_False

Bool_45445_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45491
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45489
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45489:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45490
    ; ####increment refcount
    add qword [rax + 0], 1

lab45490:
    jmp lab45492

lab45491:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45492:
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
    ; jump and_
    jmp and_

Bool_45445_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45495
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45493
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45493:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45494
    ; ####increment refcount
    add qword [rax + 0], 1

lab45494:
    jmp lab45496

lab45495:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45496:
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
    ; jump and_
    jmp and_

lift_is_cat_1_:
    ; substitute (board !-> board)(a2 !-> a2)(x2 !-> x2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a5: Bool = (a2, x2)\{ ... \};
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
    je lab45508
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45509

lab45508:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45506
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45499
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45497
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45498

lab45497:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45498:

lab45499:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45502
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45500
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45501

lab45500:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45501:

lab45502:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45505
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45503
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45504

lab45503:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45504:

lab45505:
    jmp lab45507

lab45506:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45507:

lab45509:
    ; #load tag
    lea rdi, [rel Bool_45510]
    ; new a6: Bool = (a5)\{ ... \};
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
    je lab45522
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45523

lab45522:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45520
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45513
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45511
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45512

lab45511:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45512:

lab45513:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45516
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45514
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45515

lab45514:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45515:

lab45516:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45519
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45518

lab45517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45518:

lab45519:
    jmp lab45521

lab45520:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45521:

lab45523:
    ; #load tag
    lea rdi, [rel Bool_45524]
    ; let x7: Player = O();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (board !-> board)(x7 !-> x7)(a6 !-> a6);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump is_win_for_
    jmp is_win_for_

Bool_45524:
    jmp near Bool_45524_True
    jmp near Bool_45524_False

Bool_45524_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45526
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab45525
    ; ####increment refcount
    add qword [rax + 0], 1

lab45525:
    jmp lab45527

lab45526:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab45527:
    ; let x6: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x6 !-> x6)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_45524_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45529
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab45528
    ; ####increment refcount
    add qword [rax + 0], 1

lab45528:
    jmp lab45530

lab45529:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab45530:
    ; let x6: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x6 !-> x6)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump not_
    jmp not_

Bool_45510:
    jmp near Bool_45510_True
    jmp near Bool_45510_False

Bool_45510_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45533
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45531
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45531:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45532
    ; ####increment refcount
    add qword [rax + 0], 1

lab45532:
    jmp lab45534

lab45533:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45534:
    ; let x5: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x5 !-> x5)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

Bool_45510_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45537
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab45535
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45535:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab45536
    ; ####increment refcount
    add qword [rax + 0], 1

lab45536:
    jmp lab45538

lab45537:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab45538:
    ; let x5: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x5 !-> x5)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

fold_i_:
    ; substitute (f !-> f)(start !-> start)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_45539]
    add rcx, r11
    jmp rcx

List_i64_45539:
    jmp near List_i64_45539_Nil
    jmp near List_i64_45539_Cons

List_i64_45539_Nil:
    ; substitute (start !-> start)(a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab45542
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab45540
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab45541

lab45540:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab45541:

lab45542:
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

List_i64_45539_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab45544
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab45543
    ; ####increment refcount
    add qword [r12 + 0], 1

lab45543:
    mov r11, [r10 + 40]
    jmp lab45545

lab45544:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab45545:
    ; substitute (f0 !-> f)(start !-> start)(i !-> i)(a0 !-> a0)(is !-> is)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab45546
    ; ####increment refcount
    add qword [rax + 0], 1

lab45546:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov r10, r8
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; new a1: _Cont = (a0, is, f)\{ ... \};
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
    je lab45558
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45559

lab45558:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45556
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45549
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45547
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45548

lab45547:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45548:

lab45549:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45552
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45550
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45551

lab45550:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45551:

lab45552:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45555
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45553
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45554

lab45553:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45554:

lab45555:
    jmp lab45557

lab45556:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45557:

lab45559:
    ; #load tag
    lea r11, [rel _Cont_45560]
    ; substitute (start !-> start)(i !-> i)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov r8, r10
    mov r10, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; invoke f0 Apply2
    jmp r11

_Cont_45560:

_Cont_45560_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45564
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab45561
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45561:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab45562
    ; ####increment refcount
    add qword [r8 + 0], 1

lab45562:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab45563
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45563:
    jmp lab45565

lab45564:
    ; ##... or release blocks onto linear free list when loading
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

lab45565:
    ; substitute (f !-> f)(x0 !-> x0)(is !-> is)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; jump fold_i_
    jmp fold_i_

list_extreme_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_45566]
    add rcx, r9
    jmp rcx

List_i64_45566:
    jmp near List_i64_45566_Nil
    jmp near List_i64_45566_Cons

List_i64_45566_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab45569
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab45567
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab45568

lab45567:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab45568:

lab45569:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x0 <- 0;
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_45566_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab45571
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab45570
    ; ####increment refcount
    add qword [r10 + 0], 1

lab45570:
    mov r9, [r8 + 40]
    jmp lab45572

lab45571:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab45572:
    ; substitute (f !-> f)(i !-> i)(is !-> is)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump fold_i_
    jmp fold_i_

listmax_:
    ; new x0: Fun2[i64, i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun2_i64_i64_i64_45573]
    ; substitute (x0 !-> x0)(l !-> l)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump list_extreme_
    jmp list_extreme_

Fun2_i64_i64_i64_45573:

Fun2_i64_i64_i64_45573_Apply2:
    ; if b < a \{ ... \}
    cmp rdi, rdx
    jl lab45574
    ; substitute (b !-> b)(a1 !-> a1);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a1 Ret
    jmp rdi

lab45574:
    ; substitute (a !-> a)(a1 !-> a1);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; invoke a1 Ret
    jmp rdi

listmin_:
    ; new x0: Fun2[i64, i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun2_i64_i64_i64_45575]
    ; substitute (x0 !-> x0)(l !-> l)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump list_extreme_
    jmp list_extreme_

Fun2_i64_i64_i64_45575:

Fun2_i64_i64_i64_45575_Apply2:
    ; if a < b \{ ... \}
    cmp rdx, rdi
    jl lab45576
    ; substitute (b !-> b)(a1 !-> a1);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a1 Ret
    jmp rdi

lab45576:
    ; substitute (a !-> a)(a1 !-> a1);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; invoke a1 Ret
    jmp rdi

rows_:
    ; lit x1 <- 0;
    mov rdi, 0
    ; lit x3 <- 1;
    mov r9, 1
    ; lit x5 <- 2;
    mov r11, 2
    ; let x6: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x4: List[i64] = Cons(x5, x6);
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
    je lab45588
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45589

lab45588:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45586
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45579
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45577
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45578

lab45577:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45578:

lab45579:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45582
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45580
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45581

lab45580:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45581:

lab45582:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45585
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45583
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45584

lab45583:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45584:

lab45585:
    jmp lab45587

lab45586:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45587:

lab45589:
    ; #load tag
    mov r11, 5
    ; let x2: List[i64] = Cons(x3, x4);
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
    je lab45601
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45602

lab45601:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45599
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45592
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45590
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45591

lab45590:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45591:

lab45592:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45595
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45593
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45594

lab45593:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45594:

lab45595:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45598
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45596
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45597

lab45596:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45597:

lab45598:
    jmp lab45600

lab45599:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45600:

lab45602:
    ; #load tag
    mov r9, 5
    ; let x0: List[i64] = Cons(x1, x2);
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
    je lab45614
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45615

lab45614:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45612
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45605
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45603
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45604

lab45603:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45604:

lab45605:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45608
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45606
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45607

lab45606:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45607:

lab45608:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45611
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45609
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45610

lab45609:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45610:

lab45611:
    jmp lab45613

lab45612:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45613:

lab45615:
    ; #load tag
    mov rdi, 5
    ; lit x9 <- 3;
    mov r9, 3
    ; lit x11 <- 4;
    mov r11, 4
    ; lit x13 <- 5;
    mov r13, 5
    ; let x14: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x12: List[i64] = Cons(x13, x14);
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
    je lab45627
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45628

lab45627:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45625
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45618
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45616
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45617

lab45616:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45617:

lab45618:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45621
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45619
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45620

lab45619:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45620:

lab45621:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45624
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45622
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45623

lab45622:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45623:

lab45624:
    jmp lab45626

lab45625:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45626:

lab45628:
    ; #load tag
    mov r13, 5
    ; let x10: List[i64] = Cons(x11, x12);
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
    je lab45640
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45641

lab45640:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45638
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45631
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45629
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45630

lab45629:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45630:

lab45631:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45634
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45632
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45633

lab45632:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45633:

lab45634:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45637
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45635
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45636

lab45635:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45636:

lab45637:
    jmp lab45639

lab45638:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45639:

lab45641:
    ; #load tag
    mov r11, 5
    ; let x8: List[i64] = Cons(x9, x10);
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
    je lab45653
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45654

lab45653:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45651
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45644
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45642
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45643

lab45642:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45643:

lab45644:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45647
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45645
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45646

lab45645:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45646:

lab45647:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45650
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45648
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45649

lab45648:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45649:

lab45650:
    jmp lab45652

lab45651:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45652:

lab45654:
    ; #load tag
    mov r9, 5
    ; lit x17 <- 6;
    mov r11, 6
    ; lit x19 <- 7;
    mov r13, 7
    ; lit x21 <- 8;
    mov r15, 8
    ; let x22: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x20: List[i64] = Cons(x21, x22);
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
    je lab45666
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab45667

lab45666:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45664
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45657
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45655
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45656

lab45655:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45656:

lab45657:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45660
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45658
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45659

lab45658:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45659:

lab45660:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45663
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45661
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45662

lab45661:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45662:

lab45663:
    jmp lab45665

lab45664:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45665:

lab45667:
    ; #load tag
    mov r15, 5
    ; let x18: List[i64] = Cons(x19, x20);
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
    je lab45679
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45680

lab45679:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45677
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45670
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45668
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45669

lab45668:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45669:

lab45670:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45673
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45671
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45672

lab45671:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45672:

lab45673:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45676
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45674
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45675

lab45674:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45675:

lab45676:
    jmp lab45678

lab45677:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45678:

lab45680:
    ; #load tag
    mov r13, 5
    ; let x16: List[i64] = Cons(x17, x18);
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
    je lab45692
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45693

lab45692:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45690
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45683
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45681
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45682

lab45681:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45682:

lab45683:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45686
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45684
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45685

lab45684:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45685:

lab45686:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45689
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45687
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45688

lab45687:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45688:

lab45689:
    jmp lab45691

lab45690:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45691:

lab45693:
    ; #load tag
    mov r11, 5
    ; let x23: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x15: List[List[i64]] = Cons(x16, x23);
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
    je lab45705
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45706

lab45705:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45703
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45695

lab45694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45695:

lab45696:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45698

lab45697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45698:

lab45699:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45701

lab45700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45701:

lab45702:
    jmp lab45704

lab45703:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45704:

lab45706:
    ; #load tag
    mov r11, 5
    ; let x7: List[List[i64]] = Cons(x8, x15);
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
    je lab45718
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45719

lab45718:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45716
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45709
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45707
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45708

lab45707:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45708:

lab45709:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45712
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45710
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45711

lab45710:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45711:

lab45712:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45715
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45713
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45714

lab45713:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45714:

lab45715:
    jmp lab45717

lab45716:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45717:

lab45719:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x7 !-> x7)(a0 !-> a0);
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

cols_:
    ; lit x1 <- 0;
    mov rdi, 0
    ; lit x3 <- 3;
    mov r9, 3
    ; lit x5 <- 6;
    mov r11, 6
    ; let x6: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x4: List[i64] = Cons(x5, x6);
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
    je lab45731
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45732

lab45731:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45729
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45722
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45720
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45721

lab45720:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45721:

lab45722:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45725
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45723
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45724

lab45723:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45724:

lab45725:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45727

lab45726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45727:

lab45728:
    jmp lab45730

lab45729:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45730:

lab45732:
    ; #load tag
    mov r11, 5
    ; let x2: List[i64] = Cons(x3, x4);
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
    je lab45744
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45745

lab45744:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45742
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45734

lab45733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45734:

lab45735:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45737

lab45736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45737:

lab45738:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45740

lab45739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45740:

lab45741:
    jmp lab45743

lab45742:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45743:

lab45745:
    ; #load tag
    mov r9, 5
    ; let x0: List[i64] = Cons(x1, x2);
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
    je lab45757
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45758

lab45757:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45755
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45748
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45746
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45747

lab45746:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45747:

lab45748:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45751
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45749
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45750

lab45749:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45750:

lab45751:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45754
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45752
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45753

lab45752:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45753:

lab45754:
    jmp lab45756

lab45755:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45756:

lab45758:
    ; #load tag
    mov rdi, 5
    ; lit x9 <- 1;
    mov r9, 1
    ; lit x11 <- 4;
    mov r11, 4
    ; lit x13 <- 7;
    mov r13, 7
    ; let x14: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x12: List[i64] = Cons(x13, x14);
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
    je lab45770
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45771

lab45770:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45768
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45760

lab45759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45760:

lab45761:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45763

lab45762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45763:

lab45764:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45767
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45765
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45766

lab45765:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45766:

lab45767:
    jmp lab45769

lab45768:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45769:

lab45771:
    ; #load tag
    mov r13, 5
    ; let x10: List[i64] = Cons(x11, x12);
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
    je lab45783
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45784

lab45783:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45781
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45774
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45772
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45773

lab45772:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45773:

lab45774:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45777
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45775
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45776

lab45775:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45776:

lab45777:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45780
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45778
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45779

lab45778:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45779:

lab45780:
    jmp lab45782

lab45781:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45782:

lab45784:
    ; #load tag
    mov r11, 5
    ; let x8: List[i64] = Cons(x9, x10);
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
    je lab45796
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45797

lab45796:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45794
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45787
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45785
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45786

lab45785:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45786:

lab45787:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45790
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45788
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45789

lab45788:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45789:

lab45790:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45793
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45791
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45792

lab45791:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45792:

lab45793:
    jmp lab45795

lab45794:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45795:

lab45797:
    ; #load tag
    mov r9, 5
    ; lit x17 <- 2;
    mov r11, 2
    ; lit x19 <- 5;
    mov r13, 5
    ; lit x21 <- 8;
    mov r15, 8
    ; let x22: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; let x20: List[i64] = Cons(x21, x22);
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
    je lab45809
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab45810

lab45809:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45807
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45800
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45798
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45799

lab45798:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45799:

lab45800:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45803
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45801
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45802

lab45801:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45802:

lab45803:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45806
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45804
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45805

lab45804:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45805:

lab45806:
    jmp lab45808

lab45807:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45808:

lab45810:
    ; #load tag
    mov r15, 5
    ; let x18: List[i64] = Cons(x19, x20);
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
    je lab45822
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45823

lab45822:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45820
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45813
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45811
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45812

lab45811:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45812:

lab45813:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45815

lab45814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45815:

lab45816:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45818

lab45817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45818:

lab45819:
    jmp lab45821

lab45820:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45821:

lab45823:
    ; #load tag
    mov r13, 5
    ; let x16: List[i64] = Cons(x17, x18);
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
    je lab45835
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45836

lab45835:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45833
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45826
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45824
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45825

lab45824:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45825:

lab45826:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45829
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45827
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45828

lab45827:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45828:

lab45829:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45831

lab45830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45831:

lab45832:
    jmp lab45834

lab45833:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45834:

lab45836:
    ; #load tag
    mov r11, 5
    ; let x23: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x15: List[List[i64]] = Cons(x16, x23);
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
    je lab45848
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45849

lab45848:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45846
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45839
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45837
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45838

lab45837:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45838:

lab45839:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45842
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45840
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45841

lab45840:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45841:

lab45842:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45844

lab45843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45844:

lab45845:
    jmp lab45847

lab45846:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45847:

lab45849:
    ; #load tag
    mov r11, 5
    ; let x7: List[List[i64]] = Cons(x8, x15);
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
    je lab45861
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45862

lab45861:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45859
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45852
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45850
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45851

lab45850:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45851:

lab45852:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45855
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45853
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45854

lab45853:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45854:

lab45855:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45858
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45856
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45857

lab45856:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45857:

lab45858:
    jmp lab45860

lab45859:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45860:

lab45862:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x7 !-> x7)(a0 !-> a0);
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

diags_:
    ; lit x1 <- 0;
    mov rdi, 0
    ; lit x3 <- 4;
    mov r9, 4
    ; lit x5 <- 8;
    mov r11, 8
    ; let x6: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x4: List[i64] = Cons(x5, x6);
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
    je lab45874
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45875

lab45874:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45872
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45864

lab45863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45864:

lab45865:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45867

lab45866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45867:

lab45868:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45870

lab45869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45870:

lab45871:
    jmp lab45873

lab45872:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45873:

lab45875:
    ; #load tag
    mov r11, 5
    ; let x2: List[i64] = Cons(x3, x4);
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
    je lab45887
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45888

lab45887:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45885
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45878
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45876
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45877

lab45876:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45877:

lab45878:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45881
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45879
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45880

lab45879:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45880:

lab45881:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45883

lab45882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45883:

lab45884:
    jmp lab45886

lab45885:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45886:

lab45888:
    ; #load tag
    mov r9, 5
    ; let x0: List[i64] = Cons(x1, x2);
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
    je lab45900
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45901

lab45900:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45898
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45891
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45889
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45890

lab45889:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45890:

lab45891:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45894
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45892
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45893

lab45892:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45893:

lab45894:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45896

lab45895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45896:

lab45897:
    jmp lab45899

lab45898:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45899:

lab45901:
    ; #load tag
    mov rdi, 5
    ; lit x9 <- 2;
    mov r9, 2
    ; lit x11 <- 4;
    mov r11, 4
    ; lit x13 <- 6;
    mov r13, 6
    ; let x14: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; let x12: List[i64] = Cons(x13, x14);
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
    je lab45913
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab45914

lab45913:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45911
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45904
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45902
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45903

lab45902:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45903:

lab45904:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45907
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45905
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45906

lab45905:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45906:

lab45907:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45910
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45908
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45909

lab45908:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45909:

lab45910:
    jmp lab45912

lab45911:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45912:

lab45914:
    ; #load tag
    mov r13, 5
    ; let x10: List[i64] = Cons(x11, x12);
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
    je lab45926
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab45927

lab45926:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45924
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45917
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45915
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45916

lab45915:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45916:

lab45917:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45920
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45918
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45919

lab45918:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45919:

lab45920:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45923
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45921
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45922

lab45921:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45922:

lab45923:
    jmp lab45925

lab45924:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45925:

lab45927:
    ; #load tag
    mov r11, 5
    ; let x8: List[i64] = Cons(x9, x10);
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
    je lab45939
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45940

lab45939:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45937
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45930
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45928
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45929

lab45928:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45929:

lab45930:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45933
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45931
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45932

lab45931:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45932:

lab45933:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45936
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45934
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45935

lab45934:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45935:

lab45936:
    jmp lab45938

lab45937:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45938:

lab45940:
    ; #load tag
    mov r9, 5
    ; let x15: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; let x7: List[List[i64]] = Cons(x8, x15);
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
    je lab45952
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45953

lab45952:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45950
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45943
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45941
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45942

lab45941:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45942:

lab45943:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45946
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45944
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45945

lab45944:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45945:

lab45946:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45949
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45947
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45948

lab45947:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45948:

lab45949:
    jmp lab45951

lab45950:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45951:

lab45953:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x7 !-> x7)(a0 !-> a0);
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

is_occupied_:
    ; new a1: Option[Player] = (a0)\{ ... \};
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
    je lab45965
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45966

lab45965:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45963
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45956
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45954
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45955

lab45954:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45955:

lab45956:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45959
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45957
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45958

lab45957:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45958:

lab45959:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45961

lab45960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45961:

lab45962:
    jmp lab45964

lab45963:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45964:

lab45966:
    ; #load tag
    lea r9, [rel Option_Player_45967]
    ; jump nth_
    jmp nth_

Option_Player_45967:
    jmp near Option_Player_45967_None
    jmp near Option_Player_45967_Some

Option_Player_45967_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab45969
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab45968
    ; ####increment refcount
    add qword [rax + 0], 1

lab45968:
    jmp lab45970

lab45969:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab45970:
    ; let x0: Option[Player] = None();
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
    ; jump is_some_
    jmp is_some_

Option_Player_45967_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45972
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab45971
    ; ####increment refcount
    add qword [rsi + 0], 1

lab45971:
    jmp lab45973

lab45972:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab45973:
    ; substitute (a0 !-> a0)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: Option[Player] = Some(a2);
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
    je lab45985
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab45986

lab45985:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45983
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45976
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45974
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45975

lab45974:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45975:

lab45976:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45979
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45977
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45978

lab45977:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45978:

lab45979:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45982
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45980
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45981

lab45980:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45981:

lab45982:
    jmp lab45984

lab45983:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45984:

lab45986:
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
    ; jump is_some_
    jmp is_some_

player_occupies_:
    ; substitute (i !-> i)(board !-> board)(p !-> p)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: Option[Player] = (p, a0)\{ ... \};
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
    je lab45998
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab45999

lab45998:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab45996
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab45989
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45987
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45988

lab45987:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45988:

lab45989:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab45992
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45990
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45991

lab45990:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45991:

lab45992:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab45995
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab45993
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab45994

lab45993:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab45994:

lab45995:
    jmp lab45997

lab45996:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab45997:

lab45999:
    ; #load tag
    lea r9, [rel Option_Player_46000]
    ; substitute (board !-> board)(i !-> i)(a1 !-> a1);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump find_
    jmp find_

Option_Player_46000:
    jmp near Option_Player_46000_None
    jmp near Option_Player_46000_Some

Option_Player_46000_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46003
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46001
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46001:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46002
    ; ####increment refcount
    add qword [rax + 0], 1

lab46002:
    jmp lab46004

lab46003:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46004:
    ; substitute (a0 !-> a0);
    ; #erase p
    cmp rax, 0
    je lab46007
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab46005
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab46006

lab46005:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab46006:

lab46007:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Option_Player_46000_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab46010
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab46008
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46008:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab46009
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46009:
    jmp lab46011

lab46010:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab46011:
    ; substitute (p !-> p)(p0 !-> p0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump player_eq_
    jmp player_eq_

has_trip_:
    ; substitute (a0 !-> a0)(l !-> l)(p !-> p)(board !-> board);
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
    ; new x0: Fun[i64, Bool] = (p, board)\{ ... \};
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
    je lab46023
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46024

lab46023:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46021
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46014
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46012
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46013

lab46012:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46013:

lab46014:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46017
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46015
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46016

lab46015:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46016:

lab46017:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46020
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46018
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46019

lab46018:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46019:

lab46020:
    jmp lab46022

lab46021:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46022:

lab46024:
    ; #load tag
    lea r9, [rel Fun_i64_Bool_46025]
    ; substitute (x0 !-> x0)(l !-> l)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump all_i_
    jmp all_i_

Fun_i64_Bool_46025:

Fun_i64_Bool_46025_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46028
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46026
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46026:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46027
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46027:
    jmp lab46029

lab46028:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46029:
    ; substitute (p !-> p)(board !-> board)(i !-> i)(a1 !-> a1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rax, r8
    ; jump player_occupies_
    jmp player_occupies_

has_row_:
    ; substitute (a0 !-> a0)(p !-> p)(board !-> board);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new x0: Fun[List[i64], Bool] = (p, board)\{ ... \};
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
    je lab46041
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46042

lab46041:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46039
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46032
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46030
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46031

lab46030:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46031:

lab46032:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46034

lab46033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46034:

lab46035:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46038
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46036
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46037

lab46036:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46037:

lab46038:
    jmp lab46040

lab46039:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46040:

lab46042:
    ; #load tag
    lea rdi, [rel Fun_List_i64_Bool_46043]
    ; new a2: List[List[i64]] = (a0, x0)\{ ... \};
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
    je lab46055
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab46056

lab46055:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46053
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46046
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46044
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46045

lab46044:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46045:

lab46046:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46049
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46047
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46048

lab46047:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46048:

lab46049:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46052
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46050
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46051

lab46050:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46051:

lab46052:
    jmp lab46054

lab46053:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46054:

lab46056:
    ; #load tag
    lea rdx, [rel List_List_i64_46057]
    ; jump rows_
    jmp rows_

List_List_i64_46057:
    jmp near List_List_i64_46057_Nil
    jmp near List_List_i64_46057_Cons

List_List_i64_46057_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46060
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46058
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46058:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46059
    ; ####increment refcount
    add qword [rax + 0], 1

lab46059:
    jmp lab46061

lab46060:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46061:
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
    ; jump exists_
    jmp exists_

List_List_i64_46057_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46064
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46062
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46062:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46063
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46063:
    jmp lab46065

lab46064:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46065:
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
    ; let x1: List[List[i64]] = Cons(a3, as0);
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
    je lab46077
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46078

lab46077:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46075
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46068
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46066
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46067

lab46066:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46067:

lab46068:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46071
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46069
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46070

lab46069:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46070:

lab46071:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46073

lab46072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46073:

lab46074:
    jmp lab46076

lab46075:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46076:

lab46078:
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
    ; jump exists_
    jmp exists_

Fun_List_i64_Bool_46043:

Fun_List_i64_Bool_46043_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46081
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46079
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46079:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46080
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46080:
    jmp lab46082

lab46081:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46082:
    ; substitute (board !-> board)(p !-> p)(l !-> l)(a1 !-> a1);
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
    ; jump has_trip_
    jmp has_trip_

has_col_:
    ; substitute (a0 !-> a0)(p !-> p)(board !-> board);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new x0: Fun[List[i64], Bool] = (p, board)\{ ... \};
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
    je lab46094
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46095

lab46094:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46092
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46085
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46083
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46084

lab46083:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46084:

lab46085:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46088
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46086
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46087

lab46086:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46087:

lab46088:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46091
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46089
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46090

lab46089:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46090:

lab46091:
    jmp lab46093

lab46092:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46093:

lab46095:
    ; #load tag
    lea rdi, [rel Fun_List_i64_Bool_46096]
    ; new a2: List[List[i64]] = (a0, x0)\{ ... \};
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
    je lab46108
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab46109

lab46108:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46106
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46099
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46097
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46098

lab46097:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46098:

lab46099:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46102
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46100
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46101

lab46100:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46101:

lab46102:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46105
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46103
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46104

lab46103:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46104:

lab46105:
    jmp lab46107

lab46106:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46107:

lab46109:
    ; #load tag
    lea rdx, [rel List_List_i64_46110]
    ; jump cols_
    jmp cols_

List_List_i64_46110:
    jmp near List_List_i64_46110_Nil
    jmp near List_List_i64_46110_Cons

List_List_i64_46110_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46113
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46111
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46111:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46112
    ; ####increment refcount
    add qword [rax + 0], 1

lab46112:
    jmp lab46114

lab46113:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46114:
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
    ; jump exists_
    jmp exists_

List_List_i64_46110_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46117
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46115
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46115:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46116
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46116:
    jmp lab46118

lab46117:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46118:
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
    ; let x1: List[List[i64]] = Cons(a3, as0);
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
    je lab46130
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46131

lab46130:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46128
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46121
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46119
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46120

lab46119:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46120:

lab46121:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46124
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46122
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46123

lab46122:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46123:

lab46124:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46127
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46125
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46126

lab46125:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46126:

lab46127:
    jmp lab46129

lab46128:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46129:

lab46131:
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
    ; jump exists_
    jmp exists_

Fun_List_i64_Bool_46096:

Fun_List_i64_Bool_46096_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46134
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46132
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46132:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46133
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46133:
    jmp lab46135

lab46134:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46135:
    ; substitute (board !-> board)(p !-> p)(l !-> l)(a1 !-> a1);
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
    ; jump has_trip_
    jmp has_trip_

has_diag_:
    ; substitute (a0 !-> a0)(p !-> p)(board !-> board);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new x0: Fun[List[i64], Bool] = (p, board)\{ ... \};
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
    je lab46147
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46148

lab46147:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46145
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46138
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46136
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46137

lab46136:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46137:

lab46138:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46140

lab46139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46140:

lab46141:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46144
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46142
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46143

lab46142:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46143:

lab46144:
    jmp lab46146

lab46145:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46146:

lab46148:
    ; #load tag
    lea rdi, [rel Fun_List_i64_Bool_46149]
    ; new a2: List[List[i64]] = (a0, x0)\{ ... \};
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
    je lab46161
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab46162

lab46161:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46159
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46152
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46150
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46151

lab46150:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46151:

lab46152:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46155
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46153
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46154

lab46153:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46154:

lab46155:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46158
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46156
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46157

lab46156:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46157:

lab46158:
    jmp lab46160

lab46159:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46160:

lab46162:
    ; #load tag
    lea rdx, [rel List_List_i64_46163]
    ; jump diags_
    jmp diags_

List_List_i64_46163:
    jmp near List_List_i64_46163_Nil
    jmp near List_List_i64_46163_Cons

List_List_i64_46163_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46166
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46164
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46164:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46165
    ; ####increment refcount
    add qword [rax + 0], 1

lab46165:
    jmp lab46167

lab46166:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46167:
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
    ; jump exists_
    jmp exists_

List_List_i64_46163_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46170
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46168
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46168:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46169
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46169:
    jmp lab46171

lab46170:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46171:
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
    ; let x1: List[List[i64]] = Cons(a3, as0);
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
    je lab46183
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46184

lab46183:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46181
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46174
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46172
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46173

lab46172:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46173:

lab46174:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46177
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46175
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46176

lab46175:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46176:

lab46177:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46180
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46178
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46179

lab46178:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46179:

lab46180:
    jmp lab46182

lab46181:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46182:

lab46184:
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
    ; jump exists_
    jmp exists_

Fun_List_i64_Bool_46149:

Fun_List_i64_Bool_46149_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46187
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46185
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46185:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46186
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46186:
    jmp lab46188

lab46187:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46188:
    ; substitute (board !-> board)(p !-> p)(l !-> l)(a1 !-> a1);
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
    ; jump has_trip_
    jmp has_trip_

is_win_for_:
    ; substitute (board0 !-> board)(p0 !-> p)(a0 !-> a0)(board !-> board)(p !-> p);
    ; #share board
    cmp rax, 0
    je lab46189
    ; ####increment refcount
    add qword [rax + 0], 1

lab46189:
    ; #share p
    cmp rsi, 0
    je lab46190
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46190:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov r12, rsi
    mov r13, rdi
    ; new a1: Bool = (a0, board, p)\{ ... \};
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
    je lab46202
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46203

lab46202:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46200
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46193
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46191
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46192

lab46191:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46192:

lab46193:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46196
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46194
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46195

lab46194:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46195:

lab46196:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46199
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46197
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46198

lab46197:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46198:

lab46199:
    jmp lab46201

lab46200:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46201:

lab46203:
    ; #load tag
    lea r9, [rel Bool_46204]
    ; jump has_row_
    jmp has_row_

Bool_46204:
    jmp near Bool_46204_True
    jmp near Bool_46204_False

Bool_46204_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46208
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46205
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46205:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46206
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46206:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46207
    ; ####increment refcount
    add qword [rax + 0], 1

lab46207:
    jmp lab46209

lab46208:
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

lab46209:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; jump lift_is_win_for_0_
    jmp lift_is_win_for_0_

Bool_46204_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46213
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46210
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46210:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46211
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46211:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46212
    ; ####increment refcount
    add qword [rax + 0], 1

lab46212:
    jmp lab46214

lab46213:
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

lab46214:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; jump lift_is_win_for_0_
    jmp lift_is_win_for_0_

lift_is_win_for_0_:
    ; substitute (p !-> p)(board !-> board)(a0 !-> a0)(x0 !-> x0);
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
    je lab46226
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46227

lab46226:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46224
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46217
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46215
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46216

lab46215:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46216:

lab46217:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46219

lab46218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46219:

lab46220:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46222

lab46221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46222:

lab46223:
    jmp lab46225

lab46224:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46225:

lab46227:
    ; #load tag
    lea r9, [rel Bool_46228]
    ; substitute (p0 !-> p)(board0 !-> board)(a2 !-> a2)(p !-> p)(board !-> board);
    ; #share board
    cmp rsi, 0
    je lab46229
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46229:
    ; #share p
    cmp rax, 0
    je lab46230
    ; ####increment refcount
    add qword [rax + 0], 1

lab46230:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov r12, rsi
    mov r13, rdi
    ; new a3: Bool = (a2, p, board)\{ ... \};
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
    je lab46242
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46243

lab46242:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46240
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46233
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46231
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46232

lab46231:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46232:

lab46233:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46236
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46234
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46235

lab46234:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46235:

lab46236:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46239
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46237
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46238

lab46237:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46238:

lab46239:
    jmp lab46241

lab46240:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46241:

lab46243:
    ; #load tag
    lea r9, [rel Bool_46244]
    ; substitute (board0 !-> board0)(p0 !-> p0)(a3 !-> a3);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump has_col_
    jmp has_col_

Bool_46244:
    jmp near Bool_46244_True
    jmp near Bool_46244_False

Bool_46244_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46248
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46245
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46245:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46246
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46246:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46247
    ; ####increment refcount
    add qword [rax + 0], 1

lab46247:
    jmp lab46249

lab46248:
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

lab46249:
    ; let x2: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a2 !-> a2)(board !-> board)(p !-> p)(x2 !-> x2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_is_win_for_1_
    jmp lift_is_win_for_1_

Bool_46244_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46253
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46250
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46250:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46251
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46251:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46252
    ; ####increment refcount
    add qword [rax + 0], 1

lab46252:
    jmp lab46254

lab46253:
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

lab46254:
    ; let x2: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a2 !-> a2)(board !-> board)(p !-> p)(x2 !-> x2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_is_win_for_1_
    jmp lift_is_win_for_1_

Bool_46228:
    jmp near Bool_46228_True
    jmp near Bool_46228_False

Bool_46228_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46257
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46255
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46255:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46256
    ; ####increment refcount
    add qword [rax + 0], 1

lab46256:
    jmp lab46258

lab46257:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46258:
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

Bool_46228_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46261
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46259
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46259:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46260
    ; ####increment refcount
    add qword [rax + 0], 1

lab46260:
    jmp lab46262

lab46261:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46262:
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

lift_is_win_for_1_:
    ; substitute (p !-> p)(board !-> board)(a2 !-> a2)(x2 !-> x2);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a4: Bool = (a2, x2)\{ ... \};
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
    je lab46274
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46275

lab46274:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46272
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46265
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46263
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46264

lab46263:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46264:

lab46265:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46268
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46266
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46267

lab46266:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46267:

lab46268:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46271
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46269
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46270

lab46269:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46270:

lab46271:
    jmp lab46273

lab46272:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46273:

lab46275:
    ; #load tag
    lea r9, [rel Bool_46276]
    ; substitute (board !-> board)(p !-> p)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump has_diag_
    jmp has_diag_

Bool_46276:
    jmp near Bool_46276_True
    jmp near Bool_46276_False

Bool_46276_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46279
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46277
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46277:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46278
    ; ####increment refcount
    add qword [rax + 0], 1

lab46278:
    jmp lab46280

lab46279:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46280:
    ; let x3: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x3 !-> x3)(a2 !-> a2);
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

Bool_46276_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46283
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46281
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46281:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46282
    ; ####increment refcount
    add qword [rax + 0], 1

lab46282:
    jmp lab46284

lab46283:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46284:
    ; let x3: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x3 !-> x3)(a2 !-> a2);
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

is_win_:
    ; substitute (board0 !-> board)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab46285
    ; ####increment refcount
    add qword [rax + 0], 1

lab46285:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a1: Bool = (a0, board)\{ ... \};
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
    je lab46297
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46298

lab46297:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46295
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46288
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46286
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46287

lab46286:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46287:

lab46288:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46291
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46289
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46290

lab46289:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46290:

lab46291:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46294
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46292
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46293

lab46292:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46293:

lab46294:
    jmp lab46296

lab46295:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46296:

lab46298:
    ; #load tag
    lea rdi, [rel Bool_46299]
    ; let x1: Player = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (board0 !-> board0)(x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump is_win_for_
    jmp is_win_for_

Bool_46299:
    jmp near Bool_46299_True
    jmp near Bool_46299_False

Bool_46299_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46302
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46300
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46300:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46301
    ; ####increment refcount
    add qword [rax + 0], 1

lab46301:
    jmp lab46303

lab46302:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46303:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_is_win_0_
    jmp lift_is_win_0_

Bool_46299_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46306
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46304
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46304:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46305
    ; ####increment refcount
    add qword [rax + 0], 1

lab46305:
    jmp lab46307

lab46306:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46307:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; jump lift_is_win_0_
    jmp lift_is_win_0_

lift_is_win_0_:
    ; substitute (board !-> board)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0, x0)\{ ... \};
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
    je lab46319
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46320

lab46319:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46317
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46310
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46308
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46309

lab46308:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46309:

lab46310:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46313
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46311
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46312

lab46311:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46312:

lab46313:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46316
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46314
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46315

lab46314:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46315:

lab46316:
    jmp lab46318

lab46317:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46318:

lab46320:
    ; #load tag
    lea rdi, [rel Bool_46321]
    ; let x3: Player = O();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (board !-> board)(x3 !-> x3)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump is_win_for_
    jmp is_win_for_

Bool_46321:
    jmp near Bool_46321_True
    jmp near Bool_46321_False

Bool_46321_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46324
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46322
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46322:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46323
    ; ####increment refcount
    add qword [rax + 0], 1

lab46323:
    jmp lab46325

lab46324:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46325:
    ; let x2: Bool = True();
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
    ; jump or_
    jmp or_

Bool_46321_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46328
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46326
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46326:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46327
    ; ####increment refcount
    add qword [rax + 0], 1

lab46327:
    jmp lab46329

lab46328:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46329:
    ; let x2: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
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
    ; jump or_
    jmp or_

game_over_:
    ; substitute (board0 !-> board)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab46330
    ; ####increment refcount
    add qword [rax + 0], 1

lab46330:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a1: Bool = (a0, board)\{ ... \};
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
    je lab46342
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46343

lab46342:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46340
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46333
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46331
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46332

lab46331:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46332:

lab46333:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46336
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46334
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46335

lab46334:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46335:

lab46336:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46339
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46337
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46338

lab46337:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46338:

lab46339:
    jmp lab46341

lab46340:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46341:

lab46343:
    ; #load tag
    lea rdi, [rel Bool_46344]
    ; jump is_win_
    jmp is_win_

Bool_46344:
    jmp near Bool_46344_True
    jmp near Bool_46344_False

Bool_46344_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46347
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46345
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46345:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46346
    ; ####increment refcount
    add qword [rax + 0], 1

lab46346:
    jmp lab46348

lab46347:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46348:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_game_over_0_
    jmp lift_game_over_0_

Bool_46344_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46351
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46349
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46349:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46350
    ; ####increment refcount
    add qword [rax + 0], 1

lab46350:
    jmp lab46352

lab46351:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46352:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; jump lift_game_over_0_
    jmp lift_game_over_0_

lift_game_over_0_:
    ; substitute (board !-> board)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0, x0)\{ ... \};
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
    je lab46364
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46365

lab46364:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46362
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46355
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46353
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46354

lab46353:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46354:

lab46355:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46358
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46356
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46357

lab46356:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46357:

lab46358:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46361
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46359
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46360

lab46359:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46360:

lab46361:
    jmp lab46363

lab46362:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46363:

lab46365:
    ; #load tag
    lea rdi, [rel Bool_46366]
    ; jump is_cat_
    jmp is_cat_

Bool_46366:
    jmp near Bool_46366_True
    jmp near Bool_46366_False

Bool_46366_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46369
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46367
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46367:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46368
    ; ####increment refcount
    add qword [rax + 0], 1

lab46368:
    jmp lab46370

lab46369:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46370:
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

Bool_46366_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46373
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46371
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46371:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46372
    ; ####increment refcount
    add qword [rax + 0], 1

lab46372:
    jmp lab46374

lab46373:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46374:
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

score_:
    ; let x0: Player = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (board0 !-> board)(x0 !-> x0)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab46375
    ; ####increment refcount
    add qword [rax + 0], 1

lab46375:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: Bool = (a0, board)\{ ... \};
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
    je lab46387
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46388

lab46387:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46385
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46378
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46376
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46377

lab46376:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46377:

lab46378:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46381
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46379
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46380

lab46379:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46380:

lab46381:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46384
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46382
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46383

lab46382:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46383:

lab46384:
    jmp lab46386

lab46385:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46386:

lab46388:
    ; #load tag
    lea r9, [rel Bool_46389]
    ; jump is_win_for_
    jmp is_win_for_

Bool_46389:
    jmp near Bool_46389_True
    jmp near Bool_46389_False

Bool_46389_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46392
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46390
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46390:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46391
    ; ####increment refcount
    add qword [rax + 0], 1

lab46391:
    jmp lab46393

lab46392:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46393:
    ; substitute (a0 !-> a0);
    ; #erase board
    cmp rsi, 0
    je lab46396
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab46394
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab46395

lab46394:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab46395:

lab46396:
    ; lit x2 <- 1;
    mov rdi, 1
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Bool_46389_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46399
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46397
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46397:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46398
    ; ####increment refcount
    add qword [rax + 0], 1

lab46398:
    jmp lab46400

lab46399:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46400:
    ; let x1: Player = O();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(board !-> board)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0)\{ ... \};
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
    je lab46412
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46413

lab46412:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46410
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46403
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46401
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46402

lab46401:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46402:

lab46403:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46406
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46404
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46405

lab46404:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46405:

lab46406:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46409
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46407
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46408

lab46407:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46408:

lab46409:
    jmp lab46411

lab46410:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46411:

lab46413:
    ; #load tag
    lea r9, [rel Bool_46414]
    ; substitute (board !-> board)(x1 !-> x1)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump is_win_for_
    jmp is_win_for_

Bool_46414:
    jmp near Bool_46414_True
    jmp near Bool_46414_False

Bool_46414_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46416
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab46415
    ; ####increment refcount
    add qword [rax + 0], 1

lab46415:
    jmp lab46417

lab46416:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab46417:
    ; lit x3 <- -1;
    mov rdi, -1
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Bool_46414_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46419
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab46418
    ; ####increment refcount
    add qword [rax + 0], 1

lab46418:
    jmp lab46420

lab46419:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab46420:
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

put_at_:
    ; if i == 0 \{ ... \}
    cmp r9, 0
    je lab46421
    ; lit x1 <- 0;
    mov r13, 0
    ; if x1 < i \{ ... \}
    cmp r13, r9
    jl lab46422
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rax, 0
    je lab46425
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab46423
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab46424

lab46423:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab46424:

lab46425:
    ; #erase xs
    cmp rsi, 0
    je lab46428
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab46426
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab46427

lab46426:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab46427:

lab46428:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab46422:
    ; substitute (xs0 !-> xs)(xs !-> xs)(i !-> i)(a0 !-> a0)(x !-> x);
    ; #share xs
    cmp rsi, 0
    je lab46429
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46429:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a2: Option[Player] = (xs, i, a0, x)\{ ... \};
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
    je lab46441
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46442

lab46441:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46439
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46432
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46430
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46431

lab46430:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46431:

lab46432:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46435
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46433
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46434

lab46433:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46434:

lab46435:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46438
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46436
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46437

lab46436:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46437:

lab46438:
    jmp lab46440

lab46439:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46440:

lab46442:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab46454
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46455

lab46454:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46452
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46445
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46443
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46444

lab46443:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46444:

lab46445:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46448
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46446
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46447

lab46446:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46447:

lab46448:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46451
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46449
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46450

lab46449:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46450:

lab46451:
    jmp lab46453

lab46452:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46453:

lab46455:
    ; #load tag
    lea rdi, [rel Option_Player_46456]
    ; jump head_
    jmp head_

Option_Player_46456:
    jmp near Option_Player_46456_None
    jmp near Option_Player_46456_Some

Option_Player_46456_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46460
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46457
    ; ####increment refcount
    add qword [rax + 0], 1

lab46457:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab46458
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46458:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab46459
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46459:
    mov rdi, [rsi + 24]
    jmp lab46461

lab46460:
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

lab46461:
    ; let x2: Option[Player] = None();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(i !-> i)(x !-> x)(x2 !-> x2)(xs !-> xs);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump lift_put_at_0_
    jmp lift_put_at_0_

Option_Player_46456_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab46465
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab46462
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46462:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab46463
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46463:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab46464
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46464:
    mov r9, [r8 + 24]
    jmp lab46466

lab46465:
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
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab46466:
    ; substitute (x !-> x)(xs !-> xs)(i !-> i)(a0 !-> a0)(a9 !-> a9);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x2: Option[Player] = Some(a9);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab46478
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab46479

lab46478:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46476
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46469
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46467
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46468

lab46467:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46468:

lab46469:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46472
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46470
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46471

lab46470:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46471:

lab46472:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46475
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46473
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46474

lab46473:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46474:

lab46475:
    jmp lab46477

lab46476:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46477:

lab46479:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(i !-> i)(x !-> x)(x2 !-> x2)(xs !-> xs);
    ; #move variables
    mov r8, rax
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, r12
    mov r12, rsi
    ; jump lift_put_at_0_
    jmp lift_put_at_0_

lab46421:
    ; substitute (xs !-> xs)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r10
    mov r9, r11
    ; new a1: List[Option[Player]] = (x, a0)\{ ... \};
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
    je lab46491
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46492

lab46491:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46489
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46481

lab46480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46481:

lab46482:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46484

lab46483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46484:

lab46485:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46488
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46486
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46487

lab46486:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46487:

lab46488:
    jmp lab46490

lab46489:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46490:

lab46492:
    ; #load tag
    lea rdi, [rel List_Option_Player_46493]
    ; jump tail_
    jmp tail_

List_Option_Player_46493:
    jmp near List_Option_Player_46493_Nil
    jmp near List_Option_Player_46493_Cons

List_Option_Player_46493_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46496
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46494
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46494:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46495
    ; ####increment refcount
    add qword [rax + 0], 1

lab46495:
    jmp lab46497

lab46496:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46497:
    ; let x0: List[Option[Player]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x !-> x)(x0 !-> x0)(a0 !-> a0);
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

List_Option_Player_46493_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46500
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46498
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46498:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46499
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46499:
    jmp lab46501

lab46500:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46501:
    ; substitute (a0 !-> a0)(x !-> x)(a6 !-> a6)(as0 !-> as0);
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
    ; let x0: List[Option[Player]] = Cons(a6, as0);
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
    je lab46513
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46514

lab46513:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46511
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46504
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46502
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46503

lab46502:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46503:

lab46504:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46507
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46505
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46506

lab46505:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46506:

lab46507:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46510
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46508
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46509

lab46508:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46509:

lab46510:
    jmp lab46512

lab46511:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46512:

lab46514:
    ; #load tag
    mov r9, 5
    ; substitute (x !-> x)(x0 !-> x0)(a0 !-> a0);
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

lift_put_at_0_:
    ; substitute (xs !-> xs)(i !-> i)(x !-> x)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; new a3: List[Option[Player]] = (x2, a0)\{ ... \};
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
    je lab46526
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46527

lab46526:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46524
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46517
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46515
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46516

lab46515:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46516:

lab46517:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46520
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46518
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46519

lab46518:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46519:

lab46520:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46523
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46521
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46522

lab46521:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46522:

lab46523:
    jmp lab46525

lab46524:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46525:

lab46527:
    ; #load tag
    lea r11, [rel List_Option_Player_46528]
    ; new a4: List[Option[Player]] = (i, x, a3)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
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
    je lab46540
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46541

lab46540:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46538
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46531
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46529
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46530

lab46529:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46530:

lab46531:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46534
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46532
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46533

lab46532:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46533:

lab46534:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46537
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46535
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46536

lab46535:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46536:

lab46537:
    jmp lab46539

lab46538:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46539:

lab46541:
    ; #load tag
    lea rdi, [rel List_Option_Player_46542]
    ; jump tail_
    jmp tail_

List_Option_Player_46542:
    jmp near List_Option_Player_46542_Nil
    jmp near List_Option_Player_46542_Cons

List_Option_Player_46542_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46545
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46543
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46543:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46544
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46544:
    mov rdx, [rax + 24]
    jmp lab46546

lab46545:
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

lab46546:
    ; let x4: List[Option[Player]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a3 !-> a3)(i !-> i)(x !-> x)(x4 !-> x4);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; jump lift_put_at_1_
    jmp lift_put_at_1_

List_Option_Player_46542_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46549
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab46547
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46547:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab46548
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46548:
    mov r9, [r8 + 24]
    jmp lab46550

lab46549:
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

lab46550:
    ; substitute (a3 !-> a3)(x !-> x)(i !-> i)(a8 !-> a8)(as2 !-> as2);
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
    ; let x4: List[Option[Player]] = Cons(a8, as2);
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
    je lab46562
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46563

lab46562:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46560
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46553
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46551
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46552

lab46551:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46552:

lab46553:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46556
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46554
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46555

lab46554:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46555:

lab46556:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46559
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46557
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46558

lab46557:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46558:

lab46559:
    jmp lab46561

lab46560:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46561:

lab46563:
    ; #load tag
    mov r11, 5
    ; substitute (a3 !-> a3)(i !-> i)(x !-> x)(x4 !-> x4);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_put_at_1_
    jmp lift_put_at_1_

List_Option_Player_46528:
    jmp near List_Option_Player_46528_Nil
    jmp near List_Option_Player_46528_Cons

List_Option_Player_46528_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46566
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46564
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46564:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46565
    ; ####increment refcount
    add qword [rax + 0], 1

lab46565:
    jmp lab46567

lab46566:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46567:
    ; let x3: List[Option[Player]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
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

List_Option_Player_46528_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46570
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46568
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46568:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46569
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46569:
    jmp lab46571

lab46570:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46571:
    ; substitute (a0 !-> a0)(x2 !-> x2)(a7 !-> a7)(as1 !-> as1);
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
    ; let x3: List[Option[Player]] = Cons(a7, as1);
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
    je lab46583
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46584

lab46583:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46581
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46574
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46572
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46573

lab46572:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46573:

lab46574:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46577
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46575
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46576

lab46575:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46576:

lab46577:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46580
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46578
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46579

lab46578:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46579:

lab46580:
    jmp lab46582

lab46581:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46582:

lab46584:
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x3 !-> x3)(a0 !-> a0);
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

lift_put_at_1_:
    ; lit x5 <- 1;
    mov r13, 1
    ; x6 <- i - x5;
    mov r15, rdi
    sub r15, r13
    ; substitute (x !-> x)(x4 !-> x4)(x6 !-> x6)(a3 !-> a3);
    ; #move variables
    mov rsi, r10
    mov r10, rax
    mov rdi, r11
    mov r11, rdx
    mov rax, r8
    mov rdx, r9
    mov r9, r15
    ; jump put_at_
    jmp put_at_

move_to_:
    ; substitute (board0 !-> board)(i0 !-> i)(i !-> i)(a0 !-> a0)(board !-> board)(p !-> p);
    ; #share board
    cmp rax, 0
    je lab46585
    ; ####increment refcount
    add qword [rax + 0], 1

lab46585:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    mov rdi, r9
    ; new a1: Bool = (i, a0, board, p)\{ ... \};
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
    je lab46597
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46598

lab46597:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46595
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46588
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46586
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46587

lab46586:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46587:

lab46588:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46591
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46589
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46590

lab46589:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46590:

lab46591:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46593

lab46592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46593:

lab46594:
    jmp lab46596

lab46595:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46596:

lab46598:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab46610
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46611

lab46610:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46608
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46600

lab46599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46600:

lab46601:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46603

lab46602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46603:

lab46604:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46606

lab46605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46606:

lab46607:
    jmp lab46609

lab46608:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46609:

lab46611:
    ; #load tag
    lea r9, [rel Bool_46612]
    ; jump is_occupied_
    jmp is_occupied_

Bool_46612:
    jmp near Bool_46612_True
    jmp near Bool_46612_False

Bool_46612_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46616
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
    je lab46613
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46613:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab46614
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46614:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab46615
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46615:
    jmp lab46617

lab46616:
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
    mov rsi, [rsi + 16]

lab46617:
    ; substitute (a0 !-> a0);
    ; #erase board
    cmp r8, 0
    je lab46620
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab46618
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab46619

lab46618:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab46619:

lab46620:
    ; #erase p
    cmp r10, 0
    je lab46623
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab46621
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab46622

lab46621:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab46622:

lab46623:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

Bool_46612_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46627
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
    je lab46624
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46624:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab46625
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46625:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab46626
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46626:
    jmp lab46628

lab46627:
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
    mov rsi, [rsi + 16]

lab46628:
    ; let x0: Option[Player] = Some(p);
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
    je lab46640
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46641

lab46640:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46638
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46631
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46629
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46630

lab46629:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46630:

lab46631:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46634
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46632
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46633

lab46632:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46633:

lab46634:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46637
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46635
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46636

lab46635:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46636:

lab46637:
    jmp lab46639

lab46638:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46639:

lab46641:
    ; #load tag
    mov r11, 5
    ; substitute (x0 !-> x0)(board !-> board)(i !-> i)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; jump put_at_
    jmp put_at_

all_moves_rec_:
    ; substitute (n !-> n)(a0 !-> a0)(acc !-> acc)(board !-> board);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch board \{ ... \};
    lea rcx, [rel List_Option_Player_46642]
    add rcx, r11
    jmp rcx

List_Option_Player_46642:
    jmp near List_Option_Player_46642_Nil
    jmp near List_Option_Player_46642_Cons

List_Option_Player_46642_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; jump rev_
    jmp rev_

List_Option_Player_46642_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab46645
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab46643
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46643:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab46644
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46644:
    jmp lab46646

lab46645:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab46646:
    ; substitute (n !-> n)(a0 !-> a0)(acc !-> acc)(more !-> more)(p !-> p);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; switch p \{ ... \};
    lea rcx, [rel Option_Player_46647]
    add rcx, r13
    jmp rcx

Option_Player_46647:
    jmp near Option_Player_46647_None
    jmp near Option_Player_46647_Some

Option_Player_46647_None:
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- n + x0;
    mov r15, rdx
    add r15, r13
    ; substitute (x1 !-> x1)(a0 !-> a0)(more !-> more)(n !-> n)(acc !-> acc);
    ; #move variables
    mov r13, r9
    mov r9, r11
    mov r11, rdx
    mov r12, r8
    mov r8, r10
    mov rdx, r15
    ; let x2: List[i64] = Cons(n, acc);
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
    je lab46659
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46660

lab46659:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46657
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46650
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46648
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46649

lab46648:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46649:

lab46650:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46653
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46651
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46652

lab46651:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46652:

lab46653:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46655

lab46654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46655:

lab46656:
    jmp lab46658

lab46657:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46658:

lab46660:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(more !-> more)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump all_moves_rec_
    jmp all_moves_rec_

Option_Player_46647_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab46662
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov r13, [r12 + 56]
    mov r12, [r12 + 48]
    cmp r12, 0
    je lab46661
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46661:
    jmp lab46663

lab46662:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov r13, [r12 + 56]
    mov r12, [r12 + 48]

lab46663:
    ; substitute (n !-> n)(a0 !-> a0)(acc !-> acc)(more !-> more);
    ; #erase p0
    cmp r12, 0
    je lab46666
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab46664
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab46665

lab46664:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab46665:

lab46666:
    ; lit x3 <- 1;
    mov r13, 1
    ; x4 <- n + x3;
    mov r15, rdx
    add r15, r13
    ; substitute (x4 !-> x4)(more !-> more)(acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rdx, r15
    ; jump all_moves_rec_
    jmp all_moves_rec_

all_moves_:
    ; lit x0 <- 0;
    mov r9, 0
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x0 !-> x0)(board !-> board)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rsi, rax
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump all_moves_rec_
    jmp all_moves_rec_

successors_:
    ; substitute (board0 !-> board)(p !-> p)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab46667
    ; ####increment refcount
    add qword [rax + 0], 1

lab46667:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; new a1: List[i64] = (p, a0, board)\{ ... \};
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
    je lab46679
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46680

lab46679:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46677
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46670
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46668
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46669

lab46668:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46669:

lab46670:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46673
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46671
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46672

lab46671:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46672:

lab46673:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46676
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46674
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46675

lab46674:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46675:

lab46676:
    jmp lab46678

lab46677:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46678:

lab46680:
    ; #load tag
    lea rdi, [rel List_i64_46681]
    ; jump all_moves_
    jmp all_moves_

List_i64_46681:
    jmp near List_i64_46681_Nil
    jmp near List_i64_46681_Cons

List_i64_46681_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46685
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46682
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46682:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46683
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46683:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46684
    ; ####increment refcount
    add qword [rax + 0], 1

lab46684:
    jmp lab46686

lab46685:
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

lab46686:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_successors_0_
    jmp lift_successors_0_

List_i64_46681_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46690
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab46687
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46687:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab46688
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46688:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab46689
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46689:
    jmp lab46691

lab46690:
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

lab46691:
    ; substitute (board !-> board)(a0 !-> a0)(p !-> p)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x0: List[i64] = Cons(a3, as0);
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
    je lab46703
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46704

lab46703:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46701
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46694
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46692
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46693

lab46692:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46693:

lab46694:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46697
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46695
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46696

lab46695:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46696:

lab46697:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46700
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46698
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46699

lab46698:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46699:

lab46700:
    jmp lab46702

lab46701:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46702:

lab46704:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_successors_0_
    jmp lift_successors_0_

lift_successors_0_:
    ; substitute (a0 !-> a0)(x0 !-> x0)(p !-> p)(board !-> board);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new x1: Fun[i64, List[Option[Player]]] = (p, board)\{ ... \};
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
    je lab46716
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46717

lab46716:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46714
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46707
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46705
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46706

lab46705:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46706:

lab46707:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46710
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46708
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46709

lab46708:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46709:

lab46710:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46713
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46711
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46712

lab46711:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46712:

lab46713:
    jmp lab46715

lab46714:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46715:

lab46717:
    ; #load tag
    lea r9, [rel Fun_i64_List_Option_Player_46718]
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
    ; jump map_i_board_
    jmp map_i_board_

Fun_i64_List_Option_Player_46718:

Fun_i64_List_Option_Player_46718_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46721
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab46719
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46719:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46720
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46720:
    jmp lab46722

lab46721:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab46722:
    ; substitute (board !-> board)(p !-> p)(i !-> i)(a2 !-> a2);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; jump move_to_
    jmp move_to_

minimax_:
    ; substitute (board2 !-> board)(board !-> board)(a0 !-> a0)(p !-> p);
    ; #share board
    cmp rsi, 0
    je lab46723
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46723:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a9: Bool = (board, a0, p)\{ ... \};
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
    je lab46735
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46736

lab46735:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46733
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46726
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46724
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46725

lab46724:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46725:

lab46726:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46729
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46727
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46728

lab46727:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46728:

lab46729:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46732
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46730
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46731

lab46730:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46731:

lab46732:
    jmp lab46734

lab46733:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46734:

lab46736:
    ; #load tag
    lea rdi, [rel Bool_46737]
    ; jump game_over_
    jmp game_over_

Bool_46737:
    jmp near Bool_46737_True
    jmp near Bool_46737_False

Bool_46737_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46741
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46738
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46738:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46739
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46739:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46740
    ; ####increment refcount
    add qword [rax + 0], 1

lab46740:
    jmp lab46742

lab46741:
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

lab46742:
    ; substitute (board0 !-> board)(a0 !-> a0)(board !-> board);
    ; #share board
    cmp rax, 0
    je lab46743
    ; ####increment refcount
    add qword [rax + 0], 1

lab46743:
    ; #erase p
    cmp r8, 0
    je lab46746
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab46744
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab46745

lab46744:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab46745:

lab46746:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; new a1: _Cont = (a0, board)\{ ... \};
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
    je lab46758
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46759

lab46758:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46756
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46749
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46747
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46748

lab46747:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46748:

lab46749:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46752
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46750
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46751

lab46750:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46751:

lab46752:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46755
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46753
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46754

lab46753:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46754:

lab46755:
    jmp lab46757

lab46756:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46757:

lab46759:
    ; #load tag
    lea rdi, [rel _Cont_46760]
    ; jump score_
    jmp score_

_Cont_46760:

_Cont_46760_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab46763
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab46761
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46761:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab46762
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46762:
    jmp lab46764

lab46763:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab46764:
    ; substitute (a0 !-> a0)(board !-> board)(x1 !-> x1);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rsi, r8
    ; let x0: Pair[List[Option[Player]], i64] = Tup(board, x1);
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
    je lab46776
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46777

lab46776:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46774
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46767
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46765
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46766

lab46765:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46766:

lab46767:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46770
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46768
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46769

lab46768:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46769:

lab46770:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46773
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46771
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46772

lab46771:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46772:

lab46773:
    jmp lab46775

lab46774:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46775:

lab46777:
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
    ; jump mk_leaf_
    jmp mk_leaf_

Bool_46737_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46781
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46778
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46778:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46779
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46779:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46780
    ; ####increment refcount
    add qword [rax + 0], 1

lab46780:
    jmp lab46782

lab46781:
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

lab46782:
    ; substitute (board1 !-> board)(p0 !-> p)(p !-> p)(board !-> board)(a0 !-> a0);
    ; #share board
    cmp rax, 0
    je lab46783
    ; ####increment refcount
    add qword [rax + 0], 1

lab46783:
    ; #share p
    cmp r8, 0
    je lab46784
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46784:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov r12, rsi
    mov r13, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a6: List[List[Option[Player]]] = (p, board, a0)\{ ... \};
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
    je lab46796
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46797

lab46796:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46794
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46787
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46785
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46786

lab46785:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46786:

lab46787:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46790
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46788
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46789

lab46788:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46789:

lab46790:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46793
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46791
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46792

lab46791:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46792:

lab46793:
    jmp lab46795

lab46794:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46795:

lab46797:
    ; #load tag
    lea r9, [rel List_List_Option_Player_46798]
    ; jump successors_
    jmp successors_

List_List_Option_Player_46798:
    jmp near List_List_Option_Player_46798_Nil
    jmp near List_List_Option_Player_46798_Cons

List_List_Option_Player_46798_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46802
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46799
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46799:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46800
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46800:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46801
    ; ####increment refcount
    add qword [rax + 0], 1

lab46801:
    jmp lab46803

lab46802:
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

lab46803:
    ; let x2: List[List[Option[Player]]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(x2 !-> x2);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_minimax_0_
    jmp lift_minimax_0_

List_List_Option_Player_46798_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46807
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab46804
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46804:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab46805
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46805:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab46806
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46806:
    jmp lab46808

lab46807:
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

lab46808:
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(a15 !-> a15)(as2 !-> as2);
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
    ; let x2: List[List[Option[Player]]] = Cons(a15, as2);
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
    je lab46820
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46821

lab46820:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46818
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46811
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46809
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46810

lab46809:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46810:

lab46811:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46814
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46812
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46813

lab46812:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46813:

lab46814:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46817
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46815
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46816

lab46815:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46816:

lab46817:
    jmp lab46819

lab46818:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46819:

lab46821:
    ; #load tag
    mov r11, 5
    ; jump lift_minimax_0_
    jmp lift_minimax_0_

lift_minimax_0_:
    ; substitute (a0 !-> a0)(board !-> board)(p0 !-> p)(x2 !-> x2)(p !-> p);
    ; #share p
    cmp r8, 0
    je lab46822
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46822:
    ; #move variables
    mov r12, r8
    mov r13, r9
    ; new x3: Fun[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]] = (p)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab46834
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab46835

lab46834:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46832
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46825
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46823
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46824

lab46823:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46824:

lab46825:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46827

lab46826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46827:

lab46828:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46831
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46829
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46830

lab46829:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46830:

lab46831:
    jmp lab46833

lab46832:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46833:

lab46835:
    ; #load tag
    lea r13, [rel Fun_List_Option_Player_RoseTree_Pair_List_Option_Player_i64_46836]
    ; substitute (x3 !-> x3)(x2 !-> x2)(p0 !-> p0)(board !-> board)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a10: List[RoseTree[Pair[List[Option[Player]], i64]]] = (p0, board, a0)\{ ... \};
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
    je lab46848
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46849

lab46848:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46846
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46839
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46837
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46838

lab46837:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46838:

lab46839:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46842
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46840
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46841

lab46840:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46841:

lab46842:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46845
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46843
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46844

lab46843:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46844:

lab46845:
    jmp lab46847

lab46846:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46847:

lab46849:
    ; #load tag
    lea r9, [rel List_RoseTree_Pair_List_Option_Player_i64_46850]
    ; substitute (x2 !-> x2)(x3 !-> x3)(a10 !-> a10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_board_tree_
    jmp map_board_tree_

List_RoseTree_Pair_List_Option_Player_i64_46850:
    jmp near List_RoseTree_Pair_List_Option_Player_i64_46850_Nil
    jmp near List_RoseTree_Pair_List_Option_Player_i64_46850_Cons

List_RoseTree_Pair_List_Option_Player_i64_46850_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46854
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab46851
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46851:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab46852
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46852:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab46853
    ; ####increment refcount
    add qword [rax + 0], 1

lab46853:
    jmp lab46855

lab46854:
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

lab46855:
    ; let trees: List[RoseTree[Pair[List[Option[Player]], i64]]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(board !-> board)(p0 !-> p0)(trees !-> trees);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_minimax_1_
    jmp lift_minimax_1_

List_RoseTree_Pair_List_Option_Player_i64_46850_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46859
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab46856
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46856:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab46857
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46857:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab46858
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46858:
    jmp lab46860

lab46859:
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

lab46860:
    ; substitute (a0 !-> a0)(board !-> board)(p0 !-> p0)(a14 !-> a14)(as1 !-> as1);
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
    ; let trees: List[RoseTree[Pair[List[Option[Player]], i64]]] = Cons(a14, as1);
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
    je lab46872
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46873

lab46872:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46870
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46863
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46861
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46862

lab46861:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46862:

lab46863:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46866
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46864
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46865

lab46864:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46865:

lab46866:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46869
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46867
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46868

lab46867:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46868:

lab46869:
    jmp lab46871

lab46870:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46871:

lab46873:
    ; #load tag
    mov r11, 5
    ; jump lift_minimax_1_
    jmp lift_minimax_1_

Fun_List_Option_Player_RoseTree_Pair_List_Option_Player_i64_46836:

Fun_List_Option_Player_RoseTree_Pair_List_Option_Player_i64_46836_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46875
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab46874
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46874:
    jmp lab46876

lab46875:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab46876:
    ; substitute (p !-> p)(a7 !-> a7)(b !-> b);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a8: Player = (a7, b)\{ ... \};
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
    je lab46888
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46889

lab46888:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46886
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46879
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46877
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46878

lab46877:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46878:

lab46879:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46882
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46880
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46881

lab46880:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46881:

lab46882:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46885
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46883
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46884

lab46883:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46884:

lab46885:
    jmp lab46887

lab46886:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46887:

lab46889:
    ; #load tag
    lea rdi, [rel Player_46890]
    ; jump other_
    jmp other_

Player_46890:
    jmp near Player_46890_X
    jmp near Player_46890_O

Player_46890_X:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46893
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46891
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46891:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46892
    ; ####increment refcount
    add qword [rax + 0], 1

lab46892:
    jmp lab46894

lab46893:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46894:
    ; let x10: Player = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x10 !-> x10)(b !-> b)(a7 !-> a7);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump minimax_
    jmp minimax_

Player_46890_O:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46897
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab46895
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46895:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46896
    ; ####increment refcount
    add qword [rax + 0], 1

lab46896:
    jmp lab46898

lab46897:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab46898:
    ; let x10: Player = O();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x10 !-> x10)(b !-> b)(a7 !-> a7);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump minimax_
    jmp minimax_

lift_minimax_1_:
    ; new x4: Fun[RoseTree[Pair[List[Option[Player]], i64]], i64] = ()\{ ... \};
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    lea r13, [rel Fun_RoseTree_Pair_List_Option_Player_i64_i64_46899]
    ; substitute (x4 !-> x4)(trees0 !-> trees)(p !-> p)(trees !-> trees)(board !-> board)(a0 !-> a0);
    ; #share trees
    cmp r10, 0
    je lab46900
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46900:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r12
    mov r12, rsi
    mov rdx, r13
    mov r13, rdi
    mov rsi, r10
    mov rdi, r11
    ; new a11: List[i64] = (p, trees, board, a0)\{ ... \};
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
    je lab46912
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab46913

lab46912:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46910
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46902

lab46901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46902:

lab46903:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46905

lab46904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46905:

lab46906:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46909
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46907
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46908

lab46907:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46908:

lab46909:
    jmp lab46911

lab46910:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46911:

lab46913:
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
    je lab46925
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab46926

lab46925:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46923
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46915

lab46914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46915:

lab46916:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46918

lab46917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46918:

lab46919:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46922
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46920
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46921

lab46920:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46921:

lab46922:
    jmp lab46924

lab46923:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46924:

lab46926:
    ; #load tag
    lea r9, [rel List_i64_46927]
    ; substitute (trees0 !-> trees0)(x4 !-> x4)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_tree_i_
    jmp map_tree_i_

List_i64_46927:
    jmp near List_i64_46927_Nil
    jmp near List_i64_46927_Cons

List_i64_46927_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab46932
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab46928
    ; ####increment refcount
    add qword [rax + 0], 1

lab46928:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab46929
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46929:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab46930
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46930:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab46931
    ; ####increment refcount
    add qword [rsi + 0], 1

lab46931:
    jmp lab46933

lab46932:
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

lab46933:
    ; let scores: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(scores !-> scores)(trees !-> trees);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_minimax_2_
    jmp lift_minimax_2_

List_i64_46927_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46938
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab46934
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46934:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab46935
    ; ####increment refcount
    add qword [r14 + 0], 1

lab46935:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab46936
    ; ####increment refcount
    add qword [r12 + 0], 1

lab46936:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab46937
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46937:
    jmp lab46939

lab46938:
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

lab46939:
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(trees !-> trees)(a13 !-> a13)(as0 !-> as0);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    mov rsi, r12
    ; let scores: List[i64] = Cons(a13, as0);
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
    je lab46951
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab46952

lab46951:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46949
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46941

lab46940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46941:

lab46942:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46945
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46943
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46944

lab46943:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46944:

lab46945:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46948
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46946
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46947

lab46946:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46947:

lab46948:
    jmp lab46950

lab46949:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46950:

lab46952:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(board !-> board)(p !-> p)(scores !-> scores)(trees !-> trees);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_minimax_2_
    jmp lift_minimax_2_

Fun_RoseTree_Pair_List_Option_Player_i64_i64_46899:

Fun_RoseTree_Pair_List_Option_Player_i64_i64_46899_Apply:
    ; new a5: Pair[List[Option[Player]], i64] = (a4)\{ ... \};
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
    je lab46964
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46965

lab46964:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46962
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46954

lab46953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46954:

lab46955:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46958
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46956
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46957

lab46956:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46957:

lab46958:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46961
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46959
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46960

lab46959:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46960:

lab46961:
    jmp lab46963

lab46962:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46963:

lab46965:
    ; #load tag
    lea rdi, [rel Pair_List_Option_Player_i64_46966]
    ; jump top_
    jmp top_

Pair_List_Option_Player_i64_46966:

Pair_List_Option_Player_i64_46966_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab46968
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab46967
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46967:
    jmp lab46969

lab46968:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab46969:
    ; substitute (a4 !-> a4)(a12 !-> a12)(b0 !-> b0);
    ; #move variables
    mov rsi, rax
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x9: Pair[List[Option[Player]], i64] = Tup(a12, b0);
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
    je lab46981
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46982

lab46981:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46979
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46972
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46970
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46971

lab46970:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46971:

lab46972:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46975
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46973
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46974

lab46973:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46974:

lab46975:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46977

lab46976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46977:

lab46978:
    jmp lab46980

lab46979:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46980:

lab46982:
    ; #load tag
    mov rdi, 0
    ; substitute (x9 !-> x9)(a4 !-> a4);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump snd_
    jmp snd_

lift_minimax_2_:
    ; substitute (a0 !-> a0)(board !-> board)(trees !-> trees)(scores !-> scores)(p !-> p);
    ; #move variables
    mov rcx, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; switch p \{ ... \};
    lea rcx, [rel Player_46983]
    add rcx, r13
    jmp rcx

Player_46983:
    jmp near Player_46983_X
    jmp near Player_46983_O

Player_46983_X:
    ; substitute (scores !-> scores)(board !-> board)(trees !-> trees)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a2: _Cont = (board, trees, a0)\{ ... \};
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
    je lab46995
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab46996

lab46995:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab46993
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab46986
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46984
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46985

lab46984:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46985:

lab46986:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab46989
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46987
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46988

lab46987:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46988:

lab46989:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab46992
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab46990
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab46991

lab46990:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab46991:

lab46992:
    jmp lab46994

lab46993:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab46994:

lab46996:
    ; #load tag
    lea rdi, [rel _Cont_46997]
    ; jump listmax_
    jmp listmax_

_Cont_46997:

_Cont_46997_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47001
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab46998
    ; ####increment refcount
    add qword [r10 + 0], 1

lab46998:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab46999
    ; ####increment refcount
    add qword [r8 + 0], 1

lab46999:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47000
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47000:
    jmp lab47002

lab47001:
    ; ##... or release blocks onto linear free list when loading
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

lab47002:
    ; substitute (a0 !-> a0)(trees !-> trees)(board !-> board)(x6 !-> x6);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rax, r10
    ; let x5: Pair[List[Option[Player]], i64] = Tup(board, x6);
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
    je lab47014
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47015

lab47014:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47012
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47005
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47003
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47004

lab47003:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47004:

lab47005:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47008
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47006
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47007

lab47006:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47007:

lab47008:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47011
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47009
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47010

lab47009:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47010:

lab47011:
    jmp lab47013

lab47012:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47013:

lab47015:
    ; #load tag
    mov r9, 0
    ; substitute (x5 !-> x5)(trees !-> trees)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Rose
    jmp r9

Player_46983_O:
    ; substitute (scores !-> scores)(board !-> board)(trees !-> trees)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a3: _Cont = (board, trees, a0)\{ ... \};
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
    je lab47027
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47028

lab47027:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47025
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47018
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47016
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47017

lab47016:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47017:

lab47018:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47021
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47019
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47020

lab47019:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47020:

lab47021:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47024
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47022
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47023

lab47022:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47023:

lab47024:
    jmp lab47026

lab47025:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47026:

lab47028:
    ; #load tag
    lea rdi, [rel _Cont_47029]
    ; jump listmin_
    jmp listmin_

_Cont_47029:

_Cont_47029_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47033
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47030
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47030:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47031
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47031:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47032
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47032:
    jmp lab47034

lab47033:
    ; ##... or release blocks onto linear free list when loading
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

lab47034:
    ; substitute (a0 !-> a0)(trees !-> trees)(board !-> board)(x8 !-> x8);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rax, r10
    ; let x7: Pair[List[Option[Player]], i64] = Tup(board, x8);
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
    je lab47046
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47047

lab47046:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47044
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47037
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47035
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47036

lab47035:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47036:

lab47037:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47040
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47038
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47039

lab47038:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47039:

lab47040:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47042

lab47041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47042:

lab47043:
    jmp lab47045

lab47044:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47045:

lab47047:
    ; #load tag
    mov r9, 0
    ; substitute (x7 !-> x7)(trees !-> trees)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Rose
    jmp r9

main_loop_:
    ; let x0: Player = X();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; new a4: List[Option[Player]] = (iters, a0, x0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov [rbx + 48], r8
    mov [rbx + 40], rdi
    mov [rbx + 32], rsi
    mov [rbx + 24], rdx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab47059
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab47060

lab47059:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47057
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47050
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47048
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47049

lab47048:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47049:

lab47050:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47053
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47051
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47052

lab47051:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47052:

lab47053:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47056
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47054
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47055

lab47054:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47055:

lab47056:
    jmp lab47058

lab47057:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47058:

lab47060:
    ; #load tag
    lea rdx, [rel List_Option_Player_47061]
    ; jump empty_
    jmp empty_

List_Option_Player_47061:
    jmp near List_Option_Player_47061_Nil
    jmp near List_Option_Player_47061_Cons

List_Option_Player_47061_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47064
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47062
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47062:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47063
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47063:
    mov rdx, [rax + 24]
    jmp lab47065

lab47064:
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

lab47065:
    ; let x1: List[Option[Player]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(x0 !-> x0)(x1 !-> x1);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_Option_Player_47061_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47068
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47066
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47066:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47067
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47067:
    mov r9, [r8 + 24]
    jmp lab47069

lab47068:
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

lab47069:
    ; substitute (x0 !-> x0)(a0 !-> a0)(iters !-> iters)(a8 !-> a8)(as1 !-> as1);
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
    ; let x1: List[Option[Player]] = Cons(a8, as1);
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
    je lab47081
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47082

lab47081:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47079
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47072
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47070
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47071

lab47070:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47071:

lab47072:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47075
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47074

lab47073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47074:

lab47075:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47078
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47076
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47077

lab47076:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47077:

lab47078:
    jmp lab47080

lab47079:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47080:

lab47082:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(x0 !-> x0)(x1 !-> x1);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; substitute (x1 !-> x1)(x0 !-> x0)(iters !-> iters)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    ; new a5: RoseTree[Pair[List[Option[Player]], i64]] = (iters, a0)\{ ... \};
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
    je lab47094
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47095

lab47094:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47092
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47085
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47083
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47084

lab47083:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47084:

lab47085:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47088
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47086
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47087

lab47086:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47087:

lab47088:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47091
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47089
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47090

lab47089:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47090:

lab47091:
    jmp lab47093

lab47092:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47093:

lab47095:
    ; #load tag
    lea r9, [rel RoseTree_Pair_List_Option_Player_i64_47096]
    ; substitute (x0 !-> x0)(x1 !-> x1)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump minimax_
    jmp minimax_

RoseTree_Pair_List_Option_Player_i64_47096:

RoseTree_Pair_List_Option_Player_i64_47096_Rose:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47098
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab47097
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47097:
    mov r9, [r8 + 40]
    jmp lab47099

lab47098:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab47099:
    ; substitute (a0 !-> a0)(iters !-> iters)(a7 !-> a7)(as0 !-> as0);
    ; #move variables
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let res: RoseTree[Pair[List[Option[Player]], i64]] = Rose(a7, as0);
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
    je lab47111
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47112

lab47111:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47109
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47102
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47100
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47101

lab47100:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47101:

lab47102:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47105
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47103
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47104

lab47103:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47104:

lab47105:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47108
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47106
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47107

lab47106:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47107:

lab47108:
    jmp lab47110

lab47109:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47110:

lab47112:
    ; #load tag
    mov r9, 0
    ; lit x2 <- 1;
    mov r11, 1
    ; if iters == x2 \{ ... \}
    cmp rdi, r11
    je lab47113
    ; substitute (a0 !-> a0)(iters !-> iters);
    ; #erase res
    cmp r8, 0
    je lab47116
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab47114
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab47115

lab47114:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab47115:

lab47116:
    ; lit x5 <- 1;
    mov r9, 1
    ; x6 <- iters - x5;
    mov r11, rdi
    sub r11, r9
    ; substitute (x6 !-> x6)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; jump main_loop_
    jmp main_loop_

lab47113:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r8
    mov rdx, r9
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
    je lab47128
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47129

lab47128:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47126
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47119
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47117
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47118

lab47117:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47118:

lab47119:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47122
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47120
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47121

lab47120:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47121:

lab47122:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47125
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47123
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47124

lab47123:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47124:

lab47125:
    jmp lab47127

lab47126:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47127:

lab47129:
    ; #load tag
    lea rdi, [rel _Cont_47130]
    ; new a2: Pair[List[Option[Player]], i64] = (a1)\{ ... \};
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
    je lab47142
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47143

lab47142:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47140
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47133
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47131
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47132

lab47131:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47132:

lab47133:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47136
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47134
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47135

lab47134:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47135:

lab47136:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47139
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47137
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47138

lab47137:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47138:

lab47139:
    jmp lab47141

lab47140:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47141:

lab47143:
    ; #load tag
    lea rdi, [rel Pair_List_Option_Player_i64_47144]
    ; jump top_
    jmp top_

Pair_List_Option_Player_i64_47144:

Pair_List_Option_Player_i64_47144_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47146
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab47145
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47145:
    jmp lab47147

lab47146:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab47147:
    ; substitute (a1 !-> a1)(a6 !-> a6)(b0 !-> b0);
    ; #move variables
    mov rsi, rax
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x4: Pair[List[Option[Player]], i64] = Tup(a6, b0);
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
    je lab47159
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47160

lab47159:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47157
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47150
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47148
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47149

lab47148:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47149:

lab47150:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47153
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47151
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47152

lab47151:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47152:

lab47153:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47156
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47154
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47155

lab47154:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47155:

lab47156:
    jmp lab47158

lab47157:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47158:

lab47160:
    ; #load tag
    mov rdi, 0
    ; substitute (x4 !-> x4)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump snd_
    jmp snd_

_Cont_47130:

_Cont_47130_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47162
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab47161
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47161:
    jmp lab47163

lab47162:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab47163:
    ; println_i64 x3;
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
    ; lit x7 <- 0;
    mov rdi, 0
    ; substitute (x7 !-> x7)(a0 !-> a0);
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