    ; asmsyntax=nasm
section .note.GNU-stack noalloc noexec nowrite progbits
section .text
global asm_main0
global _asm_main0
global asm_main1
global _asm_main1
global asm_main2
global _asm_main2
global asm_main3
global _asm_main3
global asm_main4
global _asm_main4
global asm_main5
global _asm_main5

asm_main0:

_asm_main0:

asm_main1:

_asm_main1:

asm_main2:

_asm_main2:

asm_main3:

_asm_main3:

asm_main4:

_asm_main4:

asm_main5:

_asm_main5:
    ; setup
    ; save registers
    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15
    ; move parameters into place
    ; reserve space for register spills
    sub rsp, 2048
    ; initialize heap pointer
    mov rbx, rdi
    ; initialize free pointer
    mov rbp, rbx
    add rbp, 64
    ; actual code

main:
    ; lit z <- 8;
    mov rdx, 8
    ; lit y <- 6;
    mov rdi, 6
    ; lit x <- 4;
    mov r9, 4
    ; lit w <- 2;
    mov r11, 2
    ; leta q: Quad = Q(z, y, x, w);
    ;  allocate memory
    ;   store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov qword [rbx + 16], 0
    ;   acquire free block from heap register
    mov rsi, rbx
    ;   get next free block into heap register
    ;    (1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab12
    ;     initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab13

lab12:
    ;    (2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab10
    ;     mark linear free list empty
    mov qword [rbx + 0], 0
    ;     erase children of next block
    ;      check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab3
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab1
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab2

lab1:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab2:

lab3:
    ;      check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab6
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab4
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab5

lab4:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab5:

lab6:
    ;      check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab9
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab7
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab8

lab7:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab8:

lab9:
    jmp lab11

lab10:
    ;    (3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab11:

lab13:
    ;   store link to previous block
    mov [rbx + 48], rsi
    ;   store values
    mov [rbx + 40], rdx
    mov qword [rbx + 32], 0
    ;   mark unused fields with null
    mov qword [rbx + 16], 0
    ;   acquire free block from heap register
    mov rax, rbx
    ;   get next free block into heap register
    ;    (1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab25
    ;     initialize refcount of just acquired block
    mov qword [rax + 0], 0
    jmp lab26

lab25:
    ;    (2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab23
    ;     mark linear free list empty
    mov qword [rbx + 0], 0
    ;     erase children of next block
    ;      check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab16
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab14
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab15

lab14:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab15:

lab16:
    ;      check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab19
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab17
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab18

lab17:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab18:

lab19:
    ;      check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab22
    ;       check refcount
    cmp qword [rcx + 0], 0
    je lab20
    ;       either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab21

lab20:
    ;       ... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab21:

lab22:
    jmp lab24

lab23:
    ;    (3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab24:

lab26:
    ;  load tag
    mov rdx, 0
    ; switch q \{ ... \};
    lea rcx, [rel Quad27]
    jmp rcx

Quad27:

Quad27Q:
    ;  load from memory
    ;   check refcount
    cmp qword [rax + 0], 0
    je lab28
    ;   either decrement refcount and share children...
    add qword [rax + 0], -1
    ;    load link to next block
    mov rsi, [rax + 48]
    ;    load values
    mov rdx, [rax + 40]
    ;    load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    jmp lab29

lab28:
    ;   ... or release blocks onto linear free list when loading
    ;    release block
    mov [rax + 0], rbx
    mov rbx, rax
    ;    load link to next block
    mov rsi, [rax + 48]
    ;    load values
    mov rdx, [rax + 40]
    ;    release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ;    load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]

lab29:
    ; lit z <- 7;
    mov r13, 7
    ; e <- d + z;
    mov r15, rdx
    add r15, r13
    ; return e
    mov rdx, r15
    jmp cleanup
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