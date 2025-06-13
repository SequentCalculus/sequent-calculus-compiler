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
    lea r9, [rel _Cont_64905]
    ; jump main_loop_
    jmp main_loop_

_Cont_64905:

_Cont_64905_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

sum_loop_:
    ; if stop < i \{ ... \}
    cmp r9, rdx
    jl lab64906
    ; lit x0 <- 1;
    mov r15, 1
    ; x1 <- i + x0;
    mov rcx, rdx
    add rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (i !-> i)(f0 !-> f)(stop !-> stop)(f !-> f)(a0 !-> a0)(tot !-> tot)(x1 !-> x1);
    ; #share f
    cmp r10, 0
    je lab64907
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64907:
    ; #move variables
    mov r15, rdi
    mov rsi, r10
    mov rdi, r11
    ; new a3: _Cont = (stop, f, a0, tot, x1)\{ ... \};
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
    je lab64919
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab64920

lab64919:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64917
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64910
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64908
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64909

lab64908:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64909:

lab64910:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64913
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64911
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64912

lab64911:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64912:

lab64913:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64916
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64914
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64915

lab64914:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64915:

lab64916:
    jmp lab64918

lab64917:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64918:

lab64920:
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
    je lab64932
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64933

lab64932:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64930
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64923
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64921
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64922

lab64921:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64922:

lab64923:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64926
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64924
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64925

lab64924:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64925:

lab64926:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64929
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64927
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64928

lab64927:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64928:

lab64929:
    jmp lab64931

lab64930:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64931:

lab64933:
    ; #load tag
    lea r9, [rel _Cont_64934]
    ; substitute (i !-> i)(a3 !-> a3)(f0 !-> f0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; invoke f0 Apply
    jmp r9

_Cont_64934:

_Cont_64934_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64937
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load link to next block
    mov r10, [rsi + 48]
    ; ###load values
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab64935
    ; ####increment refcount
    add qword [r8 + 0], 1

lab64935:
    mov rdi, [rsi + 24]
    ; ###load values
    mov r15, [r10 + 56]
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab64936
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64936:
    jmp lab64938

lab64937:
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

lab64938:
    ; x3 <- x2 + tot;
    mov rcx, rdx
    add rcx, r13
    mov [rsp + 2024], rcx
    ; substitute (x1 !-> x1)(x3 !-> x3)(stop !-> stop)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov r13, r11
    mov r11, r9
    mov r9, rdi
    mov r12, r10
    mov r10, r8
    mov rdx, r15
    mov rdi, [rsp + 2024]
    ; jump sum_loop_
    jmp sum_loop_

lab64906:
    ; substitute (tot !-> tot)(a0 !-> a0);
    ; #erase f
    cmp r10, 0
    je lab64941
    ; ######check refcount
    cmp qword [r10 + 0], 0
    je lab64939
    ; ######either decrement refcount ...
    add qword [r10 + 0], -1
    jmp lab64940

lab64939:
    ; ######... or add block to lazy free list
    mov [r10 + 0], rbp
    mov rbp, r10

lab64940:

lab64941:
    ; #move variables
    mov rdx, rdi
    mov rsi, r12
    mov rdi, r13
    ; invoke a0 Ret
    jmp rdi

sum_:
    ; lit x0 <- 0;
    mov r13, 0
    ; substitute (start !-> start)(x0 !-> x0)(stop !-> stop)(f !-> f)(a0 !-> a0);
    ; #move variables
    mov r12, r10
    mov r10, rax
    mov rcx, rdi
    mov rdi, r13
    mov r13, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump sum_loop_
    jmp sum_loop_

motz_:
    ; lit x0 <- 1;
    mov r9, 1
    ; if n <= x0 \{ ... \}
    cmp rdx, r9
    jle lab64942
    ; substitute (n !-> n)(a0 !-> a0);
    ; lit x1 <- 2;
    mov r9, 2
    ; limit <- n - x1;
    mov r11, rdx
    sub r11, r9
    ; substitute (n !-> n)(a0 !-> a0)(limit0 !-> limit)(limit !-> limit);
    ; #move variables
    mov r9, r11
    ; new product: Fun[i64, i64] = (limit)\{ ... \};
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
    je lab64954
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab64955

lab64954:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64952
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64945
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64943
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64944

lab64943:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64944:

lab64945:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64948
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64946
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64947

lab64946:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64947:

lab64948:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64951
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64949
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64950

lab64949:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64950:

lab64951:
    jmp lab64953

lab64952:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64953:

lab64955:
    ; #load tag
    lea r11, [rel Fun_i64_i64_64956]
    ; new a1: _Cont = (a0, limit0, product)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov [rbx + 48], r10
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab64968
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab64969

lab64968:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64966
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64959
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64957
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64958

lab64957:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64958:

lab64959:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64962
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64960
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64961

lab64960:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64961:

lab64962:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64965
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64963
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64964

lab64963:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64964:

lab64965:
    jmp lab64967

lab64966:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64967:

lab64969:
    ; #load tag
    lea rdi, [rel _Cont_64970]
    ; lit x6 <- 1;
    mov r9, 1
    ; x7 <- n - x6;
    mov r11, rdx
    sub r11, r9
    ; substitute (x7 !-> x7)(a1 !-> a1);
    ; #move variables
    mov rdx, r11
    ; jump motz_
    jmp motz_

_Cont_64970:

_Cont_64970_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64973
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab64971
    ; ####increment refcount
    add qword [r10 + 0], 1

lab64971:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab64972
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64972:
    jmp lab64974

lab64973:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab64974:
    ; substitute (product !-> product)(limit0 !-> limit0)(a0 !-> a0)(x5 !-> x5);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rax, r10
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
    je lab64986
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab64987

lab64986:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab64984
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64977
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64975
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64976

lab64975:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64976:

lab64977:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64980
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64978
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64979

lab64978:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64979:

lab64980:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab64983
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64981
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64982

lab64981:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64982:

lab64983:
    jmp lab64985

lab64984:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab64985:

lab64987:
    ; #load tag
    lea r9, [rel _Cont_64988]
    ; lit x9 <- 0;
    mov r11, 0
    ; substitute (product !-> product)(x9 !-> x9)(limit0 !-> limit0)(a3 !-> a3);
    ; #move variables
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    mov r10, r8
    ; jump sum_
    jmp sum_

_Cont_64988:

_Cont_64988_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab64990
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab64989
    ; ####increment refcount
    add qword [rsi + 0], 1

lab64989:
    jmp lab64991

lab64990:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab64991:
    ; x12 <- x5 + x8;
    mov r11, r9
    add r11, rdx
    ; substitute (x12 !-> x12)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; invoke a0 Ret
    jmp rdi

Fun_i64_i64_64956:

Fun_i64_i64_64956_Apply:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab64992
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    jmp lab64993

lab64992:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]

lab64993:
    ; substitute (i0 !-> i)(a4 !-> a4)(limit !-> limit)(i !-> i);
    ; #move variables
    mov r11, rdx
    ; new a5: _Cont = (a4, limit, i)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov [rbx + 56], r11
    mov qword [rbx + 48], 0
    mov [rbx + 40], r9
    mov qword [rbx + 32], 0
    mov [rbx + 24], rdi
    mov [rbx + 16], rsi
    ; ##acquire free block from heap register
    mov rsi, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65005
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65006

lab65005:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65003
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab64996
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64994
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64995

lab64994:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64995:

lab64996:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab64999
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab64997
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab64998

lab64997:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab64998:

lab64999:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65002
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65000
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65001

lab65000:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65001:

lab65002:
    jmp lab65004

lab65003:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65004:

lab65006:
    ; #load tag
    lea rdi, [rel _Cont_65007]
    ; jump motz_
    jmp motz_

_Cont_65007:

_Cont_65007_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65009
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab65008
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65008:
    jmp lab65010

lab65009:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab65010:
    ; substitute (i !-> i)(limit !-> limit)(a4 !-> a4)(x2 !-> x2);
    ; #move variables
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; new a6: _Cont = (a4, x2)\{ ... \};
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
    je lab65022
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65023

lab65022:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65020
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65013
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65011
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65012

lab65011:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65012:

lab65013:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65016
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65014
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65015

lab65014:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65015:

lab65016:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65019
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65017
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65018

lab65017:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65018:

lab65019:
    jmp lab65021

lab65020:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65021:

lab65023:
    ; #load tag
    lea r9, [rel _Cont_65024]
    ; x4 <- limit - i;
    mov r11, rdi
    sub r11, rdx
    ; substitute (x4 !-> x4)(a6 !-> a6);
    ; #move variables
    mov rsi, r8
    mov rdi, r9
    mov rdx, r11
    ; jump motz_
    jmp motz_

_Cont_65024:

_Cont_65024_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65026
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65025
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65025:
    jmp lab65027

lab65026:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65027:
    ; x11 <- x2 * x3;
    mov r11, r9
    imul r11, rdx
    ; substitute (x11 !-> x11)(a4 !-> a4);
    ; #move variables
    mov rdx, r11
    ; invoke a4 Ret
    jmp rdi

lab64942:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    ; lit x10 <- 1;
    mov rdi, 1
    ; substitute (x10 !-> x10)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

main_loop_:
    ; substitute (n0 !-> n)(n !-> n)(a0 !-> a0)(iters !-> iters);
    ; #move variables
    mov r11, rdx
    mov rdx, rdi
    ; new a2: _Cont = (n, a0, iters)\{ ... \};
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
    je lab65039
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65040

lab65039:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65037
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65030
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65028
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65029

lab65028:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65029:

lab65030:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65033
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65031
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65032

lab65031:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65032:

lab65033:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65036
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65034
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65035

lab65034:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65035:

lab65036:
    jmp lab65038

lab65037:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65038:

lab65040:
    ; #load tag
    lea rdi, [rel _Cont_65041]
    ; jump motz_
    jmp motz_

_Cont_65041:

_Cont_65041_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65043
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab65042
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65042:
    mov rdi, [rsi + 24]
    jmp lab65044

lab65043:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab65044:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab65045
    ; substitute (iters !-> iters)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r11
    ; lit x1 <- 1;
    mov r11, 1
    ; x2 <- iters - x1;
    mov r13, rdx
    sub r13, r11
    ; substitute (x2 !-> x2)(n !-> n)(a0 !-> a0);
    ; #move variables
    mov rdx, r13
    ; jump main_loop_
    jmp main_loop_

lab65045:
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