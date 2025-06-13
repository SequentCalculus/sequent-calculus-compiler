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
    lea r9, [rel _Cont_65046]
    ; jump main_loop_
    jmp main_loop_

_Cont_65046:

_Cont_65046_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

create_:
    ; if i < n \{ ... \}
    cmp rdx, rdi
    jl lab65047
    ; substitute (n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Leaf
    add rdi, 0
    jmp rdi

lab65047:
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- i + x0;
    mov r13, rdx
    add r13, r11
    ; substitute (x1 !-> x1)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; new a2: Tree[i64] = (a0)\{ ... \};
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
    je lab65059
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65060

lab65059:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65057
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65050
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65048
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65049

lab65048:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65049:

lab65050:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65053
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65051
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65052

lab65051:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65052:

lab65053:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65056
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65054
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65055

lab65054:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65055:

lab65056:
    jmp lab65058

lab65057:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65058:

lab65060:
    ; #load tag
    lea r9, [rel Tree_i64_65061]
    ; jump create_
    jmp create_

Tree_i64_65061:
    jmp near Tree_i64_65061_Leaf
    jmp near Tree_i64_65061_Node

Tree_i64_65061_Leaf:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65063
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab65062
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65062:
    jmp lab65064

lab65063:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab65064:
    ; substitute (a0 !-> a0)(x2 !-> x2);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; let t: Tree[i64] = Leaf(x2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdi
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65076
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65077

lab65076:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65074
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65067
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65065
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65066

lab65065:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65066:

lab65067:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65070
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65068
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65069

lab65068:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65069:

lab65070:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65073
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65071
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65072

lab65071:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65072:

lab65073:
    jmp lab65075

lab65074:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65075:

lab65077:
    ; #load tag
    mov rdi, 0
    ; substitute (t !-> t)(t0 !-> t)(a0 !-> a0);
    ; #share t
    cmp rsi, 0
    je lab65078
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65078:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Node
    add r9, 5
    jmp r9

Tree_i64_65061_Node:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65080
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab65079
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65079:
    jmp lab65081

lab65080:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab65081:
    ; substitute (a0 !-> a0)(left0 !-> left0)(right0 !-> right0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let t: Tree[i64] = Node(left0, right0);
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
    je lab65093
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65094

lab65093:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65091
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65083

lab65082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65083:

lab65084:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65087
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65085
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65086

lab65085:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65086:

lab65087:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65090
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65088
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65089

lab65088:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65089:

lab65090:
    jmp lab65092

lab65091:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65092:

lab65094:
    ; #load tag
    mov rdi, 5
    ; substitute (t !-> t)(t1 !-> t)(a0 !-> a0);
    ; #share t
    cmp rsi, 0
    je lab65095
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65095:
    ; #move variables
    mov r8, rax
    mov r9, rdx
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Node
    add r9, 5
    jmp r9

lookup_:
    ; substitute (a0 !-> a0)(t !-> t);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch t \{ ... \};
    lea rcx, [rel Tree_i64_65096]
    add rcx, rdi
    jmp rcx

Tree_i64_65096:
    jmp near Tree_i64_65096_Leaf
    jmp near Tree_i64_65096_Node

Tree_i64_65096_Leaf:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65097
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab65098

lab65097:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab65098:
    ; substitute (v !-> v)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Tree_i64_65096_Node:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65101
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65099
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65099:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65100
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65100:
    jmp lab65102

lab65101:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65102:
    ; substitute (left !-> left)(a0 !-> a0);
    ; #erase right
    cmp r8, 0
    je lab65105
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab65103
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab65104

lab65103:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab65104:

lab65105:
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lookup_
    jmp lookup_

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: Tree[i64] = (n, a0, iters)\{ ... \};
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
    je lab65117
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65118

lab65117:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65115
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65108
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65106
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65107

lab65106:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65107:

lab65108:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65111
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65109
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65110

lab65109:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65110:

lab65111:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65114
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65112
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65113

lab65112:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65113:

lab65114:
    jmp lab65116

lab65115:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65116:

lab65118:
    ; #load tag
    lea rdi, [rel Tree_i64_65119]
    ; lit x1 <- 0;
    mov r9, 0
    ; substitute (x1 !-> x1)(n0 !-> n0)(a2 !-> a2);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump create_
    jmp create_

Tree_i64_65119:
    jmp near Tree_i64_65119_Leaf
    jmp near Tree_i64_65119_Node

Tree_i64_65119_Leaf:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65121
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab65120
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65120:
    mov rdi, [rsi + 24]
    jmp lab65122

lab65121:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab65122:
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0)(x6 !-> x6);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: Tree[i64] = Leaf(x6);
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
    je lab65134
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65135

lab65134:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65132
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65125
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65123
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65124

lab65123:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65124:

lab65125:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65128
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65126
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65127

lab65126:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65127:

lab65128:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65131
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65129
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65130

lab65129:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65130:

lab65131:
    jmp lab65133

lab65132:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65133:

lab65135:
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

Tree_i64_65119_Node:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65137
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65136
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65136:
    mov r9, [r8 + 24]
    jmp lab65138

lab65137:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab65138:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(left0 !-> left0)(right0 !-> right0);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: Tree[i64] = Node(left0, right0);
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
    je lab65150
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65151

lab65150:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65148
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65140

lab65139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65140:

lab65141:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65144
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65142
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65143

lab65142:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65143:

lab65144:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65147
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65145
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65146

lab65145:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65146:

lab65147:
    jmp lab65149

lab65148:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65149:

lab65151:
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
    ; new a3: _Cont = (iters, n, a0)\{ ... \};
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
    je lab65163
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65164

lab65163:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65161
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65154
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65152
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65153

lab65152:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65153:

lab65154:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65156

lab65155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65156:

lab65157:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65159

lab65158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65159:

lab65160:
    jmp lab65162

lab65161:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65162:

lab65164:
    ; #load tag
    lea rdi, [rel _Cont_65165]
    ; jump lookup_
    jmp lookup_

_Cont_65165:

_Cont_65165_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65167
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab65166
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65166:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab65168

lab65167:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab65168:
    ; lit x2 <- 1;
    mov r13, 1
    ; if iters == x2 \{ ... \}
    cmp rdi, r13
    je lab65169
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

lab65169:
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