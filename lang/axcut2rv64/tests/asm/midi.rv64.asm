// actual code
main:
// new t: ContInt = ()\{ ... \};
//  nothing to store
MV X4 X0
//  load tag
LA X5 ContInt3
// new k: ContList = (t)\{ ... \};
//  allocate memory
//   store values
SW X5 56 X2
SW X4 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X4 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab15
//     initialize refcount of just acquired block
SW X0 0 X4
JAL X0 lab16

lab15:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab13
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X5 48 X2
BEQ X5 X0 lab6
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab4
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab5

lab4:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab5:

lab6:
//      check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab9
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab7
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab8

lab7:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab8:

lab9:
//      check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab12
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab10
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab11

lab10:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab11:

lab12:
JAL X0 lab14

lab13:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab14:

lab16:
//  load tag
LA X5 ContList17
// leta zs: List = Nil();
//  nothing to store
MV X6 X0
//  load tag
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
//  move variables
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
//  move variables
MV X8 X6
MV X6 X4
MV X1 X9
MV X11 X9
MV X9 X7
MV X7 X5
MV X5 X1
// leta ys: List = Cons(xs, i);
//  allocate memory
//   store values
SW X11 56 X2
SW X0 48 X2
SW X9 40 X2
SW X8 32 X2
//   mark unused fields with null
SW X0 16 X2
//   acquire free block from heap register
MV X8 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab33
//     initialize refcount of just acquired block
SW X0 0 X8
JAL X0 lab34

lab33:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab31
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X9 48 X2
BEQ X9 X0 lab24
//       check refcount
LW X1 0 X9
BEQ X1 X0 lab22
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab23

lab22:
//       ... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab23:

lab24:
//      check child 2 for erasure
LW X9 32 X2
BEQ X9 X0 lab27
//       check refcount
LW X1 0 X9
BEQ X1 X0 lab25
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab26

lab25:
//       ... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab26:

lab27:
//      check child 1 for erasure
LW X9 16 X2
BEQ X9 X0 lab30
//       check refcount
LW X1 0 X9
BEQ X1 X0 lab28
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab29

lab28:
//       ... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab29:

lab30:
JAL X0 lab32

lab31:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab32:

lab34:
//  load tag
LI X9 4
// lit o <- -1;
LI X11 -1
// j <- n + o;
ADD X13 X5 X11
// substitute (k !-> k)(ys !-> ys)(j !-> j);
//  move variables
MV X4 X6
MV X5 X7
MV X6 X8
MV X7 X9
MV X9 X13
// jump range
JAL X0 range

lab21:
// substitute (xs !-> xs)(k !-> k);
//  move variables
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
//  move variables
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
//  move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// new j: ContInt = (k, y)\{ ... \};
//  allocate memory
//   store values
SW X9 56 X2
SW X0 48 X2
SW X7 40 X2
SW X6 32 X2
//   mark unused fields with null
SW X0 16 X2
//   acquire free block from heap register
MV X6 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab50
//     initialize refcount of just acquired block
SW X0 0 X6
JAL X0 lab51

lab50:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab48
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X7 48 X2
BEQ X7 X0 lab41
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab39
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab40

lab39:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab40:

lab41:
//      check child 2 for erasure
LW X7 32 X2
BEQ X7 X0 lab44
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab42
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab43

lab42:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab43:

lab44:
//      check child 1 for erasure
LW X7 16 X2
BEQ X7 X0 lab47
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab45
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab46

lab45:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab46:

lab47:
JAL X0 lab49

lab48:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab49:

lab51:
//  load tag
LA X7 ContInt52
// substitute (j !-> j)(ys !-> ys);
//  move variables
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
//  move variables
MV X5 X11
// invoke k Reti
JALR X0 X7 0

cleanup: