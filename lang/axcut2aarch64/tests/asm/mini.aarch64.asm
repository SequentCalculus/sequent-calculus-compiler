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

main_:
    // jump l_
    B l_

l_:
    // lit x <- 1;
    MOVZ X4, 1, LSL 0
    // lit y <- 9;
    MOVZ X6, 9, LSL 0
    // jump j_
    B j_

j_:
    // z <- x + y;
    ADD X8, X6, X4
    // println_i64 z;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X4
    MOV X22, X6
    MOV X23, X8
    // #move argument into place
    MOV X0, X8
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X4, X21
    MOV X6, X22
    MOV X8, X23
    // lit ret <- 0;
    MOVZ X10, 0, LSL 0
    // return ret
    MOV X0, X10
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