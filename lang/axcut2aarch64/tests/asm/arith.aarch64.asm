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
    // reserve space for register spills
    SUB SP, SP, 2048
    // move parameters into place
    // initialize free pointer
    MOV X1, X0
    ADD X1, X1, 64
    // actual code

main_:
    // lit a <- 1;
    MOVZ X5, 1, LSL 0
    // lit b <- 3;
    MOVZ X7, 3, LSL 0
    // c <- a - b;
    SUB X9, X5, X7
    // lit d <- 8;
    MOVZ X11, 8, LSL 0
    // lit e <- -1;
    MOVN X13, 0, LSL 0
    // f <- e * d;
    MUL X15, X13, X11
    // g <- f + c;
    ADD X17, X15, X9
    // lit h <- -6;
    MOVN X20, 5, LSL 0
    // i <- h * g;
    MUL X22, X20, X17
    // println_i64 i;
    // #save caller-save registers
    MOV X23, X0
    MOV X24, X1
    MOV X25, X5
    MOV X26, X7
    MOV X27, X9
    MOV X28, X11
    MOV X29, X13
    SUB SP, SP, 16
    STR X15, [ SP, 8 ]
    STR X17, [ SP, 0 ]
    // #move argument into place
    MOV X0, X22
    BL println_i64
    // #restore caller-save registers
    MOV X0, X23
    MOV X1, X24
    MOV X5, X25
    MOV X7, X26
    MOV X9, X27
    MOV X11, X28
    MOV X13, X29
    LDR X17, [ SP, 0 ]
    LDR X15, [ SP, 8 ]
    ADD SP, SP, 16
    // lit ret <- 0;
    MOVZ X24, 0, LSL 0
    // exit ret
    MOV X0, X24
    B cleanup

cleanup:
    // free space for register spills
    ADD SP, SP, 2048
    // restore registers
    LDP X29, X30, [ SP ], 16
    LDP X27, X28, [ SP ], 16
    LDP X25, X26, [ SP ], 16
    LDP X23, X24, [ SP ], 16
    LDP X21, X22, [ SP ], 16
    LDP X19, X20, [ SP ], 16
    RET