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
    // lit f1 <- 3;
    MOVZ X5, 3, LSL 0
    // lit f2 <- 3;
    MOVZ X7, 3, LSL 0
    // lit f3 <- 3;
    MOVZ X9, 3, LSL 0
    // lit f4 <- 3;
    MOVZ X11, 3, LSL 0
    // lit f5 <- 3;
    MOVZ X13, 3, LSL 0
    // lit f6 <- 3;
    MOVZ X15, 3, LSL 0
    // lit f7 <- 3;
    MOVZ X17, 3, LSL 0
    // lit x <- 3;
    MOVZ X20, 3, LSL 0
    // let b: Box = B(x);
    // #allocate memory
    // ##store values
    STR X20, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X19, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    STR XZR, [ X19, 0 ]
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
    MOVZ X20, 0, LSL 0
    // let bb: BoxBox = BB(b);
    // #allocate memory
    // ##store values
    STR X20, [ X0, 56 ]
    STR X19, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X19, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab25
    // ####initialize refcount of just acquired block
    STR XZR, [ X19, 0 ]
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
    MOVZ X20, 0, LSL 0
    // substitute (f1 !-> f1)(f2 !-> f2)(f3 !-> f3)(f5 !-> f5)(f6 !-> f6)(f7 !-> f7)(f4 !-> f4)(bb3 !-> bb)(bb2 !-> bb)(bb1 !-> bb);
    // #share bb
    CMP X19, 0
    BEQ lab27
    // ####increment refcount
    LDR X3, [ X19, 0 ]
    ADD X3, X3, 2
    STR X3, [ X19, 0 ]

lab27:
    // #move variables
    MOV X2, X13
    MOV X13, X15
    MOV X15, X17
    MOV X17, X11
    MOV X11, X2
    MOV X21, X19
    MOV X23, X19
    MOV X22, X20
    MOV X24, X20
    // switch bb1 \{ ... \};
    // #if there is only one clause, we can just fall through

BoxBox_28:

BoxBox_28_BB:
    // #load from memory
    LDR X3, [ X23, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab30
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X23, 0 ]
    // ###load values
    LDR X24, [ X23, 56 ]
    LDR X23, [ X23, 48 ]
    CMP X23, 0
    BEQ lab29
    // ####increment refcount
    LDR X3, [ X23, 0 ]
    ADD X3, X3, 1
    STR X3, [ X23, 0 ]

lab29:
    B lab31

lab30:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X23, 0 ]
    MOV X0, X23
    // ###load values
    LDR X24, [ X23, 56 ]
    LDR X23, [ X23, 48 ]

lab31:
    // switch b1 \{ ... \};
    // #if there is only one clause, we can just fall through

Box_32:

Box_32_B:
    // #load from memory
    LDR X3, [ X23, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab33
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X23, 0 ]
    // ###load values
    LDR X24, [ X23, 56 ]
    B lab34

lab33:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X23, 0 ]
    MOV X0, X23
    // ###load values
    LDR X24, [ X23, 56 ]

lab34:
    // let d1: Box = B(x1);
    // #allocate memory
    // ##store values
    STR X24, [ X0, 56 ]
    STR XZR, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X23, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab46
    // ####initialize refcount of just acquired block
    STR XZR, [ X23, 0 ]
    B lab47

lab46:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab44
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab37
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab35
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab36

lab35:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab36:

lab37:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab40
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab38
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab39

lab38:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab39:

lab40:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab43
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab41
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab42

lab41:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab42:

lab43:
    B lab45

lab44:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab45:

lab47:
    // #load tag
    MOVZ X24, 0, LSL 0
    // let dd1: BoxBox = BB(d1);
    // #allocate memory
    // ##store values
    STR X24, [ X0, 56 ]
    STR X23, [ X0, 48 ]
    // ##mark unused fields with null
    STR XZR, [ X0, 16 ]
    STR XZR, [ X0, 32 ]
    // ##acquire free block from heap register
    MOV X23, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab59
    // ####initialize refcount of just acquired block
    STR XZR, [ X23, 0 ]
    B lab60

lab59:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab57
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab50
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab48
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab49

lab48:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab49:

lab50:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab53
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab51
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab52

lab51:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab52:

lab53:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab56
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab54
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab55

lab54:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab55:

lab56:
    B lab58

lab57:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab58:

lab60:
    // #load tag
    MOVZ X24, 0, LSL 0
    // substitute (bb2 !-> bb2);
    // #erase bb3
    CMP X19, 0
    BEQ lab63
    // ######check refcount
    LDR X3, [ X19, 0 ]
    CMP X3, 0
    BEQ lab61
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X19, 0 ]
    B lab62

lab61:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab62:

lab63:
    // #erase dd1
    CMP X23, 0
    BEQ lab66
    // ######check refcount
    LDR X3, [ X23, 0 ]
    CMP X3, 0
    BEQ lab64
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X23, 0 ]
    B lab65

lab64:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab65:

lab66:
    // #move variables
    MOV X4, X21
    MOV X5, X22
    // lit y <- 4;
    MOVZ X7, 4, LSL 0
    // let a1: Box = B(y);
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
    BEQ lab78
    // ####initialize refcount of just acquired block
    STR XZR, [ X6, 0 ]
    B lab79

lab78:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab76
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab69
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab67
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab68

lab67:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab68:

lab69:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab72
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab70
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab71

lab70:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab71:

lab72:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab75
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab73
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab74

lab73:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab74:

lab75:
    B lab77

lab76:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab77:

lab79:
    // #load tag
    MOVZ X7, 0, LSL 0
    // substitute (a1 !-> a1)(bb2 !-> bb2);
    // #move variables
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // switch bb2 \{ ... \};
    // #if there is only one clause, we can just fall through

BoxBox_80:

BoxBox_80_BB:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab82
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    LDR X6, [ X6, 48 ]
    CMP X6, 0
    BEQ lab81
    // ####increment refcount
    LDR X3, [ X6, 0 ]
    ADD X3, X3, 1
    STR X3, [ X6, 0 ]

lab81:
    B lab83

lab82:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]
    LDR X6, [ X6, 48 ]

lab83:
    // switch b2 \{ ... \};
    // #if there is only one clause, we can just fall through

Box_84:

Box_84_B:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab85
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    B lab86

lab85:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]

lab86:
    // let a2: Box = B(x2);
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
    BEQ lab98
    // ####initialize refcount of just acquired block
    STR XZR, [ X6, 0 ]
    B lab99

lab98:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab96
    // ####mark linear free list empty
    STR XZR, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 1 for erasure
    LDR X2, [ X0, 16 ]
    CMP X2, 0
    BEQ lab89
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab87
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab88

lab87:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab88:

lab89:
    // #####check child 2 for erasure
    LDR X2, [ X0, 32 ]
    CMP X2, 0
    BEQ lab92
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab90
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab91

lab90:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab91:

lab92:
    // #####check child 3 for erasure
    LDR X2, [ X0, 48 ]
    CMP X2, 0
    BEQ lab95
    // ######check refcount
    LDR X3, [ X2, 0 ]
    CMP X3, 0
    BEQ lab93
    // ######either decrement refcount ...
    SUB X3, X3, 1
    STR X3, [ X2, 0 ]
    B lab94

lab93:
    // ######... or add block to lazy free list
    STR X1, [ X2, 0 ]
    MOV X1, X2

lab94:

lab95:
    B lab97

lab96:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab97:

lab99:
    // #load tag
    MOVZ X7, 0, LSL 0
    // switch a2 \{ ... \};
    // #if there is only one clause, we can just fall through

Box_100:

Box_100_B:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab101
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    B lab102

lab101:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]

lab102:
    // substitute (x2 !-> x2)(a1 !-> a1);
    // #move variables
    MOV X6, X4
    MOV X2, X7
    MOV X7, X5
    MOV X5, X2
    // switch a1 \{ ... \};
    // #if there is only one clause, we can just fall through

Box_103:

Box_103_B:
    // #load from memory
    LDR X3, [ X6, 0 ]
    // ##check refcount
    CMP X3, 0
    BEQ lab104
    // ##either decrement refcount and share children...
    SUB X3, X3, 1
    STR X3, [ X6, 0 ]
    // ###load values
    LDR X7, [ X6, 56 ]
    B lab105

lab104:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X6, 0 ]
    MOV X0, X6
    // ###load values
    LDR X7, [ X6, 56 ]

lab105:
    // res <- x1 + x2;
    ADD X9, X7, X5
    // println_i64 res;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X4
    MOV X22, X6
    MOV X23, X8
    // #move argument into place
    MOV X0, X9
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X4, X21
    MOV X6, X22
    MOV X8, X23
    // lit ret <- 0;
    MOVZ X11, 0, LSL 0
    // return ret
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