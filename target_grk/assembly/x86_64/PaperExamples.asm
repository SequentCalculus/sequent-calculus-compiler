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
    ; actual code

main_:
    ; create a0: _Cont = ()\{ ... \};
    ; #mark no allocation
    mov rax, 0
    ; #load tag
    lea rdx, [rel _Cont_255]
    ; jump labelex_
    jmp labelex_

_Cont_255:

_Cont_255_Ret:
    ; println_i64 x1;
    ; #save caller-save registers
    mov r12, rdx
    sub rsp, 8
    ; #move argument into place
    mov rdi, rdx
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    add rsp, 8
    ; substitute ;
    ; lit x0 <- 0;
    mov rdx, 0
    ; exit x0
    mov rax, rdx
    jmp cleanup

ex211_:
    ; lit x0 <- 2;
    mov rdi, 2
    ; lit x1 <- 3;
    mov r9, 3
    ; x2 <- x0 * x1;
    mov r11, rdi
    imul r11, r9
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

ex212_:
    ; lit x0 <- 2;
    mov rdi, 2
    ; if x0 == 0 \{ ... \}
    cmp rdi, 0
    je lab256
    ; substitute (a0 !-> a0);
    ; lit x2 <- 10;
    mov rdi, 10
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

lab256:
    ; substitute (a0 !-> a0);
    ; lit x1 <- 5;
    mov rdi, 5
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

ex22_:
    ; lit x0 <- 2;
    mov rdi, 2
    ; lit x1 <- 2;
    mov r9, 2
    ; x <- x0 * x1;
    mov r11, rdi
    imul r11, r9
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rdi, r11
    ; x2 <- x * x;
    mov r9, rdi
    imul r9, rdi
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

fac_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab257
    ; substitute (n0 !-> n)(a0 !-> a0)(n !-> n);
    ; #move variables
    mov r9, rdx
    ; create a1: _Cont = (a0, n)\{ ... \};
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
    je lab269
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab270

lab269:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab267
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab260
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab258
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab259

lab258:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab259:

lab260:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab263
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab261
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab262

lab261:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab262:

lab263:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab266
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab264
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab265

lab264:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab265:

lab266:
    jmp lab268

lab267:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab268:

lab270:
    ; #load tag
    lea rdi, [rel _Cont_271]
    ; lit x1 <- 1;
    mov r9, 1
    ; x2 <- n0 - x1;
    mov r11, rdx
    sub r11, r9
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump fac_
    jmp fac_

_Cont_271:

_Cont_271_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab273
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab272
    ; ####increment refcount
    add qword [rsi + 0], 1

lab272:
    jmp lab274

lab273:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab274:
    ; x4 <- n * x0;
    mov r11, r9
    imul r11, rdx
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab257:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x3 <- 1;
    mov rdi, 1
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

ex23_:
    ; lit x0 <- 1;
    mov rdi, 1
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump fac_
    jmp fac_

sum_:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    lea rcx, [rel List_i64_275]
    add rcx, rdi
    jmp rcx

List_i64_275:
    jmp near List_i64_275_Nil
    jmp near List_i64_275_Cons

List_i64_275_Nil:
    ; lit x1 <- 0;
    mov rdi, 0
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_275_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab277
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab276
    ; ####increment refcount
    add qword [r8 + 0], 1

lab276:
    mov rdi, [rsi + 40]
    jmp lab278

lab277:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab278:
    ; substitute (ys !-> ys)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; create a1: _Cont = (y, a0)\{ ... \};
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
    je lab290
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab291

lab290:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab288
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab281
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab279
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab280

lab279:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab280:

lab281:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab284
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab282
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab283

lab282:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab283:

lab284:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab287
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab285
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab286

lab285:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab286:

lab287:
    jmp lab289

lab288:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab289:

lab291:
    ; #load tag
    lea rdi, [rel _Cont_292]
    ; jump sum_
    jmp sum_

_Cont_292:

_Cont_292_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab294
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab293
    ; ####increment refcount
    add qword [r8 + 0], 1

lab293:
    mov rdi, [rsi + 40]
    jmp lab295

lab294:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab295:
    ; x2 <- y + x0;
    mov r11, rdi
    add r11, rdx
    ; substitute (x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

repeat_:
    ; switch a0 \{ ... \};
    lea rcx, [rel Stream_i64_296]
    add rcx, rdi
    jmp rcx

Stream_i64_296:
    jmp near Stream_i64_296_Hd
    jmp near Stream_i64_296_Tl

Stream_i64_296_Hd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab298
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab297
    ; ####increment refcount
    add qword [rsi + 0], 1

lab297:
    jmp lab299

lab298:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab299:
    ; invoke a1 Ret
    jmp rdi

Stream_i64_296_Tl:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab301
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab300
    ; ####increment refcount
    add qword [rsi + 0], 1

lab300:
    jmp lab302

lab301:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab302:
    ; jump repeat_
    jmp repeat_

swap_:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    ; #if there is only one clause, we can just fall through

Pair_i64_i64_303:

Pair_i64_i64_303_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab304
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab305

lab304:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab305:
    ; substitute (z !-> z)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Tup
    jmp r9

swaplazy_:
    ; switch a0 \{ ... \};
    lea rcx, [rel LazyPair_i64_i64_306]
    add rcx, rdi
    jmp rcx

LazyPair_i64_i64_306:
    jmp near LazyPair_i64_i64_306_Fst
    jmp near LazyPair_i64_i64_306_Snd

LazyPair_i64_i64_306_Fst:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab308
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab307
    ; ####increment refcount
    add qword [rsi + 0], 1

lab307:
    jmp lab309

lab308:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab309:
    ; substitute (a1 !-> a1)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Snd
    add rdi, 5
    jmp rdi

LazyPair_i64_i64_306_Snd:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab311
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab310
    ; ####increment refcount
    add qword [rsi + 0], 1

lab310:
    jmp lab312

lab311:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab312:
    ; substitute (a2 !-> a2)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke x Fst
    add rdi, 0
    jmp rdi

ex26_:
    ; lit x0 <- 2;
    mov rdi, 2
    ; x1 <- x0 * x0;
    mov r9, rdi
    imul r9, rdi
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

mult2_:
    ; substitute (a0 !-> a0)(a !-> a)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_313]
    add rcx, r9
    jmp rcx

List_i64_313:
    jmp near List_i64_313_Nil
    jmp near List_i64_313_Cons

List_i64_313_Nil:
    ; substitute (a0 !-> a0);
    ; #erase a
    cmp rsi, 0
    je lab316
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab314
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab315

lab314:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab315:

lab316:
    ; lit x1 <- 1;
    mov rdi, 1
    ; substitute (x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_313_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab318
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab317
    ; ####increment refcount
    add qword [r10 + 0], 1

lab317:
    mov r9, [r8 + 40]
    jmp lab319

lab318:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab319:
    ; if x == 0 \{ ... \}
    cmp r9, 0
    je lab320
    ; substitute (xs !-> xs)(a !-> a)(x !-> x)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; create a1: _Cont = (x, a0)\{ ... \};
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
    je lab332
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab333

lab332:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab330
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab323
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab321
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab322

lab321:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab322:

lab323:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab326
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab324
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab325

lab324:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab325:

lab326:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab328

lab327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab328:

lab329:
    jmp lab331

lab330:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab331:

lab333:
    ; #load tag
    lea r9, [rel _Cont_334]
    ; jump mult2_
    jmp mult2_

_Cont_334:

_Cont_334_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab336
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab335
    ; ####increment refcount
    add qword [r8 + 0], 1

lab335:
    mov rdi, [rsi + 40]
    jmp lab337

lab336:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab337:
    ; x3 <- x * x0;
    mov r11, rdi
    imul r11, rdx
    ; substitute (x3 !-> x3)(a0 !-> a0);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

lab320:
    ; substitute (a !-> a);
    ; #erase a0
    cmp rax, 0
    je lab340
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab338
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab339

lab338:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab339:

lab340:
    ; #erase xs
    cmp r10, 0
    je lab343
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab341
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab342

lab341:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab342:

lab343:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x2 <- 0;
    mov rdi, 0
    ; substitute (x2 !-> x2)(a !-> a);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a Ret
    jmp rdi

sec51_:
    ; lit x0 <- 2;
    mov rdi, 2
    ; lit x1 <- 3;
    mov r9, 3
    ; x2 <- x0 * x1;
    mov r11, rdi
    imul r11, r9
    ; substitute (a0 !-> a0)(x2 !-> x2);
    ; #move variables
    mov rdi, r11
    ; lit x3 <- 4;
    mov r9, 4
    ; x4 <- x2 * x3;
    mov r11, rdi
    imul r11, r9
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

letex_:
    ; lit x <- 2;
    mov rdi, 2
    ; x0 <- x * x;
    mov r9, rdi
    imul r9, rdi
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

labelex_:
    ; lit x0 <- 0;
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

casecase_:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; jump share_casecase_0_
    jmp share_casecase_0_

share_casecase_0_:
    ; switch x0 \{ ... \};
    lea rcx, [rel List_i64_344]
    add rcx, rdi
    jmp rcx

List_i64_344:
    jmp near List_i64_344_Nil
    jmp near List_i64_344_Cons

List_i64_344_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_344_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab346
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab345
    ; ####increment refcount
    add qword [r8 + 0], 1

lab345:
    mov rdi, [rsi + 40]
    jmp lab347

lab346:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab347:
    ; substitute (a0 !-> a0)(ys !-> ys);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch ys \{ ... \};
    lea rcx, [rel List_i64_348]
    add rcx, rdi
    jmp rcx

List_i64_348:
    jmp near List_i64_348_Nil
    jmp near List_i64_348_Cons

List_i64_348_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_348_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab350
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab349
    ; ####increment refcount
    add qword [r8 + 0], 1

lab349:
    mov rdi, [rsi + 40]
    jmp lab351

lab350:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab351:
    ; substitute (x1 !-> x1)(xs0 !-> xs0)(a0 !-> a0);
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

tltltl_:
    ; lit x0 <- 1;
    mov rdi, 1
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let a1: Stream[i64] = Tl(a0);
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
    je lab363
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab364

lab363:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab361
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab354
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab352
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab353

lab352:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab353:

lab354:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab357
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab355
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab356

lab355:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab356:

lab357:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab360
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab358
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab359

lab358:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab359:

lab360:
    jmp lab362

lab361:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab362:

lab364:
    ; #load tag
    mov rdi, 5
    ; let a2: Stream[i64] = Tl(a1);
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
    je lab376
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab377

lab376:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab374
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab367
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab365
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab366

lab365:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab366:

lab367:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab370
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab368
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab369

lab368:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab369:

lab370:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab373
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab371
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab372

lab371:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab372:

lab373:
    jmp lab375

lab374:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab375:

lab377:
    ; #load tag
    mov rdi, 5
    ; let a3: Stream[i64] = Tl(a2);
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
    je lab389
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab390

lab389:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab387
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab380
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab378
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab379

lab378:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab379:

lab380:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab383
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab381
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab382

lab381:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab382:

lab383:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab385

lab384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab385:

lab386:
    jmp lab388

lab387:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab388:

lab390:
    ; #load tag
    mov rdi, 5
    ; jump repeat_
    jmp repeat_

criticalEta1_:
    ; substitute (a0 !-> a0)(b !-> b);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; create x: Fun[i64, i64] = (b)\{ ... \};
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
    je lab402
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab403

lab402:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab400
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab393
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab391
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab392

lab391:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab392:

lab393:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab396
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab394
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab395

lab394:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab395:

lab396:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab399
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab397
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab398

lab397:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab398:

lab399:
    jmp lab401

lab400:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab401:

lab403:
    ; #load tag
    lea rdi, [rel Fun_i64_i64_404]
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab407
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab405
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab406

lab405:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab406:

lab407:
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_408:

Fun_i64_i64_408_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab410
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab409
    ; ####increment refcount
    add qword [rsi + 0], 1

lab409:
    mov rdx, [rax + 40]
    jmp lab411

lab410:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab411:
    ; substitute (a1 !-> a1);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x1 <- 3;
    mov rdi, 3
    ; substitute (x1 !-> x1)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Ret
    jmp rdi

Fun_i64_i64_404:

Fun_i64_i64_404_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab413
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab412
    ; ####increment refcount
    add qword [r8 + 0], 1

lab412:
    jmp lab414

lab413:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab414:
    ; substitute (b !-> b);
    ; #erase a2
    cmp rsi, 0
    je lab417
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab415
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab416

lab415:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab416:

lab417:
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch b \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_418:

Fun_i64_i64_418_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab420
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab419
    ; ####increment refcount
    add qword [rsi + 0], 1

lab419:
    mov rdx, [rax + 40]
    jmp lab421

lab420:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab421:
    ; substitute (a3 !-> a3);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x0 <- 1;
    mov rdi, 1
    ; substitute (x0 !-> x0)(a3 !-> a3);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a3 Ret
    jmp rdi

criticalEta2_:
    ; substitute (a0 !-> a0)(b !-> b);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; create x: Fun[i64, i64] = (b)\{ ... \};
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
    je lab433
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab434

lab433:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab431
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab424
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab422
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab423

lab422:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab423:

lab424:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab427
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab425
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab426

lab425:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab426:

lab427:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab430
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab428
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab429

lab428:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab429:

lab430:
    jmp lab432

lab431:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab432:

lab434:
    ; #load tag
    lea rdi, [rel Fun_i64_i64_435]
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rsi, 0
    je lab438
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab436
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab437

lab436:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab437:

lab438:
    ; switch a0 \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_439:

Fun_i64_i64_439_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab441
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab440
    ; ####increment refcount
    add qword [rsi + 0], 1

lab440:
    mov rdx, [rax + 40]
    jmp lab442

lab441:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab442:
    ; substitute (a1 !-> a1);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x2 <- 3;
    mov rdi, 3
    ; substitute (x2 !-> x2)(a1 !-> a1);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a1 Ret
    jmp rdi

Fun_i64_i64_435:

Fun_i64_i64_435_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab444
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab443
    ; ####increment refcount
    add qword [r8 + 0], 1

lab443:
    jmp lab445

lab444:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab445:
    ; substitute (b !-> b)(x1 !-> x1)(a00 !-> a00);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, rsi
    ; let a2: Fun[i64, i64] = Apply(x1, a00);
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
    je lab457
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab458

lab457:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab455
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab448
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab446
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab447

lab446:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab447:

lab448:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab451
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab449
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab450

lab449:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab450:

lab451:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab454
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab452
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab453

lab452:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab453:

lab454:
    jmp lab456

lab455:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab456:

lab458:
    ; #load tag
    mov rdi, 0
    ; substitute (b !-> b);
    ; #erase a2
    cmp rsi, 0
    je lab461
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab459
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab460

lab459:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab460:

lab461:
    ; switch b \{ ... \};
    ; #if there is only one clause, we can just fall through

Fun_i64_i64_462:

Fun_i64_i64_462_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab464
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab463
    ; ####increment refcount
    add qword [rsi + 0], 1

lab463:
    mov rdx, [rax + 40]
    jmp lab465

lab464:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]

lab465:
    ; substitute (a3 !-> a3);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x0 <- 1;
    mov rdi, 1
    ; substitute (x0 !-> x0)(a3 !-> a3);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a3 Ret
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