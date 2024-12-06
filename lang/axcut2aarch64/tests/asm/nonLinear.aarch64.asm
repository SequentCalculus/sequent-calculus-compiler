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
MOVZ X4, 3, LSL 0
MOVZ X6, 3, LSL 0
MOVZ X8, 3, LSL 0
MOVZ X10, 3, LSL 0
MOVZ X12, 3, LSL 0
MOVZ X14, 3, LSL 0
MOVZ X16, 3, LSL 0
MOVZ X18, 3, LSL 0
STR X18, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X17, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab12
MOVZ X2, 0, LSL 0
STR X2, [ X17, 0 ]
B lab13

lab12:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab10
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X18, [ X0, 48 ]
CMP X18, 0
BEQ lab3
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab1
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab2

lab1:
STR X1, [ X18, 0 ]
MOV X1, X18

lab2:

lab3:
LDR X18, [ X0, 32 ]
CMP X18, 0
BEQ lab6
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab4
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab5

lab4:
STR X1, [ X18, 0 ]
MOV X1, X18

lab5:

lab6:
LDR X18, [ X0, 16 ]
CMP X18, 0
BEQ lab9
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab7
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab8

lab7:
STR X1, [ X18, 0 ]
MOV X1, X18

lab8:

lab9:
B lab11

lab10:
ADD X1, X0, 64

lab11:

lab13:
MOVZ X18, 0, LSL 0
STR X18, [ X0, 56 ]
STR X17, [ X0, 48 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X17, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab25
MOVZ X2, 0, LSL 0
STR X2, [ X17, 0 ]
B lab26

lab25:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab23
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X18, [ X0, 48 ]
CMP X18, 0
BEQ lab16
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab14
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab15

lab14:
STR X1, [ X18, 0 ]
MOV X1, X18

lab15:

lab16:
LDR X18, [ X0, 32 ]
CMP X18, 0
BEQ lab19
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab17
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab18

lab17:
STR X1, [ X18, 0 ]
MOV X1, X18

lab18:

lab19:
LDR X18, [ X0, 16 ]
CMP X18, 0
BEQ lab22
LDR X2, [ X18, 0 ]
CMP X2, 0
BEQ lab20
SUB X2, X2, 1
STR X2, [ X18, 0 ]
B lab21

lab20:
STR X1, [ X18, 0 ]
MOV X1, X18

lab21:

lab22:
B lab24

lab23:
ADD X1, X0, 64

lab24:

lab26:
MOVZ X18, 0, LSL 0
CMP X17, 0
BEQ lab27
LDR X2, [ X17, 0 ]
ADD X2, X2, 2
STR X2, [ X17, 0 ]

lab27:
MOV X2, X12
MOV X12, X14
MOV X14, X16
MOV X16, X10
MOV X10, X2
MOV X19, X17
MOV X21, X17
MOV X20, X18
MOV X22, X18
ADR X2, BoxBox28
BR X2

BoxBox28:

BoxBox28BB:
LDR X2, [ X21, 0 ]
CMP X2, 0
BEQ lab31
SUB X2, X2, 1
STR X2, [ X21, 0 ]
LDR X22, [ X21, 56 ]
LDR X21, [ X21, 48 ]
CMP X21, 0
BEQ lab30
LDR X2, [ X21, 0 ]
ADD X2, X2, 1
STR X2, [ X21, 0 ]

lab30:
B lab32

lab31:
STR X0, [ X21, 0 ]
MOV X0, X21
LDR X22, [ X21, 56 ]
LDR X21, [ X21, 48 ]

lab32:
ADR X2, Box33
BR X2

Box33:

Box33B:
LDR X2, [ X21, 0 ]
CMP X2, 0
BEQ lab34
SUB X2, X2, 1
STR X2, [ X21, 0 ]
LDR X22, [ X21, 56 ]
B lab35

lab34:
STR X0, [ X21, 0 ]
MOV X0, X21
LDR X22, [ X21, 56 ]

lab35:
STR X22, [ X0, 56 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 48 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X21, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab47
MOVZ X2, 0, LSL 0
STR X2, [ X21, 0 ]
B lab48

lab47:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab45
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X22, [ X0, 48 ]
CMP X22, 0
BEQ lab38
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab36
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab37

lab36:
STR X1, [ X22, 0 ]
MOV X1, X22

lab37:

lab38:
LDR X22, [ X0, 32 ]
CMP X22, 0
BEQ lab41
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab39
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab40

lab39:
STR X1, [ X22, 0 ]
MOV X1, X22

lab40:

lab41:
LDR X22, [ X0, 16 ]
CMP X22, 0
BEQ lab44
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab42
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab43

lab42:
STR X1, [ X22, 0 ]
MOV X1, X22

lab43:

lab44:
B lab46

lab45:
ADD X1, X0, 64

lab46:

lab48:
MOVZ X22, 0, LSL 0
STR X22, [ X0, 56 ]
STR X21, [ X0, 48 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 32 ]
MOVZ X2, 0, LSL 0
STR X2, [ X0, 16 ]
MOV X21, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab60
MOVZ X2, 0, LSL 0
STR X2, [ X21, 0 ]
B lab61

lab60:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab58
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X22, [ X0, 48 ]
CMP X22, 0
BEQ lab51
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab49
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab50

lab49:
STR X1, [ X22, 0 ]
MOV X1, X22

lab50:

lab51:
LDR X22, [ X0, 32 ]
CMP X22, 0
BEQ lab54
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab52
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab53

lab52:
STR X1, [ X22, 0 ]
MOV X1, X22

lab53:

lab54:
LDR X22, [ X0, 16 ]
CMP X22, 0
BEQ lab57
LDR X2, [ X22, 0 ]
CMP X2, 0
BEQ lab55
SUB X2, X2, 1
STR X2, [ X22, 0 ]
B lab56

lab55:
STR X1, [ X22, 0 ]
MOV X1, X22

lab56:

lab57:
B lab59

lab58:
ADD X1, X0, 64

lab59:

lab61:
MOVZ X22, 0, LSL 0
CMP X21, 0
BEQ lab64
LDR X2, [ X21, 0 ]
CMP X2, 0
BEQ lab62
SUB X2, X2, 1
STR X2, [ X21, 0 ]
B lab63

lab62:
STR X1, [ X21, 0 ]
MOV X1, X21

lab63:

lab64:
CMP X17, 0
BEQ lab67
LDR X2, [ X17, 0 ]
CMP X2, 0
BEQ lab65
SUB X2, X2, 1
STR X2, [ X17, 0 ]
B lab66

lab65:
STR X1, [ X17, 0 ]
MOV X1, X17

lab66:

lab67:
MOV X3, X19
MOV X4, X20
MOVZ X6, 4, LSL 0
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
BEQ lab79
MOVZ X2, 0, LSL 0
STR X2, [ X5, 0 ]
B lab80

lab79:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab77
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X6, [ X0, 48 ]
CMP X6, 0
BEQ lab70
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab68
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab69

lab68:
STR X1, [ X6, 0 ]
MOV X1, X6

lab69:

lab70:
LDR X6, [ X0, 32 ]
CMP X6, 0
BEQ lab73
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab71
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab72

lab71:
STR X1, [ X6, 0 ]
MOV X1, X6

lab72:

lab73:
LDR X6, [ X0, 16 ]
CMP X6, 0
BEQ lab76
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab74
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab75

lab74:
STR X1, [ X6, 0 ]
MOV X1, X6

lab75:

lab76:
B lab78

lab77:
ADD X1, X0, 64

lab78:

lab80:
MOVZ X6, 0, LSL 0
MOV X2, X5
MOV X5, X3
MOV X3, X2
MOV X2, X6
MOV X6, X4
MOV X4, X2
ADR X2, BoxBox81
BR X2

BoxBox81:

BoxBox81BB:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab84
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
LDR X5, [ X5, 48 ]
CMP X5, 0
BEQ lab83
LDR X2, [ X5, 0 ]
ADD X2, X2, 1
STR X2, [ X5, 0 ]

lab83:
B lab85

lab84:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]
LDR X5, [ X5, 48 ]

lab85:
ADR X2, Box86
BR X2

Box86:

Box86B:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab87
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab88

lab87:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab88:
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
BEQ lab100
MOVZ X2, 0, LSL 0
STR X2, [ X5, 0 ]
B lab101

lab100:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab98
MOVZ X2, 0, LSL 0
STR X2, [ X0, 0 ]
LDR X6, [ X0, 48 ]
CMP X6, 0
BEQ lab91
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab89
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab90

lab89:
STR X1, [ X6, 0 ]
MOV X1, X6

lab90:

lab91:
LDR X6, [ X0, 32 ]
CMP X6, 0
BEQ lab94
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab92
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab93

lab92:
STR X1, [ X6, 0 ]
MOV X1, X6

lab93:

lab94:
LDR X6, [ X0, 16 ]
CMP X6, 0
BEQ lab97
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab95
SUB X2, X2, 1
STR X2, [ X6, 0 ]
B lab96

lab95:
STR X1, [ X6, 0 ]
MOV X1, X6

lab96:

lab97:
B lab99

lab98:
ADD X1, X0, 64

lab99:

lab101:
MOVZ X6, 0, LSL 0
ADR X2, Box102
BR X2

Box102:

Box102B:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab103
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab104

lab103:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab104:
MOV X5, X3
MOV X2, X6
MOV X6, X4
MOV X4, X2
ADR X2, Box105
BR X2

Box105:

Box105B:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab106
SUB X2, X2, 1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab107

lab106:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab107:
ADD X8, X6, X4
MOV X1, X8
B cleanup
// cleanup

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