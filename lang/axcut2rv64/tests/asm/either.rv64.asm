// actual code
main:
// lit z <- 1;
LI X5 1
// lit x <- 9;
LI X7 9
// leta p: Either = Right(x);
SW X7 56 X2
SW X0 48 X2
SW X0 32 X2
SW X0 16 X2
MV X6 X2
LW X2 0 X2
BEQ X2 X0 lab12
SW X0 0 X6
JAL X0 lab13

lab12:
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab10
SW X0 0 X2
LW X7 48 X2
BEQ X7 X0 lab3
LW X1 0 X7
BEQ X1 X0 lab1
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab2

lab1:
SW X3 0 X7
MV X3 X7

lab2:

lab3:
LW X7 32 X2
BEQ X7 X0 lab6
LW X1 0 X7
BEQ X1 X0 lab4
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab5

lab4:
SW X3 0 X7
MV X3 X7

lab5:

lab6:
LW X7 16 X2
BEQ X7 X0 lab9
LW X1 0 X7
BEQ X1 X0 lab7
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab8

lab7:
SW X3 0 X7
MV X3 X7

lab8:

lab9:
JAL X0 lab11

lab10:
ADD X3 X2 64

lab11:

lab13:
LI X7 4
// switch p \{ ... \};
LA X1 Either14
ADD X1 X1 X7
JALR X0 X1 0

Either14:
JAL X0 Either14Left
JAL X0 Either14Right

Either14Left:
LW X1 0 X6
BEQ X1 X0 lab15
ADD X1 X1 -1
SW X1 0 X6
LW X7 56 X6
JAL X0 lab16

lab15:
SW X2 0 X6
MV X2 X6
LW X7 56 X6

lab16:
// Done
JAL X0 cleanup

Either14Right:
LW X1 0 X6
BEQ X1 X0 lab17
ADD X1 X1 -1
SW X1 0 X6
LW X7 56 X6
JAL X0 lab18

lab17:
SW X2 0 X6
MV X2 X6
LW X7 56 X6

lab18:
// c <- b + z;
ADD X9 X7 X5
// return c
MV X11 X9
JAL X0 cleanup

cleanup: