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
// Setup
// Save registers
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
// Move parameters into place
// Initialize free pointer
MOV X1, X0
ADD X1, X1, 64
// Actual code

main:
MOVZ X3, 0, LSL 0
MOVZ X4, 0, LSL 0
MOVZ X6, 5, LSL 0
STR X6, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
STR X4, [ X0, 40 ]
STR X3, [ X0, 32 ]
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
MOVZ X4, 4, LSL 0
MOVZ X6, 7, LSL 0
STR X6, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
STR X4, [ X0, 40 ]
STR X3, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X3, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab25
MOVZ X2, 0, LSL 0
STR X2, [ X3, 0 ]
B lab26

lab25:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab23
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X4, [ X0, 48 ]
CMP X4, 0
BEQ lab16
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab14
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab15

lab14:
STR X1, [ X4, 0 ]
MOV X1, X4

lab15:

lab16:
LDR X4, [ X0, 32 ]
CMP X4, 0
BEQ lab19
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab17
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab18

lab17:
STR X1, [ X4, 0 ]
MOV X1, X4

lab18:

lab19:
LDR X4, [ X0, 16 ]
CMP X4, 0
BEQ lab22
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab20
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab21

lab20:
STR X1, [ X4, 0 ]
MOV X1, X4

lab21:

lab22:
B lab24

lab23:
ADD X1, X0, 64

lab24:

lab26:
MOVZ X4, 4, LSL 0
MOVZ X6, 9, LSL 0
STR X6, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
STR X4, [ X0, 40 ]
STR X3, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X3, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab38
MOVZ X2, 0, LSL 0
STR X2, [ X3, 0 ]
B lab39

lab38:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab36
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X4, [ X0, 48 ]
CMP X4, 0
BEQ lab29
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab27
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab28

lab27:
STR X1, [ X4, 0 ]
MOV X1, X4

lab28:

lab29:
LDR X4, [ X0, 32 ]
CMP X4, 0
BEQ lab32
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab30
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab31

lab30:
STR X1, [ X4, 0 ]
MOV X1, X4

lab31:

lab32:
LDR X4, [ X0, 16 ]
CMP X4, 0
BEQ lab35
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab33
SUB X2, X2, 1
STR X2, [ X4, 0 ]
B lab34

lab33:
STR X1, [ X4, 0 ]
MOV X1, X4

lab34:

lab35:
B lab37

lab36:
ADD X1, X0, 64

lab37:

lab39:
MOVZ X4, 4, LSL 0
ADR X2, List40
ADD X2, X2, X4
BR X2

List40:
B List40Nil
B List40Cons

List40Nil:
B cleanup

List40Cons:
LDR X2, [ X3, 0 ]
CMP X2, 0
BEQ lab43
SUB X2, X2, 1
STR X2, [ X3, 0 ]
LDR X6, [ X3, 56 ]
LDR X4, [ X3, 40 ]
LDR X3, [ X3, 32 ]
CMP X3, 0
BEQ lab42
LDR X2, [ X3, 0 ]
ADD X2, X2, 1
STR X2, [ X3, 0 ]

lab42:
B lab44

lab43:
STR X0, [ X3, 0 ]
MOV X0, X3
LDR X6, [ X3, 56 ]
LDR X4, [ X3, 40 ]
LDR X3, [ X3, 32 ]

lab44:
MOV X1, X6
B cleanup
// Cleanup

cleanup:
// Restore registers
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