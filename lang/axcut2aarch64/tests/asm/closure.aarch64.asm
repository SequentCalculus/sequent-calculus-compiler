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
    // lit a <- 9;
    MOVZ X4, 9, LSL 0
    // new f: Func = (a)\{ ... \};
    STR X4, [ X0, 56 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 48 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 32 ]
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 16 ]
    MOV X3, X0
    LDR X0, [ X0, 0 ]
    CMP X0, 0
    BEQ lab12
    MOVZ X2, 0, LSL 0
    STR X2, [ X3, 0 ]
    B lab13

lab12:
    MOV X0, X1
    LDR X1, [ X1, 0 ]
    CMP X1, 0
    BEQ lab10
    MOVZ X2, 0, LSL 0
    STR X2, [ X0, 0 ]
    LDR X4, [ X0, 48 ]
    CMP X4, 0
    BEQ lab3
    LDR X2, [ X4, 0 ]
    CMP X2, 0
    BEQ lab1
    SUB X2, X2, 1
    STR X2, [ X4, 0 ]
    B lab2

lab1:
    STR X1, [ X4, 0 ]
    MOV X1, X4

lab2:

lab3:
    LDR X4, [ X0, 32 ]
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
    LDR X4, [ X0, 16 ]
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
    B lab11

lab10:
    ADD X1, X0, 64

lab11:

lab13:
    ADR X4, Func14
    // new k: Cont = ()\{ ... \};
    MOVZ X5, 0, LSL 0
    ADR X6, Cont15
    // lit y <- 1;
    MOVZ X8, 1, LSL 0
    // substitute (y !-> y)(k !-> k)(f !-> f);
    MOV X7, X3
    MOV X2, X8
    MOV X8, X4
    MOV X4, X2
    // invoke f Ap
    BR X8

Cont15:

Cont15Ret:
    // return r
    MOV X1, X4
    B cleanup

Func14:

Func14Ap:
    LDR X2, [ X7, 0 ]
    CMP X2, 0
    BEQ lab16
    SUB X2, X2, 1
    STR X2, [ X7, 0 ]
    LDR X8, [ X7, 56 ]
    B lab17

lab16:
    STR X0, [ X7, 0 ]
    MOV X0, X7
    LDR X8, [ X7, 56 ]

lab17:
    // b <- a + x;
    ADD X10, X8, X4
    // substitute (b !-> b)(k !-> k);
    MOV X4, X10
    // invoke k Ret
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