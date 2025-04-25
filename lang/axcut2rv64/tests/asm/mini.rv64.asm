// actual code
main_:
// jump l_
JAL X0 l_

l_:
// lit x <- 1;
LI X5 1
// lit y <- 9;
LI X7 9
// jump j_
JAL X0 j_

j_:
// z <- x + y;
ADD X9 X7 X5
// exit z
MV X10 X9
JAL X0 cleanup

cleanup: