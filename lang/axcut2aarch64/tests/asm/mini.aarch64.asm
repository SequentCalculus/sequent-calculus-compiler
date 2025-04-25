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
    // jump l_
    B l_

l_:
    // lit x <- 1;
    MOVZ X5, 1, LSL 0
    // lit y <- 9;
    MOVZ X7, 9, LSL 0
    // jump j_
    B j_

j_:
    // z <- x + y;
    ADD X9, X7, X5
    // println_i64 z;
    // #save caller-save registers
    MOV X19, X0
    MOV X20, X1
    MOV X21, X5
    MOV X22, X7
    MOV X23, X9
    // #move argument into place
    MOV X0, X9
    BL println_i64
    // #restore caller-save registers
    MOV X0, X19
    MOV X1, X20
    MOV X5, X21
    MOV X7, X22
    MOV X9, X23
    // lit ret <- 0;
    MOVZ X11, 0, LSL 0
    // exit ret
    MOV X0, X11
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