// actual code
main:
// new t: ContInt = ...;
MV X4 X0
LA X5 ContInt3
// new k: ContList = ...;
SW X5 56 X2
SW X4 48 X2
SW X0 32 X2
SW X0 16 X2
MV X4 X2
LW X2 0 X2
BEQ X2 X0 lab15
SW X0 0 X4
JAL X0 lab16

lab15:
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab13
SW X0 0 X2
LW X5 48 X2
BEQ X5 X0 lab6
LW X1 0 X5
BEQ X1 X0 lab4
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab5

lab4:
SW X3 0 X5
MV X3 X5

lab5:

lab6:
LW X5 32 X2
BEQ X5 X0 lab9
LW X1 0 X5
BEQ X1 X0 lab7
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab8

lab7:
SW X3 0 X5
MV X3 X5

lab8:

lab9:
LW X5 16 X2
BEQ X5 X0 lab12
LW X1 0 X5
BEQ X1 X0 lab10
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab11

lab10:
SW X3 0 X5
MV X3 X5

lab11:

lab12:
JAL X0 lab14

lab13:
ADD X3 X2 64

lab14:

lab16:
LA X5 ContList17
// leta zs: List = Nil();
MV X6 X0
LI X7 0
// lit n <- 3;
LI X9 3
// substitute (k !-> k)(zs !-> zs)(n !-> n);
// jump range
JAL X0 range

ContList17:

ContList17Retl:
LW X1 0 X6
BEQ X1 X0 lab19
ADD X1 X1 -1
SW X1 0 X6
LW X7 56 X6
LW X6 48 X6
BEQ X6 X0 lab18
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab18:
JAL X0 lab20

lab19:
SW X2 0 X6
MV X2 X6
LW X7 56 X6
LW X6 48 X6

lab20:
// substitute (t !-> t)(as !-> as);
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// jump sum
JAL X0 sum

ContInt3:

ContInt3Reti:
// return r
MV X11 X5
JAL X0 cleanup

range:
// ifz i \{ ... \}
BEQ X9 X0 lab21
// substitute (n !-> i)(k !-> k)(xs !-> xs)(i !-> i);
MV X8 X6
MV X6 X4
MV X1 X9
MV X11 X9
MV X9 X7
MV X7 X5
MV X5 X1
// leta ys: List = Cons(xs, i);
SW X11 56 X2
SW X0 48 X2
SW X9 40 X2
SW X8 32 X2
SW X0 16 X2
MV X8 X2
LW X2 0 X2
BEQ X2 X0 lab33
SW X0 0 X8
JAL X0 lab34

lab33:
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab31
SW X0 0 X2
LW X9 48 X2
BEQ X9 X0 lab24
LW X1 0 X9
BEQ X1 X0 lab22
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab23

lab22:
SW X3 0 X9
MV X3 X9

lab23:

lab24:
LW X9 32 X2
BEQ X9 X0 lab27
LW X1 0 X9
BEQ X1 X0 lab25
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab26

lab25:
SW X3 0 X9
MV X3 X9

lab26:

lab27:
LW X9 16 X2
BEQ X9 X0 lab30
LW X1 0 X9
BEQ X1 X0 lab28
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab29

lab28:
SW X3 0 X9
MV X3 X9

lab29:

lab30:
JAL X0 lab32

lab31:
ADD X3 X2 64

lab32:

lab34:
LI X9 4
// lit o <- -1;
LI X11 -1
// j <- n + o;
ADD X13 X5 X11
// substitute (k !-> k)(ys !-> ys)(j !-> j);
MV X4 X6
MV X5 X7
MV X6 X8
MV X7 X9
MV X9 X13
// jump range
JAL X0 range

lab21:
// substitute (xs !-> xs)(k !-> k);
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// invoke k Retl
JALR X0 X7 0

sum:
// switch xs \{ ... \};
LA X1 List35
ADD X1 X1 X7
JALR X0 X1 0

List35:
JAL X0 List35Nil
JAL X0 List35Cons

List35Nil:
// lit z <- 0;
LI X7 0
// substitute (z !-> z)(k !-> k);
MV X6 X4
MV X1 X7
MV X7 X5
MV X5 X1
// invoke k Reti
JALR X0 X7 0

List35Cons:
LW X1 0 X6
BEQ X1 X0 lab37
ADD X1 X1 -1
SW X1 0 X6
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6
BEQ X6 X0 lab36
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab36:
JAL X0 lab38

lab37:
SW X2 0 X6
MV X2 X6
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6

lab38:
// substitute (ys !-> ys)(k !-> k)(y !-> y);
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// new j: ContInt = ...;
SW X9 56 X2
SW X0 48 X2
SW X7 40 X2
SW X6 32 X2
SW X0 16 X2
MV X6 X2
LW X2 0 X2
BEQ X2 X0 lab50
SW X0 0 X6
JAL X0 lab51

lab50:
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab48
SW X0 0 X2
LW X7 48 X2
BEQ X7 X0 lab41
LW X1 0 X7
BEQ X1 X0 lab39
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab40

lab39:
SW X3 0 X7
MV X3 X7

lab40:

lab41:
LW X7 32 X2
BEQ X7 X0 lab44
LW X1 0 X7
BEQ X1 X0 lab42
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab43

lab42:
SW X3 0 X7
MV X3 X7

lab43:

lab44:
LW X7 16 X2
BEQ X7 X0 lab47
LW X1 0 X7
BEQ X1 X0 lab45
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab46

lab45:
SW X3 0 X7
MV X3 X7

lab46:

lab47:
JAL X0 lab49

lab48:
ADD X3 X2 64

lab49:

lab51:
LA X7 ContInt52
// substitute (j !-> j)(ys !-> ys);
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// jump sum
JAL X0 sum

ContInt52:

ContInt52Reti:
LW X1 0 X6
BEQ X1 X0 lab54
ADD X1 X1 -1
SW X1 0 X6
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6
BEQ X6 X0 lab53
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab53:
JAL X0 lab55

lab54:
SW X2 0 X6
MV X2 X6
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6

lab55:
// s <- y + r;
ADD X11 X9 X5
// substitute (s !-> s)(k !-> k);
MV X5 X11
// invoke k Reti
JALR X0 X7 0

cleanup: