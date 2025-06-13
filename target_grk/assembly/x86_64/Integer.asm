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
    lea r9, [rel _Cont_48219]
    ; jump main_loop_
    jmp main_loop_

_Cont_48219:

_Cont_48219_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

eq_:
    ; if i1 == i2 \{ ... \}
    cmp rdx, rdi
    je lab48220
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab48220:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

lt_:
    ; if i1 < i2 \{ ... \}
    cmp rdx, rdi
    jl lab48221
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab48221:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

leq_:
    ; if i1 <= i2 \{ ... \}
    cmp rdx, rdi
    jle lab48222
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lab48222:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

gt_:
    ; substitute (i2 !-> i2)(i1 !-> i1)(a0 !-> a0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump lt_
    jmp lt_

geq_:
    ; substitute (i2 !-> i2)(i1 !-> i1)(a0 !-> a0);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump leq_
    jmp leq_

enum_from_then_to_:
    ; if from <= t \{ ... \}
    cmp rdx, r9
    jle lab48223
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

lab48223:
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
    je lab48235
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48236

lab48235:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48233
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48225

lab48224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48225:

lab48226:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48229
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48227
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48228

lab48227:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48228:

lab48229:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48232
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48230
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48231

lab48230:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48231:

lab48232:
    jmp lab48234

lab48233:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48234:

lab48236:
    ; #load tag
    lea r11, [rel List_i64_48237]
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

List_i64_48237:
    jmp near List_i64_48237_Nil
    jmp near List_i64_48237_Cons

List_i64_48237_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48239
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab48238
    ; ####increment refcount
    add qword [rax + 0], 1

lab48238:
    jmp lab48240

lab48239:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab48240:
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

List_i64_48237_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48242
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab48241
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48241:
    jmp lab48243

lab48242:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab48243:
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
    je lab48255
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48256

lab48255:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48253
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48246
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48244
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48245

lab48244:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48245:

lab48246:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48249
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48247
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48248

lab48247:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48248:

lab48249:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48252
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48250
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48251

lab48250:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48251:

lab48252:
    jmp lab48254

lab48253:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48254:

lab48256:
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

bench_lscomp2_:
    ; substitute (a0 !-> a0)(t1 !-> t1)(a !-> a)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(ls !-> ls);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rax
    mov rax, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_48257]
    add rcx, [rsp + 2008]
    jmp rcx

List_i64_48257:
    jmp near List_i64_48257_Nil
    jmp near List_i64_48257_Cons

List_i64_48257_Nil:
    ; substitute (t1 !-> t1)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov r14, rax
    mov r9, r15
    mov r15, rdx
    mov rax, rsi
    mov rdx, rdi
    mov r12, r10
    mov rdi, r13
    mov r13, r11
    mov r11, [rsp + 2024]
    ; jump bench_lscomp1_
    jmp bench_lscomp1_

List_i64_48257_Cons:
    ; #load from memory
    mov rcx, [rsp + 2016]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab48259
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
    je lab48258
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48258:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]
    jmp lab48260

lab48259:
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
    mov rax, [rsp + 2040]

lab48260:
    ; substitute (b !-> b)(op0 !-> op)(a6 !-> a)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a !-> a)(t2 !-> t2)(a0 !-> a0)(t1 !-> t1);
    ; #share op
    cmp r10, 0
    je lab48261
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48261:
    ; #move variables
    mov [rsp + 1984], rax
    mov [rsp + 1976], rdx
    mov [rsp + 1968], rsi
    mov [rsp + 1960], rdi
    mov rdx, [rsp + 2008]
    mov [rsp + 2008], r9
    mov rsi, r10
    mov rdi, r11
    ; new a1: Either[i64, Bool] = (op, bstart, bstep, blim, a, t2, a0, t1)\{ ... \};
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
    je lab48273
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48274

lab48273:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48271
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48264
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48262
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48263

lab48262:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48263:

lab48264:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48267
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48265
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48266

lab48265:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48266:

lab48267:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48270
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48268
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48269

lab48268:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48269:

lab48270:
    jmp lab48272

lab48271:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48272:

lab48274:
    ; ##store link to previous block
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
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
    je lab48286
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48287

lab48286:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48284
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48277
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48275
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48276

lab48275:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48276:

lab48277:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48280
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48278
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48279

lab48278:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48279:

lab48280:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48283
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48281
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48282

lab48281:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48282:

lab48283:
    jmp lab48285

lab48284:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48285:

lab48287:
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
    je lab48299
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab48300

lab48299:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48297
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48290
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48288
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48289

lab48288:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48289:

lab48290:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48293
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48291
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48292

lab48291:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48292:

lab48293:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48296
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48294
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48295

lab48294:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48295:

lab48296:
    jmp lab48298

lab48297:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48298:

lab48300:
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
    je lab48312
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48313

lab48312:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48310
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48303
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48301
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48302

lab48301:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48302:

lab48303:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48306
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48304
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48305

lab48304:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48305:

lab48306:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48309
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48307
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48308

lab48307:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48308:

lab48309:
    jmp lab48311

lab48310:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48311:

lab48313:
    ; #load tag
    lea r11, [rel Either_i64_Bool_48314]
    ; substitute (a6 !-> a6)(op0 !-> op0)(b !-> b)(a1 !-> a1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let a3: Fun[i64, Either[i64, Bool]] = Ap(b, a1);
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
    je lab48326
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48327

lab48326:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48324
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48317
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48315
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48316

lab48315:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48316:

lab48317:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48320
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48318
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48319

lab48318:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48319:

lab48320:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48323
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48321
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48322

lab48321:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48322:

lab48323:
    jmp lab48325

lab48324:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48325:

lab48327:
    ; #load tag
    mov r9, 0
    ; substitute (a6 !-> a6)(a3 !-> a3)(op0 !-> op0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke op0 Ap
    jmp r9

Either_i64_Bool_48314:
    jmp near Either_i64_Bool_48314_Left
    jmp near Either_i64_Bool_48314_Right

Either_i64_Bool_48314_Left:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48332
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab48328
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48328:
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
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab48329
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48329:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab48330
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48330:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48331
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48331:
    mov rax, [rsp + 2040]
    jmp lab48333

lab48332:
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
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab48333:
    ; substitute (t1 !-> t1)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a !-> a)(t2 !-> t2)(a0 !-> a0)(a5 !-> a5);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    mov rax, [rsp + 2000]
    ; let x0: Either[i64, Bool] = Left(a5);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48345
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48346

lab48345:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48343
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48336
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48334
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48335

lab48334:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48335:

lab48336:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48339
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48337
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48338

lab48337:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48338:

lab48339:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48342
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48340
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48341

lab48340:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48341:

lab48342:
    jmp lab48344

lab48343:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48344:

lab48346:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a !-> a)(a0 !-> a0)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(t2 !-> t2)(x0 !-> x0);
    ; #move variables
    mov r14, rsi
    mov rsi, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], rax
    mov [rsp + 2040], r15
    mov r15, rdi
    mov rdi, [rsp + 2008]
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], rdx
    mov rdx, [rsp + 2040]
    mov rcx, r13
    mov r13, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_bench_lscomp2_0_
    jmp lift_bench_lscomp2_0_

Either_i64_Bool_48314_Right:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48351
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r8, [rsi + 48]
    ; ###load values
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab48347
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48347:
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
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab48348
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48348:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab48349
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48349:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48350
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48350:
    mov rax, [rsp + 2040]
    jmp lab48352

lab48351:
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
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab48352:
    ; substitute (t1 !-> t1)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a !-> a)(t2 !-> t2)(a0 !-> a0)(b0 !-> b0);
    ; #move variables
    mov rcx, [rsp + 2000]
    mov [rsp + 2000], rax
    mov rax, rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    ; let x0: Either[i64, Bool] = Right(b0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48364
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48365

lab48364:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48362
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48355
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48353
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48354

lab48353:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48354:

lab48355:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48358
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48356
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48357

lab48356:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48357:

lab48358:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48361
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48359
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48360

lab48359:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48360:

lab48361:
    jmp lab48363

lab48362:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48363:

lab48365:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a !-> a)(a0 !-> a0)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(t2 !-> t2)(x0 !-> x0);
    ; #move variables
    mov r14, rsi
    mov rsi, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], rax
    mov [rsp + 2040], r15
    mov r15, rdi
    mov rdi, [rsp + 2008]
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], rdx
    mov rdx, [rsp + 2040]
    mov rcx, r13
    mov r13, r11
    mov r11, r9
    mov r9, rcx
    ; jump lift_bench_lscomp2_0_
    jmp lift_bench_lscomp2_0_

lift_bench_lscomp2_0_:
    ; substitute (a !-> a)(t2 !-> t2)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, rcx
    ; new a2: List[Either[i64, Bool]] = (a0, x0)\{ ... \};
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
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48377
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48378

lab48377:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48375
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48368
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48366
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48367

lab48366:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48367:

lab48368:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48371
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48369
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48370

lab48369:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48370:

lab48371:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48374
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48372
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48373

lab48372:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48373:

lab48374:
    jmp lab48376

lab48375:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48376:

lab48378:
    ; #load tag
    lea rcx, [rel List_Either_i64_Bool_48379]
    mov [rsp + 2008], rcx
    ; substitute (t2 !-> t2)(t1 !-> t1)(a !-> a)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rcx, r15
    mov r15, r13
    mov r13, r11
    mov r11, rcx
    mov r10, r14
    mov rsi, [rsp + 2032]
    ; jump bench_lscomp2_
    jmp bench_lscomp2_

List_Either_i64_Bool_48379:
    jmp near List_Either_i64_Bool_48379_Nil
    jmp near List_Either_i64_Bool_48379_Cons

List_Either_i64_Bool_48379_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48382
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab48380
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48380:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab48381
    ; ####increment refcount
    add qword [rax + 0], 1

lab48381:
    jmp lab48383

lab48382:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab48383:
    ; let x1: List[Either[i64, Bool]] = Nil();
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

List_Either_i64_Bool_48379_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48386
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab48384
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48384:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab48385
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48385:
    jmp lab48387

lab48386:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab48387:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a4 !-> a4)(as0 !-> as0);
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
    ; let x1: List[Either[i64, Bool]] = Cons(a4, as0);
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
    je lab48399
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48400

lab48399:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48397
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48390
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48388
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48389

lab48388:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48389:

lab48390:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48393
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48391
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48392

lab48391:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48392:

lab48393:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48396
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48394
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48395

lab48394:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48395:

lab48396:
    jmp lab48398

lab48397:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48398:

lab48400:
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

bench_lscomp1_:
    ; substitute (a0 !-> a0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(ls !-> ls);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_48401]
    add rcx, r15
    jmp rcx

List_i64_48401:
    jmp near List_i64_48401_Nil
    jmp near List_i64_48401_Cons

List_i64_48401_Nil:
    ; substitute (a0 !-> a0);
    ; #erase op
    cmp r12, 0
    je lab48404
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab48402
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab48403

lab48402:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab48403:

lab48404:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_48401_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab48406
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48405
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48405:
    mov r15, [r14 + 40]
    jmp lab48407

lab48406:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 40]

lab48407:
    ; substitute (blim0 !-> blim)(bstart0 !-> bstart)(bstep0 !-> bstep)(blim !-> blim)(op !-> op)(a !-> a)(t1 !-> t1)(a0 !-> a0)(bstart !-> bstart)(bstep !-> bstep);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 1992], rdi
    mov [rsp + 1976], r9
    mov rdx, r11
    ; new a1: List[i64] = (blim, op, a, t1, a0, bstart, bstep)\{ ... \};
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
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48419
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48420

lab48419:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48417
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48410
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48408
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48409

lab48408:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48409:

lab48410:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48413
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48411
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48412

lab48411:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48412:

lab48413:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48416
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48414
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48415

lab48414:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48415:

lab48416:
    jmp lab48418

lab48417:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48418:

lab48420:
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
    je lab48432
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48433

lab48432:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48430
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48423
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48421
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48422

lab48421:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48422:

lab48423:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48426
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48424
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48425

lab48424:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48425:

lab48426:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48429
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48427
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48428

lab48427:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48428:

lab48429:
    jmp lab48431

lab48430:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48431:

lab48433:
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
    je lab48445
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48446

lab48445:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48443
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48436
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48434
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48435

lab48434:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48435:

lab48436:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48439
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48437
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48438

lab48437:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48438:

lab48439:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48442
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48440
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48441

lab48440:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48441:

lab48442:
    jmp lab48444

lab48443:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48444:

lab48446:
    ; #load tag
    lea r11, [rel List_i64_48447]
    ; x1 <- bstart0 + bstep0;
    mov r13, rdi
    add r13, r9
    ; substitute (bstart0 !-> bstart0)(x1 !-> x1)(blim0 !-> blim0)(a1 !-> a1);
    ; #move variables
    mov r9, rdx
    mov rdx, rdi
    mov rdi, r13
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_48447:
    jmp near List_i64_48447_Nil
    jmp near List_i64_48447_Cons

List_i64_48447_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48451
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab48448
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48448:
    mov rdx, [rax + 24]
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48449
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48449:
    mov r9, [r8 + 24]
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48450
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48450:
    jmp lab48452

lab48451:
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
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab48452:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (x0 !-> x0)(t1 !-> t1)(a !-> a)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a0 !-> a0);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rax, [rsp + 2016]
    mov [rsp + 2016], r12
    ; jump bench_lscomp2_
    jmp bench_lscomp2_

List_i64_48447_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48456
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48453
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48453:
    mov r9, [r8 + 24]
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48454
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48454:
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48455
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48455:
    mov rax, [rsp + 2040]
    jmp lab48457

lab48456:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab48457:
    ; substitute (bstep !-> bstep)(bstart !-> bstart)(blim !-> blim)(op !-> op)(a !-> a)(t1 !-> t1)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    mov [rsp + 2000], rsi
    ; let x0: List[i64] = Cons(a3, as0);
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
    je lab48469
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48470

lab48469:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48467
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48460
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48458
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48459

lab48458:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48459:

lab48460:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48463
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48461
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48462

lab48461:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48462:

lab48463:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48466
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48464
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48465

lab48464:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48465:

lab48466:
    jmp lab48468

lab48467:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48468:

lab48470:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (x0 !-> x0)(t1 !-> t1)(a !-> a)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a0 !-> a0);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r9
    mov r9, r13
    mov r13, rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, [rsp + 2040]
    mov rsi, r14
    mov rax, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    ; jump bench_lscomp2_
    jmp bench_lscomp2_

integerbench_:
    ; substitute (alim !-> alim)(astart !-> astart)(astep !-> astep)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a1: List[i64] = (op, bstart, bstep, blim, a0)\{ ... \};
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
    je lab48482
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48483

lab48482:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48480
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48473
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48471
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48472

lab48471:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48472:

lab48473:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48476
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48474
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48475

lab48474:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48475:

lab48476:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48479
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48477
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48478

lab48477:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48478:

lab48479:
    jmp lab48481

lab48480:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48481:

lab48483:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab48495
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48496

lab48495:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48493
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48486
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48484
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48485

lab48484:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48485:

lab48486:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48489
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48487
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48488

lab48487:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48488:

lab48489:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48492
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48490
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48491

lab48490:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48491:

lab48492:
    jmp lab48494

lab48493:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48494:

lab48496:
    ; #load tag
    lea r11, [rel List_i64_48497]
    ; x1 <- astart + astep;
    mov r13, rdi
    add r13, r9
    ; substitute (astart !-> astart)(x1 !-> x1)(alim !-> alim)(a1 !-> a1);
    ; #move variables
    mov r9, rdx
    mov rdx, rdi
    mov rdi, r13
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_48497:
    jmp near List_i64_48497_Nil
    jmp near List_i64_48497_Cons

List_i64_48497_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48500
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab48498
    ; ####increment refcount
    add qword [rax + 0], 1

lab48498:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab48499
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48499:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab48501

lab48500:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab48501:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump bench_lscomp1_
    jmp bench_lscomp1_

List_i64_48497_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48504
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48502
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48502:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48503
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48503:
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    jmp lab48505

lab48504:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]

lab48505:
    ; substitute (a0 !-> a0)(blim !-> blim)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, [rsp + 2032]
    mov [rsp + 2032], rsi
    ; let x0: List[i64] = Cons(a3, as0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
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
    je lab48517
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48518

lab48517:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48515
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48508
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48506
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48507

lab48506:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48507:

lab48508:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48510

lab48509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48510:

lab48511:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48514
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48512
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48513

lab48512:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48513:

lab48514:
    jmp lab48516

lab48515:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48516:

lab48518:
    ; #load tag
    mov r15, 5
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov r12, r8
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; jump bench_lscomp1_
    jmp bench_lscomp1_

int_lscomp2_:
    ; substitute (a0 !-> a0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(ls !-> ls);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rax
    mov rax, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_48519]
    add rcx, [rsp + 2008]
    jmp rcx

List_i64_48519:
    jmp near List_i64_48519_Nil
    jmp near List_i64_48519_Cons

List_i64_48519_Nil:
    ; substitute (t1 !-> t1)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rax, r12
    mov rdx, r13
    mov r12, [rsp + 2032]
    mov r13, [rsp + 2024]
    ; jump int_lscomp1_
    jmp int_lscomp1_

List_i64_48519_Cons:
    ; #load from memory
    mov rcx, [rsp + 2016]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab48521
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
    je lab48520
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48520:
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rax, [rsp + 2040]
    jmp lab48522

lab48521:
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
    mov rax, [rsp + 2040]

lab48522:
    ; substitute (b !-> b)(op0 !-> op)(a6 !-> a)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(bstep !-> bstep)(t2 !-> t2)(a0 !-> a0)(bstart !-> bstart);
    ; #share op
    cmp qword [rsp + 2032], 0
    je lab48523
    mov rcx, [rsp + 2032]
    add qword [rcx + 0], 1

lab48523:
    ; #move variables
    mov [rsp + 1984], rax
    mov [rsp + 1976], rdx
    mov [rsp + 1960], rdi
    mov rdx, [rsp + 2008]
    mov [rsp + 2008], r9
    mov r9, r15
    mov rsi, [rsp + 2032]
    mov rdi, [rsp + 2024]
    ; new a1: Either[i64, Bool] = (blim, t1, a, op, bstep, t2, a0, bstart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab48535
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48536

lab48535:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48533
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48526
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48524
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48525

lab48524:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48525:

lab48526:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48528

lab48527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48528:

lab48529:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48531

lab48530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48531:

lab48532:
    jmp lab48534

lab48533:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48534:

lab48536:
    ; ##store link to previous block
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
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
    je lab48548
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48549

lab48548:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48546
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48539
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48537
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48538

lab48537:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48538:

lab48539:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48542
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48540
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48541

lab48540:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48541:

lab48542:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48545
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48543
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48544

lab48543:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48544:

lab48545:
    jmp lab48547

lab48546:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48547:

lab48549:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab48561
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab48562

lab48561:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48559
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48552
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48550
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48551

lab48550:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48551:

lab48552:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48555
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48553
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48554

lab48553:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48554:

lab48555:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48558
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48556
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48557

lab48556:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48557:

lab48558:
    jmp lab48560

lab48559:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48560:

lab48562:
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
    je lab48574
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48575

lab48574:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48572
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48565
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48563
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48564

lab48563:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48564:

lab48565:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48568
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48566
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48567

lab48566:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48567:

lab48568:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48571
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48569
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48570

lab48569:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48570:

lab48571:
    jmp lab48573

lab48572:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48573:

lab48575:
    ; #load tag
    lea r11, [rel Either_i64_Bool_48576]
    ; substitute (a6 !-> a6)(op0 !-> op0)(b !-> b)(a1 !-> a1);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; let a3: Fun[i64, Either[i64, Bool]] = Ap(b, a1);
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
    je lab48588
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48589

lab48588:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48586
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48579
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48577
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48578

lab48577:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48578:

lab48579:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48582
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48580
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48581

lab48580:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48581:

lab48582:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48585
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48583
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48584

lab48583:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48584:

lab48585:
    jmp lab48587

lab48586:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48587:

lab48589:
    ; #load tag
    mov r9, 0
    ; substitute (a6 !-> a6)(a3 !-> a3)(op0 !-> op0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke op0 Ap
    jmp r9

Either_i64_Bool_48576:
    jmp near Either_i64_Bool_48576_Left
    jmp near Either_i64_Bool_48576_Right

Either_i64_Bool_48576_Left:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48594
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
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48590
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48590:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48591
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48591:
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
    je lab48592
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48592:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48593
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48593:
    mov rax, [rsp + 2040]
    jmp lab48595

lab48594:
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
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
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

lab48595:
    ; substitute (bstart !-> bstart)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(bstep !-> bstep)(t2 !-> t2)(a0 !-> a0)(a5 !-> a5);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    ; let x0: Either[i64, Bool] = Left(a5);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48607
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48608

lab48607:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48605
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48598
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48596
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48597

lab48596:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48597:

lab48598:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48601
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48599
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48600

lab48599:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48600:

lab48601:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48604
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48602
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48603

lab48602:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48603:

lab48604:
    jmp lab48606

lab48605:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48606:

lab48608:
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a !-> a)(a0 !-> a0)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(t2 !-> t2)(x0 !-> x0);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r9
    mov r9, rdi
    mov rdi, [rsp + 2040]
    mov rsi, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r8
    mov r14, r12
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; jump lift_int_lscomp2_0_
    jmp lift_int_lscomp2_0_

Either_i64_Bool_48576_Right:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48613
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
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48609
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48609:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48610
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48610:
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
    je lab48611
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48611:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48612
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48612:
    mov rax, [rsp + 2040]
    jmp lab48614

lab48613:
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
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
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

lab48614:
    ; substitute (bstart !-> bstart)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(bstep !-> bstep)(t2 !-> t2)(a0 !-> a0)(b0 !-> b0);
    ; #move variables
    mov [rsp + 2000], rax
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    ; let x0: Either[i64, Bool] = Right(b0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1992]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48626
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48627

lab48626:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48624
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48617
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48615
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48616

lab48615:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48616:

lab48617:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48619

lab48618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48619:

lab48620:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48622

lab48621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48622:

lab48623:
    jmp lab48625

lab48624:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48625:

lab48627:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a !-> a)(a0 !-> a0)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(t2 !-> t2)(x0 !-> x0);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r9
    mov r9, rdi
    mov rdi, [rsp + 2040]
    mov rsi, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r8
    mov r14, r12
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    ; jump lift_int_lscomp2_0_
    jmp lift_int_lscomp2_0_

lift_int_lscomp2_0_:
    ; substitute (a !-> a)(t2 !-> t2)(blim !-> blim)(bstart !-> bstart)(bstep !-> bstep)(op !-> op)(t1 !-> t1)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], rsi
    mov rsi, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, rcx
    ; new a2: List[Either[i64, Bool]] = (a0, x0)\{ ... \};
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
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48639
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48640

lab48639:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48637
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48629

lab48628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48629:

lab48630:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48633
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48631
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48632

lab48631:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48632:

lab48633:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48636
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48634
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48635

lab48634:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48635:

lab48636:
    jmp lab48638

lab48637:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48638:

lab48640:
    ; #load tag
    lea rcx, [rel List_Either_i64_Bool_48641]
    mov [rsp + 2008], rcx
    ; substitute (t2 !-> t2)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r11
    mov r11, r9
    mov r9, r13
    mov r13, [rsp + 2024]
    mov [rsp + 2024], r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, rsi
    mov r12, [rsp + 2032]
    mov [rsp + 2032], r14
    ; jump int_lscomp2_
    jmp int_lscomp2_

List_Either_i64_Bool_48641:
    jmp near List_Either_i64_Bool_48641_Nil
    jmp near List_Either_i64_Bool_48641_Cons

List_Either_i64_Bool_48641_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48644
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab48642
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48642:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab48643
    ; ####increment refcount
    add qword [rax + 0], 1

lab48643:
    jmp lab48645

lab48644:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab48645:
    ; let x1: List[Either[i64, Bool]] = Nil();
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

List_Either_i64_Bool_48641_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48648
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab48646
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48646:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab48647
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48647:
    jmp lab48649

lab48648:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab48649:
    ; substitute (x0 !-> x0)(a0 !-> a0)(a4 !-> a4)(as0 !-> as0);
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
    ; let x1: List[Either[i64, Bool]] = Cons(a4, as0);
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
    je lab48661
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48662

lab48661:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48659
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48652
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48650
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48651

lab48650:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48651:

lab48652:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48655
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48653
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48654

lab48653:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48654:

lab48655:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48658
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48656
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48657

lab48656:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48657:

lab48658:
    jmp lab48660

lab48659:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48660:

lab48662:
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

int_lscomp1_:
    ; substitute (a0 !-> a0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(ls !-> ls);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    ; switch ls \{ ... \};
    lea rcx, [rel List_i64_48663]
    add rcx, r15
    jmp rcx

List_i64_48663:
    jmp near List_i64_48663_Nil
    jmp near List_i64_48663_Cons

List_i64_48663_Nil:
    ; substitute (a0 !-> a0);
    ; #erase op
    cmp r12, 0
    je lab48666
    ; ######check refcount
    cmp qword [r12 + 0], 0
    je lab48664
    ; ######either decrement refcount ...
    add qword [r12 + 0], -1
    jmp lab48665

lab48664:
    ; ######... or add block to lazy free list
    mov [r12 + 0], rbp
    mov rbp, r12

lab48665:

lab48666:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_48663_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r14 + 0], 0
    je lab48668
    ; ##either decrement refcount and share children...
    add qword [r14 + 0], -1
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48667
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48667:
    mov r15, [r14 + 40]
    jmp lab48669

lab48668:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r14 + 40]

lab48669:
    ; substitute (blim0 !-> blim)(bstart0 !-> bstart)(bstep0 !-> bstep)(blim !-> blim)(op !-> op)(a !-> a)(t1 !-> t1)(a0 !-> a0)(bstart !-> bstart)(bstep !-> bstep);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 1992], rdi
    mov [rsp + 1976], r9
    mov rdx, r11
    ; new a1: List[i64] = (blim, op, a, t1, a0, bstart, bstep)\{ ... \};
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
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48681
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48682

lab48681:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48679
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48672
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48670
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48671

lab48670:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48671:

lab48672:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48675
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48673
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48674

lab48673:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48674:

lab48675:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48678
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48676
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48677

lab48676:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48677:

lab48678:
    jmp lab48680

lab48679:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48680:

lab48682:
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
    je lab48694
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48695

lab48694:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48692
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48685
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48683
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48684

lab48683:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48684:

lab48685:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48687

lab48686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48687:

lab48688:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48691
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48689
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48690

lab48689:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48690:

lab48691:
    jmp lab48693

lab48692:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48693:

lab48695:
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
    je lab48707
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48708

lab48707:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48705
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48698
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48696
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48697

lab48696:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48697:

lab48698:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48701
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48699
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48700

lab48699:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48700:

lab48701:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48704
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48702
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48703

lab48702:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48703:

lab48704:
    jmp lab48706

lab48705:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48706:

lab48708:
    ; #load tag
    lea r11, [rel List_i64_48709]
    ; x1 <- bstart0 + bstep0;
    mov r13, rdi
    add r13, r9
    ; substitute (bstart0 !-> bstart0)(x1 !-> x1)(blim0 !-> blim0)(a1 !-> a1);
    ; #move variables
    mov r9, rdx
    mov rdx, rdi
    mov rdi, r13
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_48709:
    jmp near List_i64_48709_Nil
    jmp near List_i64_48709_Cons

List_i64_48709_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48713
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab48710
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48710:
    mov rdx, [rax + 24]
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48711
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48711:
    mov r9, [r8 + 24]
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48712
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48712:
    jmp lab48714

lab48713:
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
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab48714:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, r11
    mov r11, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    mov rcx, r15
    mov r15, r9
    mov r9, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    mov rax, [rsp + 2016]
    mov [rsp + 2016], r12
    mov r12, r10
    ; jump int_lscomp2_
    jmp int_lscomp2_

List_i64_48709_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48718
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48715
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48715:
    mov r9, [r8 + 24]
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48716
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48716:
    mov r13, [r12 + 24]
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48717
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48717:
    mov rax, [rsp + 2040]
    jmp lab48719

lab48718:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab48719:
    ; substitute (bstep !-> bstep)(bstart !-> bstart)(blim !-> blim)(op !-> op)(a !-> a)(t1 !-> t1)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    mov [rsp + 2000], rsi
    ; let x0: List[i64] = Cons(a3, as0);
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
    je lab48731
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48732

lab48731:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48729
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48722
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48720
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48721

lab48720:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48721:

lab48722:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48725
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48723
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48724

lab48723:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48724:

lab48725:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48728
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48726
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48727

lab48726:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48727:

lab48728:
    jmp lab48730

lab48729:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48730:

lab48732:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(t1 !-> t1)(a !-> a)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    mov [rsp + 2024], r11
    mov r11, r9
    mov r9, rdx
    mov rdx, [rsp + 2040]
    mov rax, [rsp + 2016]
    mov rcx, [rsp + 2032]
    mov [rsp + 2016], rcx
    mov [rsp + 2032], r10
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    mov r12, r14
    ; jump int_lscomp2_
    jmp int_lscomp2_

intbench_:
    ; substitute (alim !-> alim)(astart !-> astart)(astep !-> astep)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(a0 !-> a0);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a1: List[i64] = (op, bstart, bstep, blim, a0)\{ ... \};
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
    je lab48744
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48745

lab48744:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48742
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48735
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48733
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48734

lab48733:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48734:

lab48735:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48738
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48736
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48737

lab48736:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48737:

lab48738:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48741
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48739
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48740

lab48739:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48740:

lab48741:
    jmp lab48743

lab48742:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48743:

lab48745:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab48757
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48758

lab48757:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48755
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48748
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48746
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48747

lab48746:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48747:

lab48748:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48751
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48749
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48750

lab48749:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48750:

lab48751:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48754
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48752
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48753

lab48752:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48753:

lab48754:
    jmp lab48756

lab48755:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48756:

lab48758:
    ; #load tag
    lea r11, [rel List_i64_48759]
    ; x1 <- astart + astep;
    mov r13, rdi
    add r13, r9
    ; substitute (astart !-> astart)(x1 !-> x1)(alim !-> alim)(a1 !-> a1);
    ; #move variables
    mov r9, rdx
    mov rdx, rdi
    mov rdi, r13
    ; jump enum_from_then_to_
    jmp enum_from_then_to_

List_i64_48759:
    jmp near List_i64_48759_Nil
    jmp near List_i64_48759_Cons

List_i64_48759_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48762
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab48760
    ; ####increment refcount
    add qword [rax + 0], 1

lab48760:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab48761
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48761:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab48763

lab48762:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab48763:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump int_lscomp1_
    jmp int_lscomp1_

List_i64_48759_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48766
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48764
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48764:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48765
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48765:
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    jmp lab48767

lab48766:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    ; ###release block
    mov [r12 + 0], rbx
    mov rbx, r12
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]

lab48767:
    ; substitute (a0 !-> a0)(blim !-> blim)(op !-> op)(bstart !-> bstart)(bstep !-> bstep)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, [rsp + 2032]
    mov [rsp + 2032], rsi
    ; let x0: List[i64] = Cons(a3, as0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
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
    je lab48779
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48780

lab48779:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48777
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48770
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48768
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48769

lab48768:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48769:

lab48770:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48773
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48771
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48772

lab48771:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48772:

lab48773:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48776
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48774
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48775

lab48774:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48775:

lab48776:
    jmp lab48778

lab48777:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48778:

lab48780:
    ; #load tag
    mov r15, 5
    ; substitute (x0 !-> x0)(bstart !-> bstart)(bstep !-> bstep)(blim !-> blim)(op !-> op)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov r12, r8
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; jump int_lscomp1_
    jmp int_lscomp1_

runbench_:
    ; substitute (alim2 !-> alim)(iop !-> iop)(astart2 !-> astart)(astep2 !-> astep)(alim !-> alim)(astep !-> astep)(astart !-> astart)(jop !-> jop)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 2024], r9
    mov r15, r11
    mov rdx, r13
    ; new a1: List[Either[i64, Bool]] = (alim, astep, astart, jop, a0)\{ ... \};
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
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48792
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48793

lab48792:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48790
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48783
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48781
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48782

lab48781:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48782:

lab48783:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48786
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48784
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48785

lab48784:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48785:

lab48786:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48788

lab48787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48788:

lab48789:
    jmp lab48791

lab48790:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48791:

lab48793:
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
    je lab48805
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab48806

lab48805:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48803
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48796
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48794
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48795

lab48794:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48795:

lab48796:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48799
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48797
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48798

lab48797:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48798:

lab48799:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48802
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48800
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48801

lab48800:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48801:

lab48802:
    jmp lab48804

lab48803:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48804:

lab48806:
    ; #load tag
    lea r13, [rel List_Either_i64_Bool_48807]
    ; substitute (iop !-> iop)(astart2 !-> astart2)(astep2 !-> astep2)(alim2 !-> alim2)(astart20 !-> astart2)(astep20 !-> astep2)(alim20 !-> alim2)(a1 !-> a1);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov [rsp + 2008], r13
    mov r13, r9
    mov r9, r11
    mov r15, r11
    mov r11, rdx
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rax, rsi
    mov [rsp + 2016], r12
    ; jump intbench_
    jmp intbench_

List_Either_i64_Bool_48807:
    jmp near List_Either_i64_Bool_48807_Nil
    jmp near List_Either_i64_Bool_48807_Cons

List_Either_i64_Bool_48807_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48810
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab48808
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48808:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48809
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48809:
    mov r9, [r8 + 24]
    jmp lab48811

lab48810:
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
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab48811:
    ; let res1: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (jop !-> jop)(astart !-> astart)(astep !-> astep)(alim !-> alim)(astart0 !-> astart)(astep0 !-> astep)(alim0 !-> alim)(a0 !-> a0);
    ; #erase res1
    cmp r14, 0
    je lab48814
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab48812
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab48813

lab48812:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab48813:

lab48814:
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov rcx, r9
    mov [rsp + 2008], r13
    mov r13, r9
    mov r9, rdi
    mov r15, rdi
    mov rdi, rcx
    mov rax, r10
    mov [rsp + 2016], r12
    ; jump integerbench_
    jmp integerbench_

List_Either_i64_Bool_48807_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48817
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48815
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48815:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48816
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48816:
    mov r13, [r12 + 24]
    jmp lab48818

lab48817:
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
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]

lab48818:
    ; substitute (a0 !-> a0)(jop !-> jop)(alim !-> alim)(astep !-> astep)(astart !-> astart)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    ; let res1: List[Either[i64, Bool]] = Cons(a2, as0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48830
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48831

lab48830:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48828
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48821
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48819
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48820

lab48819:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48820:

lab48821:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48824
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48822
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48823

lab48822:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48823:

lab48824:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48827
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48825
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48826

lab48825:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48826:

lab48827:
    jmp lab48829

lab48828:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48829:

lab48831:
    ; #load tag
    mov r15, 5
    ; substitute (jop !-> jop)(astart !-> astart)(astep !-> astep)(alim !-> alim)(astart1 !-> astart)(astep1 !-> astep)(alim1 !-> alim)(a0 !-> a0);
    ; #erase res1
    cmp r14, 0
    je lab48834
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab48832
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab48833

lab48832:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab48833:

lab48834:
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov rax, rsi
    mov rdx, rdi
    mov rcx, r11
    mov r15, r11
    mov r11, r9
    mov [rsp + 2024], r9
    mov r9, rcx
    mov rdi, r13
    ; jump integerbench_
    jmp integerbench_

runalltests_:
    ; substitute (astart !-> astart)(astep !-> astep)(alim !-> alim)(a0 !-> a0);
    ; #move variables
    mov r10, [rsp + 2032]
    mov r11, [rsp + 2024]
    ; new z_add: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    lea r13, [rel Fun_i64_Fun_i64_Either_i64_Bool_48835]
    ; new z_sub: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    lea r15, [rel Fun_i64_Fun_i64_Either_i64_Bool_48836]
    ; new z_mul: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48837]
    mov [rsp + 2024], rcx
    ; new z_div: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48838]
    mov [rsp + 2008], rcx
    ; new z_mod: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48839]
    mov [rsp + 1992], rcx
    ; new z_equal: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1984], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48840]
    mov [rsp + 1976], rcx
    ; new z_lt: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1968], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48841]
    mov [rsp + 1960], rcx
    ; new z_leq: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1952], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48842]
    mov [rsp + 1944], rcx
    ; new z_gt: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1936], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48843]
    mov [rsp + 1928], rcx
    ; new z_geq: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1920], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48844]
    mov [rsp + 1912], rcx
    ; new x10: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1904], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_48845]
    mov [rsp + 1896], rcx
    ; substitute (astart0 !-> astart)(astep0 !-> astep)(alim0 !-> alim)(x10 !-> x10)(z_add !-> z_add)(z_sub !-> z_sub)(z_mul !-> z_mul)(z_div !-> z_div)(z_mod !-> z_mod)(z_equal !-> z_equal)(z_lt !-> z_lt)(z_leq !-> z_leq)(z_gt !-> z_gt)(z_geq !-> z_geq)(a0 !-> a0)(astart !-> astart)(astep !-> astep)(alim !-> alim);
    ; #move variables
    mov [rsp + 1880], rdx
    mov [rsp + 1864], rdi
    mov [rsp + 1848], r9
    mov rcx, [rsp + 1904]
    mov [rsp + 1904], r10
    mov r10, rcx
    mov rcx, [rsp + 1896]
    mov [rsp + 1896], r11
    mov r11, rcx
    ; new a80: List[Either[i64, Bool]] = (z_sub, z_mul, z_div, z_mod, z_equal, z_lt, z_leq, z_gt, z_geq, a0, astart, astep, alim)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1848]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1864]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1880]
    mov [rbx + 24], rcx
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1888], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48857
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48858

lab48857:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48855
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48848
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48846
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48847

lab48846:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48847:

lab48848:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48851
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48849
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48850

lab48849:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48850:

lab48851:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48854
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48852
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48853

lab48852:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48853:

lab48854:
    jmp lab48856

lab48855:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48856:

lab48858:
    ; ##store link to previous block
    mov rcx, [rsp + 1888]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1896]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1904]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1912]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1920]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1920], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48870
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48871

lab48870:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48868
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48861
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48859
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48860

lab48859:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48860:

lab48861:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48864
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48862
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48863

lab48862:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48863:

lab48864:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48867
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48865
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48866

lab48865:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48866:

lab48867:
    jmp lab48869

lab48868:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48869:

lab48871:
    ; ##store link to previous block
    mov rcx, [rsp + 1920]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1936]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1944]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48883
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48884

lab48883:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48881
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48874
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48872
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48873

lab48872:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48873:

lab48874:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48877
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48875
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48876

lab48875:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48876:

lab48877:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48880
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48878
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48879

lab48878:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48879:

lab48880:
    jmp lab48882

lab48881:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48882:

lab48884:
    ; ##store link to previous block
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48896
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48897

lab48896:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48894
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48887
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48885
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48886

lab48885:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48886:

lab48887:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48889

lab48888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48889:

lab48890:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48892

lab48891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48892:

lab48893:
    jmp lab48895

lab48894:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48895:

lab48897:
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
    je lab48909
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48910

lab48909:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48907
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48899

lab48898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48899:

lab48900:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48902

lab48901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48902:

lab48903:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48905

lab48904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48905:

lab48906:
    jmp lab48908

lab48907:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48908:

lab48910:
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
    je lab48922
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48923

lab48922:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48920
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48912

lab48911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48912:

lab48913:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48915

lab48914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48915:

lab48916:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48918

lab48917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48918:

lab48919:
    jmp lab48921

lab48920:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48921:

lab48923:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_48924]
    ; substitute (z_add !-> z_add)(x10 !-> x10)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a80 !-> a80);
    ; #move variables
    mov rcx, r13
    mov r13, r9
    mov [rsp + 2008], r9
    mov r9, rdx
    mov [rsp + 1992], r15
    mov r15, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, rdi
    mov [rsp + 2024], rdi
    mov rdi, rcx
    mov rsi, r10
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_48924:
    jmp near List_Either_i64_Bool_48924_Nil
    jmp near List_Either_i64_Bool_48924_Cons

List_Either_i64_Bool_48924_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48935
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab48925
    ; ####increment refcount
    add qword [rsi + 0], 1

lab48925:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab48926
    ; ####increment refcount
    add qword [rax + 0], 1

lab48926:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48927
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48927:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48928
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48928:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48929
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48929:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48930
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48930:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab48931
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48931:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48932
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48932:
    mov rax, [rsp + 2000]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab48933
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48933:
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab48934
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48934:
    mov rax, [rsp + 1968]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rax, [rsp + 2040]
    jmp lab48936

lab48935:
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1984], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    mov rax, [rsp + 1968]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rax, [rsp + 2040]

lab48936:
    ; let add: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1920], 0
    ; #load tag
    mov qword [rsp + 1912], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul)(z_sub !-> z_sub);
    ; #erase add
    cmp qword [rsp + 1920], 0
    je lab48939
    ; ######check refcount
    mov rcx, [rsp + 1920]
    cmp qword [rcx + 0], 0
    je lab48937
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48938

lab48937:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48938:

lab48939:
    ; #move variables
    mov [rsp + 1936], rax
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], r15
    mov r15, r13
    mov r13, r9
    mov r9, [rsp + 1960]
    mov [rsp + 1960], r11
    mov r11, [rsp + 1944]
    mov [rsp + 1944], rdi
    mov rdi, [rsp + 1928]
    mov [rsp + 1928], rdx
    mov rdx, rcx
    mov [rsp + 1952], rsi
    mov rax, [rsp + 1984]
    mov [rsp + 1984], r14
    mov r14, r12
    mov r12, r8
    mov [rsp + 1968], r10
    mov rcx, [rsp + 2000]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2032]
    mov [rsp + 2000], rcx
    mov rcx, [rsp + 2040]
    mov [rsp + 2032], rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 1992], rcx
    mov rcx, [rsp + 2040]
    mov [rsp + 2024], rcx
    ; jump lift_runalltests_0_
    jmp lift_runalltests_0_

List_Either_i64_Bool_48924_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48950
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48940
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48940:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab48941
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48941:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48942
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48942:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab48943
    ; ####increment refcount
    add qword [r12 + 0], 1

lab48943:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab48944
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48944:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab48945
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48945:
    mov rax, [rsp + 2000]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab48946
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48946:
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab48947
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48947:
    mov rax, [rsp + 1968]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1936], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1952], rcx
    cmp rcx, 0
    je lab48948
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48948:
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1968], rcx
    cmp rcx, 0
    je lab48949
    ; ####increment refcount
    add qword [rcx + 0], 1

lab48949:
    mov rax, [rsp + 1936]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1896], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1912], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1928], rcx
    mov rax, [rsp + 2040]
    jmp lab48951

lab48950:
    ; ##... or release blocks onto linear free list when loading
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1984], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    mov rax, [rsp + 1968]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1936], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 32]
    mov [rsp + 1952], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1968], rcx
    mov rax, [rsp + 1936]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1896], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1912], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1928], rcx
    mov rax, [rsp + 2040]

lab48951:
    ; substitute (alim !-> alim)(astep !-> astep)(z_sub !-> z_sub)(z_mul !-> z_mul)(z_div !-> z_div)(z_mod !-> z_mod)(z_equal !-> z_equal)(z_lt !-> z_lt)(z_leq !-> z_leq)(z_gt !-> z_gt)(z_geq !-> z_geq)(a0 !-> a0)(astart !-> astart)(a97 !-> a97)(as8 !-> as8);
    ; #move variables
    mov [rsp + 1920], rax
    mov rcx, [rsp + 1896]
    mov [rsp + 1896], rdi
    mov rdi, [rsp + 1912]
    mov [rsp + 1912], rdx
    mov rdx, rcx
    mov [rsp + 1904], rsi
    ; let add: List[Either[i64, Bool]] = Cons(a97, as8);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1896]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1904]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1912]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1920]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1920], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab48963
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab48964

lab48963:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48961
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48954
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48952
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48953

lab48952:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48953:

lab48954:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48957
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48955
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48956

lab48955:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48956:

lab48957:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48960
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48958
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48959

lab48958:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48959:

lab48960:
    jmp lab48962

lab48961:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48962:

lab48964:
    ; #load tag
    mov qword [rsp + 1912], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul)(z_sub !-> z_sub);
    ; #erase add
    cmp qword [rsp + 1920], 0
    je lab48967
    ; ######check refcount
    mov rcx, [rsp + 1920]
    cmp qword [rcx + 0], 0
    je lab48965
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48966

lab48965:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48966:

lab48967:
    ; #move variables
    mov rcx, [rsp + 1944]
    mov [rsp + 1944], r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1936], r8
    mov rcx, [rsp + 1928]
    mov [rsp + 1928], r9
    mov r9, rcx
    mov rax, [rsp + 1952]
    mov [rsp + 1952], r10
    mov rcx, [rsp + 2032]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1968]
    mov [rsp + 2032], rcx
    mov [rsp + 1968], r14
    mov r14, [rsp + 2040]
    mov rcx, [rsp + 2024]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1960]
    mov [rsp + 2024], rcx
    mov [rsp + 1960], r15
    mov r15, [rsp + 2040]
    mov rcx, [rsp + 1984]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 1984], rcx
    mov rcx, [rsp + 2040]
    mov [rsp + 2016], rcx
    mov rcx, [rsp + 1976]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1976], rcx
    mov rcx, [rsp + 2040]
    mov [rsp + 2008], rcx
    ; jump lift_runalltests_0_
    jmp lift_runalltests_0_

Fun_i64_Fun_i64_Either_i64_Bool_48845:

Fun_i64_Fun_i64_Either_i64_Bool_48845_Ap:
    ; switch a28 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_48968:

Fun_i64_Either_i64_Bool_48968_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48970
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab48969
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48969:
    mov rdi, [rsi + 40]
    jmp lab48971

lab48970:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab48971:
    ; x29 <- a70 + b9;
    mov r11, rdx
    add r11, rdi
    ; substitute (x29 !-> x29)(a29 !-> a29);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a29 Left
    add rdi, 0
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48844:

Fun_i64_Fun_i64_Either_i64_Bool_48844_Ap:
    ; switch a31 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_48972:

Fun_i64_Either_i64_Bool_48972_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48974
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab48973
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48973:
    mov rdi, [rsi + 40]
    jmp lab48975

lab48974:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab48975:
    ; new a33: Bool = (a32)\{ ... \};
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
    je lab48987
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab48988

lab48987:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48985
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48978
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48976
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48977

lab48976:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48977:

lab48978:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48981
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48979
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48980

lab48979:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48980:

lab48981:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48984
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48982
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48983

lab48982:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48983:

lab48984:
    jmp lab48986

lab48985:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48986:

lab48988:
    ; #load tag
    lea r9, [rel Bool_48989]
    ; jump geq_
    jmp geq_

Bool_48989:
    jmp near Bool_48989_True
    jmp near Bool_48989_False

Bool_48989_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48991
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab48990
    ; ####increment refcount
    add qword [rax + 0], 1

lab48990:
    jmp lab48992

lab48991:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab48992:
    ; let x9: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x9 !-> x9)(a32 !-> a32);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a32 Right
    add rdi, 5
    jmp rdi

Bool_48989_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab48994
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab48993
    ; ####increment refcount
    add qword [rax + 0], 1

lab48993:
    jmp lab48995

lab48994:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab48995:
    ; let x9: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x9 !-> x9)(a32 !-> a32);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a32 Right
    add rdi, 5
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48843:

Fun_i64_Fun_i64_Either_i64_Bool_48843_Ap:
    ; switch a34 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_48996:

Fun_i64_Either_i64_Bool_48996_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48998
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab48997
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48997:
    mov rdi, [rsi + 40]
    jmp lab48999

lab48998:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab48999:
    ; new a36: Bool = (a35)\{ ... \};
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
    je lab49011
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49012

lab49011:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49009
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49002
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49000
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49001

lab49000:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49001:

lab49002:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49005
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49003
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49004

lab49003:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49004:

lab49005:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49008
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49006
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49007

lab49006:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49007:

lab49008:
    jmp lab49010

lab49009:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49010:

lab49012:
    ; #load tag
    lea r9, [rel Bool_49013]
    ; jump gt_
    jmp gt_

Bool_49013:
    jmp near Bool_49013_True
    jmp near Bool_49013_False

Bool_49013_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49015
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49014
    ; ####increment refcount
    add qword [rax + 0], 1

lab49014:
    jmp lab49016

lab49015:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49016:
    ; let x8: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x8 !-> x8)(a35 !-> a35);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a35 Right
    add rdi, 5
    jmp rdi

Bool_49013_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49018
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49017
    ; ####increment refcount
    add qword [rax + 0], 1

lab49017:
    jmp lab49019

lab49018:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49019:
    ; let x8: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x8 !-> x8)(a35 !-> a35);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a35 Right
    add rdi, 5
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48842:

Fun_i64_Fun_i64_Either_i64_Bool_48842_Ap:
    ; switch a37 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49020:

Fun_i64_Either_i64_Bool_49020_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49022
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49021
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49021:
    mov rdi, [rsi + 40]
    jmp lab49023

lab49022:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49023:
    ; new a39: Bool = (a38)\{ ... \};
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
    je lab49035
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49036

lab49035:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49033
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49026
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49024
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49025

lab49024:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49025:

lab49026:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49029
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49027
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49028

lab49027:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49028:

lab49029:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49032
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49030
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49031

lab49030:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49031:

lab49032:
    jmp lab49034

lab49033:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49034:

lab49036:
    ; #load tag
    lea r9, [rel Bool_49037]
    ; jump leq_
    jmp leq_

Bool_49037:
    jmp near Bool_49037_True
    jmp near Bool_49037_False

Bool_49037_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49039
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49038
    ; ####increment refcount
    add qword [rax + 0], 1

lab49038:
    jmp lab49040

lab49039:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49040:
    ; let x7: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x7 !-> x7)(a38 !-> a38);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a38 Right
    add rdi, 5
    jmp rdi

Bool_49037_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49042
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49041
    ; ####increment refcount
    add qword [rax + 0], 1

lab49041:
    jmp lab49043

lab49042:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49043:
    ; let x7: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x7 !-> x7)(a38 !-> a38);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a38 Right
    add rdi, 5
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48841:

Fun_i64_Fun_i64_Either_i64_Bool_48841_Ap:
    ; switch a40 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49044:

Fun_i64_Either_i64_Bool_49044_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49046
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49045
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49045:
    mov rdi, [rsi + 40]
    jmp lab49047

lab49046:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49047:
    ; new a42: Bool = (a41)\{ ... \};
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
    je lab49059
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49060

lab49059:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49057
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49050
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49048
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49049

lab49048:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49049:

lab49050:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49053
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49051
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49052

lab49051:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49052:

lab49053:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49056
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49054
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49055

lab49054:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49055:

lab49056:
    jmp lab49058

lab49057:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49058:

lab49060:
    ; #load tag
    lea r9, [rel Bool_49061]
    ; jump lt_
    jmp lt_

Bool_49061:
    jmp near Bool_49061_True
    jmp near Bool_49061_False

Bool_49061_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49063
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49062
    ; ####increment refcount
    add qword [rax + 0], 1

lab49062:
    jmp lab49064

lab49063:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49064:
    ; let x6: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x6 !-> x6)(a41 !-> a41);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a41 Right
    add rdi, 5
    jmp rdi

Bool_49061_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49066
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49065
    ; ####increment refcount
    add qword [rax + 0], 1

lab49065:
    jmp lab49067

lab49066:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49067:
    ; let x6: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x6 !-> x6)(a41 !-> a41);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a41 Right
    add rdi, 5
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48840:

Fun_i64_Fun_i64_Either_i64_Bool_48840_Ap:
    ; switch a43 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49068:

Fun_i64_Either_i64_Bool_49068_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49070
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49069
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49069:
    mov rdi, [rsi + 40]
    jmp lab49071

lab49070:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49071:
    ; new a45: Bool = (a44)\{ ... \};
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
    je lab49083
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49084

lab49083:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49081
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49074
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49072
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49073

lab49072:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49073:

lab49074:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49077
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49075
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49076

lab49075:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49076:

lab49077:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49080
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49078
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49079

lab49078:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49079:

lab49080:
    jmp lab49082

lab49081:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49082:

lab49084:
    ; #load tag
    lea r9, [rel Bool_49085]
    ; jump eq_
    jmp eq_

Bool_49085:
    jmp near Bool_49085_True
    jmp near Bool_49085_False

Bool_49085_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49087
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49086
    ; ####increment refcount
    add qword [rax + 0], 1

lab49086:
    jmp lab49088

lab49087:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49088:
    ; let x5: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x5 !-> x5)(a44 !-> a44);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a44 Right
    add rdi, 5
    jmp rdi

Bool_49085_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49090
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49089
    ; ####increment refcount
    add qword [rax + 0], 1

lab49089:
    jmp lab49091

lab49090:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49091:
    ; let x5: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x5 !-> x5)(a44 !-> a44);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a44 Right
    add rdi, 5
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48839:

Fun_i64_Fun_i64_Either_i64_Bool_48839_Ap:
    ; switch a46 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49092:

Fun_i64_Either_i64_Bool_49092_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49094
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49093
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49093:
    mov rdi, [rsi + 40]
    jmp lab49095

lab49094:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49095:
    ; x4 <- a64 % b3;
    mov rcx, rdx
    mov r11, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rax, r11
    mov r11, rdx
    mov rdx, rcx
    ; substitute (x4 !-> x4)(a47 !-> a47);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a47 Left
    add rdi, 0
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48838:

Fun_i64_Fun_i64_Either_i64_Bool_48838_Ap:
    ; switch a49 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49096:

Fun_i64_Either_i64_Bool_49096_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49098
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49097
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49097:
    mov rdi, [rsi + 40]
    jmp lab49099

lab49098:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49099:
    ; x3 <- a63 / b2;
    mov rcx, rdx
    mov r11, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rdx, rax
    mov rax, r11
    mov r11, rdx
    mov rdx, rcx
    ; substitute (x3 !-> x3)(a50 !-> a50);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a50 Left
    add rdi, 0
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48837:

Fun_i64_Fun_i64_Either_i64_Bool_48837_Ap:
    ; switch a52 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49100:

Fun_i64_Either_i64_Bool_49100_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49102
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49101
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49101:
    mov rdi, [rsi + 40]
    jmp lab49103

lab49102:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49103:
    ; x2 <- a62 * b1;
    mov r11, rdx
    imul r11, rdi
    ; substitute (x2 !-> x2)(a53 !-> a53);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a53 Left
    add rdi, 0
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48836:

Fun_i64_Fun_i64_Either_i64_Bool_48836_Ap:
    ; switch a55 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49104:

Fun_i64_Either_i64_Bool_49104_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49106
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49105
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49105:
    mov rdi, [rsi + 40]
    jmp lab49107

lab49106:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49107:
    ; x1 <- a61 - b0;
    mov r11, rdx
    sub r11, rdi
    ; substitute (x1 !-> x1)(a56 !-> a56);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a56 Left
    add rdi, 0
    jmp rdi

Fun_i64_Fun_i64_Either_i64_Bool_48835:

Fun_i64_Fun_i64_Either_i64_Bool_48835_Ap:
    ; switch a58 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49108:

Fun_i64_Either_i64_Bool_49108_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49110
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49109
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49109:
    mov rdi, [rsi + 40]
    jmp lab49111

lab49110:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49111:
    ; x0 <- a + b;
    mov r11, rdx
    add r11, rdi
    ; substitute (x0 !-> x0)(a59 !-> a59);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a59 Left
    add rdi, 0
    jmp rdi

lift_runalltests_0_:
    ; new x11: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1920], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49112]
    mov [rsp + 1912], rcx
    ; substitute (x11 !-> x11)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_sub !-> z_sub)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul)(z_div !-> z_div)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1904], rax
    mov [rsp + 1896], rdx
    mov [rsp + 1880], rdi
    mov [rsp + 1864], r9
    mov rdx, [rsp + 1912]
    mov [rsp + 1912], r11
    mov rcx, [rsp + 1936]
    mov [rsp + 1936], r12
    mov r12, rcx
    mov rcx, [rsp + 1928]
    mov [rsp + 1928], r13
    mov r13, rcx
    mov rax, [rsp + 1920]
    ; new a81: List[Either[i64, Bool]] = (z_equal, z_geq, z_gt, z_leq, z_lt, z_mod, z_mul, z_div, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1864]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1880]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1896]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1904]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1904], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49124
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49125

lab49124:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49122
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49115
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49113
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49114

lab49113:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49114:

lab49115:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49118
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49116
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49117

lab49116:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49117:

lab49118:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49121
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49119
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49120

lab49119:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49120:

lab49121:
    jmp lab49123

lab49122:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49123:

lab49125:
    ; ##store link to previous block
    mov rcx, [rsp + 1904]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1912]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1928]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1936]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1936], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49137
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49138

lab49137:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49135
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49128
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49126
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49127

lab49126:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49127:

lab49128:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49131
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49129
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49130

lab49129:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49130:

lab49131:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49134
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49132
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49133

lab49132:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49133:

lab49134:
    jmp lab49136

lab49135:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49136:

lab49138:
    ; ##store link to previous block
    mov rcx, [rsp + 1936]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1960]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49150
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49151

lab49150:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49148
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49141
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49139
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49140

lab49139:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49140:

lab49141:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49144
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49142
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49143

lab49142:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49143:

lab49144:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49147
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49145
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49146

lab49145:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49146:

lab49147:
    jmp lab49149

lab49148:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49149:

lab49151:
    ; ##store link to previous block
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab49163
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49164

lab49163:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49161
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49154
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49152
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49153

lab49152:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49153:

lab49154:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49157
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49155
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49156

lab49155:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49156:

lab49157:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49160
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49158
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49159

lab49158:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49159:

lab49160:
    jmp lab49162

lab49161:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49162:

lab49164:
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49176
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49177

lab49176:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49174
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49167
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49165
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49166

lab49165:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49166:

lab49167:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49170
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49168
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49169

lab49168:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49169:

lab49170:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49173
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49171
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49172

lab49171:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49172:

lab49173:
    jmp lab49175

lab49174:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49175:

lab49177:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49189
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49190

lab49189:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49187
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49180
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49178
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49179

lab49178:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49179:

lab49180:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49183
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49181
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49182

lab49181:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49182:

lab49183:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49186
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49184
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49185

lab49184:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49185:

lab49186:
    jmp lab49188

lab49187:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49188:

lab49190:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49191]
    ; substitute (z_sub !-> z_sub)(x11 !-> x11)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a81 !-> a81);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49191:
    jmp near List_Either_i64_Bool_49191_Nil
    jmp near List_Either_i64_Bool_49191_Cons

List_Either_i64_Bool_49191_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49201
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab49192
    ; ####increment refcount
    add qword [rax + 0], 1

lab49192:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab49193
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49193:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab49194
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49194:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab49195
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49195:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49196
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49196:
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49197
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49197:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49198
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49198:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49199
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49199:
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab49200
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49200:
    mov rax, [rsp + 2040]
    jmp lab49202

lab49201:
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
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
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
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    mov rax, [rsp + 2040]

lab49202:
    ; let sub: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1936], 0
    ; #load tag
    mov qword [rsp + 1928], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul);
    ; #erase sub
    cmp qword [rsp + 1936], 0
    je lab49205
    ; ######check refcount
    mov rcx, [rsp + 1936]
    cmp qword [rcx + 0], 0
    je lab49203
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49204

lab49203:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49204:

lab49205:
    ; #move variables
    mov [rsp + 1968], r14
    mov r14, rax
    mov rcx, [rsp + 1976]
    mov [rsp + 2040], rcx
    mov [rsp + 1976], r13
    mov r13, [rsp + 2008]
    mov [rsp + 2008], r9
    mov r9, [rsp + 1944]
    mov rcx, [rsp + 2024]
    mov [rsp + 1944], rcx
    mov [rsp + 2024], rdi
    mov rdi, [rsp + 1960]
    mov [rsp + 1960], r15
    mov r15, rdx
    mov rdx, [rsp + 2040]
    mov rcx, [rsp + 2032]
    mov [rsp + 1952], rcx
    mov [rsp + 2032], rsi
    mov rax, [rsp + 1984]
    mov [rsp + 1984], r12
    mov r12, [rsp + 2016]
    mov [rsp + 2016], r8
    mov [rsp + 2000], r10
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], r11
    mov r11, rcx
    ; jump lift_runalltests_1_
    jmp lift_runalltests_1_

List_Either_i64_Bool_49191_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49215
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab49206
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49206:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab49207
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49207:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49208
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49208:
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49209
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49209:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49210
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49210:
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
    je lab49211
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49211:
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49212
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49212:
    mov rax, [rsp + 1984]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1952], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab49213
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49213:
    mov rax, [rsp + 1952]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1912], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1952], rcx
    cmp rcx, 0
    je lab49214
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49214:
    mov rax, [rsp + 2040]
    jmp lab49216

lab49215:
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
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1952], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    mov rax, [rsp + 1952]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1912], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1952], rcx
    mov rax, [rsp + 2040]

lab49216:
    ; substitute (astart !-> astart)(alim !-> alim)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul)(z_div !-> z_div)(astep !-> astep)(a0 !-> a0)(a96 !-> a96)(as7 !-> as7);
    ; #move variables
    mov [rsp + 1936], rax
    mov rcx, [rsp + 1912]
    mov [rsp + 1912], rdi
    mov rdi, [rsp + 1928]
    mov [rsp + 1928], rdx
    mov rdx, rcx
    mov [rsp + 1920], rsi
    ; let sub: List[Either[i64, Bool]] = Cons(a96, as7);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1912]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1920]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1928]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1936]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1936], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49228
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49229

lab49228:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49226
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49219
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49217
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49218

lab49217:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49218:

lab49219:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49222
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49220
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49221

lab49220:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49221:

lab49222:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49225
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49223
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49224

lab49223:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49224:

lab49225:
    jmp lab49227

lab49226:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49227:

lab49229:
    ; #load tag
    mov qword [rsp + 1928], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_mul !-> z_mul);
    ; #erase sub
    cmp qword [rsp + 1936], 0
    je lab49232
    ; ######check refcount
    mov rcx, [rsp + 1936]
    cmp qword [rcx + 0], 0
    je lab49230
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49231

lab49230:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49231:

lab49232:
    ; #move variables
    mov rcx, [rsp + 1944]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 1992]
    mov [rsp + 1944], rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov r9, rdx
    mov rdx, [rsp + 2040]
    mov rax, [rsp + 1952]
    mov rcx, [rsp + 2000]
    mov [rsp + 1952], rcx
    mov [rsp + 2000], r14
    mov r14, r8
    mov rcx, [rsp + 2016]
    mov [rsp + 1968], rcx
    mov [rsp + 2016], r12
    mov r12, [rsp + 1984]
    mov rcx, [rsp + 2032]
    mov [rsp + 1984], rcx
    mov [rsp + 2032], r10
    mov rcx, [rsp + 1960]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1960], rcx
    mov [rsp + 2008], r13
    mov r13, [rsp + 1976]
    mov rcx, [rsp + 2024]
    mov [rsp + 1976], rcx
    mov [rsp + 2024], r11
    mov r11, [rsp + 2040]
    ; jump lift_runalltests_1_
    jmp lift_runalltests_1_

Fun_i64_Fun_i64_Either_i64_Bool_49112:

Fun_i64_Fun_i64_Either_i64_Bool_49112_Ap:
    ; switch a25 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49233:

Fun_i64_Either_i64_Bool_49233_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49235
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49234
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49234:
    mov rdi, [rsi + 40]
    jmp lab49236

lab49235:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49236:
    ; x28 <- a71 - b10;
    mov r11, rdx
    sub r11, rdi
    ; substitute (x28 !-> x28)(a26 !-> a26);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a26 Left
    add rdi, 0
    jmp rdi

lift_runalltests_1_:
    ; new x12: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1936], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49237]
    mov [rsp + 1928], rcx
    ; substitute (x12 !-> x12)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_mul !-> z_mul)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_div !-> z_div)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1920], rax
    mov [rsp + 1912], rdx
    mov [rsp + 1896], rdi
    mov [rsp + 1880], r9
    mov rdx, [rsp + 1928]
    mov [rsp + 1928], r11
    mov rcx, [rsp + 1952]
    mov [rsp + 1952], r12
    mov r12, rcx
    mov rcx, [rsp + 1944]
    mov [rsp + 1944], r13
    mov r13, rcx
    mov rax, [rsp + 1936]
    ; new a82: List[Either[i64, Bool]] = (z_equal, z_geq, z_gt, z_leq, z_lt, z_mod, z_div, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1880]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1896]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1912]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1920]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1920], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49249
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49250

lab49249:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49247
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49240
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49238
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49239

lab49238:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49239:

lab49240:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49243
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49241
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49242

lab49241:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49242:

lab49243:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49246
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49244
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49245

lab49244:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49245:

lab49246:
    jmp lab49248

lab49247:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49248:

lab49250:
    ; ##store link to previous block
    mov rcx, [rsp + 1920]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49262
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49263

lab49262:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49260
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49253
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49251
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49252

lab49251:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49252:

lab49253:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49256
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49254
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49255

lab49254:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49255:

lab49256:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49259
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49257
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49258

lab49257:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49258:

lab49259:
    jmp lab49261

lab49260:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49261:

lab49263:
    ; ##store link to previous block
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 32], rcx
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49275
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49276

lab49275:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49273
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49266
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49264
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49265

lab49264:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49265:

lab49266:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49269
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49267
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49268

lab49267:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49268:

lab49269:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49272
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49270
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49271

lab49270:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49271:

lab49272:
    jmp lab49274

lab49273:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49274:

lab49276:
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
    je lab49288
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49289

lab49288:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49286
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49279
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49277
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49278

lab49277:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49278:

lab49279:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49282
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49280
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49281

lab49280:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49281:

lab49282:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49285
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49283
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49284

lab49283:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49284:

lab49285:
    jmp lab49287

lab49286:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49287:

lab49289:
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
    je lab49301
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49302

lab49301:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49299
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49292
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49290
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49291

lab49290:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49291:

lab49292:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49295
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49293
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49294

lab49293:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49294:

lab49295:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49298
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49296
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49297

lab49296:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49297:

lab49298:
    jmp lab49300

lab49299:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49300:

lab49302:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49303]
    ; substitute (z_mul !-> z_mul)(x12 !-> x12)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a82 !-> a82);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49303:
    jmp near List_Either_i64_Bool_49303_Nil
    jmp near List_Either_i64_Bool_49303_Cons

List_Either_i64_Bool_49303_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49312
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab49304
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49304:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab49305
    ; ####increment refcount
    add qword [rax + 0], 1

lab49305:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49306
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49306:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49307
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49307:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab49308
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49308:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49309
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49309:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49310
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49310:
    mov rax, [rsp + 2000]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab49311
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49311:
    mov rax, [rsp + 2040]
    jmp lab49313

lab49312:
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    mov rax, [rsp + 2040]

lab49313:
    ; let mul: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1952], 0
    ; #load tag
    mov qword [rsp + 1944], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod);
    ; #erase mul
    cmp qword [rsp + 1952], 0
    je lab49316
    ; ######check refcount
    mov rcx, [rsp + 1952]
    cmp qword [rcx + 0], 0
    je lab49314
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49315

lab49314:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49315:

lab49316:
    ; #move variables
    mov [rsp + 1968], r14
    mov r14, rax
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], r11
    mov r11, [rsp + 2008]
    mov [rsp + 2008], r9
    mov r9, [rsp + 1960]
    mov [rsp + 1960], r15
    mov r15, rdx
    mov rdx, rcx
    mov [rsp + 1984], r12
    mov r12, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], r13
    mov r13, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, rcx
    mov [rsp + 2016], r8
    mov rax, [rsp + 2000]
    mov [rsp + 2000], r10
    ; jump lift_runalltests_2_
    jmp lift_runalltests_2_

List_Either_i64_Bool_49303_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49325
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49317
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49317:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49318
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49318:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab49319
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49319:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49320
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49320:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49321
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49321:
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49322
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49322:
    mov rax, [rsp + 2000]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab49323
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49323:
    mov rax, [rsp + 1968]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1968], rcx
    cmp rcx, 0
    je lab49324
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49324:
    mov rax, [rsp + 2040]
    jmp lab49326

lab49325:
    ; ##... or release blocks onto linear free list when loading
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 32]
    mov [rsp + 2016], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1968], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    mov rax, [rsp + 1968]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1928], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1968], rcx
    mov rax, [rsp + 2040]

lab49326:
    ; substitute (astart !-> astart)(alim !-> alim)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(z_div !-> z_div)(astep !-> astep)(a0 !-> a0)(a95 !-> a95)(as6 !-> as6);
    ; #move variables
    mov [rsp + 1952], rax
    mov rcx, [rsp + 1928]
    mov [rsp + 1928], rdi
    mov rdi, [rsp + 1944]
    mov [rsp + 1944], rdx
    mov rdx, rcx
    mov [rsp + 1936], rsi
    ; let mul: List[Either[i64, Bool]] = Cons(a95, as6);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1936]
    mov [rbx + 48], rcx
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 32], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49338
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49339

lab49338:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49336
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49328

lab49327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49328:

lab49329:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49332
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49330
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49331

lab49330:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49331:

lab49332:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49335
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49333
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49334

lab49333:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49334:

lab49335:
    jmp lab49337

lab49336:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49337:

lab49339:
    ; #load tag
    mov qword [rsp + 1944], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod);
    ; #erase mul
    cmp qword [rsp + 1952], 0
    je lab49342
    ; ######check refcount
    mov rcx, [rsp + 1952]
    cmp qword [rcx + 0], 0
    je lab49340
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49341

lab49340:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49341:

lab49342:
    ; #move variables
    mov rcx, [rsp + 1960]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1960], rcx
    mov [rsp + 2008], r13
    mov r13, [rsp + 1992]
    mov [rsp + 1992], r15
    mov r15, r9
    mov r9, rdx
    mov rdx, [rsp + 2040]
    mov rax, [rsp + 1968]
    mov rcx, [rsp + 2016]
    mov [rsp + 1968], rcx
    mov [rsp + 2016], r12
    mov r12, [rsp + 2000]
    mov [rsp + 2000], r14
    mov r14, r8
    mov rcx, [rsp + 2032]
    mov [rsp + 1984], rcx
    mov [rsp + 2032], r10
    mov rcx, [rsp + 1976]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 1976], rcx
    mov [rsp + 2024], r11
    mov r11, [rsp + 2040]
    ; jump lift_runalltests_2_
    jmp lift_runalltests_2_

Fun_i64_Fun_i64_Either_i64_Bool_49237:

Fun_i64_Fun_i64_Either_i64_Bool_49237_Ap:
    ; switch a22 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49343:

Fun_i64_Either_i64_Bool_49343_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49345
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49344
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49344:
    mov rdi, [rsi + 40]
    jmp lab49346

lab49345:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49346:
    ; x27 <- a72 * b11;
    mov r11, rdx
    imul r11, rdi
    ; substitute (x27 !-> x27)(a23 !-> a23);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a23 Left
    add rdi, 0
    jmp rdi

lift_runalltests_2_:
    ; new x13: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1952], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49347]
    mov [rsp + 1944], rcx
    ; substitute (x13 !-> x13)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_div !-> z_div)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1936], rax
    mov [rsp + 1928], rdx
    mov [rsp + 1912], rdi
    mov [rsp + 1896], r9
    mov rdx, [rsp + 1944]
    mov [rsp + 1944], r11
    mov rax, [rsp + 1952]
    ; new a83: List[Either[i64, Bool]] = (z_equal, z_geq, z_gt, z_leq, z_lt, z_mod, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1896]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1912]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1928]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1936]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1936], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49359
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49360

lab49359:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49357
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49350
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49348
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49349

lab49348:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49349:

lab49350:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49353
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49351
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49352

lab49351:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49352:

lab49353:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49355

lab49354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49355:

lab49356:
    jmp lab49358

lab49357:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49358:

lab49360:
    ; ##store link to previous block
    mov rcx, [rsp + 1936]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49372
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49373

lab49372:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49370
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49363
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49361
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49362

lab49361:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49362:

lab49363:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49366
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49364
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49365

lab49364:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49365:

lab49366:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49369
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49367
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49368

lab49367:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49368:

lab49369:
    jmp lab49371

lab49370:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49371:

lab49373:
    ; ##store link to previous block
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab49385
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49386

lab49385:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49383
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49376
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49374
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49375

lab49374:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49375:

lab49376:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49379
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49377
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49378

lab49377:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49378:

lab49379:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49382
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49380
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49381

lab49380:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49381:

lab49382:
    jmp lab49384

lab49383:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49384:

lab49386:
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49398
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49399

lab49398:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49396
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49389
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49387
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49388

lab49387:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49388:

lab49389:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49392
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49390
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49391

lab49390:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49391:

lab49392:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49395
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49393
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49394

lab49393:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49394:

lab49395:
    jmp lab49397

lab49396:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49397:

lab49399:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49411
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49412

lab49411:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49409
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49402
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49400
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49401

lab49400:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49401:

lab49402:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49405
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49403
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49404

lab49403:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49404:

lab49405:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49408
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49406
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49407

lab49406:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49407:

lab49408:
    jmp lab49410

lab49409:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49410:

lab49412:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49413]
    ; substitute (z_div !-> z_div)(x13 !-> x13)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a83 !-> a83);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49413:
    jmp near List_Either_i64_Bool_49413_Nil
    jmp near List_Either_i64_Bool_49413_Cons

List_Either_i64_Bool_49413_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49421
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab49414
    ; ####increment refcount
    add qword [rax + 0], 1

lab49414:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab49415
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49415:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab49416
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49416:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab49417
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49417:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49418
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49418:
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49419
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49419:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49420
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49420:
    mov rax, [rsp + 2040]
    jmp lab49422

lab49421:
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
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
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
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
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
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab49422:
    ; let div: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1968], 0
    ; #load tag
    mov qword [rsp + 1960], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod);
    ; #erase div
    cmp qword [rsp + 1968], 0
    je lab49425
    ; ######check refcount
    mov rcx, [rsp + 1968]
    cmp qword [rcx + 0], 0
    je lab49423
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49424

lab49423:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49424:

lab49425:
    ; #move variables
    mov [rsp + 2000], r12
    mov r12, rax
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r11
    mov r11, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, [rsp + 1976]
    mov [rsp + 1976], r15
    mov r15, rdi
    mov rdi, [rsp + 1992]
    mov [rsp + 1992], r13
    mov r13, rdx
    mov rdx, rcx
    mov [rsp + 1984], r14
    mov r14, rsi
    mov [rsp + 2032], r8
    mov rax, [rsp + 2016]
    mov [rsp + 2016], r10
    ; jump lift_runalltests_3_
    jmp lift_runalltests_3_

List_Either_i64_Bool_49413_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49433
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab49426
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49426:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab49427
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49427:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49428
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49428:
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov rcx, [r14 + 32]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49429
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49429:
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49430
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49430:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49431
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49431:
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1944], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab49432
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49432:
    mov rax, [rsp + 2040]
    jmp lab49434

lab49433:
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
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
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
    mov rcx, [rax + 16]
    mov [rsp + 1984], rcx
    mov rax, [rsp + 2040]

lab49434:
    ; substitute (astart !-> astart)(alim !-> alim)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod)(astep !-> astep)(a0 !-> a0)(a94 !-> a94)(as5 !-> as5);
    ; #move variables
    mov [rsp + 1968], rax
    mov rcx, [rsp + 1944]
    mov [rsp + 1944], rdi
    mov rdi, [rsp + 1960]
    mov [rsp + 1960], rdx
    mov rdx, rcx
    mov [rsp + 1952], rsi
    ; let div: List[Either[i64, Bool]] = Cons(a94, as5);
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
    je lab49446
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49447

lab49446:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49444
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49437
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49435
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49436

lab49435:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49436:

lab49437:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49440
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49438
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49439

lab49438:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49439:

lab49440:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49443
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49441
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49442

lab49441:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49442:

lab49443:
    jmp lab49445

lab49444:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49445:

lab49447:
    ; #load tag
    mov qword [rsp + 1960], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_mod !-> z_mod);
    ; #erase div
    cmp qword [rsp + 1968], 0
    je lab49450
    ; ######check refcount
    mov rcx, [rsp + 1968]
    cmp qword [rcx + 0], 0
    je lab49448
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49449

lab49448:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49449:

lab49450:
    ; #move variables
    mov rcx, [rsp + 1976]
    mov [rsp + 2040], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1976], rcx
    mov [rsp + 2008], r15
    mov r15, r11
    mov r11, [rsp + 1992]
    mov rcx, [rsp + 2024]
    mov [rsp + 1992], rcx
    mov [rsp + 2024], r13
    mov r13, r9
    mov r9, rdx
    mov rdx, [rsp + 2040]
    mov rcx, [rsp + 2032]
    mov [rsp + 2000], rcx
    mov [rsp + 2032], r12
    mov r12, r8
    mov rax, [rsp + 1984]
    mov rcx, [rsp + 2016]
    mov [rsp + 1984], rcx
    mov [rsp + 2016], r14
    mov r14, r10
    ; jump lift_runalltests_3_
    jmp lift_runalltests_3_

Fun_i64_Fun_i64_Either_i64_Bool_49347:

Fun_i64_Fun_i64_Either_i64_Bool_49347_Ap:
    ; switch a19 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49451:

Fun_i64_Either_i64_Bool_49451_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49453
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49452
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49452:
    mov rdi, [rsi + 40]
    jmp lab49454

lab49453:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49454:
    ; x26 <- a73 / b12;
    mov rcx, rdx
    mov r11, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rdx, rax
    mov rax, r11
    mov r11, rdx
    mov rdx, rcx
    ; substitute (x26 !-> x26)(a20 !-> a20);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a20 Left
    add rdi, 0
    jmp rdi

lift_runalltests_3_:
    ; new x14: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1968], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49455]
    mov [rsp + 1960], rcx
    ; substitute (x14 !-> x14)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_mod !-> z_mod)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_equal !-> z_equal)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1952], rax
    mov [rsp + 1944], rdx
    mov [rsp + 1928], rdi
    mov [rsp + 1912], r9
    mov rdx, [rsp + 1960]
    mov [rsp + 1960], r11
    mov rcx, [rsp + 1984]
    mov [rsp + 1984], r12
    mov r12, rcx
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], r13
    mov r13, rcx
    mov rax, [rsp + 1968]
    ; new a84: List[Either[i64, Bool]] = (z_geq, z_gt, z_leq, z_lt, z_equal, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1912]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1928]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1952]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1952], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49467
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49468

lab49467:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49465
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49458
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49456
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49457

lab49456:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49457:

lab49458:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49461
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49459
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49460

lab49459:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49460:

lab49461:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49464
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49462
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49463

lab49462:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49463:

lab49464:
    jmp lab49466

lab49465:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49466:

lab49468:
    ; ##store link to previous block
    mov rcx, [rsp + 1952]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49480
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49481

lab49480:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49478
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49471
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49469
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49470

lab49469:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49470:

lab49471:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49474
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49472
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49473

lab49472:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49473:

lab49474:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49477
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49475
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49476

lab49475:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49476:

lab49477:
    jmp lab49479

lab49478:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49479:

lab49481:
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
    je lab49493
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49494

lab49493:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49491
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49484
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49482
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49483

lab49482:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49483:

lab49484:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49487
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49485
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49486

lab49485:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49486:

lab49487:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49490
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49488
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49489

lab49488:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49489:

lab49490:
    jmp lab49492

lab49491:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49492:

lab49494:
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
    je lab49506
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49507

lab49506:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49504
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49497
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49495
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49496

lab49495:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49496:

lab49497:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49500
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49498
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49499

lab49498:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49499:

lab49500:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49503
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49501
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49502

lab49501:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49502:

lab49503:
    jmp lab49505

lab49504:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49505:

lab49507:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49508]
    ; substitute (z_mod !-> z_mod)(x14 !-> x14)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a84 !-> a84);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49508:
    jmp near List_Either_i64_Bool_49508_Nil
    jmp near List_Either_i64_Bool_49508_Cons

List_Either_i64_Bool_49508_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49515
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab49509
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49509:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab49510
    ; ####increment refcount
    add qword [rax + 0], 1

lab49510:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49511
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49511:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49512
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49512:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49513
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49513:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49514
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49514:
    mov rax, [rsp + 2040]
    jmp lab49516

lab49515:
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
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab49516:
    ; let mod: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 1984], 0
    ; #load tag
    mov qword [rsp + 1976], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt);
    ; #erase mod
    cmp qword [rsp + 1984], 0
    je lab49519
    ; ######check refcount
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab49517
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49518

lab49517:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49518:

lab49519:
    ; #move variables
    mov r14, rax
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], r9
    mov r9, [rsp + 1992]
    mov [rsp + 1992], r11
    mov r11, r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov [rsp + 2016], r8
    mov [rsp + 2000], r10
    ; jump lift_runalltests_4_
    jmp lift_runalltests_4_

List_Either_i64_Bool_49508_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49526
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49520
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49520:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49521
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49521:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab49522
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49522:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49523
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49523:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49524
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49524:
    mov rax, [rsp + 2000]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    cmp rcx, 0
    je lab49525
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49525:
    mov rax, [rsp + 2040]
    jmp lab49527

lab49526:
    ; ##... or release blocks onto linear free list when loading
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
    ; ###load link to next block
    mov rcx, [rax + 48]
    mov [rsp + 2000], rcx
    ; ###load values
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2000]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1960], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 24]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2000], rcx
    mov rax, [rsp + 2040]

lab49527:
    ; substitute (astart !-> astart)(alim !-> alim)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(z_equal !-> z_equal)(astep !-> astep)(a0 !-> a0)(a93 !-> a93)(as4 !-> as4);
    ; #move variables
    mov [rsp + 1984], rax
    mov rcx, [rsp + 1960]
    mov [rsp + 1960], rdi
    mov rdi, [rsp + 1976]
    mov [rsp + 1976], rdx
    mov rdx, rcx
    mov [rsp + 1968], rsi
    ; let mod: List[Either[i64, Bool]] = Cons(a93, as4);
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
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49539
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49540

lab49539:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49537
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49530
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49528
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49529

lab49528:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49529:

lab49530:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49533
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49531
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49532

lab49531:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49532:

lab49533:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49536
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49534
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49535

lab49534:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49535:

lab49536:
    jmp lab49538

lab49537:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49538:

lab49540:
    ; #load tag
    mov qword [rsp + 1976], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt);
    ; #erase mod
    cmp qword [rsp + 1984], 0
    je lab49543
    ; ######check refcount
    mov rcx, [rsp + 1984]
    cmp qword [rcx + 0], 0
    je lab49541
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49542

lab49541:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49542:

lab49543:
    ; #move variables
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], r15
    mov r15, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, [rsp + 2000]
    mov [rsp + 2000], r14
    mov r14, r8
    mov [rsp + 2016], r12
    mov r12, [rsp + 2032]
    mov [rsp + 2032], r10
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, rcx
    ; jump lift_runalltests_4_
    jmp lift_runalltests_4_

Fun_i64_Fun_i64_Either_i64_Bool_49455:

Fun_i64_Fun_i64_Either_i64_Bool_49455_Ap:
    ; switch a16 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49544:

Fun_i64_Either_i64_Bool_49544_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49546
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49545
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49545:
    mov rdi, [rsi + 40]
    jmp lab49547

lab49546:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49547:
    ; x25 <- a74 % b13;
    mov rcx, rdx
    mov r11, rax
    mov rax, rdx
    cqo
    idiv rdi
    mov rax, r11
    mov r11, rdx
    mov rdx, rcx
    ; substitute (x25 !-> x25)(a17 !-> a17);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a17 Left
    add rdi, 0
    jmp rdi

lift_runalltests_4_:
    ; new x15: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 1984], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49548]
    mov [rsp + 1976], rcx
    ; substitute (x15 !-> x15)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_equal !-> z_equal)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1968], rax
    mov [rsp + 1960], rdx
    mov [rsp + 1944], rdi
    mov [rsp + 1928], r9
    mov rdx, [rsp + 1976]
    mov [rsp + 1976], r11
    mov rax, [rsp + 1984]
    ; new a85: List[Either[i64, Bool]] = (z_geq, z_gt, z_leq, z_lt, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1928]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1944]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
    mov rcx, [rsp + 1960]
    mov [rbx + 24], rcx
    mov rcx, [rsp + 1968]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1968], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49560
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49561

lab49560:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49558
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49551
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49549
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49550

lab49549:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49550:

lab49551:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49554
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49552
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49553

lab49552:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49553:

lab49554:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49557
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49555
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49556

lab49555:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49556:

lab49557:
    jmp lab49559

lab49558:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49559:

lab49561:
    ; ##store link to previous block
    mov rcx, [rsp + 1968]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
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
    je lab49573
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49574

lab49573:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49571
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49564
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49562
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49563

lab49562:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49563:

lab49564:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49567
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49565
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49566

lab49565:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49566:

lab49567:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49570
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49568
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49569

lab49568:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49569:

lab49570:
    jmp lab49572

lab49571:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49572:

lab49574:
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
    mov rcx, [rsp + 2032]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49586
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49587

lab49586:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49584
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49577
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49575
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49576

lab49575:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49576:

lab49577:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49580
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49578
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49579

lab49578:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49579:

lab49580:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49583
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49581
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49582

lab49581:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49582:

lab49583:
    jmp lab49585

lab49584:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49585:

lab49587:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49599
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49600

lab49599:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49597
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49590
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49588
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49589

lab49588:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49589:

lab49590:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49593
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49591
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49592

lab49591:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49592:

lab49593:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49596
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49594
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49595

lab49594:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49595:

lab49596:
    jmp lab49598

lab49597:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49598:

lab49600:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49601]
    ; substitute (z_equal !-> z_equal)(x15 !-> x15)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a85 !-> a85);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49601:
    jmp near List_Either_i64_Bool_49601_Nil
    jmp near List_Either_i64_Bool_49601_Cons

List_Either_i64_Bool_49601_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49607
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab49602
    ; ####increment refcount
    add qword [rax + 0], 1

lab49602:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab49603
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49603:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab49604
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49604:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49605
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49605:
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49606
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49606:
    jmp lab49608

lab49607:
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
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
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
    mov r14, [r14 + 16]

lab49608:
    ; let equal: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    mov qword [rsp + 1992], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt);
    ; #erase equal
    cmp qword [rsp + 2000], 0
    je lab49611
    ; ######check refcount
    mov rcx, [rsp + 2000]
    cmp qword [rcx + 0], 0
    je lab49609
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49610

lab49609:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49610:

lab49611:
    ; #move variables
    mov r12, rax
    mov rcx, r15
    mov r15, rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], r9
    mov r9, [rsp + 2008]
    mov [rsp + 2008], r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    mov [rsp + 2032], r8
    mov [rsp + 2016], r10
    ; jump lift_runalltests_5_
    jmp lift_runalltests_5_

List_Either_i64_Bool_49601_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49617
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab49612
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49612:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab49613
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49613:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49614
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49614:
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49615
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49615:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2016]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 40]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    cmp rcx, 0
    je lab49616
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49616:
    mov rax, [rsp + 2040]
    jmp lab49618

lab49617:
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
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    ; ###release block
    mov [r14 + 0], rbx
    mov rbx, r14
    ; ###load link to next block
    mov rcx, [r14 + 48]
    mov [rsp + 2016], rcx
    ; ###load values
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
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
    mov rcx, [rax + 16]
    mov [rsp + 2016], rcx
    mov rax, [rsp + 2040]

lab49618:
    ; substitute (astart !-> astart)(alim !-> alim)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt)(astep !-> astep)(a0 !-> a0)(a92 !-> a92)(as3 !-> as3);
    ; #move variables
    mov [rsp + 2000], rax
    mov rcx, [rsp + 1976]
    mov [rsp + 1976], rdi
    mov rdi, [rsp + 1992]
    mov [rsp + 1992], rdx
    mov rdx, rcx
    mov [rsp + 1984], rsi
    ; let equal: List[Either[i64, Bool]] = Cons(a92, as3);
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
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2000], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49630
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49631

lab49630:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49628
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49621
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49619
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49620

lab49619:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49620:

lab49621:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49624
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49622
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49623

lab49622:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49623:

lab49624:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49627
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49625
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49626

lab49625:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49626:

lab49627:
    jmp lab49629

lab49628:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49629:

lab49631:
    ; #load tag
    mov qword [rsp + 1992], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_lt !-> z_lt);
    ; #erase equal
    cmp qword [rsp + 2000], 0
    je lab49634
    ; ######check refcount
    mov rcx, [rsp + 2000]
    cmp qword [rcx + 0], 0
    je lab49632
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49633

lab49632:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49633:

lab49634:
    ; #move variables
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r15
    mov r15, r11
    mov r11, [rsp + 2024]
    mov [rsp + 2024], r13
    mov r13, r9
    mov r9, rdx
    mov rdx, rcx
    mov [rsp + 2032], r12
    mov r12, r8
    mov rax, [rsp + 2016]
    mov [rsp + 2016], r14
    mov r14, r10
    ; jump lift_runalltests_5_
    jmp lift_runalltests_5_

Fun_i64_Fun_i64_Either_i64_Bool_49548:

Fun_i64_Fun_i64_Either_i64_Bool_49548_Ap:
    ; switch a13 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49635:

Fun_i64_Either_i64_Bool_49635_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49637
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49636
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49636:
    mov rdi, [rsi + 40]
    jmp lab49638

lab49637:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49638:
    ; new a15: Bool = (a14)\{ ... \};
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
    je lab49650
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49651

lab49650:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49648
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49641
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49639
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49640

lab49639:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49640:

lab49641:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49644
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49642
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49643

lab49642:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49643:

lab49644:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49647
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49645
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49646

lab49645:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49646:

lab49647:
    jmp lab49649

lab49648:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49649:

lab49651:
    ; #load tag
    lea r9, [rel Bool_49652]
    ; jump eq_
    jmp eq_

Bool_49652:
    jmp near Bool_49652_True
    jmp near Bool_49652_False

Bool_49652_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49654
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49653
    ; ####increment refcount
    add qword [rax + 0], 1

lab49653:
    jmp lab49655

lab49654:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49655:
    ; let x24: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x24 !-> x24)(a14 !-> a14);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a14 Right
    add rdi, 5
    jmp rdi

Bool_49652_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49657
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49656
    ; ####increment refcount
    add qword [rax + 0], 1

lab49656:
    jmp lab49658

lab49657:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49658:
    ; let x24: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x24 !-> x24)(a14 !-> a14);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a14 Right
    add rdi, 5
    jmp rdi

lift_runalltests_5_:
    ; new x16: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2000], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49659]
    mov [rsp + 1992], rcx
    ; substitute (x16 !-> x16)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_lt !-> z_lt)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 1984], rax
    mov [rsp + 1976], rdx
    mov [rsp + 1960], rdi
    mov [rsp + 1944], r9
    mov rdx, [rsp + 1992]
    mov [rsp + 1992], r11
    mov rcx, [rsp + 2016]
    mov [rsp + 2016], r12
    mov r12, rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], r13
    mov r13, rcx
    mov rax, [rsp + 2000]
    ; new a86: List[Either[i64, Bool]] = (z_gt, z_leq, z_geq, astep, a0, alim, astart)\{ ... \};
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
    mov rcx, [rsp + 1984]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49671
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49672

lab49671:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49669
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49662
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49660
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49661

lab49660:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49661:

lab49662:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49665
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49663
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49664

lab49663:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49664:

lab49665:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49668
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49666
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49667

lab49666:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49667:

lab49668:
    jmp lab49670

lab49669:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49670:

lab49672:
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
    je lab49684
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49685

lab49684:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49682
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49675
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49673
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49674

lab49673:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49674:

lab49675:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49678
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49676
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49677

lab49676:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49677:

lab49678:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49681
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49679
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49680

lab49679:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49680:

lab49681:
    jmp lab49683

lab49682:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49683:

lab49685:
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
    je lab49697
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49698

lab49697:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49695
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49687

lab49686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49687:

lab49688:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49691
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49689
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49690

lab49689:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49690:

lab49691:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49694
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49692
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49693

lab49692:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49693:

lab49694:
    jmp lab49696

lab49695:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49696:

lab49698:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49699]
    ; substitute (z_lt !-> z_lt)(x16 !-> x16)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a86 !-> a86);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49699:
    jmp near List_Either_i64_Bool_49699_Nil
    jmp near List_Either_i64_Bool_49699_Cons

List_Either_i64_Bool_49699_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49704
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab49700
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49700:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab49701
    ; ####increment refcount
    add qword [rax + 0], 1

lab49701:
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49702
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49702:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49703
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49703:
    jmp lab49705

lab49704:
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
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
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

lab49705:
    ; let lt: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq);
    ; #erase lt
    cmp qword [rsp + 2016], 0
    je lab49708
    ; ######check refcount
    mov rcx, [rsp + 2016]
    cmp qword [rcx + 0], 0
    je lab49706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49707

lab49706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49707:

lab49708:
    ; #move variables
    mov r14, rax
    mov rcx, r13
    mov r13, r9
    mov r9, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    mov rax, r12
    mov r12, r8
    ; jump lift_runalltests_6_
    jmp lift_runalltests_6_

List_Either_i64_Bool_49699_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49713
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49709
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49709:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49710
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49710:
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49711
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49711:
    mov [rsp + 2040], rax
    mov rax, [rsp + 2032]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1992], rcx
    mov rcx, [rax + 40]
    mov [rsp + 2008], rcx
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab49712
    ; ####increment refcount
    add qword [rcx + 0], 1

lab49712:
    mov rax, [rsp + 2040]
    jmp lab49714

lab49713:
    ; ##... or release blocks onto linear free list when loading
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
    ; ###load link to next block
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    ; ###load values
    mov r15, [r12 + 40]
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
    mov rcx, [rax + 24]
    mov [rsp + 2024], rcx
    mov rcx, [rax + 16]
    mov [rsp + 2032], rcx
    mov rax, [rsp + 2040]

lab49714:
    ; substitute (astart !-> astart)(alim !-> alim)(z_gt !-> z_gt)(z_leq !-> z_leq)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(a91 !-> a91)(as2 !-> as2);
    ; #move variables
    mov [rsp + 2016], rax
    mov rcx, [rsp + 1992]
    mov [rsp + 1992], rdi
    mov rdi, [rsp + 2008]
    mov [rsp + 2008], rdx
    mov rdx, rcx
    mov [rsp + 2000], rsi
    ; let lt: List[Either[i64, Bool]] = Cons(a91, as2);
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
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49726
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49727

lab49726:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49724
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49717
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49715
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49716

lab49715:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49716:

lab49717:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49720
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49718
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49719

lab49718:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49719:

lab49720:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49723
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49721
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49722

lab49721:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49722:

lab49723:
    jmp lab49725

lab49724:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49725:

lab49727:
    ; #load tag
    mov qword [rsp + 2008], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt)(z_leq !-> z_leq);
    ; #erase lt
    cmp qword [rsp + 2016], 0
    je lab49730
    ; ######check refcount
    mov rcx, [rsp + 2016]
    cmp qword [rcx + 0], 0
    je lab49728
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49729

lab49728:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49729:

lab49730:
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, r15
    mov r15, r9
    mov r9, rdx
    mov rdx, rcx
    mov r14, r8
    mov rax, [rsp + 2032]
    mov [rsp + 2032], r10
    ; jump lift_runalltests_6_
    jmp lift_runalltests_6_

Fun_i64_Fun_i64_Either_i64_Bool_49659:

Fun_i64_Fun_i64_Either_i64_Bool_49659_Ap:
    ; switch a10 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49731:

Fun_i64_Either_i64_Bool_49731_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49733
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49732
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49732:
    mov rdi, [rsi + 40]
    jmp lab49734

lab49733:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49734:
    ; new a12: Bool = (a11)\{ ... \};
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
    je lab49746
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49747

lab49746:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49744
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49737
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49735
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49736

lab49735:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49736:

lab49737:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49740
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49738
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49739

lab49738:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49739:

lab49740:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49743
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49741
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49742

lab49741:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49742:

lab49743:
    jmp lab49745

lab49744:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49745:

lab49747:
    ; #load tag
    lea r9, [rel Bool_49748]
    ; jump lt_
    jmp lt_

Bool_49748:
    jmp near Bool_49748_True
    jmp near Bool_49748_False

Bool_49748_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49750
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49749
    ; ####increment refcount
    add qword [rax + 0], 1

lab49749:
    jmp lab49751

lab49750:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49751:
    ; let x23: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x23 !-> x23)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a11 Right
    add rdi, 5
    jmp rdi

Bool_49748_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49753
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49752
    ; ####increment refcount
    add qword [rax + 0], 1

lab49752:
    jmp lab49754

lab49753:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49754:
    ; let x23: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x23 !-> x23)(a11 !-> a11);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a11 Right
    add rdi, 5
    jmp rdi

lift_runalltests_6_:
    ; new x17: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2016], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49755]
    mov [rsp + 2008], rcx
    ; substitute (x17 !-> x17)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_leq !-> z_leq)(z_gt !-> z_gt)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 2000], rax
    mov [rsp + 1992], rdx
    mov [rsp + 1976], rdi
    mov [rsp + 1960], r9
    mov rdx, [rsp + 2008]
    mov [rsp + 2008], r11
    mov rcx, [rsp + 2032]
    mov [rsp + 2032], r12
    mov r12, rcx
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], r13
    mov r13, rcx
    mov rax, [rsp + 2016]
    ; new a87: List[Either[i64, Bool]] = (z_gt, z_geq, astep, a0, alim, astart)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1960]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    mov rcx, [rsp + 1976]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
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
    je lab49767
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49768

lab49767:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49765
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49758
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49756
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49757

lab49756:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49757:

lab49758:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49761
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49759
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49760

lab49759:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49760:

lab49761:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49764
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49762
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49763

lab49762:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49763:

lab49764:
    jmp lab49766

lab49765:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49766:

lab49768:
    ; ##store link to previous block
    mov rcx, [rsp + 2000]
    mov [rbx + 48], rcx
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 40], rcx
    mov qword [rbx + 32], 0
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
    je lab49780
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49781

lab49780:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49778
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49771
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49769
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49770

lab49769:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49770:

lab49771:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49774
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49772
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49773

lab49772:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49773:

lab49774:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49777
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49775
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49776

lab49775:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49776:

lab49777:
    jmp lab49779

lab49778:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49779:

lab49781:
    ; ##store link to previous block
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    ; ##store values
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49793
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49794

lab49793:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49791
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49784
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49782
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49783

lab49782:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49783:

lab49784:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49787
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49785
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49786

lab49785:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49786:

lab49787:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49790
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49788
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49789

lab49788:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49789:

lab49790:
    jmp lab49792

lab49791:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49792:

lab49794:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49795]
    ; substitute (z_leq !-> z_leq)(x17 !-> x17)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a87 !-> a87);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49795:
    jmp near List_Either_i64_Bool_49795_Nil
    jmp near List_Either_i64_Bool_49795_Cons

List_Either_i64_Bool_49795_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49799
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab49796
    ; ####increment refcount
    add qword [rax + 0], 1

lab49796:
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab49797
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49797:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49798
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49798:
    jmp lab49800

lab49799:
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
    mov rsi, [rsi + 16]
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab49800:
    ; let leq: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    mov qword [rsp + 2024], 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt);
    ; #erase leq
    cmp qword [rsp + 2032], 0
    je lab49803
    ; ######check refcount
    mov rcx, [rsp + 2032]
    cmp qword [rcx + 0], 0
    je lab49801
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49802

lab49801:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49802:

lab49803:
    ; #move variables
    mov r14, rax
    mov rcx, r11
    mov r11, r9
    mov r9, r15
    mov r15, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rcx, r13
    mov r13, rdi
    mov rdi, rcx
    mov rax, r10
    ; jump lift_runalltests_7_
    jmp lift_runalltests_7_

List_Either_i64_Bool_49795_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49807
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab49804
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49804:
    ; ###load link to next block
    mov r14, [r10 + 48]
    ; ###load values
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab49805
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49805:
    ; ###load values
    mov rcx, [r14 + 56]
    mov [rsp + 2008], rcx
    mov rcx, [r14 + 40]
    mov [rsp + 2024], rcx
    mov r15, [r14 + 24]
    mov r14, [r14 + 16]
    cmp r14, 0
    je lab49806
    ; ####increment refcount
    add qword [r14 + 0], 1

lab49806:
    jmp lab49808

lab49807:
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
    mov r14, [r14 + 16]

lab49808:
    ; substitute (astart !-> astart)(alim !-> alim)(z_gt !-> z_gt)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(a90 !-> a90)(as1 !-> as1);
    ; #move variables
    mov [rsp + 2032], rax
    mov rcx, [rsp + 2008]
    mov [rsp + 2008], rdi
    mov rdi, [rsp + 2024]
    mov [rsp + 2024], rdx
    mov rdx, rcx
    mov [rsp + 2016], rsi
    ; let leq: List[Either[i64, Bool]] = Cons(a90, as1);
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
    je lab49820
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49821

lab49820:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49818
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49811
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49809
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49810

lab49809:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49810:

lab49811:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49814
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49812
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49813

lab49812:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49813:

lab49814:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49817
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49815
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49816

lab49815:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49816:

lab49817:
    jmp lab49819

lab49818:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49819:

lab49821:
    ; #load tag
    mov qword [rsp + 2024], 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq)(z_gt !-> z_gt);
    ; #erase leq
    cmp qword [rsp + 2032], 0
    je lab49824
    ; ######check refcount
    mov rcx, [rsp + 2032]
    cmp qword [rcx + 0], 0
    je lab49822
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49823

lab49822:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49823:

lab49824:
    ; #move variables
    mov rcx, r15
    mov r15, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, r8
    mov r12, r10
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump lift_runalltests_7_
    jmp lift_runalltests_7_

Fun_i64_Fun_i64_Either_i64_Bool_49755:

Fun_i64_Fun_i64_Either_i64_Bool_49755_Ap:
    ; switch a7 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49825:

Fun_i64_Either_i64_Bool_49825_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49827
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49826
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49826:
    mov rdi, [rsi + 40]
    jmp lab49828

lab49827:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49828:
    ; new a9: Bool = (a8)\{ ... \};
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
    je lab49840
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49841

lab49840:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49838
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49831
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49829
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49830

lab49829:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49830:

lab49831:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49834
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49832
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49833

lab49832:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49833:

lab49834:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49837
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49835
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49836

lab49835:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49836:

lab49837:
    jmp lab49839

lab49838:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49839:

lab49841:
    ; #load tag
    lea r9, [rel Bool_49842]
    ; jump leq_
    jmp leq_

Bool_49842:
    jmp near Bool_49842_True
    jmp near Bool_49842_False

Bool_49842_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49844
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49843
    ; ####increment refcount
    add qword [rax + 0], 1

lab49843:
    jmp lab49845

lab49844:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49845:
    ; let x22: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x22 !-> x22)(a8 !-> a8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a8 Right
    add rdi, 5
    jmp rdi

Bool_49842_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49847
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49846
    ; ####increment refcount
    add qword [rax + 0], 1

lab49846:
    jmp lab49848

lab49847:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49848:
    ; let x22: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x22 !-> x22)(a8 !-> a8);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a8 Right
    add rdi, 5
    jmp rdi

lift_runalltests_7_:
    ; new x18: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov qword [rsp + 2032], 0
    ; #load tag
    lea rcx, [rel Fun_i64_Fun_i64_Either_i64_Bool_49849]
    mov [rsp + 2024], rcx
    ; substitute (x18 !-> x18)(alim0 !-> alim)(astart0 !-> astart)(astep0 !-> astep)(z_gt !-> z_gt)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(alim !-> alim)(astart !-> astart);
    ; #move variables
    mov [rsp + 2016], rax
    mov [rsp + 2008], rdx
    mov [rsp + 1992], rdi
    mov [rsp + 1976], r9
    mov rdx, [rsp + 2024]
    mov [rsp + 2024], r11
    mov rcx, r14
    mov r14, r12
    mov r12, rcx
    mov rcx, r15
    mov r15, r13
    mov r13, rcx
    mov rax, [rsp + 2032]
    ; new a88: List[Either[i64, Bool]] = (z_geq, astep, a0, alim, astart)\{ ... \};
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
    mov rcx, [rsp + 2016]
    mov [rbx + 16], rcx
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49861
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab49862

lab49861:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49859
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49852
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49850
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49851

lab49850:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49851:

lab49852:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49855
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49853
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49854

lab49853:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49854:

lab49855:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49858
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49856
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49857

lab49856:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49857:

lab49858:
    jmp lab49860

lab49859:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49860:

lab49862:
    ; ##store link to previous block
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##store values
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
    je lab49874
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49875

lab49874:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49872
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49865
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49863
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49864

lab49863:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49864:

lab49865:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49868
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49866
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49867

lab49866:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49867:

lab49868:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49871
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49869
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49870

lab49869:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49870:

lab49871:
    jmp lab49873

lab49872:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49873:

lab49875:
    ; #load tag
    lea r15, [rel List_Either_i64_Bool_49876]
    ; substitute (z_gt !-> z_gt)(x18 !-> x18)(astart0 !-> astart0)(astep0 !-> astep0)(alim0 !-> alim0)(astart00 !-> astart0)(astep00 !-> astep0)(alim00 !-> alim0)(a88 !-> a88);
    ; #move variables
    mov rsi, rax
    mov rcx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, rdx
    mov rdx, rcx
    mov [rsp + 1992], r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov [rsp + 2000], r14
    ; jump runbench_
    jmp runbench_

List_Either_i64_Bool_49876:
    jmp near List_Either_i64_Bool_49876_Nil
    jmp near List_Either_i64_Bool_49876_Cons

List_Either_i64_Bool_49876_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49879
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab49877
    ; ####increment refcount
    add qword [rax + 0], 1

lab49877:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49878
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49878:
    jmp lab49880

lab49879:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
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

lab49880:
    ; let gt: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq);
    ; #erase gt
    cmp r14, 0
    je lab49883
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab49881
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab49882

lab49881:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab49882:

lab49883:
    ; #move variables
    mov r12, rax
    mov rcx, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    mov rax, r8
    ; jump lift_runalltests_8_
    jmp lift_runalltests_8_

List_Either_i64_Bool_49876_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49886
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab49884
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49884:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab49885
    ; ####increment refcount
    add qword [r12 + 0], 1

lab49885:
    jmp lab49887

lab49886:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
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

lab49887:
    ; substitute (astart !-> astart)(alim !-> alim)(z_geq !-> z_geq)(astep !-> astep)(a0 !-> a0)(a89 !-> a89)(as0 !-> as0);
    ; #move variables
    mov r14, rax
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    ; let gt: List[Either[i64, Bool]] = Cons(a89, as0);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
    mov [rbx + 40], r15
    mov [rbx + 32], r14
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab49899
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab49900

lab49899:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49897
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49890
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49888
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49889

lab49888:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49889:

lab49890:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49893
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49891
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49892

lab49891:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49892:

lab49893:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49896
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49894
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49895

lab49894:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49895:

lab49896:
    jmp lab49898

lab49897:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49898:

lab49900:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(alim !-> alim)(astart !-> astart)(astep !-> astep)(z_geq !-> z_geq);
    ; #erase gt
    cmp r14, 0
    je lab49903
    ; ######check refcount
    cmp qword [r14 + 0], 0
    je lab49901
    ; ######either decrement refcount ...
    add qword [r14 + 0], -1
    jmp lab49902

lab49901:
    ; ######... or add block to lazy free list
    mov [r14 + 0], rbp
    mov rbp, r14

lab49902:

lab49903:
    ; #move variables
    mov rcx, r13
    mov r13, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, r8
    ; jump lift_runalltests_8_
    jmp lift_runalltests_8_

Fun_i64_Fun_i64_Either_i64_Bool_49849:

Fun_i64_Fun_i64_Either_i64_Bool_49849_Ap:
    ; switch a4 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49904:

Fun_i64_Either_i64_Bool_49904_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49906
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49905
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49905:
    mov rdi, [rsi + 40]
    jmp lab49907

lab49906:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49907:
    ; new a6: Bool = (a5)\{ ... \};
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
    je lab49919
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49920

lab49919:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49917
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49910
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49908
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49909

lab49908:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49909:

lab49910:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49912

lab49911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49912:

lab49913:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49915

lab49914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49915:

lab49916:
    jmp lab49918

lab49917:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49918:

lab49920:
    ; #load tag
    lea r9, [rel Bool_49921]
    ; jump gt_
    jmp gt_

Bool_49921:
    jmp near Bool_49921_True
    jmp near Bool_49921_False

Bool_49921_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49923
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49922
    ; ####increment refcount
    add qword [rax + 0], 1

lab49922:
    jmp lab49924

lab49923:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49924:
    ; let x21: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x21 !-> x21)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a5 Right
    add rdi, 5
    jmp rdi

Bool_49921_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49926
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49925
    ; ####increment refcount
    add qword [rax + 0], 1

lab49925:
    jmp lab49927

lab49926:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49927:
    ; let x21: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x21 !-> x21)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a5 Right
    add rdi, 5
    jmp rdi

lift_runalltests_8_:
    ; new x19: Fun[i64, Fun[i64, Either[i64, Bool]]] = ()\{ ... \};
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    lea r15, [rel Fun_i64_Fun_i64_Either_i64_Bool_49928]
    ; substitute (z_geq !-> z_geq)(x19 !-> x19)(astart !-> astart)(astep !-> astep)(alim !-> alim)(astart0 !-> astart)(astep0 !-> astep)(alim0 !-> alim)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2000], rax
    mov [rsp + 1992], rdx
    mov rdx, r13
    mov r13, rdi
    mov [rsp + 2008], rdi
    mov rdi, r15
    mov r15, r9
    mov [rsp + 2024], r11
    mov rax, r12
    mov rsi, r14
    ; jump runbench_
    jmp runbench_

Fun_i64_Fun_i64_Either_i64_Bool_49928:

Fun_i64_Fun_i64_Either_i64_Bool_49928_Ap:
    ; switch a1 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_Either_i64_Bool_49929:

Fun_i64_Either_i64_Bool_49929_Ap:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49931
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49930
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49930:
    mov rdi, [rsi + 40]
    jmp lab49932

lab49931:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab49932:
    ; new a3: Bool = (a2)\{ ... \};
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
    je lab49944
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab49945

lab49944:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49942
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49935
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49933
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49934

lab49933:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49934:

lab49935:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49938
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49936
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49937

lab49936:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49937:

lab49938:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49941
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49939
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49940

lab49939:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49940:

lab49941:
    jmp lab49943

lab49942:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49943:

lab49945:
    ; #load tag
    lea r9, [rel Bool_49946]
    ; jump geq_
    jmp geq_

Bool_49946:
    jmp near Bool_49946_True
    jmp near Bool_49946_False

Bool_49946_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49948
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49947
    ; ####increment refcount
    add qword [rax + 0], 1

lab49947:
    jmp lab49949

lab49948:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49949:
    ; let x20: Bool = True();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x20 !-> x20)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a2 Right
    add rdi, 5
    jmp rdi

Bool_49946_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49951
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab49950
    ; ####increment refcount
    add qword [rax + 0], 1

lab49950:
    jmp lab49952

lab49951:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab49952:
    ; let x20: Bool = False();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 5
    ; substitute (x20 !-> x20)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a2 Right
    add rdi, 5
    jmp rdi

test_integer_nofib_:
    ; lit x0 <- -2100000000;
    mov r9, -2100000000
    ; lit x1 <- 2100000000;
    mov r11, 2100000000
    ; lit x2 <- -2100000000;
    mov r13, -2100000000
    ; lit x3 <- -2100000000;
    mov r15, -2100000000
    ; substitute (x0 !-> x0)(n !-> n)(x1 !-> x1)(x2 !-> x2)(n0 !-> n)(x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov [rsp + 2024], rdi
    mov rdi, rdx
    mov rcx, r9
    mov r9, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    ; jump runalltests_
    jmp runalltests_

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
    lea rcx, [rel List_Either_i64_Bool_49953]
    add rcx, rdi
    jmp rcx

List_Either_i64_Bool_49953:
    jmp near List_Either_i64_Bool_49953_Nil
    jmp near List_Either_i64_Bool_49953_Cons

List_Either_i64_Bool_49953_Nil:
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

List_Either_i64_Bool_49953_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49956
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab49954
    ; ####increment refcount
    add qword [r8 + 0], 1

lab49954:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab49955
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49955:
    jmp lab49957

lab49956:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab49957:
    ; substitute (a0 !-> a0)(e !-> e);
    ; #erase es
    cmp r8, 0
    je lab49960
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab49958
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab49959

lab49958:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab49959:

lab49960:
    ; switch e \{ ... \};
    lea rcx, [rel Either_i64_Bool_49961]
    add rcx, rdi
    jmp rcx

Either_i64_Bool_49961:
    jmp near Either_i64_Bool_49961_Left
    jmp near Either_i64_Bool_49961_Right

Either_i64_Bool_49961_Left:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49962
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab49963

lab49962:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab49963:
    ; substitute (i !-> i)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Either_i64_Bool_49961_Right:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab49965
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab49964
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49964:
    jmp lab49966

lab49965:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab49966:
    ; switch b \{ ... \};
    lea rcx, [rel Bool_49967]
    add rcx, rdi
    jmp rcx

Bool_49967:
    jmp near Bool_49967_True
    jmp near Bool_49967_False

Bool_49967_True:
    ; lit x1 <- -2;
    mov rdi, -2
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

Bool_49967_False:
    ; lit x2 <- -3;
    mov rdi, -3
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; lit x0 <- 1;
    mov r11, 1
    ; if iters == x0 \{ ... \}
    cmp rdx, r11
    je lab49968
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a4: List[Either[i64, Bool]] = (n, a0, iters)\{ ... \};
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
    je lab49980
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab49981

lab49980:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab49978
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49971
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49969
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49970

lab49969:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49970:

lab49971:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49974
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49972
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49973

lab49972:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49973:

lab49974:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab49977
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49975
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49976

lab49975:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49976:

lab49977:
    jmp lab49979

lab49978:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49979:

lab49981:
    ; #load tag
    lea rdi, [rel List_Either_i64_Bool_49982]
    ; jump test_integer_nofib_
    jmp test_integer_nofib_

List_Either_i64_Bool_49982:
    jmp near List_Either_i64_Bool_49982_Nil
    jmp near List_Either_i64_Bool_49982_Cons

List_Either_i64_Bool_49982_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab49984
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab49983
    ; ####increment refcount
    add qword [rsi + 0], 1

lab49983:
    mov rdx, [rax + 24]
    jmp lab49985

lab49984:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]

lab49985:
    ; let res: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab49988
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab49986
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab49987

lab49986:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab49987:

lab49988:
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_Either_i64_Bool_49982_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab49990
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab49989
    ; ####increment refcount
    add qword [r10 + 0], 1

lab49989:
    mov r9, [r8 + 24]
    jmp lab49991

lab49990:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab49991:
    ; substitute (iters !-> iters)(a0 !-> a0)(n !-> n)(a6 !-> a6)(as1 !-> as1);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let res: List[Either[i64, Bool]] = Cons(a6, as1);
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
    je lab50003
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab50004

lab50003:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50001
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab49994
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49992
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49993

lab49992:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49993:

lab49994:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab49997
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49995
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49996

lab49995:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49996:

lab49997:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50000
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab49998
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab49999

lab49998:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab49999:

lab50000:
    jmp lab50002

lab50001:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50002:

lab50004:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(n !-> n);
    ; #erase res
    cmp r10, 0
    je lab50007
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab50005
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab50006

lab50005:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab50006:

lab50007:
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

lab49968:
    ; substitute (n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, rdi
    mov rsi, r8
    mov rdi, r9
    ; new a3: List[Either[i64, Bool]] = (a0)\{ ... \};
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
    je lab50019
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50020

lab50019:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50017
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50010
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50008
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50009

lab50008:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50009:

lab50010:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50012

lab50011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50012:

lab50013:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50015

lab50014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50015:

lab50016:
    jmp lab50018

lab50017:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50018:

lab50020:
    ; #load tag
    lea rdi, [rel List_Either_i64_Bool_50021]
    ; jump test_integer_nofib_
    jmp test_integer_nofib_

List_Either_i64_Bool_50021:
    jmp near List_Either_i64_Bool_50021_Nil
    jmp near List_Either_i64_Bool_50021_Cons

List_Either_i64_Bool_50021_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab50023
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab50022
    ; ####increment refcount
    add qword [rax + 0], 1

lab50022:
    jmp lab50024

lab50023:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab50024:
    ; let res: List[Either[i64, Bool]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_Either_i64_Bool_50021_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab50026
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab50025
    ; ####increment refcount
    add qword [r8 + 0], 1

lab50025:
    jmp lab50027

lab50026:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab50027:
    ; substitute (a0 !-> a0)(a5 !-> a5)(as0 !-> as0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let res: List[Either[i64, Bool]] = Cons(a5, as0);
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
    je lab50039
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50040

lab50039:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50037
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50030
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50028
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50029

lab50028:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50029:

lab50030:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50033
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50031
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50032

lab50031:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50032:

lab50033:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50036
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50034
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50035

lab50034:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50035:

lab50036:
    jmp lab50038

lab50037:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50038:

lab50040:
    ; #load tag
    mov rdi, 5
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_1_:
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- iters - x2;
    mov r13, rdi
    sub r13, r11
    ; substitute (x3 !-> x3)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rdi, r9
    mov r9, rdx
    mov rdx, r13
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
    je lab50052
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab50053

lab50052:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab50050
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab50043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50042

lab50041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50042:

lab50043:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab50046
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50044
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50045

lab50044:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50045:

lab50046:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab50049
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab50047
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab50048

lab50047:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab50048:

lab50049:
    jmp lab50051

lab50050:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab50051:

lab50053:
    ; #load tag
    lea rdi, [rel _Cont_50054]
    ; jump head_
    jmp head_

_Cont_50054:

_Cont_50054_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab50056
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab50055
    ; ####increment refcount
    add qword [rsi + 0], 1

lab50055:
    jmp lab50057

lab50056:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab50057:
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