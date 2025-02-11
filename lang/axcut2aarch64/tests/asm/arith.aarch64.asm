.text
.global asm_main

asm_main:
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
    // println_i64 i;
    // #save caller-save registers
    MOV X22, X0
    MOV X23, X1
    MOV X24, X4
    MOV X25, X6
    MOV X26, X8
    MOV X27, X10
    MOV X28, X12
    MOV X29, X14
    SUB SP, SP, 16
    STR X16, [ SP, 8 ]
    // #move argument into place
    MOV X0, X21
    BL println_i64
    // #restore caller-save registers
    MOV X0, X22
    MOV X1, X23
    MOV X4, X24
    MOV X6, X25
    MOV X8, X26
    MOV X10, X27
    MOV X12, X28
    MOV X14, X29
    LDR X16, [ SP, 8 ]
    ADD SP, SP, 16
    // lit ret <- 0;
    MOVZ X23, 0, LSL 0
    // return ret
    MOV X0, X23
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