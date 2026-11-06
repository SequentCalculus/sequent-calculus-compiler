// actual code
main_:
// l(...)
JAL X0 l_

l_:
// lit x_1 <- 1;
LI X4 1
// lit y_2 <- 9;
LI X5 9
// j(...)
JAL X0 j_

j_:
// z_5 <- x_4 + y_3;
ADD X6 X5 X4
// exit z_5
MV X10 X6
JAL X0 cleanup

cleanup: