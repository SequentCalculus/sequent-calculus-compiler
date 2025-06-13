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
    lea r9, [rel _Cont_48053]
    ; jump main_loop_
    jmp main_loop_

_Cont_48053:

_Cont_48053_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

sum_loop_:
    ; if stop < i \{ ... \}
    cmp r9, rdx
    jl lab48054
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; x1 <- i + x0;
    mov rcx, rdx
    add rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (i !-> i)(f0 !-> f)(stop !-> stop)(f !-> f)(k !-> k)(a0 !-> a0)(tot !-> tot)(x1 !-> x1);
    ; #share f
    cmp r10, 0
    je lab48055
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48055:
    ; #move variables
    mov [rsp + 2024], rdi
    mov rsi, r10
    mov rdi, r11
    ; new a3: _Cont = (stop, f, k, a0, tot, x1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov [rbx + 24], r15
    mov [rbx + 16], r14
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48067
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48068

lab48067:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48065
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48058
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48056
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48057

lab48056:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48057:

lab48058:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48061
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48059
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48060

lab48059:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48060:

lab48061:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48064
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48062
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48063

lab48062:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48063:

lab48064:
    jmp lab48066

lab48065:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48066:

lab48068:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab48080
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48081

lab48080:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48078
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48071
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48069
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48070

lab48069:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48070:

lab48071:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48073

lab48072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48073:

lab48074:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48077
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48075
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48076

lab48075:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48076:

lab48077:
    jmp lab48079

lab48078:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48079:

lab48081:
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
    je lab48093
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48094

lab48093:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48091
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48083

lab48082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48083:

lab48084:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48086

lab48085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48086:

lab48087:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48089

lab48088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48089:

lab48090:
    jmp lab48092

lab48091:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48092:

lab48094:
    ; #load tag
    lea r9, [rel _Cont_48095]
    ; substitute (i !-> i)(a3 !-> a3)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_48095:

_Cont_48095_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48099
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48096
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48096:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48097
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48097:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48098
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48098:
    jmp lab48100

lab48099:
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
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab48100:
    ; x3 <- x2 + tot;
    mov rcx, rdx
    add rcx, r15
    mov [rsp + 2008], rcx
    ; substitute (x1 !-> x1)(x3 !-> x3)(stop !-> stop)(f !-> f)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov r15, r13
    mov r13, r11
    mov r11, r9
    mov r9, rdi
    mov r14, r12
    mov r12, r10
    mov r10, r8
    mov rdx, [rsp + 2024]
    mov rdi, [rsp + 2008]
    ; jump sum_loop_
    jmp sum_loop_

lab48054:
    ; substitute (tot !-> tot)(k !-> k);
    ; #erase a0
    cmp r14, 0
    je lab48103
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab48101
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab48102

lab48101:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab48102:

lab48103:
    ; #erase f
    cmp r10, 0
    je lab48106
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab48104
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab48105

lab48104:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab48105:

lab48106:
    ; #move variables
    mov rdx, rdi
    mov rsi, r12
    mov rdi, r13
    ; invoke k Ret
    jmp rdi

sum_:
    ; lit x0 <- 0;
    mov r15, 0
    ; substitute (start !-> start)(x0 !-> x0)(stop !-> stop)(f !-> f)(k !-> k)(a0 !-> a0);
    ; #move variables
    mov r14, r12
    mov r12, r10
    mov r10, rax
    mov rcx, rdi
    mov rdi, r15
    mov r15, r13
    mov r13, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump sum_loop_
    jmp sum_loop_

motz_:
    ; substitute (n !-> n)(k !-> k);
    ; #erase a0
    cmp r8, 0
    je lab48109
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab48107
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab48108

lab48107:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab48108:

lab48109:
    ; lit x0 <- 1;
    mov r9, 1
    ; if n <= x0 \{ ... \}
    cmp rdx, r9
    jle lab48110
    ; substitute (n !-> n)(k !-> k);
    ; lit x1 <- 2;
    mov r9, 2
    ; limit <- n - x1;
    mov r11, rdx
    sub r11, r9
    ; substitute (n !-> n)(k !-> k)(limit0 !-> limit)(limit !-> limit);
    ; #move variables
    mov r9, r11
    ; new product: Fun[i64, i64] = (limit)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48122
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48123

lab48122:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48120
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48113
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48111
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48112

lab48111:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48112:

lab48113:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48116
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48114
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48115

lab48114:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48115:

lab48116:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48119
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48117
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48118

lab48117:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48118:

lab48119:
    jmp lab48121

lab48120:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48121:

lab48123:
    ; #load tag
    lea r11, [rel Fun_i64_i64_48124]
    ; new a4: _Cont = (k, limit0, product)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48136
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab48137

lab48136:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48134
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48127
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48125
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48126

lab48125:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48126:

lab48127:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48130
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48128
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48129

lab48128:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48129:

lab48130:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48133
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48131
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48132

lab48131:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48132:

lab48133:
    jmp lab48135

lab48134:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48135:

lab48137:
    ; #load tag
    lea rdi, [rel _Cont_48138]
    ; lit x6 <- 1;
    mov r9, 1
    ; x7 <- n - x6;
    mov r11, rdx
    sub r11, r9
    ; substitute (x7 !-> x7)(a4 !-> a4)(a40 !-> a4);
    ; #share a4
    cmp rsi, 0
    je lab48139
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48139:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    mov rdx, r11
    ; jump motz_
    jmp motz_

_Cont_48138:

_Cont_48138_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48142
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab48140
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48140:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab48141
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48141:
    jmp lab48143

lab48142:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab48143:
    ; substitute (product !-> product)(limit0 !-> limit0)(k !-> k)(x5 !-> x5);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rax, r10
    ; new b0: _Cont = (k, x5)\{ ... \};
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
    je lab48155
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48156

lab48155:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48153
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48146
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48144
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48145

lab48144:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48145:

lab48146:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48149
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48147
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48148

lab48147:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48148:

lab48149:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48152
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48150
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48151

lab48150:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48151:

lab48152:
    jmp lab48154

lab48153:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48154:

lab48156:
    ; #load tag
    lea r9, [rel _Cont_48157]
    ; lit x9 <- 0;
    mov r11, 0
    ; substitute (product !-> product)(x9 !-> x9)(limit0 !-> limit0)(b0 !-> b0)(b00 !-> b0);
    ; #share b0
    cmp r8, 0
    je lab48158
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48158:
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r13, r9
    mov r9, rdi
    mov rdi, rcx
    mov r10, r8
    mov r12, r8
    ; jump sum_
    jmp sum_

_Cont_48157:

_Cont_48157_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48160
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab48159
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48159:
    jmp lab48161

lab48160:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab48161:
    ; x12 <- x5 + x8;
    mov r11, r9
    add r11, rdx
    ; substitute (x12 !-> x12)(k !-> k);
    ; #move variables
    mov rdx, r11
    ; invoke k Ret
    jmp rdi

Fun_i64_i64_48124:

Fun_i64_i64_48124_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48162
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab48163

lab48162:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab48163:
    ; substitute (i0 !-> i)(a2 !-> a2)(limit !-> limit)(i !-> i);
    ; #move variables
    mov r11, rdx
    ; new a: _Cont = (a2, limit, i)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48175
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab48176

lab48175:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48173
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48166
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48164
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48165

lab48164:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48165:

lab48166:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48169
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48167
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48168

lab48167:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48168:

lab48169:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48172
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48170
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48171

lab48170:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48171:

lab48172:
    jmp lab48174

lab48173:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48174:

lab48176:
    ; #load tag
    lea rdi, [rel _Cont_48177]
    ; substitute (i0 !-> i0)(a !-> a)(a5 !-> a);
    ; #share a
    cmp rsi, 0
    je lab48178
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48178:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    ; jump motz_
    jmp motz_

_Cont_48177:

_Cont_48177_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48180
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab48179
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48179:
    jmp lab48181

lab48180:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab48181:
    ; substitute (i !-> i)(limit !-> limit)(a2 !-> a2)(x2 !-> x2);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new b: _Cont = (a2, x2)\{ ... \};
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
    je lab48193
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48194

lab48193:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48191
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48184
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48182
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48183

lab48182:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48183:

lab48184:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48187
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48185
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48186

lab48185:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48186:

lab48187:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48190
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48188
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48189

lab48188:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48189:

lab48190:
    jmp lab48192

lab48191:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48192:

lab48194:
    ; #load tag
    lea r9, [rel _Cont_48195]
    ; x4 <- limit - i;
    mov r11, rdi
    sub r11, rdx
    ; substitute (x4 !-> x4)(b !-> b)(b1 !-> b);
    ; #share b
    cmp r8, 0
    je lab48196
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48196:
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; jump motz_
    jmp motz_

_Cont_48195:

_Cont_48195_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48198
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab48197
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48197:
    jmp lab48199

lab48198:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab48199:
    ; x11 <- x2 * x3;
    mov r11, r9
    imul r11, rdx
    ; substitute (x11 !-> x11)(a2 !-> a2);
    ; #move variables
    mov rdx, r11
    ; invoke a2 Ret
    jmp rdi

lab48110:
    ; substitute (k !-> k);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x10 <- 1;
    mov rdi, 1
    ; substitute (x10 !-> x10)(k !-> k);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke k Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new k: _Cont = (n, a0, iters)\{ ... \};
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
    je lab48211
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab48212

lab48211:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48209
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48202
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48200
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48201

lab48200:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48201:

lab48202:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48205
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48203
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48204

lab48203:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48204:

lab48205:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48208
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48206
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48207

lab48206:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48207:

lab48208:
    jmp lab48210

lab48209:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48210:

lab48212:
    ; #load tag
    lea rdi, [rel _Cont_48213]
    ; substitute (n0 !-> n0)(k !-> k)(k0 !-> k);
    ; #share k
    cmp rsi, 0
    je lab48214
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48214:
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    ; jump motz_
    jmp motz_

_Cont_48213:

_Cont_48213_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48216
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab48215
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48215:
    mov rdi, [rsi + 24]
    jmp lab48217

lab48216:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab48217:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab48218
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

lab48218:
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