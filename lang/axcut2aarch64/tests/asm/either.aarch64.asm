.text
.global asm_main0
.global _asm_main0
.global asm_main1
.global _asm_main1
.global asm_main2
.global _asm_main2
.global asm_main3
.global _asm_main3
.global asm_main4
.global _asm_main4
.global asm_main5
.global _asm_main5
.global asm_main6
.global _asm_main6
.global asm_main7
.global _asm_main7

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

asm_main6:

_asm_main6:

asm_main7:

_asm_main7:
    // setup
    // save registers
    STR X16, [ SP, -16 ]!
    STR X17, [ SP, -16 ]!
    STR X18, [ SP, -16 ]!
    STR X19, [ SP, -16 ]!
    STR X20, [ SP, -16 ]!
    STR X21, [ SP, -16 ]!
    STR X22, [ SP, -16 ]!
    STR X23, [ SP, -16 ]!
    STR X24, [ SP, -16 ]!
    STR X25, [ SP, -16 ]!
    STR X26, [ SP, -16 ]!
    STR X27, [ SP, -16 ]!
    STR X28, [ SP, -16 ]!
    STR X29, [ SP, -16 ]!
    STR X30, [ SP, -16 ]!
    // move parameters into place
    // initialize free pointer
    MOV X1, X0
    ADD X1, X1, 64
    // actual code

main:
    // lit z <- 1;
    MOVZ X4, 1, LSL 0
    // lit x <- 9;
    MOVZ X6, 9, LSL 0
    // leta p: Either = Right(x);
    //  allocate memory
    //   store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    //   mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    //   acquire free block from heap register
    MOV X5, X0
    //   get next free block into heap register
    //    (1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    //     initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab13

lab12:
    //    (2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab10
    //     mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    //     erase children of next block
    //      check child 3 for erasure
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab3
    //       check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab1
    //       either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab2

lab1:
    //       ... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab2:

lab3:
    //      check child 2 for erasure
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab6
    //       check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab4
    //       either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab5

lab4:
    //       ... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab5:

lab6:
    //      check child 1 for erasure
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab9
    //       check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab7
    //       either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab8

lab7:
    //       ... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab8:

lab9:
    B lab11

lab10:
    //    (3) fall back to bump allocation
    ADD X1, X0, 64

lab11:

lab13:
    //  load tag
    MOVZ X6, 4, LSL 0
    // switch p \{ ... \};
    ADR X2, Either14
    ADD X2, X2, X6
    BR X2

Either14:
    B Either14Left
    B Either14Right

Either14Left:
    //  load from memory
    LDR X2, [ X5, 0 ]
    //   check refcount
    CMP X2, 0
    BEQ lab15
    //   either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    //    load values
    LDR X6, [ X5, 56 ]
    B lab16

lab15:
    //   ... or release blocks onto linear free list when loading
    //    release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    //    load values
    LDR X6, [ X5, 56 ]

lab16:
    // Done
    B cleanup

Either14Right:
    //  load from memory
    LDR X2, [ X5, 0 ]
    //   check refcount
    CMP X2, 0
    BEQ lab17
    //   either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    //    load values
    LDR X6, [ X5, 56 ]
    B lab18

lab17:
    //   ... or release blocks onto linear free list when loading
    //    release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    //    load values
    LDR X6, [ X5, 56 ]

lab18:
    // c <- b + z;
    ADD X8, X6, X4
    // return c
    MOV X1, X8
    B cleanup

cleanup:
    // restore registers
    LDR X30, [ SP ], 16
    LDR X29, [ SP ], 16
    LDR X28, [ SP ], 16
    LDR X27, [ SP ], 16
    LDR X26, [ SP ], 16
    LDR X25, [ SP ], 16
    LDR X24, [ SP ], 16
    LDR X23, [ SP ], 16
    LDR X22, [ SP ], 16
    LDR X21, [ SP ], 16
    LDR X20, [ SP ], 16
    LDR X19, [ SP ], 16
    LDR X18, [ SP ], 16
    LDR X17, [ SP ], 16
    LDR X16, [ SP ], 16
    RET