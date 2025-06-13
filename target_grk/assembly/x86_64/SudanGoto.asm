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
    lea r13, [rel _Cont_55016]
    ; jump main_loop_
    jmp main_loop_

_Cont_55016:

_Cont_55016_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

sudan_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab55017
    ; if y == 0 \{ ... \}
    cmp r9, 0
    je lab55018
    ; substitute (n0 !-> n)(x !-> x)(y0 !-> y)(k !-> k)(a0 !-> a0)(n !-> n)(y !-> y);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], r9
    ; new a: _Cont = (k, a0, n, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab55030
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab55031

lab55030:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55028
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55021
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55019
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55020

lab55019:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55020:

lab55021:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55024
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55022
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55023

lab55022:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55023:

lab55024:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55027
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55025
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55026

lab55025:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55026:

lab55027:
    jmp lab55029

lab55028:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55029:

lab55031:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab55043
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab55044

lab55043:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55041
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55034
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55032
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55033

lab55032:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55033:

lab55034:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55037
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55035
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55036

lab55035:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55036:

lab55037:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55040
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55038
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55039

lab55038:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55039:

lab55040:
    jmp lab55042

lab55041:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55042:

lab55044:
    ; #load tag
    lea r11, [rel _Cont_55045]
    ; lit x0 <- 1;
    mov r13, 1
    ; x1 <- y0 - x0;
    mov r15, r9
    sub r15, r13
    ; substitute (n0 !-> n0)(x !-> x)(x1 !-> x1)(a !-> a)(a4 !-> a);
    ; #share a
    cmp r10, 0
    je lab55046
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55046:
    ; #move variables
    mov r12, r10
    mov r13, r11
    mov r9, r15
    ; jump sudan_
    jmp sudan_

_Cont_55045:

_Cont_55045_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55049
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab55047
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55047:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab55048
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55048:
    jmp lab55050

lab55049:
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
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab55050:
    ; lit x2 <- 1;
    mov r15, 1
    ; x3 <- n - x2;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (inner !-> inner)(k !-> k)(a0 !-> a0)(x3 !-> x3)(y !-> y);
    ; #move variables
    mov r11, [rsp + 2024]
    ; x4 <- inner + y;
    mov r15, rdx
    add r15, r13
    ; substitute (x3 !-> x3)(inner !-> inner)(x4 !-> x4)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, rsi
    mov r12, r8
    mov r13, r9
    mov r9, r15
    ; jump sudan_
    jmp sudan_

lab55018:
    ; substitute (x !-> x)(k !-> k);
    ; #erase a0
    cmp r12, 0
    je lab55053
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab55051
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab55052

lab55051:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab55052:

lab55053:
    ; #move variables
    mov rdx, rdi
    mov rsi, r10
    mov rdi, r11
    ; invoke k Ret
    jmp rdi

lab55017:
    ; substitute (k !-> k)(x !-> x)(y !-> y);
    ; #erase a0
    cmp r12, 0
    je lab55056
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab55054
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab55055

lab55054:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab55055:

lab55056:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; x5 <- x + y;
    mov r11, rdi
    add r11, r9
    ; substitute (x5 !-> x5)(k !-> k);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke k Ret
    jmp rdi

main_loop_:
    ; substitute (y0 !-> y)(n0 !-> n)(x4 !-> x)(y !-> y)(a0 !-> a0)(iters !-> iters)(n !-> n)(x !-> x);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a: _Cont = (y, a0, iters, n, x)\{ ... \};
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
    je lab55068
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab55069

lab55068:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55066
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55058

lab55057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55058:

lab55059:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55061

lab55060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55061:

lab55062:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55065
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55063
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55064

lab55063:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55064:

lab55065:
    jmp lab55067

lab55066:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55067:

lab55069:
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
    je lab55081
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab55082

lab55081:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55079
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55072
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55070
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55071

lab55070:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55071:

lab55072:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55075
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55073
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55074

lab55073:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55074:

lab55075:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55078
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55076
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55077

lab55076:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55077:

lab55078:
    jmp lab55080

lab55079:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55080:

lab55082:
    ; #load tag
    lea r11, [rel _Cont_55083]
    ; substitute (n0 !-> n0)(x4 !-> x4)(y0 !-> y0)(a !-> a)(a2 !-> a);
    ; #share a
    cmp r10, 0
    je lab55084
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55084:
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r12, r10
    mov r13, r11
    ; jump sudan_
    jmp sudan_

_Cont_55083:

_Cont_55083_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55086
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab55085
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55085:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab55087

lab55086:
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

lab55087:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp r11, [rsp +2024]
    je lab55088
    ; substitute (x !-> x)(y !-> y)(a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #move variables
    mov rdx, r15
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- iters - x1;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x2 !-> x2)(n !-> n)(x !-> x)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov r11, rdi
    mov rdi, r13
    mov r13, r9
    mov r9, rdx
    mov r12, r8
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab55088:
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