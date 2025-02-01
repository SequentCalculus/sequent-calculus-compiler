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
    // jump l
    B l

l:
    // lit x <- 1;
    MOVZ X4, 1, LSL 0
    // lit y <- 9;
    MOVZ X6, 9, LSL 0
    // jump j
    B j

j:
    // z <- x + y;
    ADD X8, X6, X4
    // return z
    MOV X0, X8
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