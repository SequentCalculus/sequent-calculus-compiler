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
    lea rdx, [rel _Cont_216]
    ; lit x2 <- 1;
    mov rdi, 1
    ; lit x3 <- 2;
    mov r9, 2
    ; let x4: Pair[i64, i64] = Tup(x2, x3);
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r9
    mov qword [rbx + 48], 0
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
    je lab228
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab229

lab228:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab226
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab219
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab217
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab218

lab217:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab218:

lab219:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab222
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab220
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab221

lab220:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab221:

lab222:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab225
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab223
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab224

lab223:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab224:

lab225:
    jmp lab227

lab226:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab227:

lab229:
    ; #load tag
    mov rdi, 0
    ; substitute (x4 !-> x4)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump second_
    jmp second_

_Cont_216:

_Cont_216_Ret:
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

Pair_i64_i64_230:

Pair_i64_i64_230_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab231
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab232

lab231:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab232:
    ; substitute (b !-> b)(a !-> a)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; invoke a0 Tup
    jmp r9

diag_:
    ; substitute (x !-> x)(x0 !-> x)(a0 !-> a0);
    ; #move variables
    mov r9, rdi
    mov rdi, rdx
    mov r8, rsi
    ; invoke a0 Tup
    jmp r9

first_:
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

Pair_i64_i64_233:

Pair_i64_i64_233_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab234
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab235

lab234:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab235:
    ; substitute (a !-> a)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

second_:
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

Pair_i64_i64_236:

Pair_i64_i64_236_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab237
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab238

lab237:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab238:
    ; substitute (b !-> b)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rdi, rdx
    mov rdx, r9
    ; invoke a0 Ret
    jmp rdi

toList_:
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

Pair_i64_i64_239:

Pair_i64_i64_239_Tup:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab240
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    jmp lab241

lab240:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]

lab241:
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; let x1: List[i64] = Cons(b, x0);
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
    je lab253
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab254

lab253:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab251
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab244
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab242
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab243

lab242:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab243:

lab244:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab247
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab245
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab246

lab245:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab246:

lab247:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab250
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab248
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab249

lab248:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab249:

lab250:
    jmp lab252

lab251:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab252:

lab254:
    ; #load tag
    mov r9, 5
    ; substitute (a !-> a)(x1 !-> x1)(a0 !-> a0);
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