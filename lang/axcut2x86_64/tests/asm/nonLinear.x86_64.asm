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
    ; lit f_1 <- 3;
    mov rdx, 3
    ; lit f_2 <- 3;
    mov rdi, 3
    ; lit f_3 <- 3;
    mov r9, 3
    ; lit f_4 <- 3;
    mov r11, 3
    ; lit f_5 <- 3;
    mov r13, 3
    ; lit f_6 <- 3;
    mov r15, 3
    ; lit f_7 <- 3;
    mov qword [rsp + 2024], 3
    ; lit x_8 <- 3;
    mov qword [rsp + 2008], 3
    ; let b_9: Box = B(x_8);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab4
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab5

lab4:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab2
    ; ###(2) jump to slow path
    mov [rsp + 2040], r15
    lea r15, [rel return_from_slow_path1]
    jmp acquire_block_slow_path

return_from_slow_path1:
    mov r15, [rsp + 2040]
    jmp lab3

lab2:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab3:

lab5:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; let bb_10: BoxBox = BB(b_9);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2008]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2016]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2016], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab9
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab10

lab9:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab7
    ; ###(2) jump to slow path
    mov [rsp + 2040], r15
    lea r15, [rel return_from_slow_path6]
    jmp acquire_block_slow_path

return_from_slow_path6:
    mov r15, [rsp + 2040]
    jmp lab8

lab7:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab8:

lab10:
    ; #load tag
    mov qword [rsp + 2008], 0
    ; substitute (f_1 := f_1)(f_2 := f_2)(f_3 := f_3)(f_5 := f_5)(f_6 := f_6)(f_7 := f_7)(f_4 := f_4)(bb_13 := bb_10)(bb_12 := bb_10)(bb_11 := bb_10);
    ; #share bb_10
    cmp qword [rsp + 2016], 0
    je lab11
    ; ####increment refcount
    mov rcx, [rsp + 2016]
    add qword [rcx + 0], 2

lab11:
    ; #move variables
    mov rcx, r13
    mov r13, r15
    mov r15, [rsp + 2024]
    mov [rsp + 2024], r11
    mov r11, rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 2000], rcx
    mov rcx, [rsp + 2016]
    mov [rsp + 1984], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    mov rcx, [rsp + 2008]
    mov [rsp + 1976], rcx
    ; switch bb_11 \{ ... \};
    ; #there is only one clause, so we can just fall through

BoxBox_12:

BoxBox_12_BB:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab14
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    cmp rcx, 0
    je lab13
    ; ####increment refcount
    add qword [rcx + 0], 1

lab13:
    ; ###restore evacuated register
    mov rax, [rsp + 2040]
    jmp lab15

lab14:
    ; ##... or release blocks onto linear free list when loading
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    mov rcx, [rax + 48]
    mov [rsp + 1984], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]

lab15:
    ; switch b_14 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_16:

Box_16_B:
    ; #load from memory
    mov rcx, [rsp + 1984]
    ; ##check refcount
    cmp qword [rcx + 0], 0
    je lab17
    ; ##either decrement refcount and share children...
    add qword [rcx + 0], -1
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]
    jmp lab18

lab17:
    ; ##... or release blocks onto linear free list when loading
    ; ###evacuate additional scratch register for memory block
    mov [rsp + 2040], rax
    mov rax, [rsp + 1984]
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rcx, [rax + 56]
    mov [rsp + 1976], rcx
    ; ###restore evacuated register
    mov rax, [rsp + 2040]

lab18:
    ; let d_16: Box = B(x_15);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab22
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab23

lab22:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab20
    ; ###(2) jump to slow path
    mov [rsp + 2040], r15
    lea r15, [rel return_from_slow_path19]
    jmp acquire_block_slow_path

return_from_slow_path19:
    mov r15, [rsp + 2040]
    jmp lab21

lab20:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab21:

lab23:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; let dd_17: BoxBox = BB(d_16);
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 1976]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 1984]
    mov [rbx + 48], rcx
    ; ##mark unused fields with null
    mov qword [rbx + 16], 0
    mov qword [rbx + 32], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 1984], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab27
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab28

lab27:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab25
    ; ###(2) jump to slow path
    mov [rsp + 2040], r15
    lea r15, [rel return_from_slow_path24]
    jmp acquire_block_slow_path

return_from_slow_path24:
    mov r15, [rsp + 2040]
    jmp lab26

lab25:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab26:

lab28:
    ; #load tag
    mov qword [rsp + 1976], 0
    ; substitute (bb_12 := bb_12);
    ; #erase bb_13
    mov rcx, [rsp + 2016]
    cmp rcx, 0
    je lab31
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab29
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab30

lab29:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab30:

lab31:
    ; #erase dd_17
    mov rcx, [rsp + 1984]
    cmp rcx, 0
    je lab34
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab32
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab33

lab32:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab33:

lab34:
    ; #move variables
    mov rax, [rsp + 2000]
    mov rdx, [rsp + 1992]
    ; lit y_18 <- 4;
    mov rdi, 4
    ; let a_19: Box = B(y_18);
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
    je lab38
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab39

lab38:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab36
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path35]
    jmp acquire_block_slow_path

return_from_slow_path35:
    jmp lab37

lab36:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab37:

lab39:
    ; #load tag
    mov rdi, 0
    ; substitute (a_19 := a_19)(bb_12 := bb_12);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch bb_12 \{ ... \};
    ; #there is only one clause, so we can just fall through

BoxBox_40:

BoxBox_40_BB:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab42
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]
    cmp rsi, 0
    je lab41
    ; ####increment refcount
    add qword [rsi + 0], 1

lab41:
    jmp lab43

lab42:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]
    mov rsi, [rsi + 48]

lab43:
    ; switch b_20 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_44:

Box_44_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab45
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab46

lab45:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab46:
    ; let a_22: Box = B(x_21);
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
    je lab50
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab51

lab50:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48
    ; ###(2) jump to slow path
    lea r15, [rel return_from_slow_path47]
    jmp acquire_block_slow_path

return_from_slow_path47:
    jmp lab49

lab48:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab49:

lab51:
    ; #load tag
    mov rdi, 0
    ; switch a_22 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_52:

Box_52_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab53
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab54

lab53:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab54:
    ; substitute (x_23 := x_23)(a_19 := a_19);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch a_19 \{ ... \};
    ; #there is only one clause, so we can just fall through

Box_55:

Box_55_B:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab56
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov rdi, [rsi + 56]
    jmp lab57

lab56:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov rdi, [rsi + 56]

lab57:
    ; res_25 <- x_24 + x_23;
    mov r9, rdi
    add r9, rdx
    ; println_i64 res_25;
    ; #save caller-save registers
    mov r12, rdx
    mov r13, rdi
    mov r14, r9
    sub rsp, 8
    ; #move argument into place
    mov rdi, r9
    call println_i64
    ; #restore caller-save registers
    mov rdx, r12
    mov rdi, r13
    mov r9, r14
    add rsp, 8
    ; lit ret_26 <- 0;
    mov r11, 0
    ; exit ret_26
    mov rax, r11
    jmp cleanup

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

acquire_block_slow_path:
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab60
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab58
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab59

lab58:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab59:

lab60:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab63
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab61
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab62

lab61:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab62:

lab63:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab66
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65

lab64:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65:

lab66:
    jmp r15