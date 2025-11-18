.text
.global asm_main

asm_main:
    // setup
    // save registers
    STP X19, X20, [ SP, -16 ]!
    STP X21, X22, [ SP, -16 ]!
    STP X23, X24, [ SP, -16 ]!
    STP X25, X26, [ SP, -16 ]!
    STP X27, X28, [ SP, -16 ]!
    STP X29, X30, [ SP, -16 ]!
    // reserve space for register spills
    SUB SP, SP, 2048
    // move parameters into place
    // initialize free pointer
    MOV X1, X0
    ADD X1, X1, 64
    // actual code

main_:
    // lit z <- 8;
    MOVZ X5, 8, LSL 0
    // lit y <- 6;
    MOVZ X7, 6, LSL 0
    // lit x <- 4;
    MOVZ X9, 4, LSL 0
    // lit w <- 2;
    MOVZ X11, 2, LSL 0
    // let q: Quad = Q(z: ext i64, y: ext i64, x: ext i64, w: ext i64);
    // #allocate memory
    // ##store values
    STR X11, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X9, [ X0, 40 ]
    STR XZR, [ X0, 32 ]
    STR X7, [ X0, 24 ]
    STR XZR, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X6, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    STR XZR, [ X6, 0 ]
    B lab13

lab12:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab10
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab3
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab1
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab2

lab1:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab2:

lab3:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab6
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab4
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab5

lab4:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab5:

lab6:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab9
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab7
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab8

lab7:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab8:

lab9:
    B lab11

lab10:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab11:

lab13:
    // ##store link to previous block
    STR X6, [ X0, 48 ]
    // ##store values
    STR X5, [ X0, 40 ]
    STR XZR, [ X0, 32 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X4, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab25
    // ####initialize refcount of just acquired block
    STR XZR, [ X4, 0 ]
    B lab26

lab25:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab23
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab16
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab14
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab15

lab14:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab15:

lab16:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab19
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab17
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab18

lab17:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab18:

lab19:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab22
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab20
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab21

lab20:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab21:

lab22:
    B lab24

lab23:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab24:

lab26:
    // #load tag
    MOVZ X5, 0, LSL 0
    // switch q \{ ... \};
    // #there is only one clause, so we can just fall through

Quad_27:

Quad_27_Q:
    // #load from memory
    LDR X3, [ X4, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab28
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X4, 0 ]
    // ###load link to next block
    LDR X6, [ X4, 48 ]
    // ###load values
    LDR X5, [ X4, 40 ]
    // ###load values
    LDR X11, [ X6, 56 ]
    LDR X9, [ X6, 40 ]
    LDR X7, [ X6, 24 ]
    B lab29

lab28:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X4, 0 ]
    MOV X0, X4
    // ###load link to next block
    LDR X6, [ X4, 48 ]
    // ###load values
    LDR X5, [ X4, 40 ]
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X11, [ X6, 56 ]
    LDR X9, [ X6, 40 ]
    LDR X7, [ X6, 24 ]

lab29:
    // lit z <- 7;
    MOVZ X13, 7, LSL 0
    // e <- d + z;
    ADD X15, X5, X13
    // println_i64 e;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X5
    MOV X22, X7
    MOV X23, X9
    MOV X24, X11
    MOV X25, X13
    MOV X26, X15
    // #move argument into place
    MOV X0, X15
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X5, X21
    MOV X7, X22
    MOV X9, X23
    MOV X11, X24
    MOV X13, X25
    MOV X15, X26
    // lit ret <- 0;
    MOVZ X17, 0, LSL 0
    // exit ret
    MOV X0, X17
    B cleanup

cleanup:
    // free space for register spills
    ADD SP, SP, 2048
    // restore registers
    LDP X29, X30, [ SP ], 16
    LDP X27, X28, [ SP ], 16
    LDP X25, X26, [ SP ], 16
    LDP X23, X24, [ SP ], 16
    LDP X21, X22, [ SP ], 16
    LDP X19, X20, [ SP ], 16
    RET