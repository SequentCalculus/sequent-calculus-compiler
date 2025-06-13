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
    mov rdx, rsi
    ; actual code

main_:
    ; new a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    lea rdi, [rel _Cont_63195]
    ; jump main_loop_
    jmp main_loop_

_Cont_63195:

_Cont_63195_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

int_max_:
    ; if i1 < i2 \{ ... \}
    cmp rdx, rdi
    jl lab63196
    ; substitute (i1 !-> i1)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

lab63196:
    ; substitute (i2 !-> i2)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; invoke a0 Ret
    jmp rdi

algb2_:
    ; substitute (x !-> x)(k0j1 !-> k0j1)(k1j1 !-> k1j1)(a0 !-> a0)(yss !-> yss);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; switch yss \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_63197]
    add rcx, r13
    jmp rcx

List_Pair_i64_i64_63197:
    jmp near List_Pair_i64_i64_63197_Nil
    jmp near List_Pair_i64_i64_63197_Cons

List_Pair_i64_i64_63197_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_63197_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab63200
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    cmp r14, 0
    je lab63198
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63198:
    mov r13, [r12 + 40]
    mov r12, [r12 + 32]
    cmp r12, 0
    je lab63199
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63199:
    jmp lab63201

lab63200:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    mov r13, [r12 + 40]
    mov r12, [r12 + 32]

lab63201:
    ; substitute (x !-> x)(k0j1 !-> k0j1)(k1j1 !-> k1j1)(a0 !-> a0)(ys !-> ys)(p !-> p);
    ; #move variables
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_63202:

Pair_i64_i64_63202_Pair:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab63203
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 40]
    jmp lab63204

lab63203:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 40]

lab63204:
    ; if x == y \{ ... \}
    cmp rdx, r15
    je lab63205
    ; substitute (k0j0 !-> k0j)(k1j1 !-> k1j1)(x !-> x)(a0 !-> a0)(ys !-> ys)(y !-> y)(k0j !-> k0j);
    ; #move variables
    mov rdi, r9
    mov r9, rdx
    mov rdx, [rsp + 2024]
    ; new a2: _Cont = (x, a0, ys, y, k0j)\{ ... \};
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
    je lab63217
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab63218

lab63217:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63215
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63208
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63206
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63207

lab63206:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63207:

lab63208:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63211
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63209
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63210

lab63209:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63210:

lab63211:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63214
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63212
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63213

lab63212:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63213:

lab63214:
    jmp lab63216

lab63215:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63216:

lab63218:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab63230
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63231

lab63230:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63228
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63221
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63219
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63220

lab63219:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63220:

lab63221:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63224
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63222
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63223

lab63222:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63223:

lab63224:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63227
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63225
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63226

lab63225:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63226:

lab63227:
    jmp lab63229

lab63228:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63229:

lab63231:
    ; #load tag
    lea r9, [rel _Cont_63232]
    ; substitute (k1j1 !-> k1j1)(k0j0 !-> k0j0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump int_max_
    jmp int_max_

_Cont_63232:

_Cont_63232_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63235
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab63233
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63233:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab63234
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63234:
    jmp lab63236

lab63235:
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
    mov r10, [r10 + 16]

lab63236:
    ; substitute (a0 !-> a0)(k0j !-> k0j)(kjcurr !-> kjcurr)(x !-> x)(y !-> y)(ys !-> ys);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r15
    mov r15, r11
    mov r11, rdi
    mov rdi, rcx
    mov rax, r8
    mov r14, r10
    ; jump share_algb2_0_
    jmp share_algb2_0_

lab63205:
    ; substitute (x !-> x)(k0j1 !-> k0j1)(k0j !-> k0j)(a0 !-> a0)(ys !-> ys)(y !-> y);
    ; #move variables
    mov r9, [rsp + 2024]
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; kjcurr <- k0j1 + x0;
    mov rcx, rdi
    add rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (a0 !-> a0)(k0j !-> k0j)(kjcurr !-> kjcurr)(x !-> x)(y !-> y)(ys !-> ys);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rdi, r9
    mov rax, r10
    mov r14, r12
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    mov r9, [rsp + 2008]
    ; jump share_algb2_0_
    jmp share_algb2_0_

share_algb2_0_:
    ; substitute (a0 !-> a0)(k0j !-> k0j)(kjcurr !-> kjcurr)(x !-> x)(ys !-> ys)(y !-> y)(kjcurr0 !-> kjcurr);
    ; #move variables
    mov [rsp + 2024], r9
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    mov r12, r14
    ; let x0: Pair[i64, i64] = Pair(y, kjcurr0);
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
    je lab63248
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab63249

lab63248:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63246
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63239
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63237
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63238

lab63237:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63238:

lab63239:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63242
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63240
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63241

lab63240:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63241:

lab63242:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63245
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63243
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63244

lab63243:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63244:

lab63245:
    jmp lab63247

lab63246:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63247:

lab63249:
    ; #load tag
    mov r15, 0
    ; substitute (ys !-> ys)(k0j !-> k0j)(kjcurr !-> kjcurr)(x !-> x)(a0 !-> a0)(x0 !-> x0);
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
    je lab63261
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab63262

lab63261:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63259
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63252
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63250
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63251

lab63250:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63251:

lab63252:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63255
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63253
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63254

lab63253:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63254:

lab63255:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63258
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63256
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63257

lab63256:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63257:

lab63258:
    jmp lab63260

lab63259:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63260:

lab63262:
    ; #load tag
    lea r13, [rel List_Pair_i64_i64_63263]
    ; substitute (x !-> x)(k0j !-> k0j)(kjcurr !-> kjcurr)(ys !-> ys)(a1 !-> a1);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump algb2_
    jmp algb2_

List_Pair_i64_i64_63263:
    jmp near List_Pair_i64_i64_63263_Nil
    jmp near List_Pair_i64_i64_63263_Cons

List_Pair_i64_i64_63263_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63266
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63264
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63264:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63265
    ; ####increment refcount
    add qword [rax + 0], 1

lab63265:
    jmp lab63267

lab63266:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63267:
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

List_Pair_i64_i64_63263_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63270
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63268
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63268:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63269
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63269:
    jmp lab63271

lab63270:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63271:
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
    je lab63283
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63284

lab63283:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63281
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63274
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63272
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63273

lab63272:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63273:

lab63274:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63277
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63275
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63276

lab63275:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63276:

lab63277:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63280
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63278
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63279

lab63278:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63279:

lab63280:
    jmp lab63282

lab63281:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63282:

lab63284:
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

snd_ii_:
    ; substitute (a0 !-> a0)(p !-> p);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_63285:

Pair_i64_i64_63285_Pair:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63286
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab63287

lab63286:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab63287:
    ; substitute (i2 !-> i2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

map_pair_:
    ; substitute (f !-> f)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_63288]
    add rcx, r9
    jmp rcx

List_Pair_i64_i64_63288:
    jmp near List_Pair_i64_i64_63288_Nil
    jmp near List_Pair_i64_i64_63288_Cons

List_Pair_i64_i64_63288_Nil:
    ; substitute (a0 !-> a0);
    ; #erase f
    cmp rax, 0
    je lab63291
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab63289
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab63290

lab63289:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab63290:

lab63291:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_Pair_i64_i64_63288_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63294
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63292
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63292:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63293
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63293:
    jmp lab63295

lab63294:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63295:
    ; substitute (f0 !-> f)(p !-> p)(a0 !-> a0)(ps !-> ps)(f !-> f);
    ; #share f
    cmp rax, 0
    je lab63296
    ; ####increment refcount
    add qword [rax + 0], 1

lab63296:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a1: _Cont = (a0, ps, f)\{ ... \};
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
    je lab63308
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63309

lab63308:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63306
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63299
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63297
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63298

lab63297:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63298:

lab63299:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63302
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63300
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63301

lab63300:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63301:

lab63302:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63305
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63303
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63304

lab63303:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63304:

lab63305:
    jmp lab63307

lab63306:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63307:

lab63309:
    ; #load tag
    lea r9, [rel _Cont_63310]
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

_Cont_63310:

_Cont_63310_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63314
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab63311
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63311:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab63312
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63312:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab63313
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63313:
    jmp lab63315

lab63314:
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

lab63315:
    ; substitute (f !-> f)(ps !-> ps)(a0 !-> a0)(x0 !-> x0);
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
    je lab63327
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63328

lab63327:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63325
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63318
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63316
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63317

lab63316:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63317:

lab63318:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63321
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63319
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63320

lab63319:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63320:

lab63321:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63323

lab63322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63323:

lab63324:
    jmp lab63326

lab63325:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63326:

lab63328:
    ; #load tag
    lea r9, [rel List_i64_63329]
    ; jump map_pair_
    jmp map_pair_

List_i64_63329:
    jmp near List_i64_63329_Nil
    jmp near List_i64_63329_Cons

List_i64_63329_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63331
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63330
    ; ####increment refcount
    add qword [rax + 0], 1

lab63330:
    jmp lab63332

lab63331:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63332:
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

List_i64_63329_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63334
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63333
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63333:
    jmp lab63335

lab63334:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63335:
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
    je lab63347
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63348

lab63347:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63345
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63338
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63336
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63337

lab63336:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63337:

lab63338:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63341
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63339
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63340

lab63339:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63340:

lab63341:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63344
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63342
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63343

lab63342:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63343:

lab63344:
    jmp lab63346

lab63345:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63346:

lab63348:
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

algb1_:
    ; substitute (a0 !-> a0)(yss !-> yss)(xss !-> xss);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch xss \{ ... \};
    lea rcx, [rel List_i64_63349]
    add rcx, r9
    jmp rcx

List_i64_63349:
    jmp near List_i64_63349_Nil
    jmp near List_i64_63349_Cons

List_i64_63349_Nil:
    ; new x0: Fun[Pair[i64, i64], i64] = ()\{ ... \};
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    lea r9, [rel Fun_Pair_i64_i64_i64_63350]
    ; substitute (x0 !-> x0)(yss !-> yss)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump map_pair_
    jmp map_pair_

Fun_Pair_i64_i64_i64_63350:

Fun_Pair_i64_i64_i64_63350_Ap:
    ; jump snd_ii_
    jmp snd_ii_

List_i64_63349_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63352
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63351
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63351:
    mov r9, [r8 + 40]
    jmp lab63353

lab63352:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63353:
    ; substitute (x !-> x)(yss !-> yss)(a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: List[Pair[i64, i64]] = (a0, xs)\{ ... \};
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
    je lab63365
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63366

lab63365:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63363
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63355

lab63354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63355:

lab63356:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63359
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63357
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63358

lab63357:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63358:

lab63359:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63362
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63360
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63361

lab63360:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63361:

lab63362:
    jmp lab63364

lab63363:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63364:

lab63366:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_63367]
    ; lit x2 <- 0;
    mov r11, 0
    ; lit x3 <- 0;
    mov r13, 0
    ; substitute (x !-> x)(x2 !-> x2)(x3 !-> x3)(yss !-> yss)(a2 !-> a2);
    ; #move variables
    mov r10, rsi
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov r12, r8
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; jump algb2_
    jmp algb2_

List_Pair_i64_i64_63367:
    jmp near List_Pair_i64_i64_63367_Nil
    jmp near List_Pair_i64_i64_63367_Cons

List_Pair_i64_i64_63367_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63370
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63368
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63368:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63369
    ; ####increment refcount
    add qword [rax + 0], 1

lab63369:
    jmp lab63371

lab63370:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63371:
    ; let x1: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (xs !-> xs)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump algb1_
    jmp algb1_

List_Pair_i64_i64_63367_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63374
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63372
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63372:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63373
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63373:
    jmp lab63375

lab63374:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63375:
    ; substitute (xs !-> xs)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
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
    ; let x1: List[Pair[i64, i64]] = Cons(a3, as0);
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
    je lab63387
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63388

lab63387:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63385
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63378
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63376
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63377

lab63376:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63377:

lab63378:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63381
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63379
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63380

lab63379:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63380:

lab63381:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63384
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63382
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63383

lab63382:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63383:

lab63384:
    jmp lab63386

lab63385:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63386:

lab63388:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump algb1_
    jmp algb1_

algb_listcomp_fun_:
    ; substitute (a0 !-> a0)(listcomp_fun_para !-> listcomp_fun_para);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch listcomp_fun_para \{ ... \};
    lea rcx, [rel List_i64_63389]
    add rcx, rdi
    jmp rcx

List_i64_63389:
    jmp near List_i64_63389_Nil
    jmp near List_i64_63389_Cons

List_i64_63389_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63389_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63391
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63390
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63390:
    mov rdi, [rsi + 40]
    jmp lab63392

lab63391:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63392:
    ; lit x1 <- 0;
    mov r11, 0
    ; substitute (a0 !-> a0)(listcomp_fun_ls_t !-> listcomp_fun_ls_t)(listcomp_fun_ls_h !-> listcomp_fun_ls_h)(x1 !-> x1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    ; let x0: Pair[i64, i64] = Pair(listcomp_fun_ls_h, x1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
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
    je lab63404
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63405

lab63404:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63402
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63395
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63393
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63394

lab63393:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63394:

lab63395:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63398
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63396
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63397

lab63396:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63397:

lab63398:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63401
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63399
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63400

lab63399:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63400:

lab63401:
    jmp lab63403

lab63402:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63403:

lab63405:
    ; #load tag
    mov r9, 0
    ; substitute (listcomp_fun_ls_t !-> listcomp_fun_ls_t)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a1: List[Pair[i64, i64]] = (a0, x0)\{ ... \};
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
    je lab63417
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab63418

lab63417:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63415
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63408
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63406
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63407

lab63406:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63407:

lab63408:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63411
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63409
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63410

lab63409:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63410:

lab63411:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63414
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63412
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63413

lab63412:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63413:

lab63414:
    jmp lab63416

lab63415:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63416:

lab63418:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_63419]
    ; jump algb_listcomp_fun_
    jmp algb_listcomp_fun_

List_Pair_i64_i64_63419:
    jmp near List_Pair_i64_i64_63419_Nil
    jmp near List_Pair_i64_i64_63419_Cons

List_Pair_i64_i64_63419_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63422
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63420
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63420:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63421
    ; ####increment refcount
    add qword [rax + 0], 1

lab63421:
    jmp lab63423

lab63422:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63423:
    ; let x2: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
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

List_Pair_i64_i64_63419_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63426
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63424
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63424:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63425
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63425:
    jmp lab63427

lab63426:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63427:
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
    ; let x2: List[Pair[i64, i64]] = Cons(a2, as0);
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
    je lab63439
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63440

lab63439:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63437
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63430
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63428
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63429

lab63428:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63429:

lab63430:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63433
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63431
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63432

lab63431:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63432:

lab63433:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63436
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63434
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63435

lab63434:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63435:

lab63436:
    jmp lab63438

lab63437:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63438:

lab63440:
    ; #load tag
    mov r9, 5
    ; substitute (x0 !-> x0)(x2 !-> x2)(a0 !-> a0);
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

algb_:
    ; lit x0 <- 0;
    mov r11, 0
    ; new a1: List[i64] = (a0, x0)\{ ... \};
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
    je lab63452
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63453

lab63452:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63450
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63443
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63441
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63442

lab63441:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63442:

lab63443:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63446
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63444
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63445

lab63444:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63445:

lab63446:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63449
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63447
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63448

lab63447:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63448:

lab63449:
    jmp lab63451

lab63450:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63451:

lab63453:
    ; #load tag
    lea r9, [rel List_i64_63454]
    ; substitute (ys !-> ys)(xs !-> xs)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a2: List[Pair[i64, i64]] = (xs, a1)\{ ... \};
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
    je lab63466
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab63467

lab63466:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63464
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63457
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63455
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63456

lab63455:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63456:

lab63457:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63460
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63458
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63459

lab63458:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63459:

lab63460:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63463
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63461
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63462

lab63461:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63462:

lab63463:
    jmp lab63465

lab63464:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63465:

lab63467:
    ; #load tag
    lea rdi, [rel List_Pair_i64_i64_63468]
    ; jump algb_listcomp_fun_
    jmp algb_listcomp_fun_

List_Pair_i64_i64_63468:
    jmp near List_Pair_i64_i64_63468_Nil
    jmp near List_Pair_i64_i64_63468_Cons

List_Pair_i64_i64_63468_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63471
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63469
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63469:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63470
    ; ####increment refcount
    add qword [rax + 0], 1

lab63470:
    jmp lab63472

lab63471:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63472:
    ; let x2: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (xs !-> xs)(x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump algb1_
    jmp algb1_

List_Pair_i64_i64_63468_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63475
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63473
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63473:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63474
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63474:
    jmp lab63476

lab63475:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63476:
    ; substitute (a1 !-> a1)(xs !-> xs)(a4 !-> a4)(as1 !-> as1);
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
    ; let x2: List[Pair[i64, i64]] = Cons(a4, as1);
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
    je lab63488
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63489

lab63488:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63486
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63479
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63477
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63478

lab63477:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63478:

lab63479:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63481

lab63480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63481:

lab63482:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63484

lab63483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63484:

lab63485:
    jmp lab63487

lab63486:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63487:

lab63489:
    ; #load tag
    mov r9, 5
    ; substitute (xs !-> xs)(x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump algb1_
    jmp algb1_

List_i64_63454:
    jmp near List_i64_63454_Nil
    jmp near List_i64_63454_Cons

List_i64_63454_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63491
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63490
    ; ####increment refcount
    add qword [rax + 0], 1

lab63490:
    jmp lab63492

lab63491:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63492:
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

List_i64_63454_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63494
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63493
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63493:
    jmp lab63495

lab63494:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63495:
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
    je lab63507
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63508

lab63507:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63505
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63498
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63496
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63497

lab63496:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63497:

lab63498:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63501
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63499
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63500

lab63499:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63500:

lab63501:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63504
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63502
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63503

lab63502:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63503:

lab63504:
    jmp lab63506

lab63505:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63506:

lab63508:
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

findk_:
    ; substitute (k !-> k)(km !-> km)(m !-> m)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_Pair_i64_i64_63509]
    add rcx, r13
    jmp rcx

List_Pair_i64_i64_63509:
    jmp near List_Pair_i64_i64_63509_Nil
    jmp near List_Pair_i64_i64_63509_Cons

List_Pair_i64_i64_63509_Nil:
    ; substitute (km !-> km)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r10
    mov rdi, r11
    ; invoke a0 Ret
    jmp rdi

List_Pair_i64_i64_63509_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r12 + 0], 0
    je lab63512
    ; ##either decrement refcount and share children...
    add qword [r12 + 0], -1
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    cmp r14, 0
    je lab63510
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63510:
    mov r13, [r12 + 40]
    mov r12, [r12 + 32]
    cmp r12, 0
    je lab63511
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63511:
    jmp lab63513

lab63512:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov r15, [r12 + 56]
    mov r14, [r12 + 48]
    mov r13, [r12 + 40]
    mov r12, [r12 + 32]

lab63513:
    ; substitute (k !-> k)(km !-> km)(m !-> m)(a0 !-> a0)(xys !-> xys)(p !-> p);
    ; #move variables
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; switch p \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_63514:

Pair_i64_i64_63514_Pair:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab63515
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 40]
    jmp lab63516

lab63515:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 40]

lab63516:
    ; x0 <- x + y;
    mov rcx, r15
    add rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; if m <= x0 \{ ... \}
    cmp r9, [rsp +2008]
    jle lab63517
    ; substitute (k !-> k)(km !-> km)(m !-> m)(a0 !-> a0)(xys !-> xys);
    ; lit x4 <- 1;
    mov r15, 1
    ; x5 <- k + x4;
    mov rcx, rdx
    add rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x5 !-> x5)(km !-> km)(m !-> m)(xys !-> xys)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    mov rdx, [rsp + 2024]
    ; jump findk_
    jmp findk_

lab63517:
    ; substitute (k !-> k)(y !-> y)(x !-> x)(a0 !-> a0)(xys !-> xys);
    ; #move variables
    mov r9, r15
    mov rdi, [rsp + 2024]
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- k + x1;
    mov rcx, rdx
    add rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (k !-> k)(y !-> y)(x !-> x)(a0 !-> a0)(xys !-> xys)(x2 !-> x2);
    ; #move variables
    mov r15, [rsp + 2024]
    ; x3 <- x + y;
    mov rcx, r9
    add rcx, rdi
    mov [rsp + 2024], rcx
    ; substitute (x2 !-> x2)(k !-> k)(x3 !-> x3)(xys !-> xys)(a0 !-> a0);
    ; #move variables
    mov rdi, rdx
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    mov rdx, r15
    mov r9, [rsp + 2024]
    ; jump findk_
    jmp findk_

is_nil_:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63518]
    add rcx, rdi
    jmp rcx

List_i64_63518:
    jmp near List_i64_63518_Nil
    jmp near List_i64_63518_Cons

List_i64_63518_Nil:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_i64_63518_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63520
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63519
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63519:
    mov rdi, [rsi + 40]
    jmp lab63521

lab63520:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63521:
    ; substitute (a0 !-> a0);
    ; #erase xs
    cmp r8, 0
    je lab63524
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab63522
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab63523

lab63522:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab63523:

lab63524:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

is_singleton_:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63525]
    add rcx, rdi
    jmp rcx

List_i64_63525:
    jmp near List_i64_63525_Nil
    jmp near List_i64_63525_Cons

List_i64_63525_Nil:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

List_i64_63525_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63527
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63526
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63526:
    mov rdi, [rsi + 40]
    jmp lab63528

lab63527:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63528:
    ; switch xs \{ ... \};
    lea rcx, [rel List_i64_63529]
    add rcx, r9
    jmp rcx

List_i64_63529:
    jmp near List_i64_63529_Nil
    jmp near List_i64_63529_Cons

List_i64_63529_Nil:
    ; substitute (x !-> x)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Some
    add rdi, 5
    jmp rdi

List_i64_63529_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63531
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63530
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63530:
    mov r9, [r8 + 40]
    jmp lab63532

lab63531:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63532:
    ; substitute (a0 !-> a0);
    ; #erase xs0
    cmp r10, 0
    je lab63535
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab63533
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab63534

lab63533:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab63534:

lab63535:
    ; invoke a0 None
    add rdx, 0
    jmp rdx

take_:
    ; substitute (n !-> n)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63536]
    add rcx, r9
    jmp rcx

List_i64_63536:
    jmp near List_i64_63536_Nil
    jmp near List_i64_63536_Cons

List_i64_63536_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63536_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63538
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63537
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63537:
    mov r9, [r8 + 40]
    jmp lab63539

lab63538:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63539:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab63540
    ; substitute (n !-> n)(is !-> is)(i !-> i)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a1: List[i64] = (i, a0)\{ ... \};
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
    je lab63552
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63553

lab63552:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63550
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63543
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63541
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63542

lab63541:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63542:

lab63543:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63546
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63544
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63545

lab63544:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63545:

lab63546:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63549
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63547
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63548

lab63547:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63548:

lab63549:
    jmp lab63551

lab63550:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63551:

lab63553:
    ; #load tag
    lea r9, [rel List_i64_63554]
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- n - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(is !-> is)(a1 !-> a1);
    ; #move variables
    mov rdx, r13
    ; jump take_
    jmp take_

List_i64_63554:
    jmp near List_i64_63554_Nil
    jmp near List_i64_63554_Cons

List_i64_63554_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63556
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63555
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63555:
    mov rdx, [rax + 40]
    jmp lab63557

lab63556:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab63557:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (i !-> i)(x0 !-> x0)(a0 !-> a0);
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

List_i64_63554_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63559
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63558
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63558:
    mov r9, [r8 + 40]
    jmp lab63560

lab63559:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63560:
    ; substitute (a0 !-> a0)(i !-> i)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
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
    je lab63572
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63573

lab63572:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63570
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63562

lab63561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63562:

lab63563:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63565

lab63564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63565:

lab63566:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63569
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63567
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63568

lab63567:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63568:

lab63569:
    jmp lab63571

lab63570:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63571:

lab63573:
    ; #load tag
    mov r9, 5
    ; substitute (i !-> i)(x0 !-> x0)(a0 !-> a0);
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

lab63540:
    ; substitute (a0 !-> a0);
    ; #erase is
    cmp r10, 0
    je lab63576
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab63574
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab63575

lab63574:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab63575:

lab63576:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

drop_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab63577
    ; substitute (n !-> n)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63578]
    add rcx, r9
    jmp rcx

List_i64_63578:
    jmp near List_i64_63578_Nil
    jmp near List_i64_63578_Cons

List_i64_63578_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63578_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63580
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63579
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63579:
    mov r9, [r8 + 40]
    jmp lab63581

lab63580:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63581:
    ; substitute (n !-> n)(a0 !-> a0)(is !-> is);
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (x1 !-> x1)(is !-> is)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rdx, r13
    ; jump drop_
    jmp drop_

lab63577:
    ; substitute (a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63582]
    add rcx, rdi
    jmp rcx

List_i64_63582:
    jmp near List_i64_63582_Nil
    jmp near List_i64_63582_Cons

List_i64_63582_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63582_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63584
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63583
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63583:
    mov rdi, [rsi + 40]
    jmp lab63585

lab63584:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63585:
    ; substitute (a2 !-> a2)(as0 !-> as0)(a0 !-> a0);
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

rev_loop_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_63586]
    add rcx, r9
    jmp rcx

List_i64_63586:
    jmp near List_i64_63586_Nil
    jmp near List_i64_63586_Cons

List_i64_63586_Nil:
    ; switch acc \{ ... \};
    lea rcx, [rel List_i64_63587]
    add rcx, rdi
    jmp rcx

List_i64_63587:
    jmp near List_i64_63587_Nil
    jmp near List_i64_63587_Cons

List_i64_63587_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63587_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63589
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63588
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63588:
    mov rdi, [rsi + 40]
    jmp lab63590

lab63589:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63590:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

List_i64_63586_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63592
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63591
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63591:
    mov r9, [r8 + 40]
    jmp lab63593

lab63592:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63593:
    ; substitute (a0 !-> a0)(is !-> is)(i !-> i)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[i64] = Cons(i, acc);
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
    je lab63605
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63606

lab63605:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63603
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63596
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63594
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63595

lab63594:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63595:

lab63596:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63599
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63597
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63598

lab63597:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63598:

lab63599:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63602
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63600
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63601

lab63600:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63601:

lab63602:
    jmp lab63604

lab63603:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63604:

lab63606:
    ; #load tag
    mov r9, 5
    ; substitute (is !-> is)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump rev_loop_
    jmp rev_loop_

reverse_:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump rev_loop_
    jmp rev_loop_

zip_:
    ; substitute (a0 !-> a0)(l2 !-> l2)(l1 !-> l1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_i64_63607]
    add rcx, r9
    jmp rcx

List_i64_63607:
    jmp near List_i64_63607_Nil
    jmp near List_i64_63607_Cons

List_i64_63607_Nil:
    ; substitute (a0 !-> a0);
    ; #erase l2
    cmp rsi, 0
    je lab63610
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63608
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63609

lab63608:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63609:

lab63610:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63607_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63612
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63611
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63611:
    mov r9, [r8 + 40]
    jmp lab63613

lab63612:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63613:
    ; substitute (a0 !-> a0)(is1 !-> is1)(i1 !-> i1)(l2 !-> l2);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; switch l2 \{ ... \};
    lea rcx, [rel List_i64_63614]
    add rcx, r11
    jmp rcx

List_i64_63614:
    jmp near List_i64_63614_Nil
    jmp near List_i64_63614_Cons

List_i64_63614_Nil:
    ; substitute (a0 !-> a0);
    ; #erase is1
    cmp rsi, 0
    je lab63617
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63615
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63616

lab63615:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63616:

lab63617:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_63614_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab63619
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab63618
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63618:
    mov r11, [r10 + 40]
    jmp lab63620

lab63619:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab63620:
    ; substitute (a0 !-> a0)(is1 !-> is1)(is2 !-> is2)(i1 !-> i1)(i2 !-> i2);
    ; #move variables
    mov rcx, r13
    mov r13, r11
    mov r11, r9
    mov r9, rcx
    mov r8, r12
    ; let x0: Pair[i64, i64] = Pair(i1, i2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63632
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab63633

lab63632:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63630
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63622

lab63621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63622:

lab63623:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63626
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63624
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63625

lab63624:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63625:

lab63626:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63629
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63627
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63628

lab63627:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63628:

lab63629:
    jmp lab63631

lab63630:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63631:

lab63633:
    ; #load tag
    mov r11, 0
    ; substitute (is2 !-> is2)(is1 !-> is1)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[Pair[i64, i64]] = (a0, x0)\{ ... \};
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
    je lab63645
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63646

lab63645:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63643
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63635

lab63634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63635:

lab63636:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63639
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63637
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63638

lab63637:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63638:

lab63639:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63642
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63640
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63641

lab63640:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63641:

lab63642:
    jmp lab63644

lab63643:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63644:

lab63646:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_63647]
    ; substitute (is1 !-> is1)(is2 !-> is2)(a1 !-> a1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump zip_
    jmp zip_

List_Pair_i64_i64_63647:
    jmp near List_Pair_i64_i64_63647_Nil
    jmp near List_Pair_i64_i64_63647_Cons

List_Pair_i64_i64_63647_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63650
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63648
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63648:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63649
    ; ####increment refcount
    add qword [rax + 0], 1

lab63649:
    jmp lab63651

lab63650:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63651:
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

List_Pair_i64_i64_63647_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63654
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63652
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63652:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63653
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63653:
    jmp lab63655

lab63654:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab63655:
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
    je lab63667
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63668

lab63667:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63665
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63658
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63656
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63657

lab63656:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63657:

lab63658:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63661
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63659
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63660

lab63659:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63660:

lab63661:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63664
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63662
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63663

lab63662:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63663:

lab63664:
    jmp lab63666

lab63665:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63666:

lab63668:
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

in_list_:
    ; substitute (x !-> x)(a0 !-> a0)(ls !-> ls);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_63669]
    add rcx, r9
    jmp rcx

List_i64_63669:
    jmp near List_i64_63669_Nil
    jmp near List_i64_63669_Cons

List_i64_63669_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

List_i64_63669_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63671
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab63670
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63670:
    mov r9, [r8 + 40]
    jmp lab63672

lab63671:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab63672:
    ; if i == x \{ ... \}
    cmp r9, rdx
    je lab63673
    ; substitute (x !-> x)(is !-> is)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov r9, rdi
    mov rsi, r10
    mov rdi, r11
    ; jump in_list_
    jmp in_list_

lab63673:
    ; substitute (a0 !-> a0);
    ; #erase is
    cmp r10, 0
    je lab63676
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab63674
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab63675

lab63674:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab63675:

lab63676:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 True
    add rdx, 0
    jmp rdx

algc_:
    ; substitute (ys0 !-> ys)(n !-> n)(xs !-> xs)(ys !-> ys)(a0 !-> a0)(m !-> m);
    ; #share ys
    cmp r10, 0
    je lab63677
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63677:
    ; #move variables
    mov r15, rdx
    mov rax, r10
    mov rdx, r11
    ; new a16: Bool = (n, xs, ys, a0, m)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab63689
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab63690

lab63689:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63687
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63680
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63678
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63679

lab63678:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63679:

lab63680:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63683
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63681
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63682

lab63681:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63682:

lab63683:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63686
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63684
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63685

lab63684:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63685:

lab63686:
    jmp lab63688

lab63687:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63688:

lab63690:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab63702
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab63703

lab63702:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63700
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63693
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63691
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63692

lab63691:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63692:

lab63693:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63695

lab63694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63695:

lab63696:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63698

lab63697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63698:

lab63699:
    jmp lab63701

lab63700:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63701:

lab63703:
    ; #load tag
    lea rdi, [rel Bool_63704]
    ; jump is_nil_
    jmp is_nil_

Bool_63704:
    jmp near Bool_63704_True
    jmp near Bool_63704_False

Bool_63704_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63708
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab63705
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63705:
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab63706
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63706:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63707
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63707:
    jmp lab63709

lab63708:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab63709:
    ; substitute (a0 !-> a0);
    ; #erase xs
    cmp rsi, 0
    je lab63712
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63710
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63711

lab63710:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63711:

lab63712:
    ; #erase ys
    cmp r8, 0
    je lab63715
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab63713
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab63714

lab63713:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab63714:

lab63715:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_List_i64_List_i64_63716:

Fun_List_i64_List_i64_63716_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63719
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63717
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63717:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63718
    ; ####increment refcount
    add qword [rax + 0], 1

lab63718:
    jmp lab63720

lab63719:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63720:
    ; substitute (a1 !-> a1)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    lea rcx, [rel List_i64_63721]
    add rcx, rdi
    jmp rcx

List_i64_63721:
    jmp near List_i64_63721_Nil
    jmp near List_i64_63721_Cons

List_i64_63721_Nil:
    ; invoke a1 Nil
    add rdx, 0
    jmp rdx

List_i64_63721_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63723
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63722
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63722:
    mov rdi, [rsi + 40]
    jmp lab63724

lab63723:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63724:
    ; substitute (a24 !-> a24)(as0 !-> as0)(a1 !-> a1);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a1 Cons
    add r9, 5
    jmp r9

Bool_63704_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63728
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab63725
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63725:
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab63726
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63726:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63727
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63727:
    jmp lab63729

lab63728:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab63729:
    ; substitute (xs3 !-> xs)(xs !-> xs)(ys !-> ys)(a0 !-> a0)(m !-> m)(n !-> n);
    ; #share xs
    cmp rsi, 0
    je lab63730
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63730:
    ; #move variables
    mov r15, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a17: Option[i64] = (xs, ys, a0, m, n)\{ ... \};
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
    je lab63742
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab63743

lab63742:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63740
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63733
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63731
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63732

lab63731:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63732:

lab63733:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63736
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63734
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63735

lab63734:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63735:

lab63736:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63739
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63737
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63738

lab63737:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63738:

lab63739:
    jmp lab63741

lab63740:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63741:

lab63743:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63755
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab63756

lab63755:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63753
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63746
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63744
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63745

lab63744:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63745:

lab63746:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63749
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63747
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63748

lab63747:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63748:

lab63749:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63752
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63750
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63751

lab63750:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63751:

lab63752:
    jmp lab63754

lab63753:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63754:

lab63756:
    ; #load tag
    lea rdi, [rel Option_i64_63757]
    ; jump is_singleton_
    jmp is_singleton_

Option_i64_63757:
    jmp near Option_i64_63757_None
    jmp near Option_i64_63757_Some

Option_i64_63757_None:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63761
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab63758
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63758:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab63759
    ; ####increment refcount
    add qword [rax + 0], 1

lab63759:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63760
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63760:
    jmp lab63762

lab63761:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab63762:
    ; lit x1 <- 2;
    mov r15, 2
    ; m2 <- m / x1;
    mov rcx, rdx
    mov [rsp + 2024], rax
    mov rax, r11
    cqo
    idiv r15
    mov rdx, rax
    mov rax, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    ; substitute (xs0 !-> xs)(m20 !-> m2)(a0 !-> a0)(m !-> m)(n !-> n)(ys !-> ys)(m2 !-> m2)(xs !-> xs);
    ; #share xs
    cmp rax, 0
    je lab63763
    ; ####increment refcount
    add qword [rax + 0], 1

lab63763:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov r14, rsi
    mov r15, rdi
    mov rdi, [rsp + 2024]
    ; new a18: List[i64] = (a0, m, n, ys, m2, xs)\{ ... \};
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
    mov [rbx + 16], r14
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63775
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab63776

lab63775:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63773
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63766
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63764
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63765

lab63764:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63765:

lab63766:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63769
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63767
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63768

lab63767:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63768:

lab63769:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63772
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63770
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63771

lab63770:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63771:

lab63772:
    jmp lab63774

lab63773:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63774:

lab63776:
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
    je lab63788
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab63789

lab63788:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63786
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63779
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63777
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63778

lab63777:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63778:

lab63779:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63782
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63780
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63781

lab63780:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63781:

lab63782:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63785
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63783
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63784

lab63783:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63784:

lab63785:
    jmp lab63787

lab63786:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63787:

lab63789:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab63801
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63802

lab63801:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63799
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63792
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63790
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63791

lab63790:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63791:

lab63792:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63795
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63793
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63794

lab63793:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63794:

lab63795:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63798
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63796
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63797

lab63796:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63797:

lab63798:
    jmp lab63800

lab63799:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63800:

lab63802:
    ; #load tag
    lea r9, [rel List_i64_63803]
    ; substitute (m20 !-> m20)(xs0 !-> xs0)(a18 !-> a18);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump take_
    jmp take_

List_i64_63803:
    jmp near List_i64_63803_Nil
    jmp near List_i64_63803_Cons

List_i64_63803_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63807
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63804
    ; ####increment refcount
    add qword [rax + 0], 1

lab63804:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab63805
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63805:
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab63806
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63806:
    jmp lab63808

lab63807:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab63808:
    ; let xs1: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs !-> xs)(xs1 !-> xs1)(ys !-> ys);
    ; #move variables
    mov rcx, r13
    mov r13, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, r9
    mov r9, rcx
    mov r12, r14
    mov r14, [rsp + 2032]
    mov [rsp + 2032], r10
    ; jump lift_algc_0_
    jmp lift_algc_0_

List_i64_63803_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63812
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab63809
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63809:
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
    je lab63810
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63810:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab63811
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63811:
    jmp lab63813

lab63812:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
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
    mov r14, [r14 + 16]

lab63813:
    ; substitute (xs !-> xs)(m2 !-> m2)(a0 !-> a0)(m !-> m)(n !-> n)(ys !-> ys)(a37 !-> a37)(as11 !-> as11);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rax, [rsp + 2016]
    mov [rsp + 2016], rsi
    ; let xs1: List[i64] = Cons(a37, as11);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63825
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab63826

lab63825:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63823
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63816
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63814
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63815

lab63814:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63815:

lab63816:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63819
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63817
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63818

lab63817:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63818:

lab63819:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63822
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63820
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63821

lab63820:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63821:

lab63822:
    jmp lab63824

lab63823:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63824:

lab63826:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs !-> xs)(xs1 !-> xs1)(ys !-> ys);
    ; #move variables
    mov r12, rax
    mov rcx, r9
    mov r9, rdi
    mov rdi, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r8
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], r14
    mov r14, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r15
    mov r15, rcx
    ; jump lift_algc_0_
    jmp lift_algc_0_

Option_i64_63757_Some:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63830
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab63827
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63827:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab63828
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63828:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab63829
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63829:
    jmp lab63831

lab63830:
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
    mov rsi, [rsi + 16]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab63831:
    ; substitute (x14 !-> x)(ys !-> ys)(x !-> x)(a0 !-> a0);
    ; #erase xs
    cmp rsi, 0
    je lab63834
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab63832
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab63833

lab63832:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab63833:

lab63834:
    ; #move variables
    mov rdi, r9
    mov r9, rdx
    mov rsi, r8
    ; new a23: Bool = (x, a0)\{ ... \};
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
    je lab63846
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63847

lab63846:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63844
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63837
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63835
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63836

lab63835:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63836:

lab63837:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63840
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63838
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63839

lab63838:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63839:

lab63840:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63843
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63841
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63842

lab63841:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63842:

lab63843:
    jmp lab63845

lab63844:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63845:

lab63847:
    ; #load tag
    lea r9, [rel Bool_63848]
    ; jump in_list_
    jmp in_list_

Bool_63848:
    jmp near Bool_63848_True
    jmp near Bool_63848_False

Bool_63848_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63850
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63849
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63849:
    mov rdx, [rax + 40]
    jmp lab63851

lab63850:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab63851:
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_List_i64_List_i64_63852:

Fun_List_i64_List_i64_63852_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63855
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63853
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63853:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab63854
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63854:
    jmp lab63856

lab63855:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab63856:
    ; invoke a14 Cons
    add r9, 5
    jmp r9

Bool_63848_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63858
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63857
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63857:
    mov rdx, [rax + 40]
    jmp lab63859

lab63858:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab63859:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_List_i64_List_i64_63860:

Fun_List_i64_List_i64_63860_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63863
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab63861
    ; ####increment refcount
    add qword [rsi + 0], 1

lab63861:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab63862
    ; ####increment refcount
    add qword [rax + 0], 1

lab63862:
    jmp lab63864

lab63863:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab63864:
    ; substitute (a15 !-> a15)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x0 \{ ... \};
    lea rcx, [rel List_i64_63865]
    add rcx, rdi
    jmp rcx

List_i64_63865:
    jmp near List_i64_63865_Nil
    jmp near List_i64_63865_Cons

List_i64_63865_Nil:
    ; invoke a15 Nil
    add rdx, 0
    jmp rdx

List_i64_63865_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab63867
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab63866
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63866:
    mov rdi, [rsi + 40]
    jmp lab63868

lab63867:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab63868:
    ; substitute (a38 !-> a38)(as12 !-> as12)(a15 !-> a15);
    ; #move variables
    mov rsi, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a15 Cons
    add r9, 5
    jmp r9

lift_algc_0_:
    ; substitute (xs !-> xs)(m20 !-> m2)(m2 !-> m2)(n !-> n)(m !-> m)(xs1 !-> xs1)(ys !-> ys)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov rdx, r13
    mov r13, rdi
    mov rdi, r9
    mov rax, r12
    ; new a19: List[i64] = (m2, n, m, xs1, ys, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    mov [rbx + 24], r15
    mov [rbx + 16], r14
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63880
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab63881

lab63880:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63878
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63870

lab63869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63870:

lab63871:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63874
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63872
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63873

lab63872:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63873:

lab63874:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63876

lab63875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63876:

lab63877:
    jmp lab63879

lab63878:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63879:

lab63881:
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
    je lab63893
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab63894

lab63893:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63891
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63884
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63882
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63883

lab63882:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63883:

lab63884:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63886

lab63885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63886:

lab63887:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63889

lab63888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63889:

lab63890:
    jmp lab63892

lab63891:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63892:

lab63894:
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
    je lab63906
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63907

lab63906:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63904
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63897
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63895
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63896

lab63895:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63896:

lab63897:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63899

lab63898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63899:

lab63900:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63902

lab63901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63902:

lab63903:
    jmp lab63905

lab63904:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63905:

lab63907:
    ; #load tag
    lea r9, [rel List_i64_63908]
    ; substitute (m20 !-> m20)(xs !-> xs)(a19 !-> a19);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump drop_
    jmp drop_

List_i64_63908:
    jmp near List_i64_63908_Nil
    jmp near List_i64_63908_Cons

List_i64_63908_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63912
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab63909
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63909:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab63910
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63910:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab63911
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63911:
    jmp lab63913

lab63912:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
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

lab63913:
    ; let xs2: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], r13
    mov r13, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, [rsp + 2032]
    mov [rsp + 2032], r12
    mov r12, r10
    ; jump lift_algc_1_
    jmp lift_algc_1_

List_i64_63908_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63917
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
    je lab63914
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63914:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab63915
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63915:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab63916
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63916:
    jmp lab63918

lab63917:
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
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]

lab63918:
    ; substitute (a0 !-> a0)(ys !-> ys)(m2 !-> m2)(n !-> n)(m !-> m)(xs1 !-> xs1)(a36 !-> a36)(as10 !-> as10);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rax, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, [rsp + 2032]
    ; let xs2: List[i64] = Cons(a36, as10);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63930
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab63931

lab63930:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63928
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63921
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63919
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63920

lab63919:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63920:

lab63921:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63924
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63922
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63923

lab63922:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63923:

lab63924:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63927
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63925
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63926

lab63925:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63926:

lab63927:
    jmp lab63929

lab63928:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63929:

lab63931:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov r12, r14
    mov r14, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rcx, r13
    mov r13, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    ; jump lift_algc_1_
    jmp lift_algc_1_

lift_algc_1_:
    ; substitute (ys0 !-> ys)(xs10 !-> xs1)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(a0 !-> a0)(m !-> m);
    ; #share xs1
    cmp r12, 0
    je lab63932
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63932:
    ; #share ys
    cmp qword [rsp + 2032], 0
    je lab63933
    mov rcx, [rsp + 2032]
    add qword [rcx + 0], 1

lab63933:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 1992], rdi
    mov rsi, r12
    mov rdi, r13
    mov rax, [rsp + 2032]
    mov rdx, [rsp + 2024]
    ; new a20: List[i64] = (m2, n, xs1, xs2, ys, a0, m)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63945
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab63946

lab63945:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63943
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63936
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63934
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63935

lab63934:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63935:

lab63936:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63939
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63938

lab63937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63938:

lab63939:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63942
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63940
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63941

lab63940:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63941:

lab63942:
    jmp lab63944

lab63943:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63944:

lab63946:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63958
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab63959

lab63958:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63956
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63949
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63947
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63948

lab63947:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63948:

lab63949:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63952
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63950
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63951

lab63950:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63951:

lab63952:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63955
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63953
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63954

lab63953:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63954:

lab63955:
    jmp lab63957

lab63956:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63957:

lab63959:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab63971
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab63972

lab63971:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63969
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63961

lab63960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63961:

lab63962:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63965
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63963
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63964

lab63963:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63964:

lab63965:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63968
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63966
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63967

lab63966:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63967:

lab63968:
    jmp lab63970

lab63969:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63970:

lab63972:
    ; #load tag
    lea r9, [rel List_i64_63973]
    ; substitute (xs10 !-> xs10)(ys0 !-> ys0)(a20 !-> a20);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump algb_
    jmp algb_

List_i64_63973:
    jmp near List_i64_63973_Nil
    jmp near List_i64_63973_Cons

List_i64_63973_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab63978
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab63974
    ; ####increment refcount
    add qword [r10 + 0], 1

lab63974:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab63975
    ; ####increment refcount
    add qword [r8 + 0], 1

lab63975:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab63976
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63976:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab63977
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63977:
    jmp lab63979

lab63978:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
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
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab63979:
    ; let l1: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (a0 !-> a0)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, r15
    mov r15, r9
    mov r9, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, rdi
    mov rdi, rcx
    mov rax, r14
    mov r14, r8
    mov [rsp + 2032], r10
    mov rsi, [rsp + 2016]
    mov [rsp + 2016], r12
    ; jump lift_algc_2_
    jmp lift_algc_2_

List_i64_63973_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab63984
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab63980
    ; ####increment refcount
    add qword [r14 + 0], 1

lab63980:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab63981
    ; ####increment refcount
    add qword [r12 + 0], 1

lab63981:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab63982
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63982:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab63983
    ; ####increment refcount
    add qword [rcx + 0], 1

lab63983:
    mov rax, [rsp + 2040]
    jmp lab63985

lab63984:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab63985:
    ; substitute (m !-> m)(a0 !-> a0)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(a35 !-> a35)(as9 !-> as9);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    mov [rsp + 2000], rsi
    mov rsi, [rsp + 2016]
    ; let l1: List[i64] = Cons(a35, as9);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab63997
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab63998

lab63997:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab63995
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab63988
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63986
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63987

lab63986:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63987:

lab63988:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63991
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63989
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63990

lab63989:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63990:

lab63991:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab63994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab63992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab63993

lab63992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab63993:

lab63994:
    jmp lab63996

lab63995:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab63996:

lab63998:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov [rsp + 2040], rdi
    mov rdi, [rsp + 2008]
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r15
    mov r15, r13
    mov r13, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, [rsp + 2040]
    mov rax, rsi
    mov rsi, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r14
    mov r14, r12
    ; jump lift_algc_2_
    jmp lift_algc_2_

lift_algc_2_:
    ; substitute (ys0 !-> ys)(xs20 !-> xs2)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(a0 !-> a0)(l1 !-> l1);
    ; #share xs2
    cmp qword [rsp + 2032], 0
    je lab63999
    mov rcx, [rsp + 2032]
    add qword [rcx + 0], 1

lab63999:
    ; #share ys
    cmp qword [rsp + 2016], 0
    je lab64000
    mov rcx, [rsp + 2016]
    add qword [rcx + 0], 1

lab64000:
    ; #move variables
    mov [rsp + 2000], rax
    mov [rsp + 1992], rdx
    mov [rsp + 1984], rsi
    mov [rsp + 1976], rdi
    mov rsi, [rsp + 2032]
    mov rdi, [rsp + 2024]
    mov rax, [rsp + 2016]
    mov rdx, [rsp + 2008]
    ; new a11: List[i64] = (m, m2, n, xs1, xs2, ys, a0, l1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64012
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64013

lab64012:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64010
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64003
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64001
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64002

lab64001:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64002:

lab64003:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64005

lab64004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64005:

lab64006:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64009
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64007
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64008

lab64007:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64008:

lab64009:
    jmp lab64011

lab64010:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64011:

lab64013:
    ; ##store link to previous block
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    mov [rbx + 24], r15
    mov [rbx + 16], r14
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64025
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab64026

lab64025:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64023
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64015

lab64014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64015:

lab64016:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64019
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64017
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64018

lab64017:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64018:

lab64019:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64022
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64020
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64021

lab64020:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64021:

lab64022:
    jmp lab64024

lab64023:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64024:

lab64026:
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
    je lab64038
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64039

lab64038:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64036
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64029
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64027
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64028

lab64027:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64028:

lab64029:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64032
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64030
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64031

lab64030:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64031:

lab64032:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64035
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64033
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64034

lab64033:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64034:

lab64035:
    jmp lab64037

lab64036:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64037:

lab64039:
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
    je lab64051
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64052

lab64051:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64049
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64042
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64040
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64041

lab64040:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64041:

lab64042:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64045
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64043
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64044

lab64043:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64044:

lab64045:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64048
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64046
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64047

lab64046:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64047:

lab64048:
    jmp lab64050

lab64049:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64050:

lab64052:
    ; #load tag
    lea r9, [rel List_i64_64053]
    ; substitute (xs20 !-> xs20)(ys0 !-> ys0)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a12: List[i64] = (ys0, a11)\{ ... \};
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
    je lab64065
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64066

lab64065:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64063
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64056
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64054
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64055

lab64054:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64055:

lab64056:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64059
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64057
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64058

lab64057:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64058:

lab64059:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64062
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64060
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64061

lab64060:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64061:

lab64062:
    jmp lab64064

lab64063:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64064:

lab64066:
    ; #load tag
    lea rdi, [rel List_i64_64067]
    ; jump reverse_
    jmp reverse_

List_i64_64067:
    jmp near List_i64_64067_Nil
    jmp near List_i64_64067_Cons

List_i64_64067_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64070
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab64068
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64068:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64069
    ; ####increment refcount
    add qword [rax + 0], 1

lab64069:
    jmp lab64071

lab64070:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab64071:
    ; let x3: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a11 !-> a11)(x3 !-> x3)(ys0 !-> ys0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_algc_6_
    jmp lift_algc_6_

List_i64_64067_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64074
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab64072
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64072:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64073
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64073:
    jmp lab64075

lab64074:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab64075:
    ; substitute (a11 !-> a11)(ys0 !-> ys0)(a34 !-> a34)(as8 !-> as8);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x3: List[i64] = Cons(a34, as8);
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
    je lab64087
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64088

lab64087:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64085
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64078
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64076
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64077

lab64076:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64077:

lab64078:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64081
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64079
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64080

lab64079:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64080:

lab64081:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64084
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64082
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64083

lab64082:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64083:

lab64084:
    jmp lab64086

lab64085:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64086:

lab64088:
    ; #load tag
    mov r9, 5
    ; substitute (a11 !-> a11)(x3 !-> x3)(ys0 !-> ys0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_algc_6_
    jmp lift_algc_6_

List_i64_64053:
    jmp near List_i64_64053_Nil
    jmp near List_i64_64053_Cons

List_i64_64053_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64094
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64089
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64089:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab64090
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64090:
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64091
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64091:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64092
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64092:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab64093
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64093:
    jmp lab64095

lab64094:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
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
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]

lab64095:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a0 !-> a0)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(x2 !-> x2)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, [rsp + 2032]
    mov [rsp + 2032], r10
    mov rsi, [rsp + 2016]
    mov [rsp + 2016], r12
    mov rcx, [rsp + 2000]
    mov [rsp + 2000], r14
    mov r14, rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], r15
    mov r15, rcx
    ; jump lift_algc_3_
    jmp lift_algc_3_

List_i64_64053_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64101
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
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64096
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64096:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab64097
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64097:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab64098
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64098:
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab64099
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64099:
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64100
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64100:
    mov rax, [rsp + 2040]
    jmp lab64102

lab64101:
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
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab64102:
    ; substitute (l1 !-> l1)(a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(a32 !-> a32)(as6 !-> as6);
    ; #move variables
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], rdi
    mov rdi, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    mov rax, [rsp + 1984]
    mov [rsp + 1984], rsi
    mov rsi, [rsp + 2000]
    ; let x2: List[i64] = Cons(a32, as6);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64114
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64115

lab64114:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64112
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64105
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64103
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64104

lab64103:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64104:

lab64105:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64108
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64106
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64107

lab64106:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64107:

lab64108:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64111
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64109
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64110

lab64109:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64110:

lab64111:
    jmp lab64113

lab64112:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64113:

lab64115:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(x2 !-> x2)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, [rsp + 2000]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 2000], rcx
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r14
    mov r14, [rsp + 2040]
    mov rcx, [rsp + 1992]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r15
    mov r15, [rsp + 2040]
    ; jump lift_algc_3_
    jmp lift_algc_3_

lift_algc_6_:
    ; substitute (ys !-> ys)(x3 !-> x3)(a11 !-> a11);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a13: List[i64] = (x3, a11)\{ ... \};
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
    je lab64127
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64128

lab64127:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64125
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64118
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64116
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64117

lab64116:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64117:

lab64118:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64121
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64119
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64120

lab64119:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64120:

lab64121:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64124
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64122
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64123

lab64122:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64123:

lab64124:
    jmp lab64126

lab64125:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64126:

lab64128:
    ; #load tag
    lea rdi, [rel List_i64_64129]
    ; jump reverse_
    jmp reverse_

List_i64_64129:
    jmp near List_i64_64129_Nil
    jmp near List_i64_64129_Cons

List_i64_64129_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64132
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab64130
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64130:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64131
    ; ####increment refcount
    add qword [rax + 0], 1

lab64131:
    jmp lab64133

lab64132:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab64133:
    ; let x4: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x3 !-> x3)(x4 !-> x4)(a11 !-> a11);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump algb_
    jmp algb_

List_i64_64129_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64136
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab64134
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64134:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64135
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64135:
    jmp lab64137

lab64136:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab64137:
    ; substitute (a11 !-> a11)(x3 !-> x3)(a33 !-> a33)(as7 !-> as7);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x4: List[i64] = Cons(a33, as7);
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
    je lab64149
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64150

lab64149:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64147
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64140
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64138
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64139

lab64138:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64139:

lab64140:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64143
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64141
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64142

lab64141:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64142:

lab64143:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64146
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64144
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64145

lab64144:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64145:

lab64146:
    jmp lab64148

lab64147:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64148:

lab64150:
    ; #load tag
    mov r9, 5
    ; substitute (x3 !-> x3)(x4 !-> x4)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump algb_
    jmp algb_

lift_algc_3_:
    ; substitute (x2 !-> x2)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(a0 !-> a0)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    ; new a21: List[i64] = (l1, m, m2, n, a0, xs1, xs2, ys)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64162
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64163

lab64162:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64160
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64153
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64151
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64152

lab64151:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64152:

lab64153:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64156
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64154
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64155

lab64154:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64155:

lab64156:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64159
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64157
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64158

lab64157:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64158:

lab64159:
    jmp lab64161

lab64160:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64161:

lab64163:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64175
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64176

lab64175:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64173
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64166
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64164
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64165

lab64164:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64165:

lab64166:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64169
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64167
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64168

lab64167:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64168:

lab64169:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64172
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64170
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64171

lab64170:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64171:

lab64172:
    jmp lab64174

lab64173:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64174:

lab64176:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
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
    je lab64188
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64189

lab64188:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64186
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64179
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64177
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64178

lab64177:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64178:

lab64179:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64182
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64180
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64181

lab64180:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64181:

lab64182:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64185
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64183
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64184

lab64183:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64184:

lab64185:
    jmp lab64187

lab64186:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64187:

lab64189:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab64201
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64202

lab64201:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64199
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64192
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64190
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64191

lab64190:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64191:

lab64192:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64195
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64193
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64194

lab64193:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64194:

lab64195:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64198
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64196
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64197

lab64196:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64197:

lab64198:
    jmp lab64200

lab64199:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64200:

lab64202:
    ; #load tag
    lea rdi, [rel List_i64_64203]
    ; jump reverse_
    jmp reverse_

List_i64_64203:
    jmp near List_i64_64203_Nil
    jmp near List_i64_64203_Cons

List_i64_64203_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64209
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64204
    ; ####increment refcount
    add qword [rax + 0], 1

lab64204:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64205
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64205:
    mov r11, [r10 + 24]
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64206
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64206:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64207
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64207:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab64208
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64208:
    jmp lab64210

lab64209:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
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
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]

lab64210:
    ; let l2: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a0 !-> a0)(l1 !-> l1)(l2 !-> l2)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rsi, rax
    mov [rsp + 2040], r13
    mov r13, r9
    mov r9, [rsp + 1992]
    mov rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r15
    mov r15, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, [rsp + 2040]
    mov rax, r12
    mov r8, [rsp + 2000]
    mov rcx, [rsp + 2016]
    mov [rsp + 2000], rcx
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r14
    ; jump lift_algc_4_
    jmp lift_algc_4_

List_i64_64203_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64216
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64211
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64211:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64212
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64212:
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab64213
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64213:
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab64214
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64214:
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64215
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64215:
    mov rax, [rsp + 2040]
    jmp lab64217

lab64216:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
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
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab64217:
    ; substitute (ys !-> ys)(xs2 !-> xs2)(l1 !-> l1)(m !-> m)(m2 !-> m2)(n !-> n)(a0 !-> a0)(xs1 !-> xs1)(a31 !-> a31)(as5 !-> as5);
    ; #move variables
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], rdi
    mov rdi, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    mov rax, [rsp + 1984]
    mov [rsp + 1984], rsi
    mov rsi, [rsp + 2000]
    ; let l2: List[i64] = Cons(a31, as5);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64229
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64230

lab64229:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64227
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64219

lab64218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64219:

lab64220:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64222

lab64221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64222:

lab64223:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64225

lab64224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64225:

lab64226:
    jmp lab64228

lab64227:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64228:

lab64230:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(l1 !-> l1)(l2 !-> l2)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 2032], rcx
    mov [rsp + 2016], rsi
    mov rsi, r8
    mov r8, [rsp + 2000]
    mov [rsp + 2000], rax
    mov rax, [rsp + 2040]
    mov rcx, [rsp + 2024]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2024], rcx
    mov [rsp + 2008], rdi
    mov rdi, r9
    mov r9, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, [rsp + 2040]
    ; jump lift_algc_4_
    jmp lift_algc_4_

lift_algc_4_:
    ; lit x5 <- 0;
    mov qword [rsp + 1976], 0
    ; lit x6 <- 0;
    mov qword [rsp + 1960], 0
    ; lit x7 <- -1;
    mov qword [rsp + 1944], -1
    ; substitute (l2 !-> l2)(l1 !-> l1)(a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a10: List[Pair[i64, i64]] = (a0, m, m2, n, xs1, xs2, ys, x5, x6, x7)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64242
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64243

lab64242:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64240
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64233
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64231
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64232

lab64231:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64232:

lab64233:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64236
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64234
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64235

lab64234:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64235:

lab64236:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64239
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64237
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64238

lab64237:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64238:

lab64239:
    jmp lab64241

lab64240:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64241:

lab64243:
    ; ##store link to previous block
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2008]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64255
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64256

lab64255:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64253
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64246
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64244
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64245

lab64244:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64245:

lab64246:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64249
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64247
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64248

lab64247:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64248:

lab64249:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64252
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64250
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64251

lab64250:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64251:

lab64252:
    jmp lab64254

lab64253:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64254:

lab64256:
    ; ##store link to previous block
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    mov [rbx + 24], r15
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64268
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab64269

lab64268:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64266
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64259
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64257
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64258

lab64257:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64258:

lab64259:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64262
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64260
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64261

lab64260:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64261:

lab64262:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64265
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64263
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64264

lab64263:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64264:

lab64265:
    jmp lab64267

lab64266:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64267:

lab64269:
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
    je lab64281
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64282

lab64281:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64279
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64271

lab64270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64271:

lab64272:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64275
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64273
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64274

lab64273:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64274:

lab64275:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64278
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64276
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64277

lab64276:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64277:

lab64278:
    jmp lab64280

lab64279:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64280:

lab64282:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab64294
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64295

lab64294:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64292
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64285
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64283
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64284

lab64283:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64284:

lab64285:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64288
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64286
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64287

lab64286:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64287:

lab64288:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64291
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64289
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64290

lab64289:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64290:

lab64291:
    jmp lab64293

lab64292:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64293:

lab64295:
    ; #load tag
    lea r9, [rel List_Pair_i64_i64_64296]
    ; substitute (l1 !-> l1)(l2 !-> l2)(a10 !-> a10);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump zip_
    jmp zip_

List_Pair_i64_i64_64296:
    jmp near List_Pair_i64_i64_64296_Nil
    jmp near List_Pair_i64_i64_64296_Cons

List_Pair_i64_i64_64296_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64301
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64297
    ; ####increment refcount
    add qword [rax + 0], 1

lab64297:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64298
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64298:
    mov r11, [r10 + 24]
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64299
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64299:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab64300
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64300:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]
    jmp lab64302

lab64301:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]

lab64302:
    ; let x8: List[Pair[i64, i64]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1968], 0
    ; #load tag
    mov qword [rsp + 1960], 0
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov [rsp + 2000], r12
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1960]
    mov [rsp + 2008], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 1960], rcx
    mov rcx, [rsp + 1976]
    mov [rsp + 2024], rcx
    mov [rsp + 1976], r15
    mov r15, [rsp + 1992]
    mov [rsp + 1992], r13
    mov r13, [rsp + 2040]
    mov [rsp + 1984], r14
    mov rcx, [rsp + 1968]
    mov [rsp + 2016], rcx
    mov rcx, [rsp + 2032]
    mov [rsp + 1968], rcx
    ; jump lift_algc_5_
    jmp lift_algc_5_

List_Pair_i64_i64_64296_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64307
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64303
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64303:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64304
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64304:
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab64305
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64305:
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64306
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64306:
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]
    jmp lab64308

lab64307:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
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
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2000], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rax, [rsp + 2040]

lab64308:
    ; substitute (x7 !-> x7)(x6 !-> x6)(a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(x5 !-> x5)(a30 !-> a30)(as4 !-> as4);
    ; #move variables
    mov [rsp + 1968], rax
    mov rcx, [rsp + 1944]
    mov [rsp + 1944], rdi
    mov rdi, [rsp + 1960]
    mov [rsp + 1960], rdx
    mov rdx, rcx
    mov [rsp + 1952], rsi
    ; let x8: List[Pair[i64, i64]] = Cons(a30, as4);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64320
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64321

lab64320:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64318
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64311
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64309
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64310

lab64309:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64310:

lab64311:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64314
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64312
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64313

lab64312:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64313:

lab64314:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64317
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64315
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64316

lab64315:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64316:

lab64317:
    jmp lab64319

lab64318:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64319:

lab64321:
    ; #load tag
    mov qword [rsp + 1960], 5
    ; substitute (a0 !-> a0)(m !-> m)(m2 !-> m2)(n !-> n)(x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov [rsp + 2040], r9
    mov r9, r13
    mov r13, [rsp + 1976]
    mov rcx, [rsp + 2008]
    mov [rsp + 1976], rcx
    mov rcx, [rsp + 1960]
    mov [rsp + 2008], rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1960], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 1992], rcx
    mov [rsp + 2024], rdx
    mov rdx, [rsp + 2040]
    mov rcx, r11
    mov r11, r15
    mov r15, rdi
    mov rdi, rcx
    mov rax, r8
    mov rcx, [rsp + 2016]
    mov [rsp + 1984], rcx
    mov rcx, [rsp + 1968]
    mov [rsp + 2016], rcx
    mov rcx, [rsp + 2000]
    mov [rsp + 1968], rcx
    mov rcx, [rsp + 2032]
    mov [rsp + 2000], rcx
    ; jump lift_algc_5_
    jmp lift_algc_5_

lift_algc_5_:
    ; substitute (x8 !-> x8)(x7 !-> x7)(x6 !-> x6)(x5 !-> x5)(n !-> n)(m2 !-> m2)(m !-> m)(a0 !-> a0)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rax
    mov rax, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    mov rcx, r15
    mov r15, r9
    mov r9, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; new a22: _Cont = (n, m2, m, a0, xs1, xs2, ys)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1992]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64333
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64334

lab64333:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64331
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64324
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64322
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64323

lab64322:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64323:

lab64324:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64327
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64325
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64326

lab64325:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64326:

lab64327:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64330
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64328
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64329

lab64328:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64329:

lab64330:
    jmp lab64332

lab64331:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64332:

lab64334:
    ; ##store link to previous block
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64346
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64347

lab64346:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64344
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64336

lab64335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64336:

lab64337:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64340
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64338
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64339

lab64338:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64339:

lab64340:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64343
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64341
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64342

lab64341:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64342:

lab64343:
    jmp lab64345

lab64344:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64345:

lab64347:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64359
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64360

lab64359:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64357
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64350
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64348
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64349

lab64348:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64349:

lab64350:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64353
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64351
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64352

lab64351:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64352:

lab64353:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64355

lab64354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64355:

lab64356:
    jmp lab64358

lab64357:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64358:

lab64360:
    ; #load tag
    lea r13, [rel _Cont_64361]
    ; substitute (x5 !-> x5)(x6 !-> x6)(x7 !-> x7)(x8 !-> x8)(a22 !-> a22);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump findk_
    jmp findk_

_Cont_64361:

_Cont_64361_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64366
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64362
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64362:
    mov r11, [r10 + 24]
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64363
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64363:
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab64364
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64364:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab64365
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64365:
    jmp lab64367

lab64366:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
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
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]

lab64367:
    ; substitute (k !-> k)(n !-> n)(m2 !-> m2)(m !-> m)(ys !-> ys)(xs1 !-> xs1)(xs2 !-> xs2)(a0 !-> a0);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], r12
    mov r12, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, rcx
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_List_i64_List_i64_64368:

Fun_List_i64_List_i64_64368_Ap:
    ; #load from memory
    mov rcx, [rsp + 2016]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab64371
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab64369
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64369:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab64370
    ; ####increment refcount
    add qword [rcx + 0], 1

lab64370:
    mov rax, [rsp + 2040]
    jmp lab64372

lab64371:
    ; ##... or release blocks onto linear free list when loading
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab64372:
    ; substitute (k0 !-> k)(a2 !-> a2)(m20 !-> m2)(x !-> x)(ys0 !-> ys)(xs1 !-> xs1)(xs2 !-> xs2)(ys !-> ys)(m !-> m)(k !-> k)(n !-> n)(m2 !-> m2);
    ; #share ys
    cmp r12, 0
    je lab64373
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64373:
    ; #move variables
    mov [rsp + 1976], rdx
    mov [rsp + 1960], rdi
    mov [rsp + 1944], r9
    mov rdi, [rsp + 1992]
    mov [rsp + 1992], r11
    mov r10, [rsp + 2016]
    mov [rsp + 2016], r12
    mov r11, [rsp + 2008]
    mov [rsp + 2008], r13
    mov rsi, [rsp + 2000]
    ; new f1: Fun[List[i64], List[i64]] = (xs2, ys, m, k, n, m2)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64385
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64386

lab64385:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64383
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64376
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64374
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64375

lab64374:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64375:

lab64376:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64379
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64377
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64378

lab64377:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64378:

lab64379:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64382
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64380
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64381

lab64380:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64381:

lab64382:
    jmp lab64384

lab64383:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64384:

lab64386:
    ; ##store link to previous block
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 2008]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64398
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64399

lab64398:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64396
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64388

lab64387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64388:

lab64389:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64392
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64390
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64391

lab64390:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64391:

lab64392:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64395
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64393
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64394

lab64393:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64394:

lab64395:
    jmp lab64397

lab64396:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64397:

lab64399:
    ; ##store link to previous block
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64411
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64412

lab64411:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64409
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64402
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64400
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64401

lab64400:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64401:

lab64402:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64405
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64403
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64404

lab64403:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64404:

lab64405:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64408
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64406
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64407

lab64406:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64407:

lab64408:
    jmp lab64410

lab64409:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64410:

lab64412:
    ; #load tag
    lea rcx, [rel Fun_List_i64_List_i64_64413]
    mov [rsp + 2024], rcx
    ; substitute (f1 !-> f1)(a2 !-> a2)(x !-> x)(m20 !-> m20)(ys0 !-> ys0)(xs1 !-> xs1)(k0 !-> k0);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    mov r8, r10
    mov rax, [rsp + 2032]
    ; new f2: Fun[List[i64], List[i64]] = (m20, ys0, xs1, k0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    mov [rbx + 24], r13
    mov [rbx + 16], r12
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64425
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64426

lab64425:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64423
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64416
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64414
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64415

lab64414:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64415:

lab64416:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64419
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64417
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64418

lab64417:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64418:

lab64419:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64422
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64420
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64421

lab64420:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64421:

lab64422:
    jmp lab64424

lab64423:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64424:

lab64426:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64438
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64439

lab64438:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64436
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64429
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64427
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64428

lab64427:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64428:

lab64429:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64432
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64430
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64431

lab64430:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64431:

lab64432:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64435
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64433
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64434

lab64433:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64434:

lab64435:
    jmp lab64437

lab64436:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64437:

lab64439:
    ; #load tag
    lea r11, [rel Fun_List_i64_List_i64_64440]
    ; substitute (f1 !-> f1)(x !-> x)(a2 !-> a2)(f2 !-> f2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a3: List[i64] = (a2, f2)\{ ... \};
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
    je lab64452
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64453

lab64452:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64450
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64443
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64441
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64442

lab64441:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64442:

lab64443:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64446
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64444
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64445

lab64444:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64445:

lab64446:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64449
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64447
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64448

lab64447:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64448:

lab64449:
    jmp lab64451

lab64450:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64451:

lab64453:
    ; #load tag
    lea r9, [rel List_i64_64454]
    ; substitute (x !-> x)(a3 !-> a3)(f1 !-> f1);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f1 Ap
    jmp r9

List_i64_64454:
    jmp near List_i64_64454_Nil
    jmp near List_i64_64454_Cons

List_i64_64454_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64457
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab64455
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64455:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64456
    ; ####increment refcount
    add qword [rax + 0], 1

lab64456:
    jmp lab64458

lab64457:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab64458:
    ; let x13: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x13 !-> x13)(a2 !-> a2)(f2 !-> f2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke f2 Ap
    jmp r9

List_i64_64454_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64461
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab64459
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64459:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64460
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64460:
    jmp lab64462

lab64461:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab64462:
    ; substitute (f2 !-> f2)(a2 !-> a2)(a29 !-> a29)(as3 !-> as3);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x13: List[i64] = Cons(a29, as3);
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
    je lab64474
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64475

lab64474:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64472
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64465
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64463
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64464

lab64463:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64464:

lab64465:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64468
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64466
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64467

lab64466:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64467:

lab64468:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64471
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64469
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64470

lab64469:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64470:

lab64471:
    jmp lab64473

lab64472:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64473:

lab64475:
    ; #load tag
    mov r9, 5
    ; substitute (x13 !-> x13)(a2 !-> a2)(f2 !-> f2);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke f2 Ap
    jmp r9

Fun_List_i64_List_i64_64440:

Fun_List_i64_List_i64_64440_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64478
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64476
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64476:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab64477
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64477:
    jmp lab64479

lab64478:
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
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab64479:
    ; substitute (k0 !-> k0)(xs1 !-> xs1)(m20 !-> m20)(ys0 !-> ys0)(a28 !-> a28)(a01 !-> a01);
    ; #move variables
    mov r14, rsi
    mov rsi, r12
    mov r12, rax
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    ; let a4: Fun[List[i64], List[i64]] = Ap(a28, a01);
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
    je lab64491
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64492

lab64491:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64489
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64482
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64480
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64481

lab64480:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64481:

lab64482:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64485
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64483
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64484

lab64483:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64484:

lab64485:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64488
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64486
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64487

lab64486:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64487:

lab64488:
    jmp lab64490

lab64489:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64490:

lab64492:
    ; #load tag
    mov r13, 0
    ; substitute (k00 !-> k0)(ys0 !-> ys0)(m20 !-> m20)(xs1 !-> xs1)(a4 !-> a4)(k0 !-> k0);
    ; #move variables
    mov r15, rdx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; new a5: List[i64] = (m20, xs1, a4, k0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab64504
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64505

lab64504:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64502
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64495
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64493
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64494

lab64493:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64494:

lab64495:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64498
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64496
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64497

lab64496:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64497:

lab64498:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64501
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64499
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64500

lab64499:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64500:

lab64501:
    jmp lab64503

lab64502:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64503:

lab64505:
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
    je lab64517
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64518

lab64517:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64515
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64508
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64506
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64507

lab64506:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64507:

lab64508:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64510

lab64509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64510:

lab64511:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64514
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64512
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64513

lab64512:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64513:

lab64514:
    jmp lab64516

lab64515:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64516:

lab64518:
    ; #load tag
    lea r9, [rel List_i64_64519]
    ; jump take_
    jmp take_

List_i64_64519:
    jmp near List_i64_64519_Nil
    jmp near List_i64_64519_Cons

List_i64_64519_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64522
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab64520
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64520:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab64521
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64521:
    jmp lab64523

lab64522:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab64523:
    ; let x12: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (m20 !-> m20)(k0 !-> k0)(xs1 !-> xs1)(x12 !-> x12)(a4 !-> a4);
    ; #move variables
    mov r10, r12
    mov r12, r8
    mov r8, rsi
    mov rcx, r11
    mov r11, r13
    mov r13, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump algc_
    jmp algc_

List_i64_64519_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64526
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64524
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64524:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab64525
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64525:
    jmp lab64527

lab64526:
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
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab64527:
    ; substitute (k0 !-> k0)(a4 !-> a4)(m20 !-> m20)(xs1 !-> xs1)(a27 !-> a27)(as2 !-> as2);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov r14, rsi
    mov rsi, r12
    ; let x12: List[i64] = Cons(a27, as2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64539
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64540

lab64539:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64537
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64530
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64528
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64529

lab64528:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64529:

lab64530:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64533
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64531
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64532

lab64531:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64532:

lab64533:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64536
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64534
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64535

lab64534:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64535:

lab64536:
    jmp lab64538

lab64537:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64538:

lab64540:
    ; #load tag
    mov r13, 5
    ; substitute (m20 !-> m20)(k0 !-> k0)(xs1 !-> xs1)(x12 !-> x12)(a4 !-> a4);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r10
    mov r10, r12
    mov r12, rsi
    ; jump algc_
    jmp algc_

Fun_List_i64_List_i64_64413:

Fun_List_i64_List_i64_64413_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64543
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64541
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64541:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab64542
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64542:
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    jmp lab64544

lab64543:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]

lab64544:
    ; substitute (m2 !-> m2)(n !-> n)(xs2 !-> xs2)(ys !-> ys)(m !-> m)(k !-> k)(a26 !-> a26)(a00 !-> a00);
    ; #move variables
    mov [rsp + 2032], rax
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov [rsp + 2016], rsi
    ; let a6: Fun[List[i64], List[i64]] = Ap(a26, a00);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 2024]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64556
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab64557

lab64556:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64554
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64547
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64545
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64546

lab64545:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64546:

lab64547:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64550
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64548
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64549

lab64548:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64549:

lab64550:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64553
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64551
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64552

lab64551:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64552:

lab64553:
    jmp lab64555

lab64554:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64555:

lab64557:
    ; #load tag
    mov qword [rsp + 2024], 0
    ; x9 <- m - m2;
    mov rcx, r13
    sub rcx, rdx
    mov [rsp + 2008], rcx
    ; substitute (x9 !-> x9)(n !-> n)(xs2 !-> xs2)(ys !-> ys)(a6 !-> a6)(k !-> k);
    ; #move variables
    mov r12, [rsp + 2032]
    mov r13, [rsp + 2024]
    mov rdx, [rsp + 2008]
    ; x10 <- n - k;
    mov rcx, rdi
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (k !-> k)(ys !-> ys)(xs2 !-> xs2)(x9 !-> x9)(a6 !-> a6)(x10 !-> x10);
    ; #move variables
    mov rdi, r11
    mov r11, rdx
    mov rsi, r10
    mov rdx, r15
    mov r15, [rsp + 2024]
    ; new a9: List[i64] = (xs2, x9, a6, x10)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov qword [rbx + 48], 0
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
    je lab64569
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64570

lab64569:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64567
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64560
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64558
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64559

lab64558:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64559:

lab64560:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64562

lab64561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64562:

lab64563:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64565

lab64564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64565:

lab64566:
    jmp lab64568

lab64567:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64568:

lab64570:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab64582
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64583

lab64582:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64580
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64573
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64571
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64572

lab64571:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64572:

lab64573:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64576
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64574
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64575

lab64574:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64575:

lab64576:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64579
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64577
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64578

lab64577:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64578:

lab64579:
    jmp lab64581

lab64580:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64581:

lab64583:
    ; #load tag
    lea r9, [rel List_i64_64584]
    ; jump drop_
    jmp drop_

List_i64_64584:
    jmp near List_i64_64584_Nil
    jmp near List_i64_64584_Cons

List_i64_64584_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64587
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64585
    ; ####increment refcount
    add qword [rax + 0], 1

lab64585:
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab64586
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64586:
    mov rdi, [rsi + 24]
    jmp lab64588

lab64587:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab64588:
    ; let x11: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (x9 !-> x9)(x10 !-> x10)(xs2 !-> xs2)(x11 !-> x11)(a6 !-> a6);
    ; #move variables
    mov r10, r12
    mov r12, r8
    mov r8, rax
    mov rcx, rdi
    mov rdi, r11
    mov r11, r13
    mov r13, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump algc_
    jmp algc_

List_i64_64584_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64591
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64589
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64589:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab64590
    ; ####increment refcount
    add qword [r12 + 0], 1

lab64590:
    mov r11, [r10 + 24]
    jmp lab64592

lab64591:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]

lab64592:
    ; substitute (x10 !-> x10)(a6 !-> a6)(xs2 !-> xs2)(x9 !-> x9)(a25 !-> a25)(as1 !-> as1);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov r14, rsi
    mov rsi, r12
    ; let x11: List[i64] = Cons(a25, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64604
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64605

lab64604:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64602
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64595
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64593
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64594

lab64593:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64594:

lab64595:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64598
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64596
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64597

lab64596:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64597:

lab64598:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64600

lab64599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64600:

lab64601:
    jmp lab64603

lab64602:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64603:

lab64605:
    ; #load tag
    mov r13, 5
    ; substitute (x9 !-> x9)(x10 !-> x10)(xs2 !-> xs2)(x11 !-> x11)(a6 !-> a6);
    ; #move variables
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r10, r12
    mov r12, rsi
    ; jump algc_
    jmp algc_

list_len_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_64606]
    add rcx, rdi
    jmp rcx

List_i64_64606:
    jmp near List_i64_64606_Nil
    jmp near List_i64_64606_Cons

List_i64_64606_Nil:
    ; lit x2 <- 0;
    mov rdi, 0
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_64606_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64608
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab64607
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64607:
    mov rdi, [rsi + 40]
    jmp lab64609

lab64608:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab64609:
    ; substitute (a0 !-> a0)(is !-> is);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; lit x0 <- 1;
    mov r9, 1
    ; substitute (is !-> is)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a1: _Cont = (a0, x0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
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
    je lab64621
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64622

lab64621:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64619
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64612
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64610
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64611

lab64610:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64611:

lab64612:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64615
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64613
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64614

lab64613:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64614:

lab64615:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64618
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64616
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64617

lab64616:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64617:

lab64618:
    jmp lab64620

lab64619:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64620:

lab64622:
    ; #load tag
    lea rdi, [rel _Cont_64623]
    ; jump list_len_
    jmp list_len_

_Cont_64623:

_Cont_64623_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64625
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab64624
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64624:
    jmp lab64626

lab64625:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab64626:
    ; x3 <- x0 + x1;
    mov r11, r9
    add r11, rdx
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lcss_:
    ; substitute (xs0 !-> xs)(ys !-> ys)(a0 !-> a0)(xs !-> xs);
    ; #share xs
    cmp rax, 0
    je lab64627
    ; ####increment refcount
    add qword [rax + 0], 1

lab64627:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; new a1: _Cont = (ys, a0, xs)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov [rbx + 32], r8
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64639
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64640

lab64639:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64637
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64629

lab64628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64629:

lab64630:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64633
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64631
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64632

lab64631:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64632:

lab64633:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64635

lab64634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64635:

lab64636:
    jmp lab64638

lab64637:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64638:

lab64640:
    ; #load tag
    lea rdi, [rel _Cont_64641]
    ; jump list_len_
    jmp list_len_

_Cont_64641:

_Cont_64641_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64645
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab64642
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64642:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab64643
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64643:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab64644
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64644:
    jmp lab64646

lab64645:
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

lab64646:
    ; substitute (ys0 !-> ys)(ys !-> ys)(a0 !-> a0)(xs !-> xs)(x0 !-> x0);
    ; #share ys
    cmp rsi, 0
    je lab64647
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64647:
    ; #move variables
    mov r13, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a2: _Cont = (ys, a0, xs, x0)\{ ... \};
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
    je lab64659
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64660

lab64659:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64657
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64650
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64648
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64649

lab64648:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64649:

lab64650:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64653
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64651
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64652

lab64651:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64652:

lab64653:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64655

lab64654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64655:

lab64656:
    jmp lab64658

lab64657:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64658:

lab64660:
    ; ##store link to previous block
    mov [rbx + 48], r8
    ; ##store values
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
    je lab64672
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64673

lab64672:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64670
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64663
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64661
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64662

lab64661:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64662:

lab64663:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64666
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64664
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64665

lab64664:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64665:

lab64666:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64669
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64667
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64668

lab64667:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64668:

lab64669:
    jmp lab64671

lab64670:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64671:

lab64673:
    ; #load tag
    lea rdi, [rel _Cont_64674]
    ; jump list_len_
    jmp list_len_

_Cont_64674:

_Cont_64674_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64678
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab64675
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64675:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab64676
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64676:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab64677
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64677:
    jmp lab64679

lab64678:
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
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab64679:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (x1 !-> x1)(ys !-> ys)(x0 !-> x0)(xs !-> xs)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov r12, r14
    mov r14, r8
    mov rcx, r13
    mov r13, r15
    mov r15, r9
    mov r9, rcx
    ; let a3: Fun[List[i64], List[i64]] = Ap(x2, a0);
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
    je lab64691
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64692

lab64691:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64689
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64682
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64680
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64681

lab64680:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64681:

lab64682:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64684

lab64683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64684:

lab64685:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64687

lab64686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64687:

lab64688:
    jmp lab64690

lab64689:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64690:

lab64692:
    ; #load tag
    mov r13, 0
    ; substitute (x0 !-> x0)(x1 !-> x1)(xs !-> xs)(ys !-> ys)(a3 !-> a3);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, r10
    mov r10, rsi
    ; jump algc_
    jmp algc_

enum_from_then_to_:
    ; if from <= t \{ ... \}
    cmp rdx, r9
    jle lab64693
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab64693:
    ; substitute (from0 !-> from)(then !-> then)(t !-> t)(a0 !-> a0)(from !-> from);
    ; #move variables
    mov r13, rdx
    ; new a1: List[i64] = (a0, from)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov qword [rbx + 48], 0
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
    je lab64705
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64706

lab64705:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64703
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64696
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64694
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64695

lab64694:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64695:

lab64696:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64699
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64697
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64698

lab64697:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64698:

lab64699:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64701

lab64700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64701:

lab64702:
    jmp lab64704

lab64703:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64704:

lab64706:
    ; #load tag
    lea r11, [rel List_i64_64707]
    ; lit x1 <- 2;
    mov r13, 2
    ; x2 <- x1 * then;
    mov r15, r13
    imul r15, rdi
    ; substitute (from0 !-> from0)(then !-> then)(t !-> t)(a1 !-> a1)(x2 !-> x2);
    ; #move variables
    mov r13, r15
    ; x3 <- x2 - from0;
    mov r15, r13
    sub r15, rdx
    ; substitute (then !-> then)(x3 !-> x3)(t !-> t)(a1 !-> a1);
    ; #move variables
    mov rdx, rdi
    mov rdi, r15
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_64707:
    jmp near List_i64_64707_Nil
    jmp near List_i64_64707_Cons

List_i64_64707_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64709
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64708
    ; ####increment refcount
    add qword [rax + 0], 1

lab64708:
    jmp lab64710

lab64709:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab64710:
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

List_i64_64707_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64712
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64711
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64711:
    jmp lab64713

lab64712:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab64713:
    ; substitute (from !-> from)(a0 !-> a0)(a4 !-> a4)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r10, rsi
    mov rsi, r8
    ; let x0: List[i64] = Cons(a4, as0);
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
    je lab64725
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64726

lab64725:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64723
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64716
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64714
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64715

lab64714:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64715:

lab64716:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64719
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64717
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64718

lab64717:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64718:

lab64719:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64722
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64720
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64721

lab64720:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64721:

lab64722:
    jmp lab64724

lab64723:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64724:

lab64726:
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

lcss_main_:
    ; new a1: List[i64] = (d, e, f, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov qword [rbx + 32], 0
    mov [rbx + 24], r13
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64738
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64739

lab64738:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64736
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64729
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64727
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64728

lab64727:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64728:

lab64729:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64732
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64730
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64731

lab64730:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64731:

lab64732:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64734

lab64733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64734:

lab64735:
    jmp lab64737

lab64736:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64737:

lab64739:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r10, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64751
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64752

lab64751:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64749
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64742
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64740
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64741

lab64740:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64741:

lab64742:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64745
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64743
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64744

lab64743:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64744:

lab64745:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64748
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64746
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64747

lab64746:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64747:

lab64748:
    jmp lab64750

lab64749:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64750:

lab64752:
    ; #load tag
    lea r11, [rel List_i64_64753]
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_64753:
    jmp near List_i64_64753_Nil
    jmp near List_i64_64753_Cons

List_i64_64753_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64755
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab64754
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64754:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab64756

lab64755:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab64756:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(d !-> d)(e !-> e)(f !-> f)(x0 !-> x0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r10
    ; jump lift_lcss_main_0_
    jmp lift_lcss_main_0_

List_i64_64753_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64758
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab64757
    ; ####increment refcount
    add qword [r14 + 0], 1

lab64757:
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab64759

lab64758:
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
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]

lab64759:
    ; substitute (a0 !-> a0)(f !-> f)(d !-> d)(e !-> e)(a4 !-> a4)(as1 !-> as1);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    ; let x0: List[i64] = Cons(a4, as1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
    mov [rbx + 40], r13
    mov qword [rbx + 32], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r12, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64771
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64772

lab64771:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64769
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64762
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64760
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64761

lab64760:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64761:

lab64762:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64765
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64763
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64764

lab64763:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64764:

lab64765:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64768
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64766
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64767

lab64766:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64767:

lab64768:
    jmp lab64770

lab64769:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64770:

lab64772:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(d !-> d)(e !-> e)(f !-> f)(x0 !-> x0);
    ; #move variables
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump lift_lcss_main_0_
    jmp lift_lcss_main_0_

lift_lcss_main_0_:
    ; substitute (f !-> f)(d !-> d)(e !-> e)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a2: List[i64] = (a0, x0)\{ ... \};
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
    je lab64784
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64785

lab64784:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64782
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64775
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64773
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64774

lab64773:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64774:

lab64775:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64778
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64776
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64777

lab64776:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64777:

lab64778:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64781
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64779
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64780

lab64779:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64780:

lab64781:
    jmp lab64783

lab64782:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64783:

lab64785:
    ; #load tag
    lea r11, [rel List_i64_64786]
    ; substitute (d !-> d)(e !-> e)(f !-> f)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_64786:
    jmp near List_i64_64786_Nil
    jmp near List_i64_64786_Cons

List_i64_64786_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64789
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab64787
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64787:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab64788
    ; ####increment refcount
    add qword [rax + 0], 1

lab64788:
    jmp lab64790

lab64789:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab64790:
    ; let x1: List[i64] = Nil();
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
    ; jump lcss_
    jmp lcss_

List_i64_64786_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64793
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab64791
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64791:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab64792
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64792:
    jmp lab64794

lab64793:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab64794:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
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
    je lab64806
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64807

lab64806:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64804
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64797
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64795
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64796

lab64795:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64796:

lab64797:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64800
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64798
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64799

lab64798:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64799:

lab64800:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64803
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64801
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64802

lab64801:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64802:

lab64803:
    jmp lab64805

lab64804:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64805:

lab64807:
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
    ; jump lcss_
    jmp lcss_

test_lcss_nofib_:
    ; lit x0 <- 1;
    mov rdi, 1
    ; lit x1 <- 2;
    mov r9, 2
    ; lit x2 <- 60;
    mov r11, 60
    ; lit x3 <- 30;
    mov r13, 30
    ; lit x4 <- 31;
    mov r15, 31
    ; lit x5 <- 90;
    mov qword [rsp + 2024], 90
    ; substitute (x0 !-> x0)(x1 !-> x1)(x2 !-> x2)(x3 !-> x3)(x4 !-> x4)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2032], rax
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, r13
    mov r13, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    ; jump lcss_main_
    jmp lcss_main_

head_:
    ; substitute (a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_64808]
    add rcx, rdi
    jmp rcx

List_i64_64808:
    jmp near List_i64_64808_Nil
    jmp near List_i64_64808_Cons

List_i64_64808_Nil:
    ; lit x0 <- -1;
    mov rdi, -1
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_64808_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64810
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab64809
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64809:
    mov rdi, [rsi + 40]
    jmp lab64811

lab64810:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab64811:
    ; substitute (i !-> i)(a0 !-> a0);
    ; #erase is
    cmp r8, 0
    je lab64814
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab64812
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab64813

lab64812:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab64813:

lab64814:
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; lit x0 <- 1;
    mov r9, 1
    ; if iters == x0 \{ ... \}
    cmp rdx, r9
    je lab64815
    ; substitute (iters !-> iters)(a0 !-> a0);
    ; new a4: List[i64] = (iters, a0)\{ ... \};
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
    je lab64827
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab64828

lab64827:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64825
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64818
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64816
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64817

lab64816:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64817:

lab64818:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64821
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64819
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64820

lab64819:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64820:

lab64821:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64824
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64822
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64823

lab64822:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64823:

lab64824:
    jmp lab64826

lab64825:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64826:

lab64828:
    ; #load tag
    lea rdx, [rel List_i64_64829]
    ; jump test_lcss_nofib_
    jmp test_lcss_nofib_

List_i64_64829:
    jmp near List_i64_64829_Nil
    jmp near List_i64_64829_Cons

List_i64_64829_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64831
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab64830
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64830:
    mov rdx, [rax + 40]
    jmp lab64832

lab64831:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab64832:
    ; let res: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(iters !-> iters);
    ; #erase res
    cmp r8, 0
    je lab64835
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab64833
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab64834

lab64833:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab64834:

lab64835:
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_i64_64829_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64837
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab64836
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64836:
    mov r9, [r8 + 40]
    jmp lab64838

lab64837:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab64838:
    ; substitute (a0 !-> a0)(iters !-> iters)(a6 !-> a6)(as1 !-> as1);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    ; let res: List[i64] = Cons(a6, as1);
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
    je lab64850
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64851

lab64850:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64848
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64840

lab64839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64840:

lab64841:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64844
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64842
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64843

lab64842:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64843:

lab64844:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64847
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64845
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64846

lab64845:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64846:

lab64847:
    jmp lab64849

lab64848:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64849:

lab64851:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(iters !-> iters);
    ; #erase res
    cmp r8, 0
    je lab64854
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab64852
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab64853

lab64852:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab64853:

lab64854:
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lab64815:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; new a3: List[i64] = (a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], rdx
    mov [rbx + 48], rax
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rax, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64866
    ; ####initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab64867

lab64866:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64864
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64857
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64855
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64856

lab64855:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64856:

lab64857:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64860
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64858
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64859

lab64858:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64859:

lab64860:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64863
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64861
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64862

lab64861:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64862:

lab64863:
    jmp lab64865

lab64864:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64865:

lab64867:
    ; #load tag
    lea rdx, [rel List_i64_64868]
    ; jump test_lcss_nofib_
    jmp test_lcss_nofib_

List_i64_64868:
    jmp near List_i64_64868_Nil
    jmp near List_i64_64868_Cons

List_i64_64868_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab64870
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab64869
    ; ####increment refcount
    add qword [rax + 0], 1

lab64869:
    jmp lab64871

lab64870:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab64871:
    ; let res: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_i64_64868_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64873
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab64872
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64872:
    jmp lab64874

lab64873:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab64874:
    ; substitute (a0 !-> a0)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let res: List[i64] = Cons(a5, as0);
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
    je lab64886
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64887

lab64886:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64884
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64876

lab64875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64876:

lab64877:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64880
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64878
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64879

lab64878:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64879:

lab64880:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64883
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64881
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64882

lab64881:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64882:

lab64883:
    jmp lab64885

lab64884:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64885:

lab64887:
    ; #load tag
    mov rdi, 5
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_1_:
    ; lit x2 <- 1;
    mov r9, 1
    ; x3 <- iters - x2;
    mov r11, rdi
    sub r11, r9
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; jump main_loop_
    jmp main_loop_

lift_main_loop_0_:
    ; substitute (res !-> res)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a1: _Cont = (a0)\{ ... \};
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
    je lab64899
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64900

lab64899:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64897
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64889

lab64888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64889:

lab64890:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64892

lab64891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64892:

lab64893:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64896
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64894
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64895

lab64894:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64895:

lab64896:
    jmp lab64898

lab64897:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64898:

lab64900:
    ; #load tag
    lea rdi, [rel _Cont_64901]
    ; jump head_
    jmp head_

_Cont_64901:

_Cont_64901_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64903
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab64902
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64902:
    jmp lab64904

lab64903:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab64904:
    ; println_i64 x1;
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
    ; lit x4 <- 0;
    mov rdi, 0
    ; substitute (x4 !-> x4)(a0 !-> a0);
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