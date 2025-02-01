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
    // lit a <- 1;
    MOVZ X4, 1, LSL 0
    // lit b <- 3;
    MOVZ X6, 3, LSL 0
    // c <- a - b;
    SUB X8, X4, X6
    // lit d <- 8;
    MOVZ X10, 8, LSL 0
    // lit e <- -1;
    MOVN X12, 0, LSL 0
    // f <- e * d;
    MUL X14, X12, X10
    // g <- f + c;
    ADD X16, X14, X8
    // lit h <- -6;
    MOVN X19, 5, LSL 0
    // i <- h * g;
    MUL X21, X19, X16
    // return i
    MOV X0, X21
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