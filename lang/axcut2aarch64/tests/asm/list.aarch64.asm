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
    // leta ws: List = Nil();
    // #mark no allocation
    MOVZ X3, 0, LSL 0
    // #load tag
    MOVZ X4, 0, LSL 0
    // lit z <- 5;
    MOVZ X6, 5, LSL 0
    // leta zs: List = Cons(z, ws);
    // #allocate memory
    // ##store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X4, [ X0, 40 ]
    STR X3, [ X0, 32 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X3, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
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
    LDR X4, [ X0, 48 ]
    CMP X4, 0
    BEQ lab3
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab1
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab2

lab1:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab2:

lab3:
    // #####check child 2 for erasure
    LDR X4, [ X0, 32 ]
    CMP X4, 0
    BEQ lab6
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab4
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab5

lab4:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab5:

lab6:
    // #####check child 1 for erasure
    LDR X4, [ X0, 16 ]
    CMP X4, 0
    BEQ lab9
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab7
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab8

lab7:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab8:

lab9:
    B lab11

lab10:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab11:

lab13:
    // #load tag
    MOVZ X4, 4, LSL 0
    // lit y <- 7;
    MOVZ X6, 7, LSL 0
    // leta ys: List = Cons(y, zs);
    // #allocate memory
    // ##store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X4, [ X0, 40 ]
    STR X3, [ X0, 32 ]
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
    MOVZ X4, 4, LSL 0
    // lit x <- 9;
    MOVZ X6, 9, LSL 0
    // leta xs: List = Cons(x, ys);
    // #allocate memory
    // ##store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X4, [ X0, 40 ]
    STR X3, [ X0, 32 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X3, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab38
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
    B lab39

lab38:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab36
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X4, [ X0, 48 ]
    CMP X4, 0
    BEQ lab29
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab27
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab28

lab27:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab28:

lab29:
    // #####check child 2 for erasure
    LDR X4, [ X0, 32 ]
    CMP X4, 0
    BEQ lab32
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab30
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab31

lab30:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab31:

lab32:
    // #####check child 1 for erasure
    LDR X4, [ X0, 16 ]
    CMP X4, 0
    BEQ lab35
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab33
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab34

lab33:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab34:

lab35:
    B lab37

lab36:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab37:

lab39:
    // #load tag
    MOVZ X4, 4, LSL 0
    // switch xs \{ ... \};
    ADR X2, List40
    ADD X2, X2, X4
    BR X2

List40:
    B List40Nil
    B List40Cons

List40Nil:
    // Done
    B cleanup

List40Cons:
    // #load from memory
    LDR X2, [ X3, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab43
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X3, 0 ]
    // ###load values
    LDR X6, [ X3, 56 ]
    LDR X4, [ X3, 40 ]
    LDR X3, [ X3, 32 ]
    CMP X3, 0
    BEQ lab42
    // ####increment refcount
    LDR X2, [ X3, 0 ]
    ADD X2, X2, 1
    STR X2, [ X3, 0 ]

lab42:
    B lab44

lab43:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X3, 0 ]
    MOV X0, X3
    // ###load values
    LDR X6, [ X3, 56 ]
    LDR X4, [ X3, 40 ]
    LDR X3, [ X3, 32 ]

lab44:
    // return a
    MOV X0, X6
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