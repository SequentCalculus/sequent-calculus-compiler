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
    // lit z <- 1;
    MOVZ X5, 1, LSL 0
    // lit x <- 9;
    MOVZ X7, 9, LSL 0
    // let p: Either = Right(x);
    // #allocate memory
    // ##store values
    STR X7, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
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
    // #load tag
    MOVZ X7, 4, LSL 0
    // switch p \{ ... \};
    ADR X2, Either_14
    ADD X2, X2, X7
    BR X2

Either_14:
    B Either_14_Left
    B Either_14_Right

Either_14_Left:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab15
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    B lab16

lab15:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]

lab16:
    // Done
    B cleanup

Either_14_Right:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab17
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    B lab18

lab17:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]

lab18:
    // c <- b + z;
    ADD X9, X7, X5
    // println_i64 c;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X5
    MOV X22, X7
    MOV X23, X9
    // #move argument into place
    MOV X0, X9
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X5, X21
    MOV X7, X22
    MOV X9, X23
    // lit ret <- 0;
    MOVZ X11, 0, LSL 0
    // exit ret
    MOV X0, X11
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