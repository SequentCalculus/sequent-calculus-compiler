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
    MOVZ X4, 1, LSL 0
    MOVZ X6, 3, LSL 0
    SUB X8, X4, X6
    MOVZ X10, 8, LSL 0
    MOVN X12, 0, LSL 0
    MUL X14, X12, X10
    ADD X16, X14, X8
    MOVN X18, 5, LSL 0
    MUL X20, X18, X16
    MOV X1, X20
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