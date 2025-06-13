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
    lea r9, [rel _Cont_44560]
    ; jump main_loop_
    jmp main_loop_

_Cont_44560:

_Cont_44560_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

attempt_:
    ; if i == 0 \{ ... \}
    cmp rdx, 0
    je lab44561
    ; lit x0 <- 1;
    mov r9, 1
    ; x1 <- i - x0;
    mov r11, rdx
    sub r11, r9
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; new a3: Option[i64] = (a0)\{ ... \};
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
    je lab44573
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44574

lab44573:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44571
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44564
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44562
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44563

lab44562:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44563:

lab44564:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44567
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44565
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44566

lab44565:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44566:

lab44567:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44570
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44568
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44569

lab44568:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44569:

lab44570:
    jmp lab44572

lab44571:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44572:

lab44574:
    ; #load tag
    lea rdi, [rel Option_i64_44575]
    ; jump attempt_
    jmp attempt_

Option_i64_44575:
    jmp near Option_i64_44575_None
    jmp near Option_i64_44575_Some

Option_i64_44575_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44577
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab44576
    ; ####increment refcount
    add qword [rax + 0], 1

lab44576:
    jmp lab44578

lab44577:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab44578:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

Option_i64_44575_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44580
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab44579
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44579:
    jmp lab44581

lab44580:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab44581:
    ; lit x2 <- 1;
    mov r9, 1
    ; x3 <- x + x2;
    mov r11, rdx
    add r11, r9
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

lab44561:
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: Option[i64] = (n, a0, iters)\{ ... \};
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
    je lab44593
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab44594

lab44593:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab44591
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab44584
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44582
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44583

lab44582:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44583:

lab44584:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab44587
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44585
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44586

lab44585:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44586:

lab44587:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab44590
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab44588
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab44589

lab44588:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab44589:

lab44590:
    jmp lab44592

lab44591:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab44592:

lab44594:
    ; #load tag
    lea rdi, [rel Option_i64_44595]
    ; jump attempt_
    jmp attempt_

Option_i64_44595:
    jmp near Option_i64_44595_None
    jmp near Option_i64_44595_Some

Option_i64_44595_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab44597
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab44596
    ; ####increment refcount
    add qword [rsi + 0], 1

lab44596:
    mov rdx, [rax + 24]
    jmp lab44598

lab44597:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab44598:
    ; lit res <- -1;
    mov r11, -1
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(res !-> res);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump share_main_loop_0_
    jmp share_main_loop_0_

Option_i64_44595_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab44600
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab44599
    ; ####increment refcount
    add qword [r8 + 0], 1

lab44599:
    mov rdi, [rsi + 24]
    jmp lab44601

lab44600:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab44601:
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x !-> x);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump share_main_loop_0_
    jmp share_main_loop_0_

share_main_loop_0_:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp rdi, r13
    je lab44602
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- iters - x1;
    mov r13, rdi
    sub r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab44602:
    ; substitute (a0 !-> a0)(res !-> res);
    ; #move variables
    mov rdi, r11
    ; println_i64 res;
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