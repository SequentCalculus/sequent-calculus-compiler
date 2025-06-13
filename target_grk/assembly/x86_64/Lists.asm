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
    ; lit x1 <- 1;
    mov rdx, 1
    ; lit x2 <- 2;
    mov rdi, 2
    ; x3 <- x1 + x2;
    mov r9, rdx
    add r9, rdi
    ; substitute (x3 !-> x3);
    ; #move variables
    mov rdx, r9
    ; lit x4 <- 2;
    mov rdi, 2
    ; lit x5 <- 3;
    mov r9, 3
    ; lit x6 <- 4;
    mov r11, 4
    ; let x7: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; let x8: List[i64] = Cons(x6, x7);
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
    je lab517
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab518

lab517:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab515
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab508
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab506
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab507

lab506:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab507:

lab508:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab510

lab509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab510:

lab511:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab514
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab512
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab513

lab512:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab513:

lab514:
    jmp lab516

lab515:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab516:

lab518:
    ; #load tag
    mov r11, 5
    ; let x9: List[i64] = Cons(x5, x8);
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
    je lab530
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab531

lab530:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab528
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab521
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab519
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab520

lab519:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab520:

lab521:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab524
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab522
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab523

lab522:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab523:

lab524:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab527
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab525
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab526

lab525:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab526:

lab527:
    jmp lab529

lab528:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab529:

lab531:
    ; #load tag
    mov r9, 5
    ; let x10: List[i64] = Cons(x4, x9);
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
    je lab543
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab544

lab543:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab541
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab534
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab532
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab533

lab532:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab533:

lab534:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab537
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab535
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab536

lab535:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab536:

lab537:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab540
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab538
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab539

lab538:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab539:

lab540:
    jmp lab542

lab541:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab542:

lab544:
    ; #load tag
    mov rdi, 5
    ; let l: List[i64] = Cons(x3, x10);
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
    je lab556
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab557

lab556:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab554
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab547
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab545
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab546

lab545:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab546:

lab547:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab550
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab548
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab549

lab548:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab549:

lab550:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab553
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab551
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab552

lab551:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab552:

lab553:
    jmp lab555

lab554:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab555:

lab557:
    ; #load tag
    mov rdx, 5
    ; lit x11 <- 0;
    mov rdi, 0
    ; substitute (l2 !-> l)(x11 !-> x11)(l !-> l);
    ; #share l
    cmp rax, 0
    je lab558
    ; ####increment refcount
    add qword [rax + 0], 1

lab558:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    ; create a2: _Cont = (x11, l)\{ ... \};
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
    je lab570
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab571

lab570:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab568
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab561
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab559
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab560

lab559:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab560:

lab561:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab564
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab562
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab563

lab562:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab563:

lab564:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab567
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab565
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab566

lab565:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab566:

lab567:
    jmp lab569

lab568:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab569:

lab571:
    ; #load tag
    lea rdi, [rel _Cont_572]
    ; jump len_
    jmp len_

_Cont_572:

_Cont_572_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab574
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab573
    ; ####increment refcount
    add qword [r8 + 0], 1

lab573:
    mov rdi, [rsi + 40]
    jmp lab575

lab574:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab575:
    ; if x11 < x12 \{ ... \}
    cmp rdi, rdx
    jl lab576
    ; substitute (l !-> l);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; lit x <- 0;
    mov rdi, 0
    ; jump share_main_1_
    jmp share_main_1_

lab576:
    ; substitute (l0 !-> l)(l !-> l);
    ; #share l
    cmp r8, 0
    je lab577
    ; ####increment refcount
    add qword [r8 + 0], 1

lab577:
    ; #move variables
    mov rax, r8
    mov rsi, r8
    mov rdx, r9
    mov rdi, r9
    ; create a3: _Cont = (l)\{ ... \};
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
    je lab589
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab590

lab589:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab587
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab580
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab578
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab579

lab578:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab579:

lab580:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab583
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab581
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab582

lab581:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab582:

lab583:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab586
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab584
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab585

lab584:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab585:

lab586:
    jmp lab588

lab587:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab588:

lab590:
    ; #load tag
    lea rdi, [rel _Cont_591]
    ; jump len_
    jmp len_

_Cont_591:

_Cont_591_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab593
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab592
    ; ####increment refcount
    add qword [rsi + 0], 1

lab592:
    jmp lab594

lab593:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab594:
    ; lit x14 <- 1;
    mov r9, 1
    ; x <- x13 + x14;
    mov r11, rdx
    add r11, r9
    ; substitute (l !-> l)(x !-> x);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rdi, r11
    ; jump share_main_1_
    jmp share_main_1_

share_main_1_:
    ; substitute (x !-> x)(l !-> l);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_595]
    add rcx, rdi
    jmp rcx

List_i64_595:
    jmp near List_i64_595_Nil
    jmp near List_i64_595_Cons

List_i64_595_Nil:
    ; substitute ;
    ; let l1: List[i64] = Nil();
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    mov rdx, 0
    ; jump share_main_0_
    jmp share_main_0_

List_i64_595_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab597
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab596
    ; ####increment refcount
    add qword [r8 + 0], 1

lab596:
    mov rdi, [rsi + 40]
    jmp lab598

lab597:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab598:
    ; substitute (zs !-> zs)(z !-> z)(x !-> x);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; create x1: Fun[i64, i64] = (z, x)\{ ... \};
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
    je lab610
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab611

lab610:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab608
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab600

lab599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab600:

lab601:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab603

lab602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab603:

lab604:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab606

lab605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab606:

lab607:
    jmp lab609

lab608:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab609:

lab611:
    ; #load tag
    lea rdi, [rel Fun_i64_i64_612]
    ; create a2: List[i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel List_i64_613]
    ; substitute (x1 !-> x1)(zs !-> zs)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_
    jmp map_

List_i64_613:
    jmp near List_i64_613_Nil
    jmp near List_i64_613_Cons

List_i64_613_Nil:
    ; let l1: List[i64] = Nil();
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    mov rdx, 0
    ; jump share_main_0_
    jmp share_main_0_

List_i64_613_Cons:
    ; let l1: List[i64] = Cons(x4, xs0);
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
    je lab625
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab626

lab625:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab623
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab616
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab614
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab615

lab614:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab615:

lab616:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab619
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab617
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab618

lab617:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab618:

lab619:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab622
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab620
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab621

lab620:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab621:

lab622:
    jmp lab624

lab623:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab624:

lab626:
    ; #load tag
    mov rdx, 5
    ; jump share_main_0_
    jmp share_main_0_

Fun_i64_i64_612:

Fun_i64_i64_612_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab627
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    jmp lab628

lab627:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]

lab628:
    ; x2 <- x + n;
    mov r13, r11
    add r13, rdx
    ; substitute (x2 !-> x2)(a1 !-> a1)(z !-> z);
    ; #move variables
    mov rdx, r13
    ; x3 <- x2 - z;
    mov r11, rdx
    sub r11, r9
    ; substitute (x3 !-> x3)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; invoke a1 Ret
    jmp rdi

share_main_0_:
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel _Cont_629]
    ; jump mult_
    jmp mult_

_Cont_629:

_Cont_629_Ret:
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
    lea rcx, [rel List_i64_630]
    add rcx, r9
    jmp rcx

List_i64_630:
    jmp near List_i64_630_Nil
    jmp near List_i64_630_Cons

List_i64_630_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab633
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab631
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab632

lab631:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab632:

lab633:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_630_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab635
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab634
    ; ####increment refcount
    add qword [r10 + 0], 1

lab634:
    mov r9, [r8 + 40]
    jmp lab636

lab635:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab636:
    ; substitute (f0 !-> f)(x !-> x)(a0 !-> a0)(xs !-> xs)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab637
    ; ####increment refcount
    add qword [rax + 0], 1

lab637:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; create a1: _Cont = (a0, xs, f)\{ ... \};
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
    je lab649
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab650

lab649:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab647
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab640
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab638
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab639

lab638:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab639:

lab640:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab642

lab641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab642:

lab643:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab645

lab644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab645:

lab646:
    jmp lab648

lab647:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab648:

lab650:
    ; #load tag
    lea r9, [rel _Cont_651]
    ; substitute (x !-> x)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_651:

_Cont_651_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab655
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab652
    ; ####increment refcount
    add qword [r10 + 0], 1

lab652:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab653
    ; ####increment refcount
    add qword [r8 + 0], 1

lab653:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab654
    ; ####increment refcount
    add qword [rsi + 0], 1

lab654:
    jmp lab656

lab655:
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

lab656:
    ; substitute (f !-> f)(xs !-> xs)(a0 !-> a0)(x0 !-> x0);
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
    ; create a2: List[i64] = (a0, x0)\{ ... \};
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
    je lab668
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab669

lab668:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab666
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab659
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab657
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab658

lab657:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab658:

lab659:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab661

lab660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab661:

lab662:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab665
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab663
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab664

lab663:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab664:

lab665:
    jmp lab667

lab666:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab667:

lab669:
    ; #load tag
    lea r9, [rel List_i64_670]
    ; jump map_
    jmp map_

List_i64_670:
    jmp near List_i64_670_Nil
    jmp near List_i64_670_Cons

List_i64_670_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab672
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab671
    ; ####increment refcount
    add qword [rax + 0], 1

lab671:
    jmp lab673

lab672:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab673:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
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

List_i64_670_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab675
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab674
    ; ####increment refcount
    add qword [r8 + 0], 1

lab674:
    jmp lab676

lab675:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab676:
    ; substitute (x0 !-> x0)(a0 !-> a0)(x2 !-> x2)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x1: List[i64] = Cons(x2, xs0);
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
    je lab688
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab689

lab688:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab686
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab679
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab677
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab678

lab677:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab678:

lab679:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab681

lab680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab681:

lab682:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab684

lab683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab684:

lab685:
    jmp lab687

lab686:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab687:

lab689:
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

len_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_690]
    add rcx, rdi
    jmp rcx

List_i64_690:
    jmp near List_i64_690_Nil
    jmp near List_i64_690_Cons

List_i64_690_Nil:
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

List_i64_690_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab692
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab691
    ; ####increment refcount
    add qword [r8 + 0], 1

lab691:
    mov rdi, [rsi + 40]
    jmp lab693

lab692:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab693:
    ; substitute (a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; lit x0 <- 1;
    mov r9, 1
    ; substitute (xs !-> xs)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; create a1: _Cont = (a0, x0)\{ ... \};
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
    je lab705
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab706

lab705:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab703
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab695

lab694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab695:

lab696:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab698

lab697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab698:

lab699:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab701

lab700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab701:

lab702:
    jmp lab704

lab703:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab704:

lab706:
    ; #load tag
    lea rdi, [rel _Cont_707]
    ; jump len_
    jmp len_

_Cont_707:

_Cont_707_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab709
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab708
    ; ####increment refcount
    add qword [rsi + 0], 1

lab708:
    jmp lab710

lab709:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab710:
    ; x3 <- x0 + x1;
    mov r11, r9
    add r11, rdx
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

foldr_:
    ; substitute (f !-> f)(st !-> st)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_711]
    add rcx, r11
    jmp rcx

List_i64_711:
    jmp near List_i64_711_Nil
    jmp near List_i64_711_Cons

List_i64_711_Nil:
    ; substitute (st !-> st)(a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab714
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab712
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab713

lab712:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab713:

lab714:
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

List_i64_711_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab716
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab715
    ; ####increment refcount
    add qword [r12 + 0], 1

lab715:
    mov r11, [r10 + 40]
    jmp lab717

lab716:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab717:
    ; substitute (f0 !-> f)(st !-> st)(ys !-> ys)(y !-> y)(a0 !-> a0)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab718
    ; ####increment refcount
    add qword [rax + 0], 1

lab718:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rcx, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; create a1: _Cont = (y, a0, f)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
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
    je lab730
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab731

lab730:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab728
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab721
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab719
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab720

lab719:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab720:

lab721:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab724
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab722
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab723

lab722:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab723:

lab724:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab727
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab725
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab726

lab725:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab726:

lab727:
    jmp lab729

lab728:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab729:

lab731:
    ; #load tag
    lea r11, [rel _Cont_732]
    ; jump foldr_
    jmp foldr_

_Cont_732:

_Cont_732_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab735
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab733
    ; ####increment refcount
    add qword [r10 + 0], 1

lab733:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab734
    ; ####increment refcount
    add qword [r8 + 0], 1

lab734:
    mov rdi, [rsi + 24]
    jmp lab736

lab735:
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

lab736:
    ; substitute (y !-> y)(x0 !-> x0)(a0 !-> a0)(f !-> f);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke f Apply2
    jmp r11

mult_:
    ; create x0: Fun2[i64, i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun2_i64_i64_i64_737]
    ; lit x1 <- 1;
    mov r11, 1
    ; substitute (x0 !-> x0)(x1 !-> x1)(l !-> l)(a0 !-> a0);
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
    ; jump foldr_
    jmp foldr_

Fun2_i64_i64_i64_737:

Fun2_i64_i64_i64_737_Apply2:
    ; x2 <- x * y;
    mov r11, rdx
    imul r11, rdi
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a1 Ret
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