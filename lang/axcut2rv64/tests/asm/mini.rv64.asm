// actual code
main:
// jump l
JAL X0 l

l:
// lit x <- 1;
LI X5 1
// lit y <- 9;
LI X7 9
// jump j
JAL X0 j

j:
// z <- x + y;
ADD X9 X7 X5
// return z
MV X11 X9
JAL X0 cleanup

cleanup: