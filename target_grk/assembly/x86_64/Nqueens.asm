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
    lea r9, [rel _Cont_65170]
    ; jump main_loop_
    jmp main_loop_

_Cont_65170:

_Cont_65170_Ret:
    ; return x0
    mov rax, rdx
    jmp cleanup

and_:
    ; substitute (a0 !-> a0)(b2 !-> b2)(b1 !-> b1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch b1 \{ ... \};
    lea rcx, [rel Bool_65171]
    add rcx, r9
    jmp rcx

Bool_65171:
    jmp near Bool_65171_True
    jmp near Bool_65171_False

Bool_65171_True:
    ; switch b2 \{ ... \};
    lea rcx, [rel Bool_65172]
    add rcx, rdi
    jmp rcx

Bool_65172:
    jmp near Bool_65172_True
    jmp near Bool_65172_False

Bool_65172_True:
    ; invoke a0 True
    add rdx, 0
    jmp rdx

Bool_65172_False:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

Bool_65171_False:
    ; substitute (a0 !-> a0);
    ; #erase b2
    cmp rsi, 0
    je lab65175
    ; ######check refcount
    cmp qword [rsi + 0], 0
    je lab65173
    ; ######either decrement refcount ...
    add qword [rsi + 0], -1
    jmp lab65174

lab65173:
    ; ######... or add block to lazy free list
    mov [rsi + 0], rbp
    mov rbp, rsi

lab65174:

lab65175:
    ; invoke a0 False
    add rdx, 5
    jmp rdx

neq_i_:
    ; if i1 == i2 \{ ... \}
    cmp rdx, rdi
    je lab65176
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

lab65176:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 False
    add rdx, 5
    jmp rdx

length_loop_:
    ; substitute (a0 !-> a0)(acc !-> acc)(l !-> l);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_65177]
    add rcx, r9
    jmp rcx

List_List_i64_65177:
    jmp near List_List_i64_65177_Nil
    jmp near List_List_i64_65177_Cons

List_List_i64_65177_Nil:
    ; substitute (acc !-> acc)(a0 !-> a0);
    ; #move variables
    mov rsi, rax
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; invoke a0 Ret
    jmp rdi

List_List_i64_65177_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65180
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab65178
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65178:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab65179
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65179:
    jmp lab65181

lab65180:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab65181:
    ; substitute (a0 !-> a0)(acc !-> acc)(iss !-> iss);
    ; #erase is
    cmp r8, 0
    je lab65184
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab65182
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab65183

lab65182:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab65183:

lab65184:
    ; #move variables
    mov r8, r10
    mov r9, r11
    ; lit x0 <- 1;
    mov r11, 1
    ; x1 <- acc + x0;
    mov r13, rdi
    add r13, r11
    ; substitute (iss !-> iss)(x1 !-> x1)(a0 !-> a0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rdi, r13
    ; jump length_loop_
    jmp length_loop_

length_:
    ; lit x0 <- 0;
    mov r9, 0
    ; substitute (l !-> l)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; jump length_loop_
    jmp length_loop_

appendRev_:
    ; substitute (a0 !-> a0)(l2 !-> l2)(l1 !-> l1);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_List_i64_65185]
    add rcx, r9
    jmp rcx

List_List_i64_65185:
    jmp near List_List_i64_65185_Nil
    jmp near List_List_i64_65185_Cons

List_List_i64_65185_Nil:
    ; switch l2 \{ ... \};
    lea rcx, [rel List_List_i64_65186]
    add rcx, rdi
    jmp rcx

List_List_i64_65186:
    jmp near List_List_i64_65186_Nil
    jmp near List_List_i64_65186_Cons

List_List_i64_65186_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_65186_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65189
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65187
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65187:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65188
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65188:
    jmp lab65190

lab65189:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65190:
    ; substitute (a1 !-> a1)(as0 !-> as0)(a0 !-> a0);
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

List_List_i64_65185_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65193
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab65191
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65191:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab65192
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65192:
    jmp lab65194

lab65193:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab65194:
    ; substitute (a0 !-> a0)(iss !-> iss)(is !-> is)(l2 !-> l2);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; let x0: List[List[i64]] = Cons(is, l2);
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
    je lab65206
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65207

lab65206:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65204
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65197
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65195
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65196

lab65195:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65196:

lab65197:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65200
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65198
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65199

lab65198:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65199:

lab65200:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65203
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65201
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65202

lab65201:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65202:

lab65203:
    jmp lab65205

lab65204:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65205:

lab65207:
    ; #load tag
    mov r9, 5
    ; substitute (iss !-> iss)(x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump appendRev_
    jmp appendRev_

rev_:
    ; let x0: List[List[i64]] = Nil();
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
    ; jump appendRev_
    jmp appendRev_

append_:
    ; substitute (l1 !-> l1)(a0 !-> a0)(l2 !-> l2);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    ; switch l2 \{ ... \};
    lea rcx, [rel List_List_i64_65208]
    add rcx, r9
    jmp rcx

List_List_i64_65208:
    jmp near List_List_i64_65208_Nil
    jmp near List_List_i64_65208_Cons

List_List_i64_65208_Nil:
    ; substitute (a0 !-> a0)(l1 !-> l1);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; switch l1 \{ ... \};
    lea rcx, [rel List_List_i64_65209]
    add rcx, rdi
    jmp rcx

List_List_i64_65209:
    jmp near List_List_i64_65209_Nil
    jmp near List_List_i64_65209_Cons

List_List_i64_65209_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_65209_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65212
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65210
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65210:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65211
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65211:
    jmp lab65213

lab65212:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65213:
    ; substitute (a2 !-> a2)(as0 !-> as0)(a0 !-> a0);
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

List_List_i64_65208_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65216
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    cmp r10, 0
    je lab65214
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65214:
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab65215
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65215:
    jmp lab65217

lab65216:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r10, [r8 + 48]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab65217:
    ; new a1: List[List[i64]] = (a0, is, iss)\{ ... \};
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
    je lab65229
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65230

lab65229:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65227
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65220
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65218
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65219

lab65218:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65219:

lab65220:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65223
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65221
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65222

lab65221:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65222:

lab65223:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65226
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65224
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65225

lab65224:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65225:

lab65226:
    jmp lab65228

lab65227:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65228:

lab65230:
    ; #load tag
    lea rdi, [rel List_List_i64_65231]
    ; jump rev_
    jmp rev_

List_List_i64_65231:
    jmp near List_List_i64_65231_Nil
    jmp near List_List_i64_65231_Cons

List_List_i64_65231_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65235
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov r8, [rax + 48]
    cmp r8, 0
    je lab65232
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65232:
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65233
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65233:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65234
    ; ####increment refcount
    add qword [rax + 0], 1

lab65234:
    jmp lab65236

lab65235:
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

lab65236:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; jump lift_append_0_
    jmp lift_append_0_

List_List_i64_65231_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65240
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab65237
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65237:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65238
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65238:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab65239
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65239:
    jmp lab65241

lab65240:
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

lab65241:
    ; substitute (iss !-> iss)(is !-> is)(a0 !-> a0)(a3 !-> a3)(as1 !-> as1);
    ; #move variables
    mov rcx, r12
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a3, as1);
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
    je lab65253
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65254

lab65253:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65251
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65244
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65242
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65243

lab65242:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65243:

lab65244:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65247
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65245
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65246

lab65245:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65246:

lab65247:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65250
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65248
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65249

lab65248:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65249:

lab65250:
    jmp lab65252

lab65251:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65252:

lab65254:
    ; #load tag
    mov r11, 5
    ; substitute (a0 !-> a0)(is !-> is)(iss !-> iss)(x0 !-> x0);
    ; #move variables
    mov rcx, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump lift_append_0_
    jmp lift_append_0_

lift_append_0_:
    ; substitute (a0 !-> a0)(x0 !-> x0)(is !-> is)(iss !-> iss);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    ; let x1: List[List[i64]] = Cons(is, iss);
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
    je lab65266
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65267

lab65266:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65264
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65257
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65255
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65256

lab65255:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65256:

lab65257:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65260
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65258
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65259

lab65258:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65259:

lab65260:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65263
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65261
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65262

lab65261:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65262:

lab65263:
    jmp lab65265

lab65264:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65265:

lab65267:
    ; #load tag
    mov r9, 5
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
    ; jump appendRev_
    jmp appendRev_

safe_:
    ; substitute (x !-> x)(d !-> d)(a0 !-> a0)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, r8
    mov r8, rcx
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_i64_65268]
    add rcx, r11
    jmp rcx

List_i64_65268:
    jmp near List_i64_65268_Nil
    jmp near List_i64_65268_Cons

List_i64_65268_Nil:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; invoke a0 True
    add rdx, 0
    jmp rdx

List_i64_65268_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab65270
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab65269
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65269:
    mov r11, [r10 + 40]
    jmp lab65271

lab65270:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]

lab65271:
    ; substitute (x10 !-> x)(q0 !-> q)(a0 !-> a0)(q !-> q)(l0 !-> l0)(x !-> x)(d !-> d);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov rdi, r11
    ; new a1: Bool = (a0, q, l0, x, d)\{ ... \};
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
    je lab65283
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65284

lab65283:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65281
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65274
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65272
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65273

lab65272:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65273:

lab65274:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65277
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65275
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65276

lab65275:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65276:

lab65277:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65280
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65278
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65279

lab65278:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65279:

lab65280:
    jmp lab65282

lab65281:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65282:

lab65284:
    ; ##store link to previous block
    mov [rbx + 48], r12
    ; ##store values
    mov [rbx + 40], r11
    mov qword [rbx + 32], 0
    mov [rbx + 24], r9
    mov [rbx + 16], r8
    ; ##acquire free block from heap register
    mov r8, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65296
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65297

lab65296:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65294
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65287
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65285
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65286

lab65285:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65286:

lab65287:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65290
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65288
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65289

lab65288:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65289:

lab65290:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65293
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65291
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65292

lab65291:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65292:

lab65293:
    jmp lab65295

lab65294:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65295:

lab65297:
    ; #load tag
    lea r9, [rel Bool_65298]
    ; jump neq_i_
    jmp neq_i_

Bool_65298:
    jmp near Bool_65298_True
    jmp near Bool_65298_False

Bool_65298_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65301
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65299
    ; ####increment refcount
    add qword [rax + 0], 1

lab65299:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab65300
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65300:
    jmp lab65302

lab65301:
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

lab65302:
    ; let x0: Bool = True();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a0 !-> a0)(d !-> d)(l0 !-> l0)(q !-> q)(x !-> x)(x0 !-> x0);
    ; #move variables
    mov rcx, r13
    mov r13, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump lift_safe_0_
    jmp lift_safe_0_

Bool_65298_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65305
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65303
    ; ####increment refcount
    add qword [rax + 0], 1

lab65303:
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab65304
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65304:
    jmp lab65306

lab65305:
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

lab65306:
    ; let x0: Bool = False();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 5
    ; substitute (a0 !-> a0)(d !-> d)(l0 !-> l0)(q !-> q)(x !-> x)(x0 !-> x0);
    ; #move variables
    mov rcx, r13
    mov r13, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump lift_safe_0_
    jmp lift_safe_0_

lift_safe_0_:
    ; substitute (x !-> x)(d !-> d)(l0 !-> l0)(q !-> q)(a0 !-> a0)(x0 !-> x0);
    ; #move variables
    mov r12, rax
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; new a2: Bool = (a0, x0)\{ ... \};
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
    je lab65318
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65319

lab65318:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65316
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65309
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65307
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65308

lab65307:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65308:

lab65309:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65312
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65310
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65311

lab65310:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65311:

lab65312:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65315
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65313
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65314

lab65313:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65314:

lab65315:
    jmp lab65317

lab65316:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65317:

lab65319:
    ; #load tag
    lea r13, [rel Bool_65320]
    ; substitute (x10 !-> x)(d0 !-> d)(q0 !-> q)(q !-> q)(a2 !-> a2)(x !-> x)(d !-> d)(l0 !-> l0);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov [rsp + 2016], r8
    mov [rsp + 2008], r9
    mov r9, r11
    ; new a3: Bool = (q, a2, x, d, l0)\{ ... \};
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
    je lab65332
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab65333

lab65332:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65330
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65323
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65321
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65322

lab65321:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65322:

lab65323:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65326
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65324
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65325

lab65324:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65325:

lab65326:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65329
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65327
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65328

lab65327:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65328:

lab65329:
    jmp lab65331

lab65330:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65331:

lab65333:
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
    je lab65345
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65346

lab65345:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65343
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65336
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65334
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65335

lab65334:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65335:

lab65336:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65339
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65337
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65338

lab65337:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65338:

lab65339:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65342
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65340
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65341

lab65340:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65341:

lab65342:
    jmp lab65344

lab65343:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65344:

lab65346:
    ; #load tag
    lea r11, [rel Bool_65347]
    ; x3 <- q0 + d0;
    mov r13, r9
    add r13, rdi
    ; substitute (x10 !-> x10)(x3 !-> x3)(a3 !-> a3);
    ; #move variables
    mov r8, r10
    mov r9, r11
    mov rdi, r13
    ; jump neq_i_
    jmp neq_i_

Bool_65347:
    jmp near Bool_65347_True
    jmp near Bool_65347_False

Bool_65347_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65350
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65348
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65348:
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab65349
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65349:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab65351

lab65350:
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
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab65351:
    ; let x2: Bool = True();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 0
    ; substitute (a2 !-> a2)(d !-> d)(l0 !-> l0)(q !-> q)(x !-> x)(x2 !-> x2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    mov r8, r12
    ; jump lift_safe_1_
    jmp lift_safe_1_

Bool_65347_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65354
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65352
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65352:
    mov rdx, [rax + 24]
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab65353
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65353:
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]
    jmp lab65355

lab65354:
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
    mov r12, [r8 + 48]
    mov r11, [r8 + 40]
    mov r9, [r8 + 24]

lab65355:
    ; let x2: Bool = False();
    ; #mark no allocation
    mov r14, 0
    ; #load tag
    mov r15, 5
    ; substitute (a2 !-> a2)(d !-> d)(l0 !-> l0)(q !-> q)(x !-> x)(x2 !-> x2);
    ; #move variables
    mov rcx, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rcx, r13
    mov r13, r9
    mov r9, rcx
    mov r8, r12
    ; jump lift_safe_1_
    jmp lift_safe_1_

Bool_65320:
    jmp near Bool_65320_True
    jmp near Bool_65320_False

Bool_65320_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65358
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65356
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65356:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65357
    ; ####increment refcount
    add qword [rax + 0], 1

lab65357:
    jmp lab65359

lab65358:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65359:
    ; let x1: Bool = True();
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
    ; jump and_
    jmp and_

Bool_65320_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65362
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65360
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65360:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65361
    ; ####increment refcount
    add qword [rax + 0], 1

lab65361:
    jmp lab65363

lab65362:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65363:
    ; let x1: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
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
    ; jump and_
    jmp and_

lift_safe_1_:
    ; substitute (x !-> x)(d !-> d)(l0 !-> l0)(q !-> q)(a2 !-> a2)(x2 !-> x2);
    ; #move variables
    mov r12, rax
    mov rcx, r13
    mov r13, rdx
    mov rdx, rcx
    ; new a5: Bool = (a2, x2)\{ ... \};
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
    je lab65375
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65376

lab65375:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65373
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65366
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65364
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65365

lab65364:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65365:

lab65366:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65369
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65367
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65368

lab65367:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65368:

lab65369:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65372
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65370
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65371

lab65370:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65371:

lab65372:
    jmp lab65374

lab65373:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65374:

lab65376:
    ; #load tag
    lea r13, [rel Bool_65377]
    ; substitute (x10 !-> x)(d0 !-> d)(q !-> q)(l0 !-> l0)(a5 !-> a5)(x !-> x)(d !-> d);
    ; #move variables
    mov r15, rdx
    mov [rsp + 2024], rdi
    mov r10, r8
    mov rcx, r11
    mov r11, r9
    mov r9, rcx
    ; new a6: Bool = (l0, a5, x, d)\{ ... \};
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
    je lab65389
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65390

lab65389:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65387
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65380
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65378
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65379

lab65378:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65379:

lab65380:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65383
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65381
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65382

lab65381:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65382:

lab65383:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65386
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65384
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65385

lab65384:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65385:

lab65386:
    jmp lab65388

lab65387:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65388:

lab65390:
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
    je lab65402
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65403

lab65402:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65400
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65393
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65391
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65392

lab65391:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65392:

lab65393:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65396
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65394
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65395

lab65394:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65395:

lab65396:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65399
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65397
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65398

lab65397:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65398:

lab65399:
    jmp lab65401

lab65400:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65401:

lab65403:
    ; #load tag
    lea r11, [rel Bool_65404]
    ; x6 <- q - d0;
    mov r13, r9
    sub r13, rdi
    ; substitute (x10 !-> x10)(x6 !-> x6)(a6 !-> a6);
    ; #move variables
    mov r8, r10
    mov r9, r11
    mov rdi, r13
    ; jump neq_i_
    jmp neq_i_

Bool_65404:
    jmp near Bool_65404_True
    jmp near Bool_65404_False

Bool_65404_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65407
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65405
    ; ####increment refcount
    add qword [rax + 0], 1

lab65405:
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab65406
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65406:
    jmp lab65408

lab65407:
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
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab65408:
    ; let x5: Bool = True();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a5 !-> a5)(d !-> d)(l0 !-> l0)(x !-> x)(x5 !-> x5);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_safe_2_
    jmp lift_safe_2_

Bool_65404_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65411
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65409
    ; ####increment refcount
    add qword [rax + 0], 1

lab65409:
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab65410
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65410:
    jmp lab65412

lab65411:
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
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab65412:
    ; let x5: Bool = False();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 5
    ; substitute (a5 !-> a5)(d !-> d)(l0 !-> l0)(x !-> x)(x5 !-> x5);
    ; #move variables
    mov r8, rax
    mov rcx, rdi
    mov rdi, r11
    mov r11, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    ; jump lift_safe_2_
    jmp lift_safe_2_

Bool_65377:
    jmp near Bool_65377_True
    jmp near Bool_65377_False

Bool_65377_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65415
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65413
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65413:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65414
    ; ####increment refcount
    add qword [rax + 0], 1

lab65414:
    jmp lab65416

lab65415:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65416:
    ; let x4: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x2 !-> x2)(x4 !-> x4)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

Bool_65377_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65419
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65417
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65417:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65418
    ; ####increment refcount
    add qword [rax + 0], 1

lab65418:
    jmp lab65420

lab65419:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65420:
    ; let x4: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x2 !-> x2)(x4 !-> x4)(a2 !-> a2);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

lift_safe_2_:
    ; substitute (x !-> x)(d !-> d)(l0 !-> l0)(a5 !-> a5)(x5 !-> x5);
    ; #move variables
    mov r10, rax
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; new a8: Bool = (a5, x5)\{ ... \};
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
    je lab65432
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65433

lab65432:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65430
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65423
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65421
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65422

lab65421:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65422:

lab65423:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65426
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65424
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65425

lab65424:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65425:

lab65426:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65429
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65427
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65428

lab65427:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65428:

lab65429:
    jmp lab65431

lab65430:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65431:

lab65433:
    ; #load tag
    lea r11, [rel Bool_65434]
    ; lit x8 <- 1;
    mov r13, 1
    ; x9 <- d + x8;
    mov r15, rdi
    add r15, r13
    ; substitute (x !-> x)(x9 !-> x9)(l0 !-> l0)(a8 !-> a8);
    ; #move variables
    mov rdi, r15
    ; jump safe_
    jmp safe_

Bool_65434:
    jmp near Bool_65434_True
    jmp near Bool_65434_False

Bool_65434_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65437
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65435
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65435:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65436
    ; ####increment refcount
    add qword [rax + 0], 1

lab65436:
    jmp lab65438

lab65437:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65438:
    ; let x7: Bool = True();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (x5 !-> x5)(x7 !-> x7)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

Bool_65434_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65441
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    cmp rsi, 0
    je lab65439
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65439:
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65440
    ; ####increment refcount
    add qword [rax + 0], 1

lab65440:
    jmp lab65442

lab65441:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rsi, [rax + 48]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65442:
    ; let x7: Bool = False();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 5
    ; substitute (x5 !-> x5)(x7 !-> x7)(a5 !-> a5);
    ; #move variables
    mov rcx, rsi
    mov rsi, r8
    mov r8, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; jump and_
    jmp and_

check_:
    ; substitute (a0 !-> a0)(acc !-> acc)(q !-> q)(l !-> l);
    ; #move variables
    mov rcx, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, rdx
    mov rdx, rcx
    ; switch l \{ ... \};
    lea rcx, [rel List_List_i64_65443]
    add rcx, r11
    jmp rcx

List_List_i64_65443:
    jmp near List_List_i64_65443_Nil
    jmp near List_List_i64_65443_Cons

List_List_i64_65443_Nil:
    ; substitute (a0 !-> a0)(acc !-> acc);
    ; switch acc \{ ... \};
    lea rcx, [rel List_List_i64_65444]
    add rcx, rdi
    jmp rcx

List_List_i64_65444:
    jmp near List_List_i64_65444_Nil
    jmp near List_List_i64_65444_Cons

List_List_i64_65444_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_65444_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65447
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65445
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65445:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65446
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65446:
    jmp lab65448

lab65447:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65448:
    ; substitute (a2 !-> a2)(as0 !-> as0)(a0 !-> a0);
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

List_List_i64_65443_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r10 + 0], 0
    je lab65451
    ; ##either decrement refcount and share children...
    add qword [r10 + 0], -1
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    cmp r12, 0
    je lab65449
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65449:
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]
    cmp r10, 0
    je lab65450
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65450:
    jmp lab65452

lab65451:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r10 + 0], rbx
    mov rbx, r10
    ; ###load values
    mov r13, [r10 + 56]
    mov r12, [r10 + 48]
    mov r11, [r10 + 40]
    mov r10, [r10 + 32]

lab65452:
    ; lit x0 <- 1;
    mov r15, 1
    ; substitute (x0 !-> x0)(b0 !-> b)(q1 !-> q)(b !-> b)(bs !-> bs)(q !-> q)(a0 !-> a0)(acc !-> acc);
    ; #share b
    cmp r10, 0
    je lab65453
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65453:
    ; #move variables
    mov [rsp + 2032], rax
    mov [rsp + 2024], rdx
    mov [rsp + 2016], rsi
    mov [rsp + 2008], rdi
    mov rdx, r15
    mov r15, r9
    mov rsi, r10
    mov rdi, r11
    ; new a1: Bool = (b, bs, q, a0, acc)\{ ... \};
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
    mov qword [rbx + 16], 0
    ; ##acquire free block from heap register
    mov r14, rbx
    ; ##get next free block into heap register
    ; ###(1) check linear free list for next block
    mov rbx, [rbx + 0]
    cmp rbx, 0
    je lab65465
    ; ####initialize refcount of just acquired block
    mov qword [r14 + 0], 0
    jmp lab65466

lab65465:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65463
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65456
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65454
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65455

lab65454:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65455:

lab65456:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65459
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65457
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65458

lab65457:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65458:

lab65459:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65462
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65460
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65461

lab65460:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65461:

lab65462:
    jmp lab65464

lab65463:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65464:

lab65466:
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
    je lab65478
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65479

lab65478:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65476
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65469
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65467
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65468

lab65467:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65468:

lab65469:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65472
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65470
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65471

lab65470:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65471:

lab65472:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65475
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65473
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65474

lab65473:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65474:

lab65475:
    jmp lab65477

lab65476:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65477:

lab65479:
    ; #load tag
    lea r11, [rel Bool_65480]
    ; substitute (q1 !-> q1)(x0 !-> x0)(b0 !-> b0)(a1 !-> a1);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov r8, rsi
    ; jump safe_
    jmp safe_

Bool_65480:
    jmp near Bool_65480_True
    jmp near Bool_65480_False

Bool_65480_True:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65485
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65481
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65481:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65482
    ; ####increment refcount
    add qword [rax + 0], 1

lab65482:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab65483
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65483:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65484
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65484:
    mov r9, [r8 + 24]
    jmp lab65486

lab65485:
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

lab65486:
    ; substitute (acc !-> acc)(bs !-> bs)(q !-> q)(a0 !-> a0)(q0 !-> q)(b !-> b);
    ; #move variables
    mov r14, rax
    mov r15, rdx
    mov rdx, r13
    mov r13, r9
    mov rax, r12
    ; let x2: List[i64] = Cons(q0, b);
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
    je lab65498
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65499

lab65498:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65496
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65489
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65487
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65488

lab65487:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65488:

lab65489:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65492
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65490
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65491

lab65490:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65491:

lab65492:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65495
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65493
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65494

lab65493:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65494:

lab65495:
    jmp lab65497

lab65496:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65497:

lab65499:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(bs !-> bs)(q !-> q)(x2 !-> x2)(acc !-> acc);
    ; #move variables
    mov rcx, r10
    mov r10, r12
    mov r12, rax
    mov rax, rcx
    mov rcx, r11
    mov r11, r13
    mov r13, rdx
    mov rdx, rcx
    ; let x1: List[List[i64]] = Cons(x2, acc);
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
    je lab65511
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65512

lab65511:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65509
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65502
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65500
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65501

lab65500:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65501:

lab65502:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65505
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65503
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65504

lab65503:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65504:

lab65505:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65508
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65506
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65507

lab65506:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65507:

lab65508:
    jmp lab65510

lab65509:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65510:

lab65512:
    ; #load tag
    mov r11, 5
    ; substitute (bs !-> bs)(x1 !-> x1)(q !-> q)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, r10
    mov r10, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump check_
    jmp check_

Bool_65480_False:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65517
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov r8, [rax + 48]
    ; ###load values
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65513
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65513:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65514
    ; ####increment refcount
    add qword [rax + 0], 1

lab65514:
    ; ###load values
    mov r13, [r8 + 56]
    mov r12, [r8 + 48]
    cmp r12, 0
    je lab65515
    ; ####increment refcount
    add qword [r12 + 0], 1

lab65515:
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65516
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65516:
    mov r9, [r8 + 24]
    jmp lab65518

lab65517:
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

lab65518:
    ; substitute (bs !-> bs)(acc !-> acc)(q !-> q)(a0 !-> a0);
    ; #erase b
    cmp rax, 0
    je lab65521
    ; ######check refcount
    cmp qword [rax + 0], 0
    je lab65519
    ; ######either decrement refcount ...
    add qword [rax + 0], -1
    jmp lab65520

lab65519:
    ; ######... or add block to lazy free list
    mov [rax + 0], rbp
    mov rbp, rax

lab65520:

lab65521:
    ; #move variables
    mov rax, rsi
    mov rdx, rdi
    mov rsi, r12
    mov rdi, r13
    ; jump check_
    jmp check_

enumerate_:
    ; if q == 0 \{ ... \}
    cmp rdx, 0
    je lab65522
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (q0 !-> q)(x0 !-> x0)(bs0 !-> bs)(a0 !-> a0)(bs !-> bs)(q !-> q)(acc !-> acc);
    ; #share bs
    cmp r8, 0
    je lab65523
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65523:
    ; #move variables
    mov r15, rdx
    mov [rsp + 2032], rsi
    mov [rsp + 2024], rdi
    mov rsi, r12
    mov r12, r8
    mov rdi, r13
    mov r13, r9
    ; new a3: List[List[i64]] = (a0, bs, q, acc)\{ ... \};
    ; #allocate memory
    ; ##store values
    mov rcx, [rsp + 2024]
    mov [rbx + 56], rcx
    mov rcx, [rsp + 2032]
    mov [rbx + 48], rcx
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
    je lab65535
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65536

lab65535:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65533
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65526
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65524
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65525

lab65524:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65525:

lab65526:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65529
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65527
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65528

lab65527:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65528:

lab65529:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65532
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65530
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65531

lab65530:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65531:

lab65532:
    jmp lab65534

lab65533:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65534:

lab65536:
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
    je lab65548
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65549

lab65548:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65546
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65539
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65537
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65538

lab65537:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65538:

lab65539:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65542
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65540
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65541

lab65540:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65541:

lab65542:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65545
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65543
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65544

lab65543:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65544:

lab65545:
    jmp lab65547

lab65546:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65547:

lab65549:
    ; #load tag
    lea r11, [rel List_List_i64_65550]
    ; substitute (bs0 !-> bs0)(x0 !-> x0)(q0 !-> q0)(a3 !-> a3);
    ; #move variables
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, r8
    ; jump check_
    jmp check_

List_List_i64_65550:
    jmp near List_List_i64_65550_Nil
    jmp near List_List_i64_65550_Cons

List_List_i64_65550_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65554
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load link to next block
    mov rsi, [rax + 48]
    ; ###load values
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65551
    ; ####increment refcount
    add qword [rax + 0], 1

lab65551:
    ; ###load values
    mov r11, [rsi + 56]
    mov r10, [rsi + 48]
    cmp r10, 0
    je lab65552
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65552:
    mov r9, [rsi + 40]
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]
    cmp rsi, 0
    je lab65553
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65553:
    jmp lab65555

lab65554:
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
    mov rdi, [rsi + 24]
    mov rsi, [rsi + 16]

lab65555:
    ; let res: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r12, 0
    ; #load tag
    mov r13, 0
    ; substitute (a0 !-> a0)(acc !-> acc)(bs !-> bs)(q !-> q)(res !-> res);
    ; #move variables
    mov r8, rsi
    mov rcx, r11
    mov r11, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r10
    ; jump lift_enumerate_0_
    jmp lift_enumerate_0_

List_List_i64_65550_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65559
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load link to next block
    mov r10, [r8 + 48]
    ; ###load values
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab65556
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65556:
    ; ###load values
    mov r15, [r10 + 56]
    mov r14, [r10 + 48]
    cmp r14, 0
    je lab65557
    ; ####increment refcount
    add qword [r14 + 0], 1

lab65557:
    mov r13, [r10 + 40]
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]
    cmp r10, 0
    je lab65558
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65558:
    jmp lab65560

lab65559:
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
    mov r11, [r10 + 24]
    mov r10, [r10 + 16]

lab65560:
    ; substitute (acc !-> acc)(q !-> q)(a0 !-> a0)(bs !-> bs)(a6 !-> a6)(as2 !-> as2);
    ; #move variables
    mov r12, rax
    mov rcx, r15
    mov r15, rdi
    mov rdi, r13
    mov r13, rdx
    mov rdx, rcx
    mov rax, r14
    mov r14, rsi
    ; let res: List[List[i64]] = Cons(a6, as2);
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
    je lab65572
    ; ####initialize refcount of just acquired block
    mov qword [r12 + 0], 0
    jmp lab65573

lab65572:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65570
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65563
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65561
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65562

lab65561:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65562:

lab65563:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65566
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65564
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65565

lab65564:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65565:

lab65566:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65569
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65567
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65568

lab65567:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65568:

lab65569:
    jmp lab65571

lab65570:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65571:

lab65573:
    ; #load tag
    mov r13, 5
    ; substitute (a0 !-> a0)(acc !-> acc)(bs !-> bs)(q !-> q)(res !-> res);
    ; #move variables
    mov rsi, rax
    mov rcx, r9
    mov r9, r11
    mov r11, rdi
    mov rdi, rdx
    mov rdx, rcx
    mov rax, r8
    mov r8, r10
    ; jump lift_enumerate_0_
    jmp lift_enumerate_0_

lab65522:
    ; substitute (a0 !-> a0)(acc !-> acc);
    ; #erase bs
    cmp r8, 0
    je lab65576
    ; ######check refcount
    cmp qword [r8 + 0], 0
    je lab65574
    ; ######either decrement refcount ...
    add qword [r8 + 0], -1
    jmp lab65575

lab65574:
    ; ######... or add block to lazy free list
    mov [r8 + 0], rbp
    mov rbp, r8

lab65575:

lab65576:
    ; #move variables
    mov rax, r10
    mov rdx, r11
    ; switch acc \{ ... \};
    lea rcx, [rel List_List_i64_65577]
    add rcx, rdi
    jmp rcx

List_List_i64_65577:
    jmp near List_List_i64_65577_Nil
    jmp near List_List_i64_65577_Cons

List_List_i64_65577_Nil:
    ; invoke a0 Nil
    add rdx, 0
    jmp rdx

List_List_i64_65577_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65580
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    cmp r8, 0
    je lab65578
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65578:
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]
    cmp rsi, 0
    je lab65579
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65579:
    jmp lab65581

lab65580:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r9, [rsi + 56]
    mov r8, [rsi + 48]
    mov rdi, [rsi + 40]
    mov rsi, [rsi + 32]

lab65581:
    ; substitute (a4 !-> a4)(as0 !-> as0)(a0 !-> a0);
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

lift_enumerate_0_:
    ; lit x1 <- 1;
    mov r15, 1
    ; x2 <- q - x1;
    mov rcx, r11
    sub rcx, r15
    mov [rsp + 2024], rcx
    ; substitute (res !-> res)(acc !-> acc)(bs !-> bs)(a0 !-> a0)(x2 !-> x2);
    ; #move variables
    mov r10, rax
    mov r11, rdx
    mov rax, r12
    mov rdx, r13
    mov r13, [rsp + 2024]
    ; new a2: List[List[i64]] = (bs, a0, x2)\{ ... \};
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
    je lab65593
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65594

lab65593:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65591
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65584
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65582
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65583

lab65582:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65583:

lab65584:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65587
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65585
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65586

lab65585:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65586:

lab65587:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65590
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65588
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65589

lab65588:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65589:

lab65590:
    jmp lab65592

lab65591:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65592:

lab65594:
    ; #load tag
    lea r9, [rel List_List_i64_65595]
    ; jump append_
    jmp append_

List_List_i64_65595:
    jmp near List_List_i64_65595_Nil
    jmp near List_List_i64_65595_Cons

List_List_i64_65595_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65598
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov r9, [rax + 56]
    mov rdi, [rax + 40]
    mov rsi, [rax + 32]
    cmp rsi, 0
    je lab65596
    ; ####increment refcount
    add qword [rsi + 0], 1

lab65596:
    mov rdx, [rax + 24]
    mov rax, [rax + 16]
    cmp rax, 0
    je lab65597
    ; ####increment refcount
    add qword [rax + 0], 1

lab65597:
    jmp lab65599

lab65598:
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

lab65599:
    ; let x3: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (x2 !-> x2)(x3 !-> x3)(bs !-> bs)(a0 !-> a0);
    ; #move variables
    mov r8, rax
    mov rcx, r9
    mov r9, rdx
    mov rdx, rcx
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump enumerate_
    jmp enumerate_

List_List_i64_65595_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65602
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r13, [r8 + 56]
    mov r11, [r8 + 40]
    mov r10, [r8 + 32]
    cmp r10, 0
    je lab65600
    ; ####increment refcount
    add qword [r10 + 0], 1

lab65600:
    mov r9, [r8 + 24]
    mov r8, [r8 + 16]
    cmp r8, 0
    je lab65601
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65601:
    jmp lab65603

lab65602:
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

lab65603:
    ; substitute (x2 !-> x2)(a0 !-> a0)(bs !-> bs)(a5 !-> a5)(as1 !-> as1);
    ; #move variables
    mov r12, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r13
    mov r13, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; let x3: List[List[i64]] = Cons(a5, as1);
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
    je lab65615
    ; ####initialize refcount of just acquired block
    mov qword [r10 + 0], 0
    jmp lab65616

lab65615:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65613
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65606
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65604
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65605

lab65604:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65605:

lab65606:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65609
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65607
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65608

lab65607:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65608:

lab65609:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65612
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65610
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65611

lab65610:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65611:

lab65612:
    jmp lab65614

lab65613:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65614:

lab65616:
    ; #load tag
    mov r11, 5
    ; substitute (x2 !-> x2)(x3 !-> x3)(bs !-> bs)(a0 !-> a0);
    ; #move variables
    mov rcx, r10
    mov r10, rsi
    mov rsi, rcx
    mov rcx, r11
    mov r11, rdi
    mov rdi, rcx
    ; jump enumerate_
    jmp enumerate_

gen_:
    ; if n == 0 \{ ... \}
    cmp rdx, 0
    je lab65617
    ; lit x2 <- 1;
    mov r11, 1
    ; x3 <- n - x2;
    mov r13, rdx
    sub r13, r11
    ; substitute (x3 !-> x3)(nq0 !-> nq)(a0 !-> a0)(nq !-> nq);
    ; #move variables
    mov r11, rdi
    mov rdx, r13
    ; new a2: List[List[i64]] = (a0, nq)\{ ... \};
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
    je lab65629
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65630

lab65629:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65627
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65620
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65618
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65619

lab65618:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65619:

lab65620:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65623
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65621
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65622

lab65621:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65622:

lab65623:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65626
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65624
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65625

lab65624:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65625:

lab65626:
    jmp lab65628

lab65627:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65628:

lab65630:
    ; #load tag
    lea r9, [rel List_List_i64_65631]
    ; jump gen_
    jmp gen_

List_List_i64_65631:
    jmp near List_List_i64_65631_Nil
    jmp near List_List_i64_65631_Cons

List_List_i64_65631_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65633
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]
    cmp rax, 0
    je lab65632
    ; ####increment refcount
    add qword [rax + 0], 1

lab65632:
    jmp lab65634

lab65633:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdi, [rax + 56]
    mov rdx, [rax + 40]
    mov rax, [rax + 32]

lab65634:
    ; let bs: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r8, 0
    ; #load tag
    mov r9, 0
    ; substitute (a0 !-> a0)(bs !-> bs)(nq !-> nq);
    ; #move variables
    mov rcx, r9
    mov r9, rdi
    mov rdi, rcx
    mov rsi, r8
    ; jump lift_gen_0_
    jmp lift_gen_0_

List_List_i64_65631_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65636
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]
    cmp r8, 0
    je lab65635
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65635:
    jmp lab65637

lab65636:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r11, [r8 + 56]
    mov r9, [r8 + 40]
    mov r8, [r8 + 32]

lab65637:
    ; substitute (nq !-> nq)(a0 !-> a0)(a3 !-> a3)(as0 !-> as0);
    ; #move variables
    mov r10, rsi
    mov rsi, r8
    mov r8, rax
    mov rcx, r11
    mov r11, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    ; let bs: List[List[i64]] = Cons(a3, as0);
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
    je lab65649
    ; ####initialize refcount of just acquired block
    mov qword [r8 + 0], 0
    jmp lab65650

lab65649:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65647
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65640
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65638
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65639

lab65638:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65639:

lab65640:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65643
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65641
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65642

lab65641:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65642:

lab65643:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65646
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65644
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65645

lab65644:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65645:

lab65646:
    jmp lab65648

lab65647:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65648:

lab65650:
    ; #load tag
    mov r9, 5
    ; substitute (a0 !-> a0)(bs !-> bs)(nq !-> nq);
    ; #move variables
    mov rcx, rdi
    mov rdi, r9
    mov r9, rdx
    mov rdx, rcx
    mov rax, rsi
    mov rsi, r8
    ; jump lift_gen_0_
    jmp lift_gen_0_

lab65617:
    ; substitute (a0 !-> a0);
    ; #move variables
    mov rax, r8
    mov rdx, r9
    ; let x0: List[i64] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; let x1: List[List[i64]] = Nil();
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

lift_gen_0_:
    ; let x4: List[List[i64]] = Nil();
    ; #mark no allocation
    mov r10, 0
    ; #load tag
    mov r11, 0
    ; substitute (nq !-> nq)(x4 !-> x4)(bs !-> bs)(a0 !-> a0);
    ; #move variables
    mov r8, rsi
    mov rsi, r10
    mov r10, rax
    mov rcx, r9
    mov r9, rdi
    mov rdi, r11
    mov r11, rdx
    mov rdx, rcx
    ; jump enumerate_
    jmp enumerate_

nsoln_:
    ; new a1: List[List[i64]] = (a0)\{ ... \};
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
    je lab65662
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65663

lab65662:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65660
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65653
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65651
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65652

lab65651:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65652:

lab65653:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65656
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65654
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65655

lab65654:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65655:

lab65656:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65659
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65657
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65658

lab65657:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65658:

lab65659:
    jmp lab65661

lab65660:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65661:

lab65663:
    ; #load tag
    lea rdi, [rel List_List_i64_65664]
    ; substitute (n !-> n)(n0 !-> n)(a1 !-> a1);
    ; #move variables
    mov r9, rdi
    mov rdi, rdx
    mov r8, rsi
    ; jump gen_
    jmp gen_

List_List_i64_65664:
    jmp near List_List_i64_65664_Nil
    jmp near List_List_i64_65664_Cons

List_List_i64_65664_Nil:
    ; #load from memory
    ; ##check refcount
    cmp qword [rax + 0], 0
    je lab65666
    ; ##either decrement refcount and share children...
    add qword [rax + 0], -1
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]
    cmp rax, 0
    je lab65665
    ; ####increment refcount
    add qword [rax + 0], 1

lab65665:
    jmp lab65667

lab65666:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rax + 0], rbx
    mov rbx, rax
    ; ###load values
    mov rdx, [rax + 56]
    mov rax, [rax + 48]

lab65667:
    ; let x0: List[List[i64]] = Nil();
    ; #mark no allocation
    mov rsi, 0
    ; #load tag
    mov rdi, 0
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump length_
    jmp length_

List_List_i64_65664_Cons:
    ; #load from memory
    ; ##check refcount
    cmp qword [r8 + 0], 0
    je lab65669
    ; ##either decrement refcount and share children...
    add qword [r8 + 0], -1
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]
    cmp r8, 0
    je lab65668
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65668:
    jmp lab65670

lab65669:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [r8 + 0], rbx
    mov rbx, r8
    ; ###load values
    mov r9, [r8 + 56]
    mov r8, [r8 + 48]

lab65670:
    ; substitute (a0 !-> a0)(a2 !-> a2)(as0 !-> as0);
    ; #move variables
    mov rcx, r8
    mov r8, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, r9
    mov r9, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; let x0: List[List[i64]] = Cons(a2, as0);
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
    je lab65682
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65683

lab65682:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65680
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65673
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65671
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65672

lab65671:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65672:

lab65673:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65676
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65674
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65675

lab65674:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65675:

lab65676:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65679
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65677
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65678

lab65677:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65678:

lab65679:
    jmp lab65681

lab65680:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65681:

lab65683:
    ; #load tag
    mov rdi, 5
    ; substitute (x0 !-> x0)(a0 !-> a0);
    ; #move variables
    mov rcx, rsi
    mov rsi, rax
    mov rax, rcx
    mov rcx, rdi
    mov rdi, rdx
    mov rdx, rcx
    ; jump length_
    jmp length_

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
    je lab65695
    ; ####initialize refcount of just acquired block
    mov qword [rsi + 0], 0
    jmp lab65696

lab65695:
    ; ###(2) check non-linear lazy free list for next block
    mov rbx, rbp
    mov rbp, [rbp + 0]
    cmp rbp, 0
    je lab65693
    ; ####mark linear free list empty
    mov qword [rbx + 0], 0
    ; ####erase children of next block
    ; #####check child 1 for erasure
    mov rcx, [rbx + 16]
    cmp rcx, 0
    je lab65686
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65684
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65685

lab65684:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65685:

lab65686:
    ; #####check child 2 for erasure
    mov rcx, [rbx + 32]
    cmp rcx, 0
    je lab65689
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65687
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65688

lab65687:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65688:

lab65689:
    ; #####check child 3 for erasure
    mov rcx, [rbx + 48]
    cmp rcx, 0
    je lab65692
    ; ######check refcount
    cmp qword [rcx + 0], 0
    je lab65690
    ; ######either decrement refcount ...
    add qword [rcx + 0], -1
    jmp lab65691

lab65690:
    ; ######... or add block to lazy free list
    mov [rcx + 0], rbp
    mov rbp, rcx

lab65691:

lab65692:
    jmp lab65694

lab65693:
    ; ###(3) fall back to bump allocation
    mov rbp, rbx
    add rbp, 64

lab65694:

lab65696:
    ; #load tag
    lea rdi, [rel _Cont_65697]
    ; jump nsoln_
    jmp nsoln_

_Cont_65697:

_Cont_65697_Ret:
    ; #load from memory
    ; ##check refcount
    cmp qword [rsi + 0], 0
    je lab65699
    ; ##either decrement refcount and share children...
    add qword [rsi + 0], -1
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    cmp r8, 0
    je lab65698
    ; ####increment refcount
    add qword [r8 + 0], 1

lab65698:
    mov rdi, [rsi + 24]
    jmp lab65700

lab65699:
    ; ##... or release blocks onto linear free list when loading
    ; ###release block
    mov [rsi + 0], rbx
    mov rbx, rsi
    ; ###load values
    mov r11, [rsi + 56]
    mov r9, [rsi + 40]
    mov r8, [rsi + 32]
    mov rdi, [rsi + 24]

lab65700:
    ; lit x0 <- 1;
    mov r13, 1
    ; if iters == x0 \{ ... \}
    cmp r11, r13
    je lab65701
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

lab65701:
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