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
    lea r9, [rel _Cont_54973]
    ; jump main_loop_
    jmp main_loop_

_Cont_54973:

_Cont_54973_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

iterate_:
    ; if i == 0 \{ ... \}
    cmp rdx, 0
    je lab54974
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- i - x0;
    mov r15, rdx
    sub r15, r13
    ; substitute (a !-> a)(f0 !-> f)(f !-> f)(a0 !-> a0)(x1 !-> x1);
    ; #share f
    cmp rsi, 0
    je lab54975
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54975:
    ; #move variables
    mov r8, rsi
    mov rdx, r9
    mov r9, rdi
    mov r13, r15
    ; new a2: _Cont = (f, a0, x1)\{ ... \};
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
    je lab54987
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab54988

lab54987:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab54985
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab54978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54977

lab54976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54977:

lab54978:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab54981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54980

lab54979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54980:

lab54981:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab54984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54983

lab54982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54983:

lab54984:
    jmp lab54986

lab54985:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab54986:

lab54988:
    ; #load tag
    lea r9, [rel _Cont_54989]
    ; substitute (a !-> a)(a2 !-> a2)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_54989:

_Cont_54989_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab54992
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab54990
    ; ####increment refcount
    add qword [r8 + 0], 1

lab54990:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab54991
    ; ####increment refcount
    add qword [rsi + 0], 1

lab54991:
    jmp lab54993

lab54992:
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

lab54993:
    ; substitute (x1 !-> x1)(f !-> f)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, r8
    ; jump iterate_
    jmp iterate_

lab54974:
    ; substitute (a !-> a)(a0 !-> a0);
    ; #erase f
    cmp rsi, 0
    je lab54996
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab54994
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab54995

lab54994:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab54995:

lab54996:
    ; #move variables
    mov rdx, r9
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; new x0: Fun[i64, i64] = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel Fun_i64_i64_54997]
    ; lit x1 <- 0;
    mov r13, 0
    ; substitute (x1 !-> x1)(n0 !-> n)(x0 !-> x0)(a0 !-> a0)(n !-> n)(iters !-> iters);
    ; #move variables
    mov r15, rdx
    mov rdx, r13
    mov r13, rdi
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; new a3: _Cont = (a0, n, iters)\{ ... \};
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
    je lab55009
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab55010

lab55009:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55007
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab54998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab54999

lab54998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab54999:

lab55000:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55002

lab55001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55002:

lab55003:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55005

lab55004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55005:

lab55006:
    jmp lab55008

lab55007:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55008:

lab55010:
    ; #load tag
    lea r11, [rel _Cont_55011]
    ; substitute (n0 !-> n0)(x0 !-> x0)(x1 !-> x1)(a3 !-> a3);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rsi, r8
    ; jump iterate_
    jmp iterate_

_Cont_55011:

_Cont_55011_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55013
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab55012
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55012:
    jmp lab55014

lab55013:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab55014:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp r11, r13
    je lab55015
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov rdx, r11
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- iters - x3;
    mov r13, rdx
    sub r13, r11
    ; substitute (x4 !-> x4)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab55015:
    ; substitute (res !-> res)(a0 !-> a0);
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

Fun_i64_i64_54997:

Fun_i64_i64_54997_Apply:
    ; lit x5 <- 1;
    mov r9, 1
    ; x6 <- x + x5;
    mov r11, rdx
    add r11, r9
    ; substitute (x6 !-> x6)(a2 !-> a2);
    ; #move variables
    mov rdx, r11
    ; invoke a2 Ret
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