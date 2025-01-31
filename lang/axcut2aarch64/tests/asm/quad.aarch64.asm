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
    STP X19, X20, [ SP, -16 ]!
    STP X21, X22, [ SP, -16 ]!
    STP X23, X24, [ SP, -16 ]!
    STP X25, X26, [ SP, -16 ]!
    STP X27, X28, [ SP, -16 ]!
    STP X29, X30, [ SP, -16 ]!
    // move parameters into place
    // initialize free pointer
    MOV X1, X0
    ADD X1, X1, 64
    // actual code

main:
    // lit z <- 8;
    MOVZ X4, 8, LSL 0
    // lit y <- 6;
    MOVZ X6, 6, LSL 0
    // lit x <- 4;
    MOVZ X8, 4, LSL 0
    // lit w <- 2;
    MOVZ X10, 2, LSL 0
    // leta q: Quad = Q(z, y, x, w);
    // #allocate memory
    // ##store values
    STR X10, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X8, [ X0, 40 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    STR X6, [ X0, 24 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X5, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab13

lab12:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab10
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab3
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab1
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab2

lab1:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab2:

lab3:
    // #####check child 2 for erasure
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab6
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab4
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab5

lab4:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab5:

lab6:
    // #####check child 1 for erasure
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab9
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab7
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab8

lab7:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab8:

lab9:
    B lab11

lab10:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab11:

lab13:
    // ##store link to previous block
    STR X5, [ X0, 48 ]
    // ##store values
    STR X4, [ X0, 40 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X3, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab25
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
    B lab26

lab25:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab23
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X4, [ X0, 48 ]
    CMP X4, 0
    BEQ lab16
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab14
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab15

lab14:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab15:

lab16:
    // #####check child 2 for erasure
    LDR X4, [ X0, 32 ]
    CMP X4, 0
    BEQ lab19
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab17
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab18

lab17:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab18:

lab19:
    // #####check child 1 for erasure
    LDR X4, [ X0, 16 ]
    CMP X4, 0
    BEQ lab22
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab20
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab21

lab20:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab21:

lab22:
    B lab24

lab23:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab24:

lab26:
    // #load tag
    MOVZ X4, 0, LSL 0
    // switch q \{ ... \};
    ADR X2, Quad27
    BR X2

Quad27:

Quad27Q:
    // #load from memory
    LDR X2, [ X3, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab28
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X3, 0 ]
    // ###load link to next block
    LDR X5, [ X3, 48 ]
    // ###load values
    LDR X4, [ X3, 40 ]
    // ###load values
    LDR X10, [ X5, 56 ]
    LDR X8, [ X5, 40 ]
    LDR X6, [ X5, 24 ]
    B lab29

lab28:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X3, 0 ]
    MOV X0, X3
    // ###load link to next block
    LDR X5, [ X3, 48 ]
    // ###load values
    LDR X4, [ X3, 40 ]
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X10, [ X5, 56 ]
    LDR X8, [ X5, 40 ]
    LDR X6, [ X5, 24 ]

lab29:
    // lit z <- 7;
    MOVZ X12, 7, LSL 0
    // e <- d + z;
    ADD X14, X4, X12
    // return e
    MOV X0, X14
    B cleanup

cleanup:
    // restore registers
    LDP X29, X30, [ SP ], 16
    LDP X27, X28, [ SP ], 16
    LDP X25, X26, [ SP ], 16
    LDP X23, X24, [ SP ], 16
    LDP X21, X22, [ SP ], 16
    LDP X19, X20, [ SP ], 16
    RET