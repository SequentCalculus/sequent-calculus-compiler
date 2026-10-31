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
    // l(...)
    B l_

l_:
    // lit x_1 <- 1;
    MOVZ X4, 1, LSL 0
    // lit y_2 <- 9;
    MOVZ X5, 9, LSL 0
    // j(...)
    B j_

j_:
    // z_5 <- x_4 + y_3;
    ADD X6, X5, X4
    // println_i64 z_5;
    // #save caller-save registers
    MOV X19, X4
    MOV X20, X5
    MOV X21, X6
    MOV X22, X0
    MOV X23, X1
    // #move argument into place
    MOV X0, X6
    BL println_i64
    // #restore caller-save registers
    MOV X4, X19
    MOV X5, X20
    MOV X6, X21
    MOV X0, X22
    MOV X1, X23
    // lit ret_6 <- 0;
    MOVZ X7, 0, LSL 0
    // exit ret_6
    MOV X0, X7
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