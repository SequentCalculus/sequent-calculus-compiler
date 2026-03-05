// actual code
main_:
// lit a_1 <- 1;
LI X5 1
// lit b_2 <- 3;
LI X7 3
// c_3 <- a_1 - b_2;
SUB X9 X5 X7
// lit d_4 <- 8;
LI X11 8
// lit e_5 <- -1;
LI X13 -1
// f_6 <- e_5 * d_4;
MUL X15 X13 X11
// g_7 <- f_6 + c_3;
ADD X17 X15 X9
// lit h_8 <- -6;
LI X19 -6
// i_9 <- h_8 * g_7;
MUL X21 X19 X17
// exit i_9
MV X10 X21
JAL X0 cleanup

cleanup: