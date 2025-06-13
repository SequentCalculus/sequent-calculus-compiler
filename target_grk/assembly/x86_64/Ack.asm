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
    ; new a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    lea r11, [rel _Cont_47164]
    ; jump main_loop_
    jmp main_loop_

_Cont_47164:

_Cont_47164_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

ack_:
    ; if m == 0 \{ ... \}
    cmp rdx, 0
    je lab47165
    ; if n == 0 \{ ... \}
    cmp rdi, 0
    je lab47166
    ; lit x4 <- 1;
    mov r11, 1
    ; x5 <- m - x4;
    mov r13, rdx
    sub r13, r11
    ; substitute (m !-> m)(n !-> n)(a0 !-> a0)(x5 !-> x5);
    ; #move variables
    mov r11, r13
    ; new a3: _Cont = (a0, x5)\{ ... \};
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
    je lab47178
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47179

lab47178:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47176
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47169
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47167
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47168

lab47167:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47168:

lab47169:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47172
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47170
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47171

lab47170:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47171:

lab47172:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47175
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47173
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47174

lab47173:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47174:

lab47175:
    jmp lab47177

lab47176:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47177:

lab47179:
    ; #load tag
    lea r9, [rel _Cont_47180]
    ; lit x7 <- 1;
    mov r11, 1
    ; x8 <- n - x7;
    mov r13, rdi
    sub r13, r11
    ; substitute (m !-> m)(x8 !-> x8)(a3 !-> a3);
    ; #move variables
    mov rdi, r13
    ; jump ack_
    jmp ack_

_Cont_47180:

_Cont_47180_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47182
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab47181
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47181:
    jmp lab47183

lab47182:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab47183:
    ; substitute (x5 !-> x5)(x6 !-> x6)(a0 !-> a0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump ack_
    jmp ack_

lab47166:
    ; substitute (m !-> m)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; lit x1 <- 1;
    mov r9, 1
    ; x2 <- m - x1;
    mov r11, rdx
    sub r11, r9
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x3 <- 1;
    mov r9, 1
    ; substitute (x2 !-> x2)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump ack_
    jmp ack_

lab47165:
    ; substitute (a0 !-> a0)(n !-> n);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; lit x0 <- 1;
    mov r9, 1
    ; x9 <- n + x0;
    mov r11, rdi
    add r11, r9
    ; substitute (x9 !-> x9)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(m0 !-> m)(n !-> n)(a0 !-> a0)(iters !-> iters)(m !-> m);
    ; #move variables
    mov r13, rdx
    mov r15, rdi
    mov rdx, r9
    ; new a2: _Cont = (n, a0, iters, m)\{ ... \};
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
    je lab47195
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47196

lab47195:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47193
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47185

lab47184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47185:

lab47186:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47189
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47187
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47188

lab47187:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47188:

lab47189:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47192
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47190
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47191

lab47190:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47191:

lab47192:
    jmp lab47194

lab47193:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47194:

lab47196:
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
    je lab47208
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47209

lab47208:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47206
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47199
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47197
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47198

lab47197:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47198:

lab47199:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47202
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47200
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47201

lab47200:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47201:

lab47202:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47205
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47203
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47204

lab47203:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47204:

lab47205:
    jmp lab47207

lab47206:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47207:

lab47209:
    ; #load tag
    lea r9, [rel _Cont_47210]
    ; substitute (m0 !-> m0)(n0 !-> n0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump ack_
    jmp ack_

_Cont_47210:

_Cont_47210_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47212
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
    je lab47211
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47211:
    jmp lab47213

lab47212:
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

lab47213:
    ; lit x0 <- 1;
    mov r15, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r15
    je lab47214
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

lab47214:
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