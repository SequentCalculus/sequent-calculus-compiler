// actual code
main:
// lit a <- 1;
LI X5 1
// lit b <- 3;
LI X7 3
// c <- a - b;
SUB X9 X5 X7
// lit d <- 8;
LI X11 8
// lit e <- -1;
LI X13 -1
// f <- e * d;
MUL X15 X13 X11
// g <- f + c;
ADD X17 X15 X9
// lit h <- -6;
LI X19 -6
// i <- h * g;
MUL X21 X19 X17
// return i
MV X10 X21
JAL X0 cleanup

cleanup: