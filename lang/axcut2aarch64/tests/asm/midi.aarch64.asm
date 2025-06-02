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
    // create t: ContInt = ()\{ ... \};
    // #mark no allocation
    MOVZ X4, 0, LSL 0
    // #load tag
    ADR X5, ContInt_1
    // create k: ContList = (t)\{ ... \};
    // #allocate memory
    // ##store values
    STR X5, [ X0, 56 ]
    STR X4, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X4, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab13
    // ####initialize refcount of just acquired block
    STR XZR, [ X4, 0 ]
    B lab14

lab13:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab11
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab4
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab2
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab3

lab2:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab3:

lab4:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab7
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab5
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab6

lab5:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab6:

lab7:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab10
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab8
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab9

lab8:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab9:

lab10:
    B lab12

lab11:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab12:

lab14:
    // #load tag
    ADR X5, ContList_15
    // let zs: List = Nil();
    // #mark no allocation
    MOVZ X6, 0, LSL 0
    // #load tag
    MOVZ X7, 0, LSL 0
    // lit n <- 3;
    MOVZ X9, 3, LSL 0
    // substitute (k !-> k)(zs !-> zs)(n !-> n);
    // jump range_
    B range_

ContList_15:

ContList_15_Retl:
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
    LDR X6, [ X6, 48 ]
    CMP X6, 0
    BEQ lab16
    // ####increment refcount
    LDR X3, [ X6, 0 ]
    ADD X3, X3, 1
    STR X3, [ X6, 0 ]

lab16:
    B lab18

lab17:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]
    LDR X6, [ X6, 48 ]

lab18:
    // substitute (t !-> t)(as !-> as);
    // #move variables
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // jump sum_
    B sum_

ContInt_1:

ContInt_1_Reti:
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

range_:
    // if i == 0 \{ ... \}
    CMP X9, 0
    BEQ lab19
    // substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
    // #move variables
    MOV X8, X6
    MOV X6, X4
    MOV X2, X9
    MOV X11, X9
    MOV X9, X7
    MOV X7, X5
    MOV X5, X2
    // let ys: List = Cons(xs, i);
    // #allocate memory
    // ##store values
    STR X11, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X9, [ X0, 40 ]
    STR X8, [ X0, 32 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X8, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab31
    // ####initialize refcount of just acquired block
    STR XZR, [ X8, 0 ]
    B lab32

lab31:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab29
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
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
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab25
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab23
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab24

lab23:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab24:

lab25:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab28
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab26
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab27

lab26:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab27:

lab28:
    B lab30

lab29:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab30:

lab32:
    // #load tag
    MOVZ X9, 4, LSL 0
    // lit o <- -1;
    MOVN X11, 0, LSL 0
    // j <- n + o;
    ADD X13, X5, X11
    // substitute (k !-> k)(ys !-> ys)(j !-> j);
    // #move variables
    MOV X4, X6
    MOV X5, X7
    MOV X6, X8
    MOV X7, X9
    MOV X9, X13
    // jump range_
    B range_

lab19:
    // substitute (xs !-> xs)(k !-> k);
    // #move variables
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // invoke k Retl
    BR X7

sum_:
    // switch xs \{ ... \};
    ADR X2, List_33
    ADD X2, X2, X7
    BR X2

List_33:
    B List_33_Nil
    B List_33_Cons

List_33_Nil:
    // lit z <- 0;
    MOVZ X7, 0, LSL 0
    // substitute (z !-> z)(k !-> k);
    // #move variables
    MOV X6, X4
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // invoke k Reti
    BR X7

List_33_Cons:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab35
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X9, [ X6, 56 ]
    LDR X7, [ X6, 40 ]
    LDR X6, [ X6, 32 ]
    CMP X6, 0
    BEQ lab34
    // ####increment refcount
    LDR X3, [ X6, 0 ]
    ADD X3, X3, 1
    STR X3, [ X6, 0 ]

lab34:
    B lab36

lab35:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X9, [ X6, 56 ]
    LDR X7, [ X6, 40 ]
    LDR X6, [ X6, 32 ]

lab36:
    // substitute (ys !-> ys)(k !-> k)(y !-> y);
    // #move variables
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // create j: ContInt = (k, y)\{ ... \};
    // #allocate memory
    // ##store values
    STR X9, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    STR X7, [ X0, 40 ]
    STR X6, [ X0, 32 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X6, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab48
    // ####initialize refcount of just acquired block
    STR XZR, [ X6, 0 ]
    B lab49

lab48:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab46
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab39
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab37
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab38

lab37:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab38:

lab39:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab42
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab40
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab41

lab40:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab41:

lab42:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab45
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab43
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab44

lab43:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab44:

lab45:
    B lab47

lab46:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab47:

lab49:
    // #load tag
    ADR X7, ContInt_50
    // substitute (j !-> j)(ys !-> ys);
    // #move variables
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // jump sum_
    B sum_

ContInt_50:

ContInt_50_Reti:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab52
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X9, [ X6, 56 ]
    LDR X7, [ X6, 40 ]
    LDR X6, [ X6, 32 ]
    CMP X6, 0
    BEQ lab51
    // ####increment refcount
    LDR X3, [ X6, 0 ]
    ADD X3, X3, 1
    STR X3, [ X6, 0 ]

lab51:
    B lab53

lab52:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X9, [ X6, 56 ]
    LDR X7, [ X6, 40 ]
    LDR X6, [ X6, 32 ]

lab53:
    // s <- y + r;
    ADD X11, X9, X5
    // substitute (s !-> s)(k !-> k);
    // #move variables
    MOV X5, X11
    // invoke k Reti
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