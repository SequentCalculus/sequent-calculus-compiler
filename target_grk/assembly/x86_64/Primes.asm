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
    lea r9, [rel _Cont_62475]
    ; jump main_loop_
    jmp main_loop_

_Cont_62475:

_Cont_62475_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

interval_list_:
    ; if n < m \{ ... \}
    cmp rdi, rdx
    jl lab62476
    ; substitute (m0 !-> m)(n !-> n)(a0 !-> a0)(m !-> m);
    ; #move variables
    mov r11, rdx
    ; new a1: List[i64] = (a0, m)\{ ... \};
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
    je lab62488
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62489

lab62488:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62486
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62479
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62477
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62478

lab62477:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62478:

lab62479:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62481

lab62480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62481:

lab62482:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62484

lab62483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62484:

lab62485:
    jmp lab62487

lab62486:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62487:

lab62489:
    ; #load tag
    lea r9, [rel List_i64_62490]
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- m0 + x1;
    mov r13, rdx
    add r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump interval_list_
    jmp interval_list_

List_i64_62490:
    jmp near List_i64_62490_Nil
    jmp near List_i64_62490_Cons

List_i64_62490_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62492
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab62491
    ; ####increment refcount
    add qword [rax + 0], 1

lab62491:
    jmp lab62493

lab62492:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab62493:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (m !-> m)(x0 !-> x0)(a0 !-> a0);
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

List_i64_62490_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62495
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab62494
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62494:
    jmp lab62496

lab62495:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab62496:
    ; substitute (m !-> m)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
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
    je lab62508
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62509

lab62508:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62506
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62499
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62497
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62498

lab62497:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62498:

lab62499:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62502
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62500
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62501

lab62500:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62501:

lab62502:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62505
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62503
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62504

lab62503:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62504:

lab62505:
    jmp lab62507

lab62506:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62507:

lab62509:
    ; #load tag
    mov r9, 5
    ; substitute (m !-> m)(x0 !-> x0)(a0 !-> a0);
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

lab62476:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

remove_multiples_:
    ; substitute (n !-> n)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_62510]
    add rcx, r9
    jmp rcx

List_i64_62510:
    jmp near List_i64_62510_Nil
    jmp near List_i64_62510_Cons

List_i64_62510_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_62510_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62512
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab62511
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62511:
    mov r9, [r8 + 40]
    jmp lab62513

lab62512:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab62513:
    ; x0 <- x % n;
    mov rcx, rdx
    mov r13, rax
    mov rax, r9
    cqo
    idiv rcx
    mov rax, r13
    mov r13, rdx
    mov rdx, rcx
    ; if x0 == 0 \{ ... \}
    cmp r13, 0
    je lab62514
    ; substitute (n !-> n)(xs !-> xs)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a2: List[i64] = (x, a0)\{ ... \};
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
    je lab62526
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62527

lab62526:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62524
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62517
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62515
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62516

lab62515:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62516:

lab62517:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62520
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62518
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62519

lab62518:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62519:

lab62520:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62523
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62521
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62522

lab62521:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62522:

lab62523:
    jmp lab62525

lab62524:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62525:

lab62527:
    ; #load tag
    lea r9, [rel List_i64_62528]
    ; jump remove_multiples_
    jmp remove_multiples_

List_i64_62528:
    jmp near List_i64_62528_Nil
    jmp near List_i64_62528_Cons

List_i64_62528_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62530
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab62529
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62529:
    mov rdx, [rax + 40]
    jmp lab62531

lab62530:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab62531:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x !-> x)(x1 !-> x1)(a0 !-> a0);
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

List_i64_62528_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62533
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab62532
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62532:
    mov r9, [r8 + 40]
    jmp lab62534

lab62533:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab62534:
    ; substitute (a0 !-> a0)(x !-> x)(a3 !-> a3)(as0 !-> as0);
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
    je lab62546
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62547

lab62546:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62544
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62537
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62535
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62536

lab62535:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62536:

lab62537:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62540
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62538
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62539

lab62538:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62539:

lab62540:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62543
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62541
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62542

lab62541:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62542:

lab62543:
    jmp lab62545

lab62544:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62545:

lab62547:
    ; #load tag
    mov r9, 5
    ; substitute (x !-> x)(x1 !-> x1)(a0 !-> a0);
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

lab62514:
    ; substitute (n !-> n)(xs !-> xs)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    mov rsi, r10
    mov rdi, r11
    ; jump remove_multiples_
    jmp remove_multiples_

sieve_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_62548]
    add rcx, rdi
    jmp rcx

List_i64_62548:
    jmp near List_i64_62548_Nil
    jmp near List_i64_62548_Cons

List_i64_62548_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_62548_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62550
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab62549
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62549:
    mov rdi, [rsi + 40]
    jmp lab62551

lab62550:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab62551:
    ; substitute (xs !-> xs)(x2 !-> x)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rdx, r9
    mov r9, rdi
    mov rax, r8
    ; new a1: List[i64] = (x, a0)\{ ... \};
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
    je lab62563
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62564

lab62563:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62561
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62554
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62552
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62553

lab62552:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62553:

lab62554:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62557
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62555
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62556

lab62555:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62556:

lab62557:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62560
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62558
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62559

lab62558:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62559:

lab62560:
    jmp lab62562

lab62561:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62562:

lab62564:
    ; #load tag
    lea r9, [rel List_i64_62565]
    ; new a2: List[i64] = (a1)\{ ... \};
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
    je lab62577
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62578

lab62577:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62575
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62568
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62566
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62567

lab62566:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62567:

lab62568:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62571
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62569
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62570

lab62569:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62570:

lab62571:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62574
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62572
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62573

lab62572:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62573:

lab62574:
    jmp lab62576

lab62575:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62576:

lab62578:
    ; #load tag
    lea r9, [rel List_i64_62579]
    ; substitute (x2 !-> x2)(xs !-> xs)(a2 !-> a2);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump remove_multiples_
    jmp remove_multiples_

List_i64_62579:
    jmp near List_i64_62579_Nil
    jmp near List_i64_62579_Cons

List_i64_62579_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62581
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab62580
    ; ####increment refcount
    add qword [rax + 0], 1

lab62580:
    jmp lab62582

lab62581:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab62582:
    ; let x1: List[i64] = Nil();
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
    ; jump sieve_
    jmp sieve_

List_i64_62579_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62584
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab62583
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62583:
    jmp lab62585

lab62584:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab62585:
    ; substitute (a1 !-> a1)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let x1: List[i64] = Cons(a4, as1);
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
    je lab62597
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62598

lab62597:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62595
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62588
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62586
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62587

lab62586:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62587:

lab62588:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62591
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62589
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62590

lab62589:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62590:

lab62591:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62593

lab62592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62593:

lab62594:
    jmp lab62596

lab62595:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62596:

lab62598:
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
    ; jump sieve_
    jmp sieve_

List_i64_62565:
    jmp near List_i64_62565_Nil
    jmp near List_i64_62565_Cons

List_i64_62565_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62600
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab62599
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62599:
    mov rdx, [rax + 40]
    jmp lab62601

lab62600:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab62601:
    ; let x0: List[i64] = Nil();
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

List_i64_62565_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62603
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab62602
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62602:
    mov r9, [r8 + 40]
    jmp lab62604

lab62603:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab62604:
    ; substitute (a0 !-> a0)(x !-> x)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
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
    je lab62616
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab62617

lab62616:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62614
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62607
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62605
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62606

lab62605:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62606:

lab62607:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62610
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62608
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62609

lab62608:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62609:

lab62610:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62613
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62611
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62612

lab62611:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62612:

lab62613:
    jmp lab62615

lab62614:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62615:

lab62617:
    ; #load tag
    mov r9, 5
    ; substitute (x !-> x)(x0 !-> x0)(a0 !-> a0);
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
    lea rcx, [rel List_i64_62618]
    add rcx, r9
    jmp rcx

List_i64_62618:
    jmp near List_i64_62618_Nil
    jmp near List_i64_62618_Cons

List_i64_62618_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_62618_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62620
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab62619
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62619:
    mov r9, [r8 + 40]
    jmp lab62621

lab62620:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab62621:
    ; substitute (a0 !-> a0)(acc !-> acc)(ps !-> ps);
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

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a3: List[i64] = (n, a0, iters)\{ ... \};
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
    je lab62633
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62634

lab62633:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62631
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62624
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62622
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62623

lab62622:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62623:

lab62624:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62627
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62625
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62626

lab62625:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62626:

lab62627:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62629

lab62628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62629:

lab62630:
    jmp lab62632

lab62631:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62632:

lab62634:
    ; #load tag
    lea rdi, [rel List_i64_62635]
    ; lit x1 <- 2;
    mov r9, 2
    ; substitute (x1 !-> x1)(n0 !-> n0)(a3 !-> a3);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump interval_list_
    jmp interval_list_

List_i64_62635:
    jmp near List_i64_62635_Nil
    jmp near List_i64_62635_Cons

List_i64_62635_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62637
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab62636
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62636:
    mov rdx, [rax + 24]
    jmp lab62638

lab62637:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab62638:
    ; let x0: List[i64] = Nil();
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

List_i64_62635_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62640
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab62639
    ; ####increment refcount
    add qword [r10 + 0], 1

lab62639:
    mov r9, [r8 + 24]
    jmp lab62641

lab62640:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab62641:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(a6 !-> a6)(as1 !-> as1);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let x0: List[i64] = Cons(a6, as1);
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
    je lab62653
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab62654

lab62653:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62651
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62644
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62642
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62643

lab62642:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62643:

lab62644:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62647
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62645
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62646

lab62645:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62646:

lab62647:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62650
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62648
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62649

lab62648:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62649:

lab62650:
    jmp lab62652

lab62651:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62652:

lab62654:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_0_:
    ; substitute (x0 !-> x0)(iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a4: List[i64] = (iters, n, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab62666
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62667

lab62666:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62664
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62657
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62655
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62656

lab62655:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62656:

lab62657:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62660
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62658
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62659

lab62658:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62659:

lab62660:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62663
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62661
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62662

lab62661:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62662:

lab62663:
    jmp lab62665

lab62664:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62665:

lab62667:
    ; #load tag
    lea rdi, [rel List_i64_62668]
    ; jump sieve_
    jmp sieve_

List_i64_62668:
    jmp near List_i64_62668_Nil
    jmp near List_i64_62668_Cons

List_i64_62668_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab62670
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab62669
    ; ####increment refcount
    add qword [r8 + 0], 1

lab62669:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab62671

lab62670:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab62671:
    ; let x: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x !-> x);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_i64_62668_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab62673
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab62672
    ; ####increment refcount
    add qword [r12 + 0], 1

lab62672:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab62674

lab62673:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab62674:
    ; substitute (a0 !-> a0)(n !-> n)(iters !-> iters)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    ; let x: List[i64] = Cons(a5, as0);
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
    je lab62686
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab62687

lab62686:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62684
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62677
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62675
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62676

lab62675:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62676:

lab62677:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62680
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62678
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62679

lab62678:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62679:

lab62680:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62683
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62681
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62682

lab62681:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62682:

lab62683:
    jmp lab62685

lab62684:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62685:

lab62687:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x !-> x);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lift_main_loop_1_:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp rdi, r13
    je lab62688
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase x
    cmp r10, 0
    je lab62691
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab62689
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab62690

lab62689:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab62690:

lab62691:
    ; lit x4 <- 1;
    mov r11, 1
    ; x5 <- iters - x4;
    mov r13, rdi
    sub r13, r11
    ; substitute (x5 !-> x5)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab62688:
    ; substitute (x !-> x)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rax, r10
    mov rdx, r11
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
    je lab62703
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab62704

lab62703:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab62701
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab62694
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62692
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62693

lab62692:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62693:

lab62694:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab62697
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62695
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62696

lab62695:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62696:

lab62697:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab62700
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab62698
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62699

lab62698:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62699:

lab62700:
    jmp lab62702

lab62701:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab62702:

lab62704:
    ; #load tag
    lea rdi, [rel _Cont_62705]
    ; jump len_
    jmp len_

_Cont_62705:

_Cont_62705_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab62707
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab62706
    ; ####increment refcount
    add qword [rsi + 0], 1

lab62706:
    jmp lab62708

lab62707:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab62708:
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
    ; lit x6 <- 0;
    mov rdi, 0
    ; substitute (x6 !-> x6)(a0 !-> a0);
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