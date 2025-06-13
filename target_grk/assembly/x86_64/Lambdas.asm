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
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    lea rdx, [rel _Cont_829]
    ; jump higherOrder_
    jmp higherOrder_

_Cont_829:

_Cont_829_Ret:
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

nonValueArguments_:
    ; lit x0 <- 1;
    mov rdi, 1
    ; lit x1 <- 2;
    mov r9, 2
    ; x2 <- x0 + x1;
    mov r11, rdi
    add r11, r9
    ; substitute (a0 !-> a0);
    ; lit x3 <- 3;
    mov rdi, 3
    ; lit x4 <- 4;
    mov r9, 4
    ; x5 <- x3 + x4;
    mov r11, rdi
    add r11, r9
    ; substitute (x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; let a3: Fun[i64, i64] = Apply(x5, a0);
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
    je lab841
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab842

lab841:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab839
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab832
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab830
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab831

lab830:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab831:

lab832:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab835
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab833
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab834

lab833:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab834:

lab835:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab838
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab836
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab837

lab836:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab837:

lab838:
    jmp lab840

lab839:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab840:

lab842:
    ; #load tag
    mov rdx, 0
    ; switch a3 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_843:

Fun_i64_i64_843_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab845
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab844
    ; ####increment refcount
    add qword [rsi + 0], 1

lab844:
    mov rdx, [rax + 40]
    jmp lab846

lab845:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab846:
    ; invoke a2 Ret
    jmp rdi

higherOrder_:
    ; create x0: Fun[i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel Fun_i64_i64_847]
    ; lit x1 <- 3;
    mov r9, 3
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- x1 + x2;
    mov r13, r9
    add r13, r11
    ; substitute (x0 !-> x0)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov r9, rdx
    mov rax, rsi
    mov rdx, rdi
    mov rdi, r13
    ; let a4: Fun[i64, i64] = Apply(x3, a0);
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
    je lab859
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab860

lab859:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab857
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab850
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab848
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab849

lab848:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab849:

lab850:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab853
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab851
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab852

lab851:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab852:

lab853:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab856
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab854
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab855

lab854:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab855:

lab856:
    jmp lab858

lab857:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab858:

lab860:
    ; #load tag
    mov rdi, 0
    ; switch a4 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_861:

Fun_i64_i64_861_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab863
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab862
    ; ####increment refcount
    add qword [r8 + 0], 1

lab862:
    mov rdi, [rsi + 40]
    jmp lab864

lab863:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab864:
    ; substitute (y !-> y)(a3 !-> a3)(x0 !-> x0);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke x0 Apply
    jmp r9

Fun_i64_i64_847:

Fun_i64_i64_847_Apply:
    ; lit x4 <- 4;
    mov r9, 4
    ; x5 <- x4 + z;
    mov r11, r9
    add r11, rdx
    ; substitute (x5 !-> x5)(a1 !-> a1);
    ; #move variables
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