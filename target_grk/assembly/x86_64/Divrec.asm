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
    lea r9, [rel _Cont_55089]
    ; jump main_loop_
    jmp main_loop_

_Cont_55089:

_Cont_55089_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

create_n_loop_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab55090
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (x1 !-> x1)(acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; let x3: Unit = Unit();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x1 !-> x1)(a0 !-> a0)(x3 !-> x3)(acc !-> acc);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x2: List[Unit] = Cons(x3, acc);
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
    je lab55102
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab55103

lab55102:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55100
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55093
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55091
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55092

lab55091:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55092:

lab55093:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55096
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55094
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55095

lab55094:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55095:

lab55096:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55099
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55097
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55098

lab55097:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55098:

lab55099:
    jmp lab55101

lab55100:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55101:

lab55103:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump create_n_loop_
    jmp create_n_loop_

lab55090:
    ; substitute (a0 !-> a0)(acc !-> acc);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch acc \{ ... \};
    lea rcx, [rel List_Unit_55104]
    add rcx, rdi
    jmp rcx

List_Unit_55104:
    jmp near List_Unit_55104_Nil
    jmp near List_Unit_55104_Cons

List_Unit_55104_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Unit_55104_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55107
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab55105
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55105:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab55106
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55106:
    jmp lab55108

lab55107:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab55108:
    ; substitute (x4 !-> x4)(xs0 !-> xs0)(a0 !-> a0);
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

create_n_:
    ; let x0: List[Unit] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (n !-> n)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump create_n_loop_
    jmp create_n_loop_

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
    lea rcx, [rel List_Unit_55109]
    add rcx, r9
    jmp rcx

List_Unit_55109:
    jmp near List_Unit_55109_Nil
    jmp near List_Unit_55109_Cons

List_Unit_55109_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_Unit_55109_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab55112
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab55110
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55110:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab55111
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55111:
    jmp lab55113

lab55112:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab55113:
    ; substitute (a0 !-> a0)(acc !-> acc)(xs !-> xs);
    ; #erase x
    cmp r8, 0
    je lab55116
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab55114
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab55115

lab55114:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab55115:

lab55116:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- acc + x0;
    mov r13, rdi
    add r13, r11
    ; substitute (xs !-> xs)(x1 !-> x1)(a0 !-> a0);
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

rec_div2_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Unit_55117]
    add rcx, rdi
    jmp rcx

List_Unit_55117:
    jmp near List_Unit_55117_Nil
    jmp near List_Unit_55117_Cons

List_Unit_55117_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Unit_55117_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55120
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab55118
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55118:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab55119
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55119:
    jmp lab55121

lab55120:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab55121:
    ; switch xs \{ ... \};
    lea rcx, [rel List_Unit_55122]
    add rcx, r9
    jmp rcx

List_Unit_55122:
    jmp near List_Unit_55122_Nil
    jmp near List_Unit_55122_Cons

List_Unit_55122_Nil:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab55125
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab55123
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab55124

lab55123:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab55124:

lab55125:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Unit_55122_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab55128
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab55126
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55126:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab55127
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55127:
    jmp lab55129

lab55128:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab55129:
    ; substitute (ys !-> ys)(x !-> x)(a0 !-> a0);
    ; #erase y
    cmp r8, 0
    je lab55132
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab55130
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab55131

lab55130:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab55131:

lab55132:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    mov rax, r10
    mov rdx, r11
    ; new a1: List[Unit] = (x, a0)\{ ... \};
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
    je lab55144
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab55145

lab55144:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55142
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55135
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55133
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55134

lab55133:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55134:

lab55135:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55138
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55136
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55137

lab55136:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55137:

lab55138:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55140

lab55139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55140:

lab55141:
    jmp lab55143

lab55142:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55143:

lab55145:
    ; #load tag
    lea rdi, [rel List_Unit_55146]
    ; jump rec_div2_
    jmp rec_div2_

List_Unit_55146:
    jmp near List_Unit_55146_Nil
    jmp near List_Unit_55146_Cons

List_Unit_55146_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab55149
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab55147
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55147:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab55148
    ; ####increment refcount
    add qword [rax + 0], 1

lab55148:
    jmp lab55150

lab55149:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab55150:
    ; let x0: List[Unit] = Nil();
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

List_Unit_55146_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab55153
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab55151
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55151:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab55152
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55152:
    jmp lab55154

lab55153:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab55154:
    ; substitute (a0 !-> a0)(x !-> x)(x1 !-> x1)(xs0 !-> xs0);
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
    ; let x0: List[Unit] = Cons(x1, xs0);
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
    je lab55166
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab55167

lab55166:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55164
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55156

lab55155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55156:

lab55157:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55159

lab55158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55159:

lab55160:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55163
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55161
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55162

lab55161:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55162:

lab55163:
    jmp lab55165

lab55164:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55165:

lab55167:
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

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: List[Unit] = (n, a0, iters)\{ ... \};
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
    je lab55179
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab55180

lab55179:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55177
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55170
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55168
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55169

lab55168:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55169:

lab55170:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55173
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55171
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55172

lab55171:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55172:

lab55173:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55176
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55174
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55175

lab55174:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55175:

lab55176:
    jmp lab55178

lab55177:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55178:

lab55180:
    ; #load tag
    lea rdi, [rel List_Unit_55181]
    ; new a3: List[Unit] = (a2)\{ ... \};
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
    je lab55193
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab55194

lab55193:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55191
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55184
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55182
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55183

lab55182:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55183:

lab55184:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55187
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55185
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55186

lab55185:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55186:

lab55187:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55190
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55188
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55189

lab55188:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55189:

lab55190:
    jmp lab55192

lab55191:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55192:

lab55194:
    ; #load tag
    lea rdi, [rel List_Unit_55195]
    ; jump create_n_
    jmp create_n_

List_Unit_55195:
    jmp near List_Unit_55195_Nil
    jmp near List_Unit_55195_Cons

List_Unit_55195_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab55197
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab55196
    ; ####increment refcount
    add qword [rax + 0], 1

lab55196:
    jmp lab55198

lab55197:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab55198:
    ; let x1: List[Unit] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x1 !-> x1)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump rec_div2_
    jmp rec_div2_

List_Unit_55195_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab55200
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab55199
    ; ####increment refcount
    add qword [r8 + 0], 1

lab55199:
    jmp lab55201

lab55200:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab55201:
    ; substitute (a2 !-> a2)(x7 !-> x7)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x1: List[Unit] = Cons(x7, xs1);
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
    je lab55213
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab55214

lab55213:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55211
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55204
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55202
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55203

lab55202:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55203:

lab55204:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55207
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55205
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55206

lab55205:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55206:

lab55207:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55210
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55208
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55209

lab55208:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55209:

lab55210:
    jmp lab55212

lab55211:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55212:

lab55214:
    ; #load tag
    mov rdi, 5
    ; substitute (x1 !-> x1)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump rec_div2_
    jmp rec_div2_

List_Unit_55181:
    jmp near List_Unit_55181_Nil
    jmp near List_Unit_55181_Cons

List_Unit_55181_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab55216
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab55215
    ; ####increment refcount
    add qword [rsi + 0], 1

lab55215:
    mov rdx, [rax + 24]
    jmp lab55217

lab55216:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab55217:
    ; let x0: List[Unit] = Nil();
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

List_Unit_55181_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab55219
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab55218
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55218:
    mov r9, [r8 + 24]
    jmp lab55220

lab55219:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab55220:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(x6 !-> x6)(xs0 !-> xs0);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: List[Unit] = Cons(x6, xs0);
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
    je lab55232
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab55233

lab55232:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55230
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55222

lab55221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55222:

lab55223:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55225

lab55224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55225:

lab55226:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55229
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55227
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55228

lab55227:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55228:

lab55229:
    jmp lab55231

lab55230:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55231:

lab55233:
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
    ; new a4: _Cont = (iters, n, a0)\{ ... \};
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
    je lab55245
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab55246

lab55245:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab55243
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab55236
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55234
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55235

lab55234:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55235:

lab55236:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab55239
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55237
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55238

lab55237:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55238:

lab55239:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab55242
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab55240
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab55241

lab55240:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab55241:

lab55242:
    jmp lab55244

lab55243:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab55244:

lab55246:
    ; #load tag
    lea rdi, [rel _Cont_55247]
    ; jump len_
    jmp len_

_Cont_55247:

_Cont_55247_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab55249
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab55248
    ; ####increment refcount
    add qword [r10 + 0], 1

lab55248:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab55250

lab55249:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab55250:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp rdi, r13
    je lab55251
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; lit x3 <- 1;
    mov r11, 1
    ; x4 <- iters - x3;
    mov r13, rdi
    sub r13, r11
    ; substitute (x4 !-> x4)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab55251:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rsi, r10
    mov rdi, r11
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
    ; lit x5 <- 0;
    mov rdi, 0
    ; substitute (x5 !-> x5)(a0 !-> a0);
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