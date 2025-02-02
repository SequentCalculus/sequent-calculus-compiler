.text
.global asm_main0
.global asm_main1
.global asm_main2
.global asm_main3
.global asm_main4
.global asm_main5
.global asm_main6
.global asm_main7

asm_main0:

asm_main1:

asm_main2:

asm_main3:

asm_main4:

asm_main5:

asm_main6:

asm_main7:
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
    // lit f1 <- 3;
    MOVZ X4, 3, LSL 0
    // lit f2 <- 3;
    MOVZ X6, 3, LSL 0
    // lit f3 <- 3;
    MOVZ X8, 3, LSL 0
    // lit f4 <- 3;
    MOVZ X10, 3, LSL 0
    // lit f5 <- 3;
    MOVZ X12, 3, LSL 0
    // lit f6 <- 3;
    MOVZ X14, 3, LSL 0
    // lit f7 <- 3;
    MOVZ X16, 3, LSL 0
    // lit x <- 3;
    MOVZ X19, 3, LSL 0
    // leta b: Box = B(x);
    // #allocate memory
    // ##store values
    STR X19, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X17, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X17, 0 ]
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
    LDR X19, [ X0, 48 ]
    CMP X19, 0
    BEQ lab3
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab1
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab2

lab1:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab2:

lab3:
    // #####check child 2 for erasure
    LDR X19, [ X0, 32 ]
    CMP X19, 0
    BEQ lab6
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab4
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab5

lab4:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab5:

lab6:
    // #####check child 1 for erasure
    LDR X19, [ X0, 16 ]
    CMP X19, 0
    BEQ lab9
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab7
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab8

lab7:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab8:

lab9:
    B lab11

lab10:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab11:

lab13:
    // #load tag
    MOVZ X19, 0, LSL 0
    // leta bb: BoxBox = BB(b);
    // #allocate memory
    // ##store values
    STR X19, [ X0, 56 ]
    STR X17, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X17, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab25
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X17, 0 ]
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
    LDR X19, [ X0, 48 ]
    CMP X19, 0
    BEQ lab16
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab14
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab15

lab14:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab15:

lab16:
    // #####check child 2 for erasure
    LDR X19, [ X0, 32 ]
    CMP X19, 0
    BEQ lab19
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab17
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab18

lab17:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab18:

lab19:
    // #####check child 1 for erasure
    LDR X19, [ X0, 16 ]
    CMP X19, 0
    BEQ lab22
    // ######check refcount
    LDR X2, [ X19, 0 ]
    CMP X2, 0
    BEQ lab20
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X19, 0 ]
    B lab21

lab20:
    // ######... or add block to lazy free list
    STR X1, [ X19, 0 ]
    MOV X1, X19

lab21:

lab22:
    B lab24

lab23:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab24:

lab26:
    // #load tag
    MOVZ X19, 0, LSL 0
    // substitute (f1 !-> f1)(f2 !-> f2)(f3 !-> f3)(f5 !-> f5)(f6 !-> f6)(f7 !-> f7)(f4 !-> f4)(bb3 !-> bb)(bb2 !-> bb)(bb1 !-> bb);
    // #share bb
    CMP X17, 0
    BEQ lab27
    // ####increment refcount
    LDR X2, [ X17, 0 ]
    ADD X2, X2, 2
    STR X2, [ X17, 0 ]

lab27:
    // #move variables
    MOV X2, X12
    MOV X12, X14
    MOV X14, X16
    MOV X16, X10
    MOV X10, X2
    MOV X20, X17
    MOV X22, X17
    MOV X21, X19
    MOV X23, X19
    // switch bb1 \{ ... \};
    ADR X2, BoxBox28
    BR X2

BoxBox28:

BoxBox28BB:
    // #load from memory
    LDR X2, [ X22, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab31
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X22, 0 ]
    // ###load values
    LDR X23, [ X22, 56 ]
    LDR X22, [ X22, 48 ]
    CMP X22, 0
    BEQ lab30
    // ####increment refcount
    LDR X2, [ X22, 0 ]
    ADD X2, X2, 1
    STR X2, [ X22, 0 ]

lab30:
    B lab32

lab31:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X22, 0 ]
    MOV X0, X22
    // ###load values
    LDR X23, [ X22, 56 ]
    LDR X22, [ X22, 48 ]

lab32:
    // switch b1 \{ ... \};
    ADR X2, Box33
    BR X2

Box33:

Box33B:
    // #load from memory
    LDR X2, [ X22, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab34
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X22, 0 ]
    // ###load values
    LDR X23, [ X22, 56 ]
    B lab35

lab34:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X22, 0 ]
    MOV X0, X22
    // ###load values
    LDR X23, [ X22, 56 ]

lab35:
    // leta d1: Box = B(x1);
    // #allocate memory
    // ##store values
    STR X23, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X22, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab47
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X22, 0 ]
    B lab48

lab47:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab45
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X23, [ X0, 48 ]
    CMP X23, 0
    BEQ lab38
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab36
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab37

lab36:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab37:

lab38:
    // #####check child 2 for erasure
    LDR X23, [ X0, 32 ]
    CMP X23, 0
    BEQ lab41
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab39
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab40

lab39:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab40:

lab41:
    // #####check child 1 for erasure
    LDR X23, [ X0, 16 ]
    CMP X23, 0
    BEQ lab44
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab42
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab43

lab42:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab43:

lab44:
    B lab46

lab45:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab46:

lab48:
    // #load tag
    MOVZ X23, 0, LSL 0
    // leta dd1: BoxBox = BB(d1);
    // #allocate memory
    // ##store values
    STR X23, [ X0, 56 ]
    STR X22, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X22, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab60
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X22, 0 ]
    B lab61

lab60:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab58
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X23, [ X0, 48 ]
    CMP X23, 0
    BEQ lab51
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab49
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab50

lab49:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab50:

lab51:
    // #####check child 2 for erasure
    LDR X23, [ X0, 32 ]
    CMP X23, 0
    BEQ lab54
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab52
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab53

lab52:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab53:

lab54:
    // #####check child 1 for erasure
    LDR X23, [ X0, 16 ]
    CMP X23, 0
    BEQ lab57
    // ######check refcount
    LDR X2, [ X23, 0 ]
    CMP X2, 0
    BEQ lab55
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X23, 0 ]
    B lab56

lab55:
    // ######... or add block to lazy free list
    STR X1, [ X23, 0 ]
    MOV X1, X23

lab56:

lab57:
    B lab59

lab58:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab59:

lab61:
    // #load tag
    MOVZ X23, 0, LSL 0
    // substitute (bb2 !-> bb2);
    // #erase dd1
    CMP X22, 0
    BEQ lab64
    // ######check refcount
    LDR X2, [ X22, 0 ]
    CMP X2, 0
    BEQ lab62
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X22, 0 ]
    B lab63

lab62:
    // ######... or add block to lazy free list
    STR X1, [ X22, 0 ]
    MOV X1, X22

lab63:

lab64:
    // #erase bb3
    CMP X17, 0
    BEQ lab67
    // ######check refcount
    LDR X2, [ X17, 0 ]
    CMP X2, 0
    BEQ lab65
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X17, 0 ]
    B lab66

lab65:
    // ######... or add block to lazy free list
    STR X1, [ X17, 0 ]
    MOV X1, X17

lab66:

lab67:
    // #move variables
    MOV X3, X20
    MOV X4, X21
    // lit y <- 4;
    MOVZ X6, 4, LSL 0
    // leta a1: Box = B(y);
    // #allocate memory
    // ##store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X5, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab79
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab80

lab79:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab77
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab70
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab68
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab69

lab68:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab69:

lab70:
    // #####check child 2 for erasure
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab73
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab71
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab72

lab71:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab72:

lab73:
    // #####check child 1 for erasure
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab76
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab74
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab75

lab74:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab75:

lab76:
    B lab78

lab77:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab78:

lab80:
    // #load tag
    MOVZ X6, 0, LSL 0
    // substitute (a1 !-> a1)(bb2 !-> bb2);
    // #move variables
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // switch bb2 \{ ... \};
    ADR X2, BoxBox81
    BR X2

BoxBox81:

BoxBox81BB:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab84
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]
    CMP X5, 0
    BEQ lab83
    // ####increment refcount
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab83:
    B lab85

lab84:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]

lab85:
    // switch b2 \{ ... \};
    ADR X2, Box86
    BR X2

Box86:

Box86B:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab87
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X6, [ X5, 56 ]
    B lab88

lab87:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X6, [ X5, 56 ]

lab88:
    // leta a2: Box = B(x2);
    // #allocate memory
    // ##store values
    STR X6, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X5, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab100
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab101

lab100:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab98
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab91
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab89
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab90

lab89:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab90:

lab91:
    // #####check child 2 for erasure
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab94
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab92
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab93

lab92:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab93:

lab94:
    // #####check child 1 for erasure
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab97
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab95
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab96

lab95:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab96:

lab97:
    B lab99

lab98:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab99:

lab101:
    // #load tag
    MOVZ X6, 0, LSL 0
    // switch a2 \{ ... \};
    ADR X2, Box102
    BR X2

Box102:

Box102B:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab103
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X6, [ X5, 56 ]
    B lab104

lab103:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X6, [ X5, 56 ]

lab104:
    // substitute (x2 !-> x2)(a1 !-> a1);
    // #move variables
    MOV X5, X3
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // switch a1 \{ ... \};
    ADR X2, Box105
    BR X2

Box105:

Box105B:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab106
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X6, [ X5, 56 ]
    B lab107

lab106:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X6, [ X5, 56 ]

lab107:
    // res <- x1 + x2;
    ADD X8, X6, X4
    // println_i64 res;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X4
    MOV X22, X6
    MOV X23, X8
    // #move argument into place
    MOV X0, X8
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X4, X21
    MOV X6, X22
    MOV X8, X23
    // lit ret <- 0;
    MOVZ X10, 0, LSL 0
    // return ret
    MOV X0, X10
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