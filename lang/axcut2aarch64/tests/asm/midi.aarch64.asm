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
    // new t: ContInt = ()\{ ... \};
    MOVZ X3, 0, LSL 0
    ADR X4, ContInt3
    // new k: ContList = (t)\{ ... \};
    STR X4, [ X0, 56 ]
    STR X3, [ X0, 48 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    MOV X3, X0
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab15
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
    B lab16

lab15:
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab13
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    LDR X4, [ X0, 48 ]
    CMP X4, 0
    BEQ lab6
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab4
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab5

lab4:
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab5:

lab6:
    LDR X4, [ X0, 32 ]
    CMP X4, 0
    BEQ lab9
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab7
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab8

lab7:
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab8:

lab9:
    LDR X4, [ X0, 16 ]
    CMP X4, 0
    BEQ lab12
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab10
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab11

lab10:
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab11:

lab12:
    B lab14

lab13:
    ADD X1, X0, 64

lab14:

lab16:
    ADR X4, ContList17
    // leta zs: List = Nil();
    MOVZ X5, 0, LSL 0
    MOVZ X6, 0, LSL 0
    // lit n <- 3;
    MOVZ X8, 3, LSL 0
    // substitute (k !-> k)(zs !-> zs)(n !-> n);
    // jump range
    B range

ContList17:

ContList17Retl:
    LDR X2, [ X5, 0 ]
    CMP X2, 0
    BEQ lab20
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]
    CMP X5, 0
    BEQ lab19
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab19:
    B lab21

lab20:
    STR X0, [ X5, 0 ]
    MOV X0, X5
    LDR X6, [ X5, 56 ]
    LDR X5, [ X5, 48 ]

lab21:
    // substitute (t !-> t)(as !-> as);
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
    MOV X1, X4
    B cleanup

range:
    // ifz i \{ ... \}
    CMP X8, 0
    BEQ lab22
    // substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
    MOV X7, X5
    MOV X5, X3
    MOV X2, X8
    MOV X10, X8
    MOV X8, X6
    MOV X6, X4
    MOV X4, X2
    // leta ys: List = Cons(xs, i);
    STR X10, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X8, [ X0, 40 ]
    STR X7, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    MOV X7, X0
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab34
    MOVZ X2, 0, LSL 0
    STR X2, [ X7, 0 ]
    B lab35

lab34:
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab32
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    LDR X8, [ X0, 48 ]
    CMP X8, 0
    BEQ lab25
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab23
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab24

lab23:
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab24:

lab25:
    LDR X8, [ X0, 32 ]
    CMP X8, 0
    BEQ lab28
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab26
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab27

lab26:
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab27:

lab28:
    LDR X8, [ X0, 16 ]
    CMP X8, 0
    BEQ lab31
    LDR X2, [ X8, 0 ]
    CMP X2, 0
    BEQ lab29
    SUB X2, X2, 1
    STR X2, [ X8, 0 ]
    B lab30

lab29:
    STR X1, [ X8, 0 ]
    MOV X1, X8

lab30:

lab31:
    B lab33

lab32:
    ADD X1, X0, 64

lab33:

lab35:
    MOVZ X8, 4, LSL 0
    // lit o <- -1;
    MOVN X10, 0, LSL 0
    // j <- n + o;
    ADD X12, X4, X10
    // substitute (k !-> k)(ys !-> ys)(j !-> j);
    MOV X3, X5
    MOV X4, X6
    MOV X5, X7
    MOV X6, X8
    MOV X8, X12
    // jump range
    B range

lab22:
    // substitute (xs !-> xs)(k !-> k);
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
    MOV X5, X3
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // invoke k Reti
    BR X6

List36Cons:
    LDR X2, [ X5, 0 ]
    CMP X2, 0
    BEQ lab39
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]
    CMP X5, 0
    BEQ lab38
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab38:
    B lab40

lab39:
    STR X0, [ X5, 0 ]
    MOV X0, X5
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]

lab40:
    // substitute (ys !-> ys)(k !-> k)(y !-> y);
    MOV X2, X5
    MOV X5, X3
    MOV X3, X2
    MOV X2, X6
    MOV X6, X4
    MOV X4, X2
    // new j: ContInt = (k, y)\{ ... \};
    STR X8, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    STR X6, [ X0, 40 ]
    STR X5, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    MOV X5, X0
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab52
    MOVZ X2, 0, LSL 0
    STR X2, [ X5, 0 ]
    B lab53

lab52:
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab50
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    LDR X6, [ X0, 48 ]
    CMP X6, 0
    BEQ lab43
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab41
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab42

lab41:
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab42:

lab43:
    LDR X6, [ X0, 32 ]
    CMP X6, 0
    BEQ lab46
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab44
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab45

lab44:
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab45:

lab46:
    LDR X6, [ X0, 16 ]
    CMP X6, 0
    BEQ lab49
    LDR X2, [ X6, 0 ]
    CMP X2, 0
    BEQ lab47
    SUB X2, X2, 1
    STR X2, [ X6, 0 ]
    B lab48

lab47:
    STR X1, [ X6, 0 ]
    MOV X1, X6

lab48:

lab49:
    B lab51

lab50:
    ADD X1, X0, 64

lab51:

lab53:
    ADR X6, ContInt54
    // substitute (j !-> j)(ys !-> ys);
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
    LDR X2, [ X5, 0 ]
    CMP X2, 0
    BEQ lab57
    SUB X2, X2, 1
    STR X2, [ X5, 0 ]
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]
    CMP X5, 0
    BEQ lab56
    LDR X2, [ X5, 0 ]
    ADD X2, X2, 1
    STR X2, [ X5, 0 ]

lab56:
    B lab58

lab57:
    STR X0, [ X5, 0 ]
    MOV X0, X5
    LDR X8, [ X5, 56 ]
    LDR X6, [ X5, 40 ]
    LDR X5, [ X5, 32 ]

lab58:
    // s <- y + r;
    ADD X10, X8, X4
    // substitute (s !-> s)(k !-> k);
    MOV X4, X10
    // invoke k Reti
    BR X6

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