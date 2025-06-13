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
    lea r13, [rel _Cont_47942]
    ; jump main_loop_
    jmp main_loop_

_Cont_47942:

_Cont_47942_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

cps_tak_:
    ; if x <= y \{ ... \}
    cmp rdx, rdi
    jle lab47943
    ; lit x0 <- 1;
    mov r15, 1
    ; x1 <- x - x0;
    mov rcx, rdx
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x1 !-> x1)(y0 !-> y)(z1 !-> z)(a0 !-> a0)(k !-> k)(z !-> z)(y !-> y)(x !-> x);
    ; #move variables
    mov [rsp + 2008], rdx
    mov rdx, [rsp + 2024]
    mov [rsp + 2024], rdi
    mov r15, r9
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; new x2: Fun = (k, z, y, x)\{ ... \};
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
    je lab47955
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab47956

lab47955:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47953
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47946
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47944
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47945

lab47944:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47945:

lab47946:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47949
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47947
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47948

lab47947:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47948:

lab47949:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47952
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47950
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47951

lab47950:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47951:

lab47952:
    jmp lab47954

lab47953:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47954:

lab47956:
    ; ##store link to previous block
    mov [rbx + 48], r14
    ; ##store values
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
    je lab47968
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab47969

lab47968:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47966
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47959
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47957
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47958

lab47957:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47958:

lab47959:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47961

lab47960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47961:

lab47962:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47965
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47963
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47964

lab47963:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47964:

lab47965:
    jmp lab47967

lab47966:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47967:

lab47969:
    ; #load tag
    lea r13, [rel Fun_47970]
    ; substitute (x1 !-> x1)(y0 !-> y0)(z1 !-> z1)(x2 !-> x2)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump cps_tak_
    jmp cps_tak_

Fun_47970:

Fun_47970_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab47972
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab47971
    ; ####increment refcount
    add qword [r8 + 0], 1

lab47971:
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab47973

lab47972:
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
    mov r11, [r10 + 24]

lab47973:
    ; lit x3 <- 1;
    mov qword [rsp + 2024], 1
    ; x4 <- y - x3;
    mov rcx, r13
    sub rcx, [rsp + 2024]
    mov [rsp + 2008], rcx
    ; substitute (x4 !-> x4)(a2 !-> a2)(x9 !-> x)(z0 !-> z)(y !-> y)(x !-> x)(z !-> z)(k !-> k)(v1 !-> v1);
    ; #move variables
    mov [rsp + 1992], rdx
    mov [rsp + 2016], r8
    mov rdx, [rsp + 2008]
    mov [rsp + 2008], r9
    mov [rsp + 2024], r11
    mov r9, r15
    ; new x5: Fun = (y, x, z, k, v1)\{ ... \};
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
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov rcx, rbx
    mov [rsp + 2032], rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab47985
    ; ####initialize refcount of just acquired block
    mov qword [rcx + 0], 0
    jmp lab47986

lab47985:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47983
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47976
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47974
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47975

lab47974:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47975:

lab47976:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47979
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47977
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47978

lab47977:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47978:

lab47979:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47982
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47980
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47981

lab47980:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47981:

lab47982:
    jmp lab47984

lab47983:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47984:

lab47986:
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
    je lab47998
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab47999

lab47998:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab47996
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab47989
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47987
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47988

lab47987:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47988:

lab47989:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab47992
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47990
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47991

lab47990:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47991:

lab47992:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab47995
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab47993
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab47994

lab47993:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab47994:

lab47995:
    jmp lab47997

lab47996:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab47997:

lab47999:
    ; #load tag
    lea r13, [rel Fun_48000]
    ; substitute (x4 !-> x4)(z0 !-> z0)(x9 !-> x9)(x5 !-> x5)(a2 !-> a2);
    ; #move variables
    mov r10, r12
    mov r12, rsi
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    ; jump cps_tak_
    jmp cps_tak_

Fun_48000:

Fun_48000_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48002
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
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    cmp r14, 0
    je lab48001
    ; ####increment refcount
    add qword [r14 + 0], 1

lab48001:
    mov r13, [r12 + 24]
    jmp lab48003

lab48002:
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
    mov r15, [r12 + 40]
    mov r14, [r12 + 32]
    mov r13, [r12 + 24]

lab48003:
    ; lit x6 <- 1;
    mov qword [rsp + 2008], 1
    ; x7 <- z - x6;
    mov rcx, r13
    sub rcx, [rsp + 2008]
    mov [rsp + 1992], rcx
    ; substitute (x7 !-> x7)(a4 !-> a4)(y !-> y)(x !-> x)(v2 !-> v2)(k !-> k)(v1 !-> v1);
    ; #move variables
    mov r13, rdx
    mov rdx, [rsp + 1992]
    ; new x8: Fun = (v2, k, v1)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov qword [rbx + 48], 0
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
    je lab48015
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab48016

lab48015:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48013
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48006
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48004
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48005

lab48004:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48005:

lab48006:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48009
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48007
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48008

lab48007:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48008:

lab48009:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48012
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48010
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48011

lab48010:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48011:

lab48012:
    jmp lab48014

lab48013:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48014:

lab48016:
    ; #load tag
    lea r13, [rel Fun_48017]
    ; substitute (x7 !-> x7)(x !-> x)(y !-> y)(x8 !-> x8)(a4 !-> a4);
    ; #move variables
    mov r10, r12
    mov r12, rsi
    mov rcx, r11
    mov r11, r13
    mov r13, rdi
    mov rdi, rcx
    ; jump cps_tak_
    jmp cps_tak_

Fun_48017:

Fun_48017_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab48019
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab48018
    ; ####increment refcount
    add qword [r10 + 0], 1

lab48018:
    mov r9, [r8 + 24]
    jmp lab48020

lab48019:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    mov r9, [r8 + 24]

lab48020:
    ; substitute (v1 !-> v1)(v2 !-> v2)(v3 !-> v3)(k !-> k)(a6 !-> a6);
    ; #move variables
    mov rcx, r13
    mov r13, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov r12, rsi
    ; jump cps_tak_
    jmp cps_tak_

lab47943:
    ; substitute (z !-> z)(a0 !-> a0)(k !-> k);
    ; #move variables
    mov rdx, r9
    mov r8, r10
    mov r9, r11
    mov rsi, r12
    mov rdi, r13
    ; invoke k Apply
    jmp r9

tak_:
    ; new x0: Fun = ()\{ ... \};
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    lea r13, [rel Fun_48021]
    ; substitute (x !-> x)(y !-> y)(z !-> z)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, r12
    mov r12, r10
    mov r10, rcx
    mov rcx, r13
    mov r13, r11
    mov r11, rcx
    ; jump cps_tak_
    jmp cps_tak_

Fun_48021:

Fun_48021_Apply:
    ; invoke a1 Ret
    jmp rdi

main_loop_:
    ; substitute (z0 !-> z)(x4 !-> x)(y0 !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x)(y !-> y);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2008], r9
    mov rdx, r11
    ; new a2: _Cont = (z, a0, iters, x, y)\{ ... \};
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
    je lab48033
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab48034

lab48033:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48031
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48024
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48022
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48023

lab48022:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48023:

lab48024:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48027
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48025
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48026

lab48025:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48026:

lab48027:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48030
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48028
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48029

lab48028:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48029:

lab48030:
    jmp lab48032

lab48031:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48032:

lab48034:
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
    je lab48046
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab48047

lab48046:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab48044
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab48037
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48035
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48036

lab48035:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48036:

lab48037:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab48040
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48038
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48039

lab48038:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48039:

lab48040:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab48043
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab48041
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab48042

lab48041:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab48042:

lab48043:
    jmp lab48045

lab48044:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab48045:

lab48047:
    ; #load tag
    lea r11, [rel _Cont_48048]
    ; substitute (x4 !-> x4)(y0 !-> y0)(z0 !-> z0)(a2 !-> a2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump tak_
    jmp tak_

_Cont_48048:

_Cont_48048_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab48050
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab48049
    ; ####increment refcount
    add qword [r8 + 0], 1

lab48049:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    jmp lab48051

lab48050:
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

lab48051:
    ; lit x0 <- 1;
    mov qword [rsp + 2024], 1
    ; if iters == x0 \{ ... \}
    cmp r11, [rsp +2024]
    je lab48052
    ; substitute (y !-> y)(z !-> z)(a0 !-> a0)(iters !-> iters)(x !-> x);
    ; #move variables
    mov rdx, r15
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- iters - x1;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (x2 !-> x2)(x !-> x)(y !-> y)(z !-> z)(a0 !-> a0);
    ; #move variables
    mov r11, rdi
    mov rdi, r13
    mov r13, r9
    mov r9, rdx
    mov r12, r8
    mov rdx, [rsp + 2024]
    ; jump main_loop_
    jmp main_loop_

lab48052:
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