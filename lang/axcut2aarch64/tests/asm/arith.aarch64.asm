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
    // lit a_1 <- 1;
    MOVZ X4, 1, LSL 0
    // lit b_2 <- 3;
    MOVZ X5, 3, LSL 0
    // c_3 <- a_1 - b_2;
    SUB X6, X4, X5
    // lit d_4 <- 8;
    MOVZ X7, 8, LSL 0
    // lit e_5 <- -1;
    MOVN X8, 0, LSL 0
    // f_6 <- e_5 * d_4;
    MUL X9, X8, X7
    // g_7 <- f_6 + c_3;
    ADD X10, X9, X6
    // lit h_8 <- -6;
    MOVN X11, 5, LSL 0
    // i_9 <- h_8 * g_7;
    MUL X12, X11, X10
    // println_i64 i_9;
    // #save caller-save registers
    MOV X19, X4
    MOV X20, X5
    MOV X21, X6
    MOV X22, X7
    MOV X23, X8
    MOV X24, X9
    MOV X25, X10
    MOV X26, X11
    MOV X27, X12
    MOV X28, X0
    MOV X29, X1
    // #move argument into place
    MOV X0, X12
    BL println_i64
    // #restore caller-save registers
    MOV X4, X19
    MOV X5, X20
    MOV X6, X21
    MOV X7, X22
    MOV X8, X23
    MOV X9, X24
    MOV X10, X25
    MOV X11, X26
    MOV X12, X27
    MOV X0, X28
    MOV X1, X29
    // lit ret_10 <- 0;
    MOVZ X13, 0, LSL 0
    // exit ret_10
    MOV X0, X13
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