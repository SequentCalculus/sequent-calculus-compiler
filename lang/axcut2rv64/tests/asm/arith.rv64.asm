// actual code
main_:
// lit a_1 <- 1;
LI X4 1
// lit b_2 <- 3;
LI X5 3
// c_3 <- a_1 - b_2;
SUB X6 X4 X5
// lit d_4 <- 8;
LI X7 8
// lit e_5 <- -1;
LI X8 -1
// f_6 <- e_5 * d_4;
MUL X9 X8 X7
// g_7 <- f_6 + c_3;
ADD X10 X9 X6
// lit h_8 <- -6;
LI X11 -6
// i_9 <- h_8 * g_7;
MUL X12 X11 X10
// exit i_9
MV X10 X12
JAL X0 cleanup

cleanup: