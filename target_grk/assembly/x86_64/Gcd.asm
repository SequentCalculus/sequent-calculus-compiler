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
    lea r9, [rel _Cont_50058]
    ; jump main_loop_
    jmp main_loop_

_Cont_50058:

_Cont_50058_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

quot_rem_:
    ; x0 <- a / b;
    mov rcx, rdx
    mov r11, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rdx, rax
    mov rax, r11
    mov r11, rdx
    mov rdx, rcx
    ; x1 <- a % b;
    mov rcx, rdx
    mov r13, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rax, r13
    mov r13, rdx
    mov rdx, rcx
    ; substitute (x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    mov rdi, r13
    ; invoke a0 Pair
    jmp r9

g_:
    ; substitute (a0 !-> a0)(v1v2v3 !-> v1v2v3)(u1u2u3 !-> u1u2u3);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch u1u2u3 \{ ... \};
    ; #if there is only one clause, we can just fall through

Triple_i64_i64_i64_50059:

Triple_i64_i64_i64_50059_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50060
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab50061

lab50060:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab50061:
    ; substitute (a0 !-> a0)(u3 !-> u3)(u1 !-> u1)(u2 !-> u2)(v1v2v3 !-> v1v2v3);
    ; #move variables
    mov r12, rsi
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    ; switch v1v2v3 \{ ... \};
    ; #if there is only one clause, we can just fall through

Triple_i64_i64_i64_50062:

Triple_i64_i64_i64_50062_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab50063
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    jmp lab50064

lab50063:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]

lab50064:
    ; if v3 == 0 \{ ... \}
    cmp qword [rsp + 2024], 0
    je lab50065
    ; substitute (v30 !-> v3)(u3 !-> u3)(u1 !-> u1)(u2 !-> u2)(v1 !-> v1)(v2 !-> v2)(v3 !-> v3)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov rdx, [rsp + 2024]
    ; new a5: Pair[i64, i64] = (u1, u2, v1, v2, v3, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
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
    je lab50077
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab50078

lab50077:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50075
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50068
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50066
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50067

lab50066:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50067:

lab50068:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50071
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50069
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50070

lab50069:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50070:

lab50071:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50073

lab50072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50073:

lab50074:
    jmp lab50076

lab50075:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50076:

lab50078:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50090
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50091

lab50090:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50088
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50081
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50079
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50080

lab50079:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50080:

lab50081:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50083

lab50082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50083:

lab50084:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50086

lab50085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50086:

lab50087:
    jmp lab50089

lab50088:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50089:

lab50091:
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
    je lab50103
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50104

lab50103:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50101
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50094
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50092
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50093

lab50092:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50093:

lab50094:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50097
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50095
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50096

lab50095:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50096:

lab50097:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50100
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50098
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50099

lab50098:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50099:

lab50100:
    jmp lab50102

lab50101:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50102:

lab50104:
    ; #load tag
    lea r9, [rel Pair_i64_i64_50105]
    ; substitute (u3 !-> u3)(v30 !-> v30)(a5 !-> a5);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump quot_rem_
    jmp quot_rem_

Pair_i64_i64_50105:

Pair_i64_i64_50105_Pair:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50107
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab50106
    ; ####increment refcount
    add qword [rcx + 0], 1

lab50106:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    jmp lab50108

lab50107:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]

lab50108:
    ; substitute (q !-> q)(r !-> r)(u1 !-> u1)(u2 !-> u2)(v1 !-> v1)(v2 !-> v2)(a0 !-> a0)(v10 !-> v1)(v20 !-> v2)(v3 !-> v3);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 1976], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2024], rcx
    mov [rsp + 2008], r13
    mov [rsp + 1992], r15
    mov rcx, [rsp + 2016]
    mov [rsp + 2032], rcx
    ; let x0: Triple[i64, i64, i64] = Trip(v10, v20, v3);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50120
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab50121

lab50120:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50118
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50111
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50109
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50110

lab50109:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50110:

lab50111:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50114
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50112
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50113

lab50112:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50113:

lab50114:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50117
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50115
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50116

lab50115:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50116:

lab50117:
    jmp lab50119

lab50118:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50119:

lab50121:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; x2 <- q * v1;
    mov rcx, rdx
    imul rcx, r13
    mov [rsp + 1992], rcx
    ; substitute (q !-> q)(r !-> r)(u1 !-> u1)(u2 !-> u2)(x2 !-> x2)(v2 !-> v2)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov r13, [rsp + 1992]
    ; x3 <- u1 - x2;
    mov rcx, r9
    sub rcx, r13
    mov [rsp + 1992], rcx
    ; substitute (q !-> q)(r !-> r)(x3 !-> x3)(u2 !-> u2)(x0 !-> x0)(v2 !-> v2)(a0 !-> a0);
    ; #move variables
    mov r12, [rsp + 2016]
    mov r13, [rsp + 2008]
    mov r9, [rsp + 1992]
    ; x4 <- q * v2;
    mov rcx, rdx
    imul rcx, r15
    mov [rsp + 2008], rcx
    ; substitute (x4 !-> x4)(r !-> r)(x3 !-> x3)(u2 !-> u2)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r14, [rsp + 2032]
    mov r15, [rsp + 2024]
    mov rdx, [rsp + 2008]
    ; x5 <- u2 - x4;
    mov rcx, r11
    sub rcx, rdx
    mov [rsp + 2024], rcx
    ; substitute (a0 !-> a0)(x0 !-> x0)(x3 !-> x3)(x5 !-> x5)(r !-> r);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    mov rsi, r12
    mov rax, r14
    mov rdx, r15
    mov r11, [rsp + 2024]
    ; let x1: Triple[i64, i64, i64] = Trip(x3, x5, r);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50133
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50134

lab50133:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50131
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50124
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50122
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50123

lab50122:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50123:

lab50124:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50127
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50125
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50126

lab50125:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50126:

lab50127:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50130
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50128
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50129

lab50128:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50129:

lab50130:
    jmp lab50132

lab50131:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50132:

lab50134:
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
    ; jump g_
    jmp g_

lab50065:
    ; substitute (u3 !-> u3)(u1 !-> u1)(u2 !-> u2)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; invoke a0 Trip
    jmp r11

gcd_e_:
    ; if x == 0 \{ ... \}
    cmp rdx, 0
    je lab50135
    ; lit x3 <- 1;
    mov r11, 1
    ; lit x4 <- 0;
    mov r13, 0
    ; substitute (a0 !-> a0)(y !-> y)(x3 !-> x3)(x4 !-> x4)(x !-> x);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r8
    ; let x2: Triple[i64, i64, i64] = Trip(x3, x4, x);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50147
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50148

lab50147:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50145
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50138
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50136
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50137

lab50136:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50137:

lab50138:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50140

lab50139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50140:

lab50141:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50144
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50142
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50143

lab50142:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50143:

lab50144:
    jmp lab50146

lab50145:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50146:

lab50148:
    ; #load tag
    mov r9, 0
    ; lit x6 <- 0;
    mov r11, 0
    ; lit x7 <- 1;
    mov r13, 1
    ; substitute (a0 !-> a0)(x2 !-> x2)(x6 !-> x6)(x7 !-> x7)(y !-> y);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    mov rsi, r8
    ; let x5: Triple[i64, i64, i64] = Trip(x6, x7, y);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50160
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50161

lab50160:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50158
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50151
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50149
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50150

lab50149:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50150:

lab50151:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50154
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50152
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50153

lab50152:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50153:

lab50154:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50156

lab50155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50156:

lab50157:
    jmp lab50159

lab50158:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50159:

lab50161:
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump g_
    jmp g_

lab50135:
    ; substitute (a0 !-> a0)(y !-> y);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; lit x0 <- 0;
    mov r9, 0
    ; lit x1 <- 1;
    mov r11, 1
    ; substitute (y !-> y)(x0 !-> x0)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; invoke a0 Trip
    jmp r11

max__:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_50162]
    add rcx, rdi
    jmp rcx

List_i64_50162:
    jmp near List_i64_50162_Nil
    jmp near List_i64_50162_Cons

List_i64_50162_Nil:
    ; lit x2 <- -1;
    mov rdi, -1
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_50162_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50164
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab50163
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50163:
    mov rdi, [rsi + 40]
    jmp lab50165

lab50164:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab50165:
    ; switch xs \{ ... \};
    lea rcx, [rel List_i64_50166]
    add rcx, r9
    jmp rcx

List_i64_50166:
    jmp near List_i64_50166_Nil
    jmp near List_i64_50166_Cons

List_i64_50166_Nil:
    ; substitute (x !-> x)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_50166_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50168
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50167
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50167:
    mov r9, [r8 + 40]
    jmp lab50169

lab50168:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab50169:
    ; if x < y \{ ... \}
    cmp rdi, r9
    jl lab50170
    ; substitute (a0 !-> a0)(x !-> x)(ys !-> ys);
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; let x1: List[i64] = Cons(x, ys);
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
    je lab50182
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50183

lab50182:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50180
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50173
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50171
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50172

lab50171:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50172:

lab50173:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50176
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50174
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50175

lab50174:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50175:

lab50176:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50179
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50177
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50178

lab50177:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50178:

lab50179:
    jmp lab50181

lab50180:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50181:

lab50183:
    ; #load tag
    mov rdi, 5
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump max__
    jmp max__

lab50170:
    ; substitute (a0 !-> a0)(y !-> y)(ys !-> ys);
    ; #move variables
    mov rdi, r9
    mov r8, r10
    mov r9, r11
    ; let x0: List[i64] = Cons(y, ys);
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
    je lab50195
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50196

lab50195:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50193
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50185

lab50184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50185:

lab50186:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50189
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50187
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50188

lab50187:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50188:

lab50189:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50192
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50190
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50191

lab50190:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50191:

lab50192:
    jmp lab50194

lab50193:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50194:

lab50196:
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
    ; jump max__
    jmp max__

enum_from_to_:
    ; if from <= t \{ ... \}
    cmp rdx, rdi
    jle lab50197
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab50197:
    ; substitute (from0 !-> from)(t !-> t)(a0 !-> a0)(from !-> from);
    ; #move variables
    mov r11, rdx
    ; new a1: List[i64] = (a0, from)\{ ... \};
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
    je lab50209
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50210

lab50209:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50207
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50200
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50198
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50199

lab50198:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50199:

lab50200:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50203
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50201
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50202

lab50201:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50202:

lab50203:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50206
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50204
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50205

lab50204:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50205:

lab50206:
    jmp lab50208

lab50207:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50208:

lab50210:
    ; #load tag
    lea r9, [rel List_i64_50211]
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- from0 + x1;
    mov r13, rdx
    add r13, r11
    ; substitute (x2 !-> x2)(t !-> t)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_50211:
    jmp near List_i64_50211_Nil
    jmp near List_i64_50211_Cons

List_i64_50211_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50213
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50212
    ; ####increment refcount
    add qword [rax + 0], 1

lab50212:
    jmp lab50214

lab50213:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50214:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (from !-> from)(x0 !-> x0)(a0 !-> a0);
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

List_i64_50211_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50216
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50215
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50215:
    jmp lab50217

lab50216:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50217:
    ; substitute (from !-> from)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
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
    je lab50229
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50230

lab50229:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50227
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50219

lab50218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50219:

lab50220:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50222

lab50221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50222:

lab50223:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50225

lab50224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50225:

lab50226:
    jmp lab50228

lab50227:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50228:

lab50230:
    ; #load tag
    mov r9, 5
    ; substitute (from !-> from)(x0 !-> x0)(a0 !-> a0);
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

test_lscomp2_:
    ; substitute (a0 !-> a0)(t1 !-> t1)(ms !-> ms)(h1 !-> h1)(p2 !-> p2);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; switch p2 \{ ... \};
    lea rcx, [rel List_i64_50231]
    add rcx, r13
    jmp rcx

List_i64_50231:
    jmp near List_i64_50231_Nil
    jmp near List_i64_50231_Cons

List_i64_50231_Nil:
    ; substitute (t1 !-> t1)(ms !-> ms)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump test_lscomp1_
    jmp test_lscomp1_

List_i64_50231_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab50233
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    cmp r14, 0
    je lab50232
    ; ####increment refcount
    add qword [r14 + 0], 1

lab50232:
    mov r13, [r12 + 40]
    jmp lab50234

lab50233:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    mov r13, [r12 + 40]

lab50234:
    ; substitute (a0 !-> a0)(t1 !-> t1)(ms !-> ms)(h1 !-> h1)(t2 !-> t2)(h10 !-> h1)(h2 !-> h2);
    ; #move variables
    mov [rsp + 2024], r13
    mov r13, r15
    mov r15, r11
    mov r12, r14
    ; let x0: Pair[i64, i64] = Pair(h10, h2);
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
    je lab50246
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab50247

lab50246:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50244
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50237
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50235
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50236

lab50235:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50236:

lab50237:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50240
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50238
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50239

lab50238:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50239:

lab50240:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50243
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50241
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50242

lab50241:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50242:

lab50243:
    jmp lab50245

lab50244:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50245:

lab50247:
    ; #load tag
    mov r15, 0
    ; substitute (t2 !-> t2)(t1 !-> t1)(ms !-> ms)(h1 !-> h1)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; new a1: List[Pair[i64, i64]] = (a0, x0)\{ ... \};
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
    je lab50259
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab50260

lab50259:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50257
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50250
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50248
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50249

lab50248:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50249:

lab50250:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50253
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50251
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50252

lab50251:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50252:

lab50253:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50256
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50254
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50255

lab50254:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50255:

lab50256:
    jmp lab50258

lab50257:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50258:

lab50260:
    ; #load tag
    lea r13, [rel List_Pair_i64_i64_50261]
    ; jump test_lscomp2_
    jmp test_lscomp2_

List_Pair_i64_i64_50261:
    jmp near List_Pair_i64_i64_50261_Nil
    jmp near List_Pair_i64_i64_50261_Cons

List_Pair_i64_i64_50261_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50264
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab50262
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50262:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50263
    ; ####increment refcount
    add qword [rax + 0], 1

lab50263:
    jmp lab50265

lab50264:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50265:
    ; let x1: List[Pair[i64, i64]] = Nil();
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
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_Pair_i64_i64_50261_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50268
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50266
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50266:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50267
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50267:
    jmp lab50269

lab50268:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50269:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a2 !-> a2)(as0 !-> as0);
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
    ; let x1: List[Pair[i64, i64]] = Cons(a2, as0);
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
    je lab50281
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50282

lab50281:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50279
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50271

lab50270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50271:

lab50272:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50275
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50273
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50274

lab50273:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50274:

lab50275:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50278
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50276
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50277

lab50276:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50277:

lab50278:
    jmp lab50280

lab50279:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50280:

lab50282:
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

test_lscomp1_:
    ; substitute (a0 !-> a0)(ms !-> ms)(p1 !-> p1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch p1 \{ ... \};
    lea rcx, [rel List_i64_50283]
    add rcx, r9
    jmp rcx

List_i64_50283:
    jmp near List_i64_50283_Nil
    jmp near List_i64_50283_Cons

List_i64_50283_Nil:
    ; substitute (a0 !-> a0);
    ; #erase ms
    cmp rsi, 0
    je lab50286
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab50284
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab50285

lab50284:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab50285:

lab50286:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_50283_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50288
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50287
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50287:
    mov r9, [r8 + 40]
    jmp lab50289

lab50288:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab50289:
    ; substitute (ms !-> ms)(t1 !-> t1)(ms0 !-> ms)(h1 !-> h1)(a0 !-> a0);
    ; #share ms
    cmp rsi, 0
    je lab50290
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50290:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rax, rsi
    mov r8, rsi
    mov rdx, rdi
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r10
    ; jump test_lscomp2_
    jmp test_lscomp2_

map_pairs_:
    ; substitute (f !-> f)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_50291]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_50291:
    jmp near List_Pair_i64_i64_50291_Nil
    jmp near List_Pair_i64_i64_50291_Cons

List_Pair_i64_i64_50291_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab50294
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab50292
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab50293

lab50292:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab50293:

lab50294:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_50291_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50297
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50295
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50295:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50296
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50296:
    jmp lab50298

lab50297:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50298:
    ; substitute (f0 !-> f)(p !-> p)(a0 !-> a0)(ps !-> ps)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab50299
    ; ####increment refcount
    add qword [rax + 0], 1

lab50299:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: Triple[i64, i64, Triple[i64, i64, i64]] = (a0, ps, f)\{ ... \};
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
    je lab50311
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50312

lab50311:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50309
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50302
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50300
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50301

lab50300:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50301:

lab50302:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50305
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50303
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50304

lab50303:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50304:

lab50305:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50308
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50306
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50307

lab50306:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50307:

lab50308:
    jmp lab50310

lab50309:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50310:

lab50312:
    ; #load tag
    lea r9, [rel Triple_i64_i64_Triple_i64_i64_i64_50313]
    ; substitute (p !-> p)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Ap
    jmp r9

Triple_i64_i64_Triple_i64_i64_i64_50313:

Triple_i64_i64_Triple_i64_i64_i64_50313_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab50317
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab50314
    ; ####increment refcount
    add qword [r14 + 0], 1

lab50314:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab50315
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50315:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab50316
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50316:
    jmp lab50318

lab50317:
    ; ##... or release blocks onto linear free list when loading
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

lab50318:
    ; substitute (f !-> f)(ps !-> ps)(a0 !-> a0)(a4 !-> a4)(b0 !-> b0)(c0 !-> c0);
    ; #move variables
    mov rcx, r15
    mov r15, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    mov rax, r14
    mov r14, r8
    mov r8, r10
    mov rsi, r12
    ; let x0: Triple[i64, i64, Triple[i64, i64, i64]] = Trip(a4, b0, c0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50330
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50331

lab50330:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50328
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50321
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50319
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50320

lab50319:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50320:

lab50321:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50323

lab50322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50323:

lab50324:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50327
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50325
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50326

lab50325:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50326:

lab50327:
    jmp lab50329

lab50328:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50329:

lab50331:
    ; #load tag
    mov r11, 0
    ; new a2: List[Triple[i64, i64, Triple[i64, i64, i64]]] = (a0, x0)\{ ... \};
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
    je lab50343
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50344

lab50343:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50341
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50334
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50332
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50333

lab50332:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50333:

lab50334:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50336

lab50335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50336:

lab50337:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50340
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50338
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50339

lab50338:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50339:

lab50340:
    jmp lab50342

lab50341:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50342:

lab50344:
    ; #load tag
    lea r9, [rel List_Triple_i64_i64_Triple_i64_i64_i64_50345]
    ; jump map_pairs_
    jmp map_pairs_

List_Triple_i64_i64_Triple_i64_i64_i64_50345:
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50345_Nil
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50345_Cons

List_Triple_i64_i64_Triple_i64_i64_i64_50345_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50348
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab50346
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50346:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50347
    ; ####increment refcount
    add qword [rax + 0], 1

lab50347:
    jmp lab50349

lab50348:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50349:
    ; let x1: List[Triple[i64, i64, Triple[i64, i64, i64]]] = Nil();
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
    ; invoke a0 Cons
    add r9, 5
    jmp r9

List_Triple_i64_i64_Triple_i64_i64_i64_50345_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50352
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50350
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50350:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50351
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50351:
    jmp lab50353

lab50352:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50353:
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
    ; let x1: List[Triple[i64, i64, Triple[i64, i64, i64]]] = Cons(a3, as0);
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
    je lab50365
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50366

lab50365:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50363
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50355

lab50354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50355:

lab50356:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50359
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50357
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50358

lab50357:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50358:

lab50359:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50362
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50360
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50361

lab50360:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50361:

lab50362:
    jmp lab50364

lab50363:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50364:

lab50366:
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

map_triples_:
    ; substitute (f !-> f)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_Triple_i64_i64_Triple_i64_i64_i64_50367]
    add rcx, r9
    jmp rcx

List_Triple_i64_i64_Triple_i64_i64_i64_50367:
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50367_Nil
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50367_Cons

List_Triple_i64_i64_Triple_i64_i64_i64_50367_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab50370
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab50368
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab50369

lab50368:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab50369:

lab50370:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Triple_i64_i64_Triple_i64_i64_i64_50367_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50373
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50371
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50371:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50372
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50372:
    jmp lab50374

lab50373:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50374:
    ; substitute (f0 !-> f)(t !-> t)(a0 !-> a0)(ts !-> ts)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab50375
    ; ####increment refcount
    add qword [rax + 0], 1

lab50375:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: _Cont = (a0, ts, f)\{ ... \};
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
    je lab50387
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50388

lab50387:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50385
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50378
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50376
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50377

lab50376:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50377:

lab50378:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50381
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50379
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50380

lab50379:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50380:

lab50381:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50384
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50382
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50383

lab50382:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50383:

lab50384:
    jmp lab50386

lab50385:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50386:

lab50388:
    ; #load tag
    lea r9, [rel _Cont_50389]
    ; substitute (t !-> t)(a1 !-> a1)(f0 !-> f0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f0 Ap
    jmp r9

_Cont_50389:

_Cont_50389_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50393
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab50390
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50390:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab50391
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50391:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab50392
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50392:
    jmp lab50394

lab50393:
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

lab50394:
    ; substitute (f !-> f)(ts !-> ts)(a0 !-> a0)(x0 !-> x0);
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
    ; new a2: List[i64] = (a0, x0)\{ ... \};
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
    je lab50406
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50407

lab50406:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50404
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50397
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50395
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50396

lab50395:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50396:

lab50397:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50400
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50398
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50399

lab50398:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50399:

lab50400:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50403
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50401
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50402

lab50401:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50402:

lab50403:
    jmp lab50405

lab50404:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50405:

lab50407:
    ; #load tag
    lea r9, [rel List_i64_50408]
    ; jump map_triples_
    jmp map_triples_

List_i64_50408:
    jmp near List_i64_50408_Nil
    jmp near List_i64_50408_Cons

List_i64_50408_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50410
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50409
    ; ####increment refcount
    add qword [rax + 0], 1

lab50409:
    jmp lab50411

lab50410:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50411:
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

List_i64_50408_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50413
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50412
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50412:
    jmp lab50414

lab50413:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50414:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
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
    je lab50426
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50427

lab50426:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50424
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50417
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50415
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50416

lab50415:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50416:

lab50417:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50420
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50418
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50419

lab50418:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50419:

lab50420:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50423
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50421
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50422

lab50421:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50422:

lab50423:
    jmp lab50425

lab50424:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50425:

lab50427:
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

abs_int_:
    ; lit x0 <- 0;
    mov r9, 0
    ; if i < x0 \{ ... \}
    cmp rdx, r9
    jl lab50428
    ; substitute (i !-> i)(a0 !-> a0);
    ; invoke a0 Ret
    jmp rdi

lab50428:
    ; substitute (i !-> i)(a0 !-> a0);
    ; lit x1 <- -1;
    mov r9, -1
    ; x2 <- x1 * i;
    mov r11, r9
    imul r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

test_:
    ; lit x0 <- 5000;
    mov r9, 5000
    ; lit x1 <- 5000;
    mov r11, 5000
    ; x2 <- x1 + d;
    mov r13, r11
    add r13, rdx
    ; substitute (x2 !-> x2)(x0 !-> x0)(a0 !-> a0)(d !-> d);
    ; #move variables
    mov r11, rdx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; new a9: List[i64] = (a0, d)\{ ... \};
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
    je lab50440
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50441

lab50440:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50438
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50431
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50429
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50430

lab50429:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50430:

lab50431:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50434
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50432
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50433

lab50432:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50433:

lab50434:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50437
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50435
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50436

lab50435:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50436:

lab50437:
    jmp lab50439

lab50438:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50439:

lab50441:
    ; #load tag
    lea r9, [rel List_i64_50442]
    ; substitute (x0 !-> x0)(x2 !-> x2)(a9 !-> a9);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_50442:
    jmp near List_i64_50442_Nil
    jmp near List_i64_50442_Cons

List_i64_50442_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50444
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50443
    ; ####increment refcount
    add qword [rax + 0], 1

lab50443:
    jmp lab50445

lab50444:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50445:
    ; let ns: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_test_0_
    jmp lift_test_0_

List_i64_50442_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50447
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50446
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50446:
    jmp lab50448

lab50447:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50448:
    ; substitute (d !-> d)(a0 !-> a0)(a18 !-> a18)(as4 !-> as4);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let ns: List[i64] = Cons(a18, as4);
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
    je lab50460
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50461

lab50460:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50458
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50451
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50449
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50450

lab50449:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50450:

lab50451:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50454
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50452
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50453

lab50452:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50453:

lab50454:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50457
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50455
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50456

lab50455:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50456:

lab50457:
    jmp lab50459

lab50458:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50459:

lab50461:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(d !-> d)(ns !-> ns);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_test_0_
    jmp lift_test_0_

lift_test_0_:
    ; lit x3 <- 10000;
    mov r11, 10000
    ; lit x4 <- 10000;
    mov r13, 10000
    ; x5 <- x4 + d;
    mov r15, r13
    add r15, rdi
    ; substitute (x5 !-> x5)(x3 !-> x3)(ns !-> ns)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rdi, r11
    mov r11, rdx
    mov rdx, r15
    ; new a10: List[i64] = (ns, a0)\{ ... \};
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
    je lab50473
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50474

lab50473:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50471
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50464
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50462
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50463

lab50462:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50463:

lab50464:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50467
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50465
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50466

lab50465:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50466:

lab50467:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50470
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50468
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50469

lab50468:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50469:

lab50470:
    jmp lab50472

lab50471:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50472:

lab50474:
    ; #load tag
    lea r9, [rel List_i64_50475]
    ; substitute (x3 !-> x3)(x5 !-> x5)(a10 !-> a10);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump enum_from_to_
    jmp enum_from_to_

List_i64_50475:
    jmp near List_i64_50475_Nil
    jmp near List_i64_50475_Cons

List_i64_50475_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50478
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab50476
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50476:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50477
    ; ####increment refcount
    add qword [rax + 0], 1

lab50477:
    jmp lab50479

lab50478:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50479:
    ; let ms: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(ms !-> ms)(ns !-> ns);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_test_1_
    jmp lift_test_1_

List_i64_50475_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50482
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50480
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50480:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50481
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50481:
    jmp lab50483

lab50482:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50483:
    ; substitute (a0 !-> a0)(ns !-> ns)(a17 !-> a17)(as3 !-> as3);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let ms: List[i64] = Cons(a17, as3);
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
    je lab50495
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50496

lab50495:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50493
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50486
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50484
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50485

lab50484:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50485:

lab50486:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50489
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50487
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50488

lab50487:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50488:

lab50489:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50492
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50490
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50491

lab50490:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50491:

lab50492:
    jmp lab50494

lab50493:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50494:

lab50496:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(ms !-> ms)(ns !-> ns);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_test_1_
    jmp lift_test_1_

lift_test_1_:
    ; new x6: Fun[Pair[i64, i64], Triple[i64, i64, Triple[i64, i64, i64]]] = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel Fun_Pair_i64_i64_Triple_i64_i64_Triple_i64_i64_i64_50497]
    ; substitute (ns !-> ns)(ms !-> ms)(a0 !-> a0)(x6 !-> x6);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a6: List[Pair[i64, i64]] = (a0, x6)\{ ... \};
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
    je lab50509
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50510

lab50509:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50507
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50499

lab50498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50499:

lab50500:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50502

lab50501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50502:

lab50503:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50506
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50504
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50505

lab50504:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50505:

lab50506:
    jmp lab50508

lab50507:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50508:

lab50510:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_50511]
    ; jump test_lscomp1_
    jmp test_lscomp1_

List_Pair_i64_i64_50511:
    jmp near List_Pair_i64_i64_50511_Nil
    jmp near List_Pair_i64_i64_50511_Cons

List_Pair_i64_i64_50511_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50514
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab50512
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50512:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab50513
    ; ####increment refcount
    add qword [rax + 0], 1

lab50513:
    jmp lab50515

lab50514:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab50515:
    ; let x7: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; jump lift_test_2_
    jmp lift_test_2_

List_Pair_i64_i64_50511_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50518
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab50516
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50516:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab50517
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50517:
    jmp lab50519

lab50518:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab50519:
    ; substitute (x6 !-> x6)(a0 !-> a0)(a16 !-> a16)(as2 !-> as2);
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
    ; let x7: List[Pair[i64, i64]] = Cons(a16, as2);
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
    je lab50531
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50532

lab50531:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50529
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50522
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50520
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50521

lab50520:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50521:

lab50522:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50525
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50523
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50524

lab50523:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50524:

lab50525:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50528
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50526
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50527

lab50526:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50527:

lab50528:
    jmp lab50530

lab50529:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50530:

lab50532:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(x6 !-> x6)(x7 !-> x7);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lift_test_2_
    jmp lift_test_2_

Fun_Pair_i64_i64_Triple_i64_i64_Triple_i64_i64_i64_50497:

Fun_Pair_i64_i64_Triple_i64_i64_Triple_i64_i64_i64_50497_Ap:
    ; substitute (a4 !-> a4)(p !-> p);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_50533:

Pair_i64_i64_50533_Pair:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50534
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab50535

lab50534:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab50535:
    ; substitute (y0 !-> y)(x12 !-> x)(y !-> y)(a4 !-> a4)(x !-> x);
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov r13, rdi
    mov rdx, r9
    ; new a5: Triple[i64, i64, i64] = (y, a4, x)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab50547
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50548

lab50547:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50545
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50538
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50536
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50537

lab50536:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50537:

lab50538:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50541
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50539
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50540

lab50539:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50540:

lab50541:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50544
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50542
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50543

lab50542:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50543:

lab50544:
    jmp lab50546

lab50545:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50546:

lab50548:
    ; #load tag
    lea r9, [rel Triple_i64_i64_i64_50549]
    ; substitute (x12 !-> x12)(y0 !-> y0)(a5 !-> a5);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump gcd_e_
    jmp gcd_e_

Triple_i64_i64_i64_50549:

Triple_i64_i64_i64_50549_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab50551
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab50550
    ; ####increment refcount
    add qword [r12 + 0], 1

lab50550:
    mov r11, [r10 + 24]
    jmp lab50552

lab50551:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]

lab50552:
    ; substitute (x !-> x)(a4 !-> a4)(y !-> y)(a13 !-> a13)(b0 !-> b0)(c0 !-> c0);
    ; #move variables
    mov rcx, r15
    mov r15, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    mov rsi, r12
    ; let x11: Triple[i64, i64, i64] = Trip(a13, b0, c0);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    mov [rbx + 24], r11
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab50564
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50565

lab50564:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50562
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50555
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50553
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50554

lab50553:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50554:

lab50555:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50558
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50556
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50557

lab50556:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50557:

lab50558:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50561
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50559
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50560

lab50559:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50560:

lab50561:
    jmp lab50563

lab50562:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50563:

lab50565:
    ; #load tag
    mov r11, 0
    ; substitute (x !-> x)(y !-> y)(x11 !-> x11)(a4 !-> a4);
    ; #move variables
    mov r8, r10
    mov r10, rsi
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; invoke a4 Trip
    jmp r11

lift_test_2_:
    ; substitute (x7 !-> x7)(x6 !-> x6)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a11: List[Triple[i64, i64, Triple[i64, i64, i64]]] = (a0)\{ ... \};
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
    je lab50577
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50578

lab50577:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50575
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50568
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50566
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50567

lab50566:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50567:

lab50568:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50571
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50569
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50570

lab50569:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50570:

lab50571:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50574
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50572
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50573

lab50572:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50573:

lab50574:
    jmp lab50576

lab50575:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50576:

lab50578:
    ; #load tag
    lea r9, [rel List_Triple_i64_i64_Triple_i64_i64_i64_50579]
    ; substitute (x6 !-> x6)(x7 !-> x7)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump map_pairs_
    jmp map_pairs_

List_Triple_i64_i64_Triple_i64_i64_i64_50579:
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50579_Nil
    jmp near List_Triple_i64_i64_Triple_i64_i64_i64_50579_Cons

List_Triple_i64_i64_Triple_i64_i64_i64_50579_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50581
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab50580
    ; ####increment refcount
    add qword [rax + 0], 1

lab50580:
    jmp lab50582

lab50581:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab50582:
    ; let tripls: List[Triple[i64, i64, Triple[i64, i64, i64]]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_test_3_
    jmp lift_test_3_

List_Triple_i64_i64_Triple_i64_i64_i64_50579_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50584
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab50583
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50583:
    jmp lab50585

lab50584:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab50585:
    ; substitute (a0 !-> a0)(a15 !-> a15)(as1 !-> as1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let tripls: List[Triple[i64, i64, Triple[i64, i64, i64]]] = Cons(a15, as1);
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
    je lab50597
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50598

lab50597:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50595
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50588
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50586
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50587

lab50586:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50587:

lab50588:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50591
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50589
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50590

lab50589:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50590:

lab50591:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50594
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50592
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50593

lab50592:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50593:

lab50594:
    jmp lab50596

lab50595:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50596:

lab50598:
    ; #load tag
    mov rdi, 5
    ; jump lift_test_3_
    jmp lift_test_3_

lift_test_3_:
    ; new x8: Fun[Triple[i64, i64, Triple[i64, i64, i64]], i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Triple_i64_i64_Triple_i64_i64_i64_i64_50599]
    ; substitute (x8 !-> x8)(tripls !-> tripls)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a12: List[i64] = (a0)\{ ... \};
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
    je lab50611
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab50612

lab50611:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50609
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50602
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50600
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50601

lab50600:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50601:

lab50602:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50605
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50603
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50604

lab50603:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50604:

lab50605:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50608
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50606
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50607

lab50606:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50607:

lab50608:
    jmp lab50610

lab50609:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50610:

lab50612:
    ; #load tag
    lea r9, [rel List_i64_50613]
    ; jump map_triples_
    jmp map_triples_

List_i64_50613:
    jmp near List_i64_50613_Nil
    jmp near List_i64_50613_Cons

List_i64_50613_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50615
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab50614
    ; ####increment refcount
    add qword [rax + 0], 1

lab50614:
    jmp lab50616

lab50615:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab50616:
    ; let rs: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (rs !-> rs)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump max__
    jmp max__

List_i64_50613_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50618
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab50617
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50617:
    jmp lab50619

lab50618:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab50619:
    ; substitute (a0 !-> a0)(a14 !-> a14)(as0 !-> as0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let rs: List[i64] = Cons(a14, as0);
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
    je lab50631
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50632

lab50631:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50629
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50622
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50620
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50621

lab50620:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50621:

lab50622:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50625
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50623
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50624

lab50623:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50624:

lab50625:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50628
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50626
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50627

lab50626:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50627:

lab50628:
    jmp lab50630

lab50629:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50630:

lab50632:
    ; #load tag
    mov rdi, 5
    ; substitute (rs !-> rs)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump max__
    jmp max__

Fun_Triple_i64_i64_Triple_i64_i64_i64_i64_50599:

Fun_Triple_i64_i64_Triple_i64_i64_i64_i64_50599_Ap:
    ; substitute (a1 !-> a1)(t !-> t);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch t \{ ... \};
    ; #if there is only one clause, we can just fall through

Triple_i64_i64_Triple_i64_i64_i64_50633:

Triple_i64_i64_Triple_i64_i64_i64_50633_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50635
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab50634
    ; ####increment refcount
    add qword [r10 + 0], 1

lab50634:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab50636

lab50635:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab50636:
    ; substitute (a1 !-> a1)(t0 !-> t0);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
    ; switch t0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Triple_i64_i64_i64_50637:

Triple_i64_i64_i64_50637_Trip:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50638
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab50639

lab50638:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab50639:
    ; x9 <- gg + u;
    mov r13, rdi
    add r13, r9
    ; substitute (a1 !-> a1)(x9 !-> x9)(v !-> v);
    ; #move variables
    mov r9, r11
    mov rdi, r13
    ; x10 <- x9 + v;
    mov r11, rdi
    add r11, r9
    ; substitute (x10 !-> x10)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; jump abs_int_
    jmp abs_int_

test_gcd_nofib_:
    ; jump test_
    jmp test_

main_loop_:
    ; lit x0 <- 1;
    mov r11, 1
    ; if iters == x0 \{ ... \}
    cmp rdx, r11
    je lab50640
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a3: _Cont = (n, a0, iters)\{ ... \};
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
    je lab50652
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50653

lab50652:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50650
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50642

lab50641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50642:

lab50643:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50645

lab50644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50645:

lab50646:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50649
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50647
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50648

lab50647:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50648:

lab50649:
    jmp lab50651

lab50650:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50651:

lab50653:
    ; #load tag
    lea rdi, [rel _Cont_50654]
    ; jump test_gcd_nofib_
    jmp test_gcd_nofib_

_Cont_50654:

_Cont_50654_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50656
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab50655
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50655:
    mov rdi, [rsi + 24]
    jmp lab50657

lab50656:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab50657:
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- iters - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab50640:
    ; substitute (n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a2: _Cont = (a0)\{ ... \};
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
    je lab50669
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50670

lab50669:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50667
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50660
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50658
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50659

lab50658:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50659:

lab50660:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50663
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50661
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50662

lab50661:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50662:

lab50663:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50666
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50664
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50665

lab50664:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50665:

lab50666:
    jmp lab50668

lab50667:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50668:

lab50670:
    ; #load tag
    lea rdi, [rel _Cont_50671]
    ; jump test_gcd_nofib_
    jmp test_gcd_nofib_

_Cont_50671:

_Cont_50671_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50673
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab50672
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50672:
    jmp lab50674

lab50673:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab50674:
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