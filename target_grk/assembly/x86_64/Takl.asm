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
    lea r13, [rel _Cont_47215]
    ; jump main_loop_
    jmp main_loop_

_Cont_47215:

_Cont_47215_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

not_:
    ; substitute (a0 !-> a0)(b !-> b);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch b \{ ... \};
    lea rcx, [rel Bool_47216]
    add rcx, rdi
    jmp rcx

Bool_47216:
    jmp near Bool_47216_True
    jmp near Bool_47216_False

Bool_47216_True:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_47216_False:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

null_:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    lea rcx, [rel List_i64_47217]
    add rcx, rdi
    jmp rcx

List_i64_47217:
    jmp near List_i64_47217_Nil
    jmp near List_i64_47217_Cons

List_i64_47217_Nil:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_i64_47217_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47219
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab47218
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47218:
    mov rdi, [rsi + 40]
    jmp lab47220

lab47219:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab47220:
    ; substitute (a0 !-> a0);
    ; #erase xs
    cmp r8, 0
    je lab47223
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab47221
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab47222

lab47221:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab47222:

lab47223:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

tail_:
    ; substitute (a0 !-> a0)(x !-> x);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch x \{ ... \};
    lea rcx, [rel List_i64_47224]
    add rcx, rdi
    jmp rcx

List_i64_47224:
    jmp near List_i64_47224_Nil
    jmp near List_i64_47224_Cons

List_i64_47224_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_47224_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47226
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab47225
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47225:
    mov rdi, [rsi + 40]
    jmp lab47227

lab47226:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab47227:
    ; substitute (a0 !-> a0)(xs !-> xs);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    ; switch xs \{ ... \};
    lea rcx, [rel List_i64_47228]
    add rcx, rdi
    jmp rcx

List_i64_47228:
    jmp near List_i64_47228_Nil
    jmp near List_i64_47228_Cons

List_i64_47228_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_47228_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47230
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab47229
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47229:
    mov rdi, [rsi + 40]
    jmp lab47231

lab47230:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab47231:
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
    lea rcx, [rel List_i64_47232]
    add rcx, r9
    jmp rcx

List_i64_47232:
    jmp near List_i64_47232_Nil
    jmp near List_i64_47232_Cons

List_i64_47232_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_i64_47232_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47234
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab47233
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47233:
    mov r9, [r8 + 40]
    jmp lab47235

lab47234:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]

lab47235:
    ; substitute (a0 !-> a0)(acc !-> acc)(xs !-> xs);
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

list_n_loop_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab47236
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- n - x0;
    mov r13, rdx
    sub r13, r11
    ; substitute (x1 !-> x1)(a0 !-> a0)(n !-> n)(a !-> a);
    ; #move variables
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov r10, rsi
    mov rsi, r8
    mov rdx, r13
    ; let x2: List[i64] = Cons(n, a);
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
    je lab47248
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47249

lab47248:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47246
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47239
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47237
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47238

lab47237:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47238:

lab47239:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47242
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47240
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47241

lab47240:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47241:

lab47242:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47245
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47243
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47244

lab47243:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47244:

lab47245:
    jmp lab47247

lab47246:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47247:

lab47249:
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
    ; jump list_n_loop_
    jmp list_n_loop_

lab47236:
    ; substitute (a0 !-> a0)(a !-> a);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; switch a \{ ... \};
    lea rcx, [rel List_i64_47250]
    add rcx, rdi
    jmp rcx

List_i64_47250:
    jmp near List_i64_47250_Nil
    jmp near List_i64_47250_Cons

List_i64_47250_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_47250_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47252
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab47251
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47251:
    mov rdi, [rsi + 40]
    jmp lab47253

lab47252:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab47253:
    ; substitute (x3 !-> x3)(xs0 !-> xs0)(a0 !-> a0);
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

list_n_:
    ; let x0: List[i64] = Nil();
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
    ; jump list_n_loop_
    jmp list_n_loop_

shorterp_:
    ; substitute (y0 !-> y)(y !-> y)(a0 !-> a0)(x !-> x);
    ; #share y
    cmp rsi, 0
    je lab47254
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47254:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rax, rsi
    mov rdx, rdi
    ; new a3: Bool = (y, a0, x)\{ ... \};
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
    je lab47266
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47267

lab47266:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47264
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47257
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47255
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47256

lab47255:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47256:

lab47257:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47260
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47258
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47259

lab47258:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47259:

lab47260:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47263
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47261
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47262

lab47261:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47262:

lab47263:
    jmp lab47265

lab47264:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47265:

lab47267:
    ; #load tag
    lea rdi, [rel Bool_47268]
    ; jump null_
    jmp null_

Bool_47268:
    jmp near Bool_47268_True
    jmp near Bool_47268_False

Bool_47268_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47272
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47269
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47269:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47270
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47270:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47271
    ; ####increment refcount
    add qword [rax + 0], 1

lab47271:
    jmp lab47273

lab47272:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47273:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a0 !-> a0)(x !-> x)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump lift_shorterp_0_
    jmp lift_shorterp_0_

Bool_47268_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47277
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47274
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47274:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47275
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47275:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47276
    ; ####increment refcount
    add qword [rax + 0], 1

lab47276:
    jmp lab47278

lab47277:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47278:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(x !-> x)(x0 !-> x0)(y !-> y);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump lift_shorterp_0_
    jmp lift_shorterp_0_

lift_shorterp_0_:
    ; substitute (x0 !-> x0)(x !-> x)(a0 !-> a0)(y !-> y);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a4: Bool = (x, a0, y)\{ ... \};
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
    je lab47290
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47291

lab47290:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47288
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47281
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47279
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47280

lab47279:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47280:

lab47281:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47284
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47282
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47283

lab47282:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47283:

lab47284:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47287
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47285
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47286

lab47285:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47286:

lab47287:
    jmp lab47289

lab47288:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47289:

lab47291:
    ; #load tag
    lea rdi, [rel Bool_47292]
    ; jump not_
    jmp not_

Bool_47292:
    jmp near Bool_47292_True
    jmp near Bool_47292_False

Bool_47292_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47296
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47293
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47293:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47294
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47294:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47295
    ; ####increment refcount
    add qword [rax + 0], 1

lab47295:
    jmp lab47297

lab47296:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47297:
    ; substitute (x5 !-> x)(a0 !-> a0)(y !-> y)(x !-> x);
    ; #share x
    cmp rax, 0
    je lab47298
    ; ####increment refcount
    add qword [rax + 0], 1

lab47298:
    ; #move variables
    mov r10, rax
    mov r11, rdx
    ; new a5: Bool = (a0, y, x)\{ ... \};
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
    je lab47310
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47311

lab47310:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47308
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47301
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47299
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47300

lab47299:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47300:

lab47301:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47304
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47302
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47303

lab47302:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47303:

lab47304:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47307
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47305
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47306

lab47305:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47306:

lab47307:
    jmp lab47309

lab47308:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47309:

lab47311:
    ; #load tag
    lea rdi, [rel Bool_47312]
    ; jump null_
    jmp null_

Bool_47312:
    jmp near Bool_47312_True
    jmp near Bool_47312_False

Bool_47312_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47316
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47313
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47313:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47314
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47314:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47315
    ; ####increment refcount
    add qword [rax + 0], 1

lab47315:
    jmp lab47317

lab47316:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47317:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp r8, 0
    je lab47320
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab47318
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab47319

lab47318:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab47319:

lab47320:
    ; #erase y
    cmp rsi, 0
    je lab47323
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab47321
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab47322

lab47321:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab47322:

lab47323:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_47312_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47327
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47324
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47324:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47325
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47325:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47326
    ; ####increment refcount
    add qword [rax + 0], 1

lab47326:
    jmp lab47328

lab47327:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47328:
    ; substitute (x !-> x)(y !-> y)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a1: List[i64] = (y, a0)\{ ... \};
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
    je lab47340
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47341

lab47340:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47338
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47331
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47329
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47330

lab47329:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47330:

lab47331:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47334
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47332
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47333

lab47332:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47333:

lab47334:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47337
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47335
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47336

lab47335:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47336:

lab47337:
    jmp lab47339

lab47338:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47339:

lab47341:
    ; #load tag
    lea rdi, [rel List_i64_47342]
    ; jump tail_
    jmp tail_

List_i64_47342:
    jmp near List_i64_47342_Nil
    jmp near List_i64_47342_Cons

List_i64_47342_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47345
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab47343
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47343:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47344
    ; ####increment refcount
    add qword [rax + 0], 1

lab47344:
    jmp lab47346

lab47345:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab47346:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(x1 !-> x1)(y !-> y);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_shorterp_1_
    jmp lift_shorterp_1_

List_i64_47342_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47349
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab47347
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47347:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab47348
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47348:
    jmp lab47350

lab47349:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab47350:
    ; substitute (a0 !-> a0)(y !-> y)(x4 !-> x4)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x1: List[i64] = Cons(x4, xs1);
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
    je lab47362
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47363

lab47362:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47360
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47353
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47351
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47352

lab47351:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47352:

lab47353:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47356
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47354
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47355

lab47354:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47355:

lab47356:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47359
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47357
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47358

lab47357:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47358:

lab47359:
    jmp lab47361

lab47360:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47361:

lab47363:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(x1 !-> x1)(y !-> y);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump lift_shorterp_1_
    jmp lift_shorterp_1_

Bool_47292_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47367
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47364
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47364:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47365
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47365:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47366
    ; ####increment refcount
    add qword [rax + 0], 1

lab47366:
    jmp lab47368

lab47367:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47368:
    ; substitute (a0 !-> a0);
    ; #erase x
    cmp rax, 0
    je lab47371
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab47369
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab47370

lab47369:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab47370:

lab47371:
    ; #erase y
    cmp r8, 0
    je lab47374
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab47372
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab47373

lab47372:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab47373:

lab47374:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; invoke a0 False
    add rdx, 5
    jmp rdx

lift_shorterp_1_:
    ; substitute (y !-> y)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a2: List[i64] = (x1, a0)\{ ... \};
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
    je lab47386
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47387

lab47386:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47384
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47377
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47375
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47376

lab47375:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47376:

lab47377:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47380
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47378
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47379

lab47378:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47379:

lab47380:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47383
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47381
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47382

lab47381:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47382:

lab47383:
    jmp lab47385

lab47384:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47385:

lab47387:
    ; #load tag
    lea rdi, [rel List_i64_47388]
    ; jump tail_
    jmp tail_

List_i64_47388:
    jmp near List_i64_47388_Nil
    jmp near List_i64_47388_Cons

List_i64_47388_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47391
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab47389
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47389:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47390
    ; ####increment refcount
    add qword [rax + 0], 1

lab47390:
    jmp lab47392

lab47391:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab47392:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump shorterp_
    jmp shorterp_

List_i64_47388_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47395
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab47393
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47393:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab47394
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47394:
    jmp lab47396

lab47395:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab47396:
    ; substitute (a0 !-> a0)(x1 !-> x1)(x3 !-> x3)(xs0 !-> xs0);
    ; #move variables
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, rsi
    mov rsi, r8
    ; let x2: List[i64] = Cons(x3, xs0);
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
    je lab47408
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47409

lab47408:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47406
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47399
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47397
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47398

lab47397:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47398:

lab47399:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47402
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47400
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47401

lab47400:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47401:

lab47402:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47405
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47403
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47404

lab47403:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47404:

lab47405:
    jmp lab47407

lab47406:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47407:

lab47409:
    ; #load tag
    mov r9, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump shorterp_
    jmp shorterp_

mas_:
    ; substitute (x14 !-> x)(y0 !-> y)(z !-> z)(a0 !-> a0)(x !-> x)(y !-> y);
    ; #share x
    cmp rax, 0
    je lab47410
    ; ####increment refcount
    add qword [rax + 0], 1

lab47410:
    ; #share y
    cmp rsi, 0
    je lab47411
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47411:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    ; new a7: Bool = (z, a0, x, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
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
    je lab47423
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47424

lab47423:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47421
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47414
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47412
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47413

lab47412:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47413:

lab47414:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47417
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47415
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47416

lab47415:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47416:

lab47417:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47420
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47418
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47419

lab47418:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47419:

lab47420:
    jmp lab47422

lab47421:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47422:

lab47424:
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
    je lab47436
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47437

lab47436:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47434
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47427
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47425
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47426

lab47425:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47426:

lab47427:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47430
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47428
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47429

lab47428:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47429:

lab47430:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47433
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47431
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47432

lab47431:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47432:

lab47433:
    jmp lab47435

lab47434:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47435:

lab47437:
    ; #load tag
    lea r9, [rel Bool_47438]
    ; substitute (y0 !-> y0)(x14 !-> x14)(a7 !-> a7);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump shorterp_
    jmp shorterp_

Bool_47438:
    jmp near Bool_47438_True
    jmp near Bool_47438_False

Bool_47438_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47443
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47439
    ; ####increment refcount
    add qword [rax + 0], 1

lab47439:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47440
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47440:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47441
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47441:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47442
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47442:
    jmp lab47444

lab47443:
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
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab47444:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(x !-> x)(x0 !-> x0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump lift_mas_0_
    jmp lift_mas_0_

Bool_47438_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47449
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47445
    ; ####increment refcount
    add qword [rax + 0], 1

lab47445:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47446
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47446:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47447
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47447:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47448
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47448:
    jmp lab47450

lab47449:
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
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab47450:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x !-> x)(x0 !-> x0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump lift_mas_0_
    jmp lift_mas_0_

lift_mas_0_:
    ; substitute (x0 !-> x0)(x !-> x)(a0 !-> a0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a8: Bool = (x, a0, y, z)\{ ... \};
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
    je lab47462
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab47463

lab47462:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47460
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47453
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47451
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47452

lab47451:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47452:

lab47453:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47456
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47454
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47455

lab47454:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47455:

lab47456:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47459
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47457
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47458

lab47457:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47458:

lab47459:
    jmp lab47461

lab47460:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47461:

lab47463:
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
    je lab47475
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47476

lab47475:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47473
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47466
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47464
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47465

lab47464:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47465:

lab47466:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47469
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47467
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47468

lab47467:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47468:

lab47469:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47472
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47470
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47471

lab47470:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47471:

lab47472:
    jmp lab47474

lab47473:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47474:

lab47476:
    ; #load tag
    lea rdi, [rel Bool_47477]
    ; jump not_
    jmp not_

Bool_47477:
    jmp near Bool_47477_True
    jmp near Bool_47477_False

Bool_47477_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47482
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47478
    ; ####increment refcount
    add qword [rax + 0], 1

lab47478:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47479
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47479:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47480
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47480:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47481
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47481:
    jmp lab47483

lab47482:
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
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab47483:
    ; substitute (a0 !-> a0)(z !-> z);
    ; #erase x
    cmp rax, 0
    je lab47486
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab47484
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab47485

lab47484:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab47485:

lab47486:
    ; #erase y
    cmp r8, 0
    je lab47489
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab47487
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab47488

lab47487:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab47488:

lab47489:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r10
    mov rdi, r11
    ; switch z \{ ... \};
    lea rcx, [rel List_i64_47490]
    add rcx, rdi
    jmp rcx

List_i64_47490:
    jmp near List_i64_47490_Nil
    jmp near List_i64_47490_Cons

List_i64_47490_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_i64_47490_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47492
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab47491
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47491:
    mov rdi, [rsi + 40]
    jmp lab47493

lab47492:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]

lab47493:
    ; substitute (x7 !-> x7)(xs0 !-> xs0)(a0 !-> a0);
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

Bool_47477_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47498
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47494
    ; ####increment refcount
    add qword [rax + 0], 1

lab47494:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47495
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47495:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47496
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47496:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47497
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47497:
    jmp lab47499

lab47498:
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
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab47499:
    ; substitute (x14 !-> x)(z0 !-> z)(y0 !-> y)(z !-> z)(x !-> x)(a0 !-> a0)(y !-> y);
    ; #share x
    cmp rax, 0
    je lab47500
    ; ####increment refcount
    add qword [rax + 0], 1

lab47500:
    ; #share y
    cmp r8, 0
    je lab47501
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47501:
    ; #share z
    cmp r10, 0
    je lab47502
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47502:
    ; #move variables
    mov r12, rax
    mov r13, rdx
    mov r14, rsi
    mov r15, rdi
    mov [rsp + 2032], r8
    mov [rsp + 2024], r9
    mov rsi, r10
    mov rdi, r11
    ; new a1: List[i64] = (z, x, a0, y)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
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
    je lab47514
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab47515

lab47514:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47512
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47505
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47503
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47504

lab47503:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47504:

lab47505:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47508
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47506
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47507

lab47506:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47507:

lab47508:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47511
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47509
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47510

lab47509:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47510:

lab47511:
    jmp lab47513

lab47512:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47513:

lab47515:
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
    je lab47527
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47528

lab47527:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47525
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47518
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47516
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47517

lab47516:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47517:

lab47518:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47521
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47519
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47520

lab47519:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47520:

lab47521:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47524
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47522
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47523

lab47522:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47523:

lab47524:
    jmp lab47526

lab47525:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47526:

lab47528:
    ; #load tag
    lea r11, [rel List_i64_47529]
    ; new a2: List[i64] = (z0, y0, a1)\{ ... \};
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
    je lab47541
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47542

lab47541:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47539
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47531

lab47530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47531:

lab47532:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47535
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47533
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47534

lab47533:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47534:

lab47535:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47538
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47536
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47537

lab47536:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47537:

lab47538:
    jmp lab47540

lab47539:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47540:

lab47542:
    ; #load tag
    lea rdi, [rel List_i64_47543]
    ; jump tail_
    jmp tail_

List_i64_47543:
    jmp near List_i64_47543_Nil
    jmp near List_i64_47543_Cons

List_i64_47543_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47547
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47544
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47544:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47545
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47545:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47546
    ; ####increment refcount
    add qword [rax + 0], 1

lab47546:
    jmp lab47548

lab47547:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47548:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x2 !-> x2)(y0 !-> y0)(z0 !-> z0)(a1 !-> a1);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

List_i64_47543_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47552
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47549
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47549:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47550
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47550:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47551
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47551:
    jmp lab47553

lab47552:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47553:
    ; substitute (a1 !-> a1)(y0 !-> y0)(z0 !-> z0)(x13 !-> x13)(xs6 !-> xs6);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x2: List[i64] = Cons(x13, xs6);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47565
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47566

lab47565:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47563
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47556
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47554
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47555

lab47554:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47555:

lab47556:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47559
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47557
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47558

lab47557:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47558:

lab47559:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47562
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47560
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47561

lab47560:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47561:

lab47562:
    jmp lab47564

lab47563:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47564:

lab47566:
    ; #load tag
    mov r11, 5
    ; substitute (x2 !-> x2)(y0 !-> y0)(z0 !-> z0)(a1 !-> a1);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

List_i64_47529:
    jmp near List_i64_47529_Nil
    jmp near List_i64_47529_Cons

List_i64_47529_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47571
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab47567
    ; ####increment refcount
    add qword [rax + 0], 1

lab47567:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab47568
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47568:
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab47569
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47569:
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab47570
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47570:
    jmp lab47572

lab47571:
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
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab47572:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(x !-> x)(x1 !-> x1)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r8
    mov r8, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    ; jump lift_mas_1_
    jmp lift_mas_1_

List_i64_47529_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47577
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab47573
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47573:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab47574
    ; ####increment refcount
    add qword [r14 + 0], 1

lab47574:
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    cmp r12, 0
    je lab47575
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47575:
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab47576
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47576:
    jmp lab47578

lab47577:
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
    mov r14, [r10 + 48]
    mov r13, [r10 + 40]
    mov r12, [r10 + 32]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab47578:
    ; substitute (y !-> y)(a0 !-> a0)(z !-> z)(x !-> x)(x12 !-> x12)(xs5 !-> xs5);
    ; #move variables
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    mov rsi, r12
    ; let x1: List[i64] = Cons(x12, xs5);
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
    je lab47590
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab47591

lab47590:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47588
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47581
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47579
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47580

lab47579:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47580:

lab47581:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47584
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47582
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47583

lab47582:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47583:

lab47584:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47587
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47585
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47586

lab47585:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47586:

lab47587:
    jmp lab47589

lab47588:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47589:

lab47591:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(x !-> x)(x1 !-> x1)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, rsi
    mov rsi, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; jump lift_mas_1_
    jmp lift_mas_1_

lift_mas_1_:
    ; substitute (z0 !-> z)(x12 !-> x)(y0 !-> y)(y !-> y)(z !-> z)(a0 !-> a0)(x !-> x)(x1 !-> x1);
    ; #share x
    cmp rsi, 0
    je lab47592
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47592:
    ; #share y
    cmp r10, 0
    je lab47593
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47593:
    ; #share z
    cmp r12, 0
    je lab47594
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47594:
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov [rsp + 2032], rsi
    mov [rsp + 2024], rdi
    mov [rsp + 2016], r8
    mov [rsp + 2008], r9
    mov r8, r10
    mov r9, r11
    mov rax, r12
    mov rdx, r13
    ; new a3: List[i64] = (y, z, a0, x, x1)\{ ... \};
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
    je lab47606
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab47607

lab47606:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47604
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47597
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47595
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47596

lab47595:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47596:

lab47597:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47600
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47598
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47599

lab47598:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47599:

lab47600:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47603
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47601
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47602

lab47601:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47602:

lab47603:
    jmp lab47605

lab47604:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47605:

lab47607:
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
    je lab47619
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47620

lab47619:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47617
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47610
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47608
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47609

lab47608:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47609:

lab47610:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47613
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47611
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47612

lab47611:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47612:

lab47613:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47616
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47614
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47615

lab47614:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47615:

lab47616:
    jmp lab47618

lab47617:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47618:

lab47620:
    ; #load tag
    lea r11, [rel List_i64_47621]
    ; substitute (y0 !-> y0)(x12 !-> x12)(z0 !-> z0)(a3 !-> a3);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a4: List[i64] = (x12, z0, a3)\{ ... \};
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
    je lab47633
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47634

lab47633:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47631
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47624
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47622
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47623

lab47622:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47623:

lab47624:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47627
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47625
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47626

lab47625:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47626:

lab47627:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47630
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47628
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47629

lab47628:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47629:

lab47630:
    jmp lab47632

lab47631:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47632:

lab47634:
    ; #load tag
    lea rdi, [rel List_i64_47635]
    ; jump tail_
    jmp tail_

List_i64_47635:
    jmp near List_i64_47635_Nil
    jmp near List_i64_47635_Cons

List_i64_47635_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47639
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47636
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47636:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47637
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47637:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47638
    ; ####increment refcount
    add qword [rax + 0], 1

lab47638:
    jmp lab47640

lab47639:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47640:
    ; let x4: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x4 !-> x4)(z0 !-> z0)(x12 !-> x12)(a3 !-> a3);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

List_i64_47635_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47644
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47641
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47641:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47642
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47642:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47643
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47643:
    jmp lab47645

lab47644:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47645:
    ; substitute (a3 !-> a3)(z0 !-> z0)(x12 !-> x12)(x11 !-> x11)(xs4 !-> xs4);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x4: List[i64] = Cons(x11, xs4);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47657
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47658

lab47657:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47655
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47648
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47646
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47647

lab47646:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47647:

lab47648:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47651
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47649
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47650

lab47649:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47650:

lab47651:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47654
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47652
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47653

lab47652:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47653:

lab47654:
    jmp lab47656

lab47655:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47656:

lab47658:
    ; #load tag
    mov r11, 5
    ; substitute (x4 !-> x4)(z0 !-> z0)(x12 !-> x12)(a3 !-> a3);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

List_i64_47621:
    jmp near List_i64_47621_Nil
    jmp near List_i64_47621_Cons

List_i64_47621_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47664
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47659
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47659:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47660
    ; ####increment refcount
    add qword [rax + 0], 1

lab47660:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47661
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47661:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47662
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47662:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47663
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47663:
    jmp lab47665

lab47664:
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
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47665:
    ; let x3: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(x !-> x)(x1 !-> x1)(x3 !-> x3)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r8
    mov r8, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r13
    mov r13, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r14
    mov r14, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, r15
    mov r15, rdi
    mov rdi, rcx
    ; jump lift_mas_2_
    jmp lift_mas_2_

List_i64_47621_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47671
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47666
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47666:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47667
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47667:
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    cmp rcx, 0
    je lab47668
    ; ####increment refcount
    add qword [rcx + 0], 1

lab47668:
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab47669
    ; ####increment refcount
    add qword [r14 + 0], 1

lab47669:
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]
    cmp r12, 0
    je lab47670
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47670:
    jmp lab47672

lab47671:
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
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov rcx, [r12 + 48]
    mov [rsp + 2032], rcx
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]
    mov r12, [r12 + 16]

lab47672:
    ; substitute (x1 !-> x1)(x !-> x)(y !-> y)(z !-> z)(a0 !-> a0)(x10 !-> x10)(xs3 !-> xs3);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, [rsp + 2032]
    mov [rsp + 2032], rsi
    mov rsi, r14
    ; let x3: List[i64] = Cons(x10, xs3);
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
    je lab47684
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab47685

lab47684:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47682
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47675
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47673
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47674

lab47673:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47674:

lab47675:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47678
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47676
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47677

lab47676:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47677:

lab47678:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47681
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47679
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47680

lab47679:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47680:

lab47681:
    jmp lab47683

lab47682:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47683:

lab47685:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(x !-> x)(x1 !-> x1)(x3 !-> x3)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r12
    mov r12, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r14
    mov r14, r10
    mov r10, rcx
    mov rcx, r15
    mov r15, r11
    mov r11, rcx
    ; jump lift_mas_2_
    jmp lift_mas_2_

lift_mas_2_:
    ; substitute (z !-> z)(x !-> x)(y !-> y)(x3 !-> x3)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r14
    mov r14, rax
    mov rax, rcx
    mov rcx, r15
    mov r15, rdx
    mov rdx, rcx
    mov rcx, r12
    mov r12, r8
    mov r8, rcx
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    ; new a5: List[i64] = (x3, x1, a0)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r15
    mov [rbx + 48], r14
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
    je lab47697
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47698

lab47697:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47695
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47688
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47686
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47687

lab47686:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47687:

lab47688:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47691
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47689
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47690

lab47689:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47690:

lab47691:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47694
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47692
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47693

lab47692:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47693:

lab47694:
    jmp lab47696

lab47695:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47696:

lab47698:
    ; #load tag
    lea r11, [rel List_i64_47699]
    ; new a6: List[i64] = (x, y, a5)\{ ... \};
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
    je lab47711
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47712

lab47711:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47709
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47702
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47700
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47701

lab47700:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47701:

lab47702:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47705
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47703
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47704

lab47703:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47704:

lab47705:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47708
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47706
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47707

lab47706:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47707:

lab47708:
    jmp lab47710

lab47709:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47710:

lab47712:
    ; #load tag
    lea rdi, [rel List_i64_47713]
    ; jump tail_
    jmp tail_

List_i64_47713:
    jmp near List_i64_47713_Nil
    jmp near List_i64_47713_Cons

List_i64_47713_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47717
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47714
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47714:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47715
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47715:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47716
    ; ####increment refcount
    add qword [rax + 0], 1

lab47716:
    jmp lab47718

lab47717:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47718:
    ; let x6: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x6 !-> x6)(x !-> x)(y !-> y)(a5 !-> a5);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

List_i64_47713_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47722
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47719
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47719:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47720
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47720:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47721
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47721:
    jmp lab47723

lab47722:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47723:
    ; substitute (a5 !-> a5)(y !-> y)(x !-> x)(x9 !-> x9)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x6: List[i64] = Cons(x9, xs2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47735
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47736

lab47735:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47733
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47726
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47724
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47725

lab47724:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47725:

lab47726:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47729
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47727
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47728

lab47727:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47728:

lab47729:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47732
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47730
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47731

lab47730:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47731:

lab47732:
    jmp lab47734

lab47733:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47734:

lab47736:
    ; #load tag
    mov r11, 5
    ; substitute (x6 !-> x6)(x !-> x)(y !-> y)(a5 !-> a5);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump mas_
    jmp mas_

List_i64_47699:
    jmp near List_i64_47699_Nil
    jmp near List_i64_47699_Cons

List_i64_47699_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47740
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47737
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47737:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47738
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47738:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47739
    ; ####increment refcount
    add qword [rax + 0], 1

lab47739:
    jmp lab47741

lab47740:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47741:
    ; let x5: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x1 !-> x1)(x3 !-> x3)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump mas_
    jmp mas_

List_i64_47699_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47745
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47742
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47742:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47743
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47743:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47744
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47744:
    jmp lab47746

lab47745:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47746:
    ; substitute (a0 !-> a0)(x1 !-> x1)(x3 !-> x3)(x8 !-> x8)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x5: List[i64] = Cons(x8, xs1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47758
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47759

lab47758:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47756
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47749
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47747
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47748

lab47747:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47748:

lab47749:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47752
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47750
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47751

lab47750:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47751:

lab47752:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47755
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47753
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47754

lab47753:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47754:

lab47755:
    jmp lab47757

lab47756:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47757:

lab47759:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(x3 !-> x3)(x5 !-> x5)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

main_loop_:
    ; substitute (z0 !-> z)(x12 !-> x)(y0 !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x)(y !-> y);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a2: List[i64] = (z, a0, iters, x, y)\{ ... \};
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
    je lab47771
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab47772

lab47771:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47769
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47762
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47760
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47761

lab47760:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47761:

lab47762:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47765
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47763
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47764

lab47763:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47764:

lab47765:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47768
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47766
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47767

lab47766:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47767:

lab47768:
    jmp lab47770

lab47769:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47770:

lab47772:
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
    je lab47784
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47785

lab47784:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47782
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47775
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47773
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47774

lab47773:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47774:

lab47775:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47778
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47776
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47777

lab47776:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47777:

lab47778:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47781
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47779
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47780

lab47779:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47780:

lab47781:
    jmp lab47783

lab47782:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47783:

lab47785:
    ; #load tag
    lea r11, [rel List_i64_47786]
    ; substitute (x12 !-> x12)(z0 !-> z0)(y0 !-> y0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; new a3: List[i64] = (z0, y0, a2)\{ ... \};
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
    je lab47798
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47799

lab47798:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47796
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47789
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47787
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47788

lab47787:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47788:

lab47789:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47792
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47790
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47791

lab47790:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47791:

lab47792:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47795
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47793
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47794

lab47793:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47794:

lab47795:
    jmp lab47797

lab47796:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47797:

lab47799:
    ; #load tag
    lea rdi, [rel List_i64_47800]
    ; jump list_n_
    jmp list_n_

List_i64_47800:
    jmp near List_i64_47800_Nil
    jmp near List_i64_47800_Cons

List_i64_47800_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47802
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47801
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47801:
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    jmp lab47803

lab47802:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]

lab47803:
    ; let x1: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a2 !-> a2)(x1 !-> x1)(y0 !-> y0)(z0 !-> z0);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r8
    mov rsi, r10
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_i64_47800_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47805
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47804
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47804:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab47806

lab47805:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab47806:
    ; substitute (a2 !-> a2)(y0 !-> y0)(z0 !-> z0)(x11 !-> x11)(xs3 !-> xs3);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    ; let x1: List[i64] = Cons(x11, xs3);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47818
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47819

lab47818:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47816
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47809
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47807
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47808

lab47807:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47808:

lab47809:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47812
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47810
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47811

lab47810:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47811:

lab47812:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47815
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47813
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47814

lab47813:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47814:

lab47815:
    jmp lab47817

lab47816:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47817:

lab47819:
    ; #load tag
    mov r11, 5
    ; substitute (a2 !-> a2)(x1 !-> x1)(y0 !-> y0)(z0 !-> z0);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r10
    ; jump lift_main_loop_1_
    jmp lift_main_loop_1_

List_i64_47786:
    jmp near List_i64_47786_Nil
    jmp near List_i64_47786_Cons

List_i64_47786_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47821
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47820
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47820:
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab47822

lab47821:
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
    mov r9, [r8 + 24]

lab47822:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(iters !-> iters)(x !-> x)(x0 !-> x0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, r15
    mov r15, rdx
    mov rdx, rcx
    mov rax, rsi
    mov r10, r14
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

List_i64_47786_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47824
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r12, [r8 + 48]
    ; ###load values
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47823
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47823:
    mov r9, [r8 + 24]
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]
    jmp lab47825

lab47824:
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
    ; ###load values
    mov rcx, [r12 + 56]
    mov [rsp + 2024], rcx
    mov r15, [r12 + 40]
    mov r13, [r12 + 24]

lab47825:
    ; substitute (y !-> y)(x !-> x)(z !-> z)(a0 !-> a0)(iters !-> iters)(x8 !-> x8)(xs0 !-> xs0);
    ; #move variables
    mov rcx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov rdi, r15
    mov r15, rdx
    mov rdx, rcx
    mov [rsp + 2032], rsi
    ; let x0: List[i64] = Cons(x8, xs0);
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
    je lab47837
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab47838

lab47837:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47835
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47828
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47826
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47827

lab47826:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47827:

lab47828:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47831
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47829
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47830

lab47829:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47830:

lab47831:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47834
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47832
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47833

lab47832:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47833:

lab47834:
    jmp lab47836

lab47835:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47836:

lab47838:
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(iters !-> iters)(x !-> x)(x0 !-> x0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r11
    mov r11, r15
    mov r15, r9
    mov r9, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r10
    mov r10, r14
    ; jump lift_main_loop_0_
    jmp lift_main_loop_0_

lift_main_loop_1_:
    ; substitute (y !-> y)(x1 !-> x1)(a2 !-> a2)(z !-> z);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; new a4: List[i64] = (x1, a2, z)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
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
    je lab47850
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47851

lab47850:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47848
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47841
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47839
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47840

lab47839:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47840:

lab47841:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47844
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47842
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47843

lab47842:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47843:

lab47844:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47847
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47845
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47846

lab47845:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47846:

lab47847:
    jmp lab47849

lab47848:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47849:

lab47851:
    ; #load tag
    lea rdi, [rel List_i64_47852]
    ; jump list_n_
    jmp list_n_

List_i64_47852:
    jmp near List_i64_47852_Nil
    jmp near List_i64_47852_Cons

List_i64_47852_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47855
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47853
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47853:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47854
    ; ####increment refcount
    add qword [rax + 0], 1

lab47854:
    jmp lab47856

lab47855:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47856:
    ; let x2: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (a2 !-> a2)(x1 !-> x1)(x2 !-> x2)(z !-> z);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    mov r8, r10
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

List_i64_47852_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47859
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47857
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47857:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47858
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47858:
    jmp lab47860

lab47859:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47860:
    ; substitute (z !-> z)(a2 !-> a2)(x1 !-> x1)(x10 !-> x10)(xs2 !-> xs2);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov r12, rsi
    mov rsi, r10
    ; let x2: List[i64] = Cons(x10, xs2);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47872
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47873

lab47872:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47870
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47863
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47861
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47862

lab47861:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47862:

lab47863:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47866
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47864
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47865

lab47864:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47865:

lab47866:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47869
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47867
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47868

lab47867:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47868:

lab47869:
    jmp lab47871

lab47870:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47871:

lab47873:
    ; #load tag
    mov r11, 5
    ; substitute (a2 !-> a2)(x1 !-> x1)(x2 !-> x2)(z !-> z);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rsi, r8
    mov r8, r10
    ; jump lift_main_loop_2_
    jmp lift_main_loop_2_

lift_main_loop_2_:
    ; substitute (z !-> z)(x1 !-> x1)(x2 !-> x2)(a2 !-> a2);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a5: List[i64] = (x1, x2, a2)\{ ... \};
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
    je lab47885
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47886

lab47885:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47883
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47876
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47874
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47875

lab47874:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47875:

lab47876:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47879
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47877
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47878

lab47877:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47878:

lab47879:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47882
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47880
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47881

lab47880:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47881:

lab47882:
    jmp lab47884

lab47883:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47884:

lab47886:
    ; #load tag
    lea rdi, [rel List_i64_47887]
    ; jump list_n_
    jmp list_n_

List_i64_47887:
    jmp near List_i64_47887_Nil
    jmp near List_i64_47887_Cons

List_i64_47887_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab47891
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab47888
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47888:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab47889
    ; ####increment refcount
    add qword [rsi + 0], 1

lab47889:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab47890
    ; ####increment refcount
    add qword [rax + 0], 1

lab47890:
    jmp lab47892

lab47891:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]

lab47892:
    ; let x3: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x1 !-> x1)(x2 !-> x2)(x3 !-> x3)(a2 !-> a2);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; jump mas_
    jmp mas_

List_i64_47887_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47896
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab47893
    ; ####increment refcount
    add qword [r12 + 0], 1

lab47893:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab47894
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47894:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab47895
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47895:
    jmp lab47897

lab47896:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]

lab47897:
    ; substitute (a2 !-> a2)(x2 !-> x2)(x1 !-> x1)(x9 !-> x9)(xs1 !-> xs1);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, r12
    mov r12, rsi
    mov rsi, r10
    ; let x3: List[i64] = Cons(x9, xs1);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r13
    mov [rbx + 48], r12
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
    je lab47909
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47910

lab47909:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47907
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47900
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47898
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47899

lab47898:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47899:

lab47900:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47903
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47901
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47902

lab47901:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47902:

lab47903:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47906
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47904
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47905

lab47904:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47905:

lab47906:
    jmp lab47908

lab47907:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47908:

lab47910:
    ; #load tag
    mov r11, 5
    ; substitute (x1 !-> x1)(x2 !-> x2)(x3 !-> x3)(a2 !-> a2);
    ; #move variables
    mov rcx, r8
    mov r8, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump mas_
    jmp mas_

lift_main_loop_0_:
    ; substitute (x0 !-> x0)(iters !-> iters)(x !-> x)(a0 !-> a0)(y !-> y)(z !-> z);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a6: _Cont = (iters, x, a0, y, z)\{ ... \};
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
    je lab47922
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab47923

lab47922:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47920
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47912

lab47911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47912:

lab47913:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47915

lab47914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47915:

lab47916:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47919
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47917
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47918

lab47917:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47918:

lab47919:
    jmp lab47921

lab47920:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47921:

lab47923:
    ; ##store link to previous block
    mov [rbx + 48], r10
    ; ##store values
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
    je lab47935
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab47936

lab47935:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47933
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47926
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47924
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47925

lab47924:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47925:

lab47926:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47928

lab47927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47928:

lab47929:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47932
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47930
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47931

lab47930:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47931:

lab47932:
    jmp lab47934

lab47933:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47934:

lab47936:
    ; #load tag
    lea rdi, [rel _Cont_47937]
    ; jump len_
    jmp len_

_Cont_47937:

_Cont_47937_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab47939
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab47938
    ; ####increment refcount
    add qword [r10 + 0], 1

lab47938:
    jmp lab47940

lab47939:
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
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab47940:
    ; lit x4 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x4 \{ ... \}
    cmp rdi, [rsp +2024]
    je lab47941
    ; substitute (z !-> z)(iters !-> iters)(x !-> x)(a0 !-> a0)(y !-> y);
    ; #move variables
    mov rdx, r15
    ; lit x5 <- 1;
    mov r15, 1
    ; x6 <- iters - x5;
    mov rcx, rdi
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x6 !-> x6)(x !-> x)(y !-> y)(z !-> z)(a0 !-> a0);
    ; #move variables
    mov rdi, r9
    mov r9, r13
    mov r13, r11
    mov r11, rdx
    mov r12, r10
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab47941:
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
    ; lit x7 <- 0;
    mov rdi, 0
    ; substitute (x7 !-> x7)(a0 !-> a0);
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