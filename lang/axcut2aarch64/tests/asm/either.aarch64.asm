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
MOVZ X4, 1, LSL 0
MOVZ X6, 9, LSL 0
STR X6, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X5, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab12
MOVZ X2, 0, LSL 0
STR X2, [ X5, 0 ]
B lab13

lab12:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab10
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X6, [ X0, 48 ]
CMP X6, 0
BEQ lab3
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab1
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab2

lab1:
STR X1, [ X6, 0 ]
MOV X1, X6

lab2:

lab3:
LDR X6, [ X0, 32 ]
CMP X6, 0
BEQ lab6
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab4
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab5

lab4:
STR X1, [ X6, 0 ]
MOV X1, X6

lab5:

lab6:
LDR X6, [ X0, 16 ]
CMP X6, 0
BEQ lab9
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab7
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab8

lab7:
STR X1, [ X6, 0 ]
MOV X1, X6

lab8:

lab9:
B lab11

lab10:
ADD X1, X0, 64

lab11:

lab13:
MOVZ X6, 4, LSL 0
ADR X2, Either14
ADD X2, X2, X6
BR X2

Either14:
B Either14Left
B Either14Right

Either14Left:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab15
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab16

lab15:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab16:
B cleanup

Either14Right:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab17
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab18

lab17:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab18:
ADD X8, X6, X4
MOV X1, X8
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