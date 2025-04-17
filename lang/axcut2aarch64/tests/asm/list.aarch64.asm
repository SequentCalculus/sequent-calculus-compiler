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
    // let ws: List = Nil();
    // #mark no allocation
    MOVZ X4, 0, LSL 0
    // #load tag
    MOVZ X5, 0, LSL 0
    // lit z <- 5;
    MOVZ X7, 5, LSL 0
    // let zs: List = Cons(z, ws);
    // #allocate memory
    // ##store values
    STR X7, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X5, [ X0, 40 ]
    STR X4, [ X0, 32 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
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
    MOVZ X5, 4, LSL 0
    // lit y <- 7;
    MOVZ X7, 7, LSL 0
    // let ys: List = Cons(y, zs);
    // #allocate memory
    // ##store values
    STR X7, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X5, [ X0, 40 ]
    STR X4, [ X0, 32 ]
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
    MOVZ X5, 4, LSL 0
    // lit x <- 9;
    MOVZ X7, 9, LSL 0
    // let xs: List = Cons(x, ys);
    // #allocate memory
    // ##store values
    STR X7, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X5, [ X0, 40 ]
    STR X4, [ X0, 32 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X4, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab38
    // ####initialize refcount of just acquired block
    STR XZR, [ X4, 0 ]
    B lab39

lab38:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab36
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab29
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab27
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab28

lab27:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab28:

lab29:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab32
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab30
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab31

lab30:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab31:

lab32:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab35
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab33
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab34

lab33:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab34:

lab35:
    B lab37

lab36:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab37:

lab39:
    // #load tag
    MOVZ X5, 4, LSL 0
    // switch xs \{ ... \};
    ADR X2, List_40
    ADD X2, X2, X5
    BR X2

List_40:
    B List_40_Nil
    B List_40_Cons

List_40_Nil:
    // Done
    B cleanup

List_40_Cons:
    // #load from memory
    LDR X3, [ X4, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab42
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X4, 0 ]
    // ###load values
    LDR X7, [ X4, 56 ]
    LDR X5, [ X4, 40 ]
    LDR X4, [ X4, 32 ]
    CMP X4, 0
    BEQ lab41
    // ####increment refcount
    LDR X3, [ X4, 0 ]
    ADD X3, X3, 1
    STR X3, [ X4, 0 ]

lab41:
    B lab43

lab42:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X4, 0 ]
    MOV X0, X4
    // ###load values
    LDR X7, [ X4, 56 ]
    LDR X5, [ X4, 40 ]
    LDR X4, [ X4, 32 ]

lab43:
    // println_i64 a;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X3
    MOV X22, X4
    MOV X23, X6
    // #move argument into place
    MOV X0, X7
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X3, X21
    MOV X4, X22
    MOV X6, X23
    // lit ret <- 0;
    MOVZ X9, 0, LSL 0
    // return ret
    MOV X0, X9
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