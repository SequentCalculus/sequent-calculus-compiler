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
    mov r9, rcx
    ; move parameters into place
    mov rdi, rdx
    ; move parameters into place
    mov rdx, rsi
    ; actual code

main_:
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel _Cont_970]
    ; jump main_loop_
    jmp main_loop_

_Cont_970:

_Cont_970_Ret:
    ; exit x0
    mov rax, rdx
    jmp cleanup

ack_:
    ; if m == 0 \{ ... \}
    cmp rdx, 0
    je lab971
    ; if n == 0 \{ ... \}
    cmp rdi, 0
    je lab972
    ; lit x4 <- 1;
    mov r13, 1
    ; x5 <- m - x4;
    mov r15, rdx
    sub r15, r13
    ; substitute (m !-> m)(n !-> n)(k !-> k)(a0 !-> a0)(x5 !-> x5);
    ; #move variables
    mov r13, r15
    ; create a: _Cont = (k, a0, x5)\{ ... \};
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
    je lab984
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab985

lab984:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab982
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab975
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab973
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab974

lab973:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab974:

lab975:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab977

lab976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab977:

lab978:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab980

lab979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab980:

lab981:
    jmp lab983

lab982:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab983:

lab985:
    ; #load tag
    lea r9, [rel _Cont_986]
    ; lit x7 <- 1;
    mov r11, 1
    ; x8 <- n - x7;
    mov r13, rdi
    sub r13, r11
    ; substitute (m !-> m)(x8 !-> x8)(a !-> a)(a1 !-> a);
    ; #share a
    cmp r8, 0
    je lab987
    ; ####increment refcount
    add qword [r8 + 0], 1

lab987:
    ; #move variables
    mov r10, r8
    mov r11, r9
    mov rdi, r13
    ; jump ack_
    jmp ack_

_Cont_986:

_Cont_986_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab990
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab988
    ; ####increment refcount
    add qword [r8 + 0], 1

lab988:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab989
    ; ####increment refcount
    add qword [rsi + 0], 1

lab989:
    jmp lab991

lab990:
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

lab991:
    ; substitute (x5 !-> x5)(x6 !-> x6)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, r8
    mov r8, rsi
    ; jump ack_
    jmp ack_

lab972:
    ; substitute (m !-> m)(a0 !-> a0)(k !-> k);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- m - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(a0 !-> a0)(k !-> k);
    ; #move variables
    mov rdx, r13
    ; lit x3 <- 1;
    mov r11, 1
    ; substitute (x2 !-> x2)(x3 !-> x3)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump ack_
    jmp ack_

lab971:
    ; substitute (k !-> k)(n !-> n);
    ; #erase a0
    cmp r10, 0
    je lab994
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab992
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab993

lab992:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab993:

lab994:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; lit x0 <- 1;
    mov r9, 1
    ; x9 <- n + x0;
    mov r11, rdi
    add r11, r9
    ; substitute (x9 !-> x9)(k !-> k);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke k Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(m0 !-> m)(n !-> n)(a0 !-> a0)(iters !-> iters)(m !-> m);
    ; #move variables
    mov r13, rdx
    mov r15, rdi
    mov rdx, r9
    ; create a: _Cont = (n, a0, iters, m)\{ ... \};
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
    je lab1006
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab1007

lab1006:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab1004
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab997
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab995
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab996

lab995:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab996:

lab997:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab1000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab999

lab998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab999:

lab1000:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab1003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab1001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab1002

lab1001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab1002:

lab1003:
    jmp lab1005

lab1004:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab1005:

lab1007:
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
    je lab1019
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab1020

lab1019:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab1017
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab1010
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab1008
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab1009

lab1008:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab1009:

lab1010:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab1013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab1011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab1012

lab1011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab1012:

lab1013:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab1016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab1014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab1015

lab1014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab1015:

lab1016:
    jmp lab1018

lab1017:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab1018:

lab1020:
    ; #load tag
    lea r9, [rel _Cont_1021]
    ; substitute (m0 !-> m0)(n0 !-> n0)(a !-> a)(a1 !-> a);
    ; #share a
    cmp r8, 0
    je lab1022
    ; ####increment refcount
    add qword [r8 + 0], 1

lab1022:
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, r8
    mov r11, r9
    ; jump ack_
    jmp ack_

_Cont_1021:

_Cont_1021_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab1024
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab1023
    ; ####increment refcount
    add qword [r8 + 0], 1

lab1023:
    jmp lab1025

lab1024:
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
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab1025:
    ; lit x0 <- 1;
    mov r15, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r15
    je lab1026
    ; substitute (m !-> m)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov rdx, r13
    ; lit x1 <- 1;
    mov r13, 1
    ; x2 <- iters - x1;
    mov r15, r11
    sub r15, r13
    ; substitute (x2 !-> x2)(m !-> m)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov r10, r8
    mov rdx, r15
    ; jump main_loop_
    jmp main_loop_

lab1026:
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