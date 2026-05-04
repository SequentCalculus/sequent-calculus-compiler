// actual code
main_:
// l(...)
JAL X0 l_

l_:
// lit x_1 <- 1;
LI X5 1
// lit y_2 <- 9;
LI X7 9
// j(...)
JAL X0 j_

j_:
// z_5 <- x_4 + y_3;
ADD X9 X7 X5
// exit z_5
MV X10 X9
JAL X0 cleanup

cleanup: