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
    // new t: ContInt = ()\{ ... \};
    // #mark no allocation
    MOVZ X3, 0, LSL 0
    // #load tag
    ADR X4, ContInt3
    // new k: ContList = (t)\{ ... \};
    // #allocate memory
    // ##store values
    STR X4, [ X0, 56 ]
    STR X3, [ X0, 48 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X3, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab15
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
    B lab16

lab15:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab13
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X4, [ X0, 48 ]
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
    // #####check child 2 for erasure
    LDR X4, [ X0, 32 ]
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
    // #####check child 1 for erasure
    LDR X4, [ X0, 16 ]
    CMP X4, 0
    BEQ lab12
    // ######check refcount
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab10
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab11

lab10:
    // ######... or add block to lazy free list
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab11:

lab12:
    B lab14

lab13:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab14:

lab16:
    // #load tag
    ADR X4, ContList17
    // leta zs: List = Nil();
    // #mark no allocation
    MOVZ X5, 0, LSL 0
    // #load tag
    MOVZ X6, 0, LSL 0
    // lit n <- 3;
    MOVZ X8, 3, LSL 0
    // substitute (k !-> k)(zs !-> zs)(n !-> n);
    // jump range
    B range

ContList17:

ContList17Retl:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab20
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]
    CMP X5, 0
    BEQ lab19
    // ####increment refcount
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab19:
    B lab21

lab20:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]

lab21:
    // substitute (t !-> t)(as !-> as);
    // #move variables
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // jump sum
    B sum

ContInt3:

ContInt3Reti:
    // return r
    MOV X0, X4
    B cleanup

range:
    // ifz i \{ ... \}
    CMP X8, 0
    BEQ lab22
    // substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
    // #move variables
    MOV X7, X5
    MOV X5, X3
    MOV X2, X8
    MOV X10, X8
    MOV X8, X6
    MOV X6, X4
    MOV X4, X2
    // leta ys: List = Cons(xs, i);
    // #allocate memory
    // ##store values
    STR X10, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X8, [ X0, 40 ]
    STR X7, [ X0, 32 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X7, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab34
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X7, 0 ]
    B lab35

lab34:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab32
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X8, [ X0, 48 ]
    CMP X8, 0
    BEQ lab25
    // ######check refcount
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab23
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab24

lab23:
    // ######... or add block to lazy free list
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab24:

lab25:
    // #####check child 2 for erasure
    LDR X8, [ X0, 32 ]
    CMP X8, 0
    BEQ lab28
    // ######check refcount
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab26
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab27

lab26:
    // ######... or add block to lazy free list
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab27:

lab28:
    // #####check child 1 for erasure
    LDR X8, [ X0, 16 ]
    CMP X8, 0
    BEQ lab31
    // ######check refcount
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab29
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab30

lab29:
    // ######... or add block to lazy free list
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab30:

lab31:
    B lab33

lab32:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab33:

lab35:
    // #load tag
    MOVZ X8, 4, LSL 0
    // lit o <- -1;
    MOVN X10, 0, LSL 0
    // j <- n + o;
    ADD X12, X4, X10
    // substitute (k !-> k)(ys !-> ys)(j !-> j);
    // #move variables
    MOV X3, X5
    MOV X4, X6
    MOV X5, X7
    MOV X6, X8
    MOV X8, X12
    // jump range
    B range

lab22:
    // substitute (xs !-> xs)(k !-> k);
    // #move variables
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // invoke k Retl
    BR X6

sum:
    // switch xs \{ ... \};
    ADR X2, List36
    ADD X2, X2, X6
    BR X2

List36:
    B List36Nil
    B List36Cons

List36Nil:
    // lit z <- 0;
    MOVZ X6, 0, LSL 0
    // substitute (z !-> z)(k !-> k);
    // #move variables
    MOV X5, X3
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // invoke k Reti
    BR X6

List36Cons:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab39
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]
    CMP X5, 0
    BEQ lab38
    // ####increment refcount
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab38:
    B lab40

lab39:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]

lab40:
    // substitute (ys !-> ys)(k !-> k)(y !-> y);
    // #move variables
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // new j: ContInt = (k, y)\{ ... \};
    // #allocate memory
    // ##store values
    STR X8, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X6, [ X0, 40 ]
    STR X5, [ X0, 32 ]
    // ##mark unused fields with null
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    // ##acquire free block from heap register
    MOV X5, X0
    // ##get next free block into heap register
    // ###(1) check linear free list for next block
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab52
    // ####initialize refcount of just acquired block
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab53

lab52:
    // ###(2) check non-linear lazy free list for next block
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab50
    // ####mark linear free list empty
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    // ####erase children of next block
    // #####check child 3 for erasure
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab43
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab41
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab42

lab41:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab42:

lab43:
    // #####check child 2 for erasure
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab46
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab44
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab45

lab44:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab45:

lab46:
    // #####check child 1 for erasure
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab49
    // ######check refcount
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab47
    // ######either decrement refcount ...
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab48

lab47:
    // ######... or add block to lazy free list
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab48:

lab49:
    B lab51

lab50:
    // ###(3) fall back to bump allocation
    ADD X1, X0, 64

lab51:

lab53:
    // #load tag
    ADR X6, ContInt54
    // substitute (j !-> j)(ys !-> ys);
    // #move variables
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // jump sum
    B sum

ContInt54:

ContInt54Reti:
    // #load from memory
    LDR X2, [ X5, 0 ]
    // ##check refcount
    CMP X2, 0
    BEQ lab57
    // ##either decrement refcount and share children...
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    // ###load values
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]
    CMP X5, 0
    BEQ lab56
    // ####increment refcount
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab56:
    B lab58

lab57:
    // ##... or release blocks onto linear free list when loading
    // ###release block
    STR X0, [ X5, 0 ]
    MOV X0, X5
    // ###load values
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]

lab58:
    // s <- y + r;
    ADD X10, X8, X4
    // substitute (s !-> s)(k !-> k);
    // #move variables
    MOV X4, X10
    // invoke k Reti
    BR X6

cleanup:
    // restore registers
    LDP X29, X30, [ SP ], 16
    LDP X27, X28, [ SP ], 16
    LDP X25, X26, [ SP ], 16
    LDP X23, X24, [ SP ], 16
    LDP X21, X22, [ SP ], 16
    LDP X19, X20, [ SP ], 16
    RET