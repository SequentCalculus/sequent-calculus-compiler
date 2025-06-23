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
    // lit a <- 9;
    MOVZ X5, 9, LSL 0
    // create f: Fun = (a)\{ ... \};
    // #allocate memory
    // ##store values
    STR X5, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X4, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    STR XZR, [ X4, 0 ]
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
    // #load tag
    ADR X5, Fun_14
    // create k: Cont = ()\{ ... \};
    // #mark no allocation
    MOVZ X6, 0, LSL 0
    // #load tag
    ADR X7, Cont_15
    // lit y <- 1;
    MOVZ X9, 1, LSL 0
    // substitute (y := y)(k := k)(f := f);
    // #move variables
    MOV X8, X4
    MOV X2, X9
    MOV X9, X5
    MOV X5, X2
    // invoke f Apply
    BR X9

Cont_15:

Cont_15_Ret:
    // println_i64 r;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X5
    // #move argument into place
    MOV X0, X5
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X5, X21
    // lit ret <- 0;
    MOVZ X7, 0, LSL 0
    // exit ret
    MOV X0, X7
    B cleanup

Fun_14:

Fun_14_Apply:
    // #load from memory
    LDR X3, [ X8, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab16
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X8, 0 ]
    // ###load values
    LDR X9, [ X8, 56 ]
    B lab17

lab16:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X8, 0 ]
    MOV X0, X8
    // ###load values
    LDR X9, [ X8, 56 ]

lab17:
    // b <- a + x;
    ADD X11, X9, X5
    // substitute (b := b)(k := k);
    // #move variables
    MOV X5, X11
    // invoke k Ret
    BR X7

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