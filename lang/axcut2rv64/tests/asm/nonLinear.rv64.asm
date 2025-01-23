// actual code
main:
// lit f1 <- 3;
LI X5 3
// lit f2 <- 3;
LI X7 3
// lit f3 <- 3;
LI X9 3
// lit f4 <- 3;
LI X11 3
// lit f5 <- 3;
LI X13 3
// lit f6 <- 3;
LI X15 3
// lit f7 <- 3;
LI X17 3
// lit x <- 3;
LI X19 3
// leta b: Box = B(x);
//  allocate memory
//   store values
SW X19 56 X2
SW X0 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X18 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab12
//     initialize refcount of just acquired block
SW X0 0 X18
JAL X0 lab13

lab12:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab10
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X19 48 X2
BEQ X19 X0 lab3
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab1
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab2

lab1:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab2:

lab3:
//      check child 2 for erasure
LW X19 32 X2
BEQ X19 X0 lab6
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab4
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab5

lab4:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab5:

lab6:
//      check child 1 for erasure
LW X19 16 X2
BEQ X19 X0 lab9
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab7
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab8

lab7:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab8:

lab9:
JAL X0 lab11

lab10:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab11:

lab13:
//  load tag
LI X19 0
// leta bb: BoxBox = BB(b);
//  allocate memory
//   store values
SW X19 56 X2
SW X18 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X18 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab25
//     initialize refcount of just acquired block
SW X0 0 X18
JAL X0 lab26

lab25:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab23
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X19 48 X2
BEQ X19 X0 lab16
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab14
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab15

lab14:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab15:

lab16:
//      check child 2 for erasure
LW X19 32 X2
BEQ X19 X0 lab19
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab17
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab18

lab17:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab18:

lab19:
//      check child 1 for erasure
LW X19 16 X2
BEQ X19 X0 lab22
//       check refcount
LW X1 0 X19
BEQ X1 X0 lab20
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X19
JAL X0 lab21

lab20:
//       ... or add block to lazy free list
SW X3 0 X19
MV X3 X19

lab21:

lab22:
JAL X0 lab24

lab23:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab24:

lab26:
//  load tag
LI X19 0
// substitute (f1 !-> f1)(f2 !-> f2)(f3 !-> f3)(f5 !-> f5)(f6 !-> f6)(f7 !-> f7)(f4 !-> f4)(bb3 !-> bb)(bb2 !-> bb)(bb1 !-> bb);
//  share bb
BEQ X18 X0 lab27
//     increment refcount
LW X1 0 X18
ADD X1 X1 2
SW X1 0 X18

lab27:
//  move variables
MV X1 X13
MV X13 X15
MV X15 X17
MV X17 X11
MV X11 X1
MV X20 X18
MV X22 X18
MV X21 X19
MV X23 X19
// switch bb1 \{ ... \};
LA X1 BoxBox28
JALR X0 X1 0

BoxBox28:

BoxBox28BB:
//  load from memory
LW X1 0 X22
//   check refcount
BEQ X1 X0 lab30
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X22
//    load values
LW X23 56 X22
LW X22 48 X22
BEQ X22 X0 lab29
//     increment refcount
LW X1 0 X22
ADD X1 X1 1
SW X1 0 X22

lab29:
JAL X0 lab31

lab30:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X22
MV X2 X22
//    load values
LW X23 56 X22
LW X22 48 X22

lab31:
// switch b1 \{ ... \};
LA X1 Box32
JALR X0 X1 0

Box32:

Box32B:
//  load from memory
LW X1 0 X22
//   check refcount
BEQ X1 X0 lab33
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X22
//    load values
LW X23 56 X22
JAL X0 lab34

lab33:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X22
MV X2 X22
//    load values
LW X23 56 X22

lab34:
// leta d1: Box = B(x1);
//  allocate memory
//   store values
SW X23 56 X2
SW X0 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X22 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab46
//     initialize refcount of just acquired block
SW X0 0 X22
JAL X0 lab47

lab46:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab44
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X23 48 X2
BEQ X23 X0 lab37
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab35
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab36

lab35:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab36:

lab37:
//      check child 2 for erasure
LW X23 32 X2
BEQ X23 X0 lab40
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab38
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab39

lab38:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab39:

lab40:
//      check child 1 for erasure
LW X23 16 X2
BEQ X23 X0 lab43
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab41
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab42

lab41:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab42:

lab43:
JAL X0 lab45

lab44:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab45:

lab47:
//  load tag
LI X23 0
// leta dd1: BoxBox = BB(d1);
//  allocate memory
//   store values
SW X23 56 X2
SW X22 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X22 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab59
//     initialize refcount of just acquired block
SW X0 0 X22
JAL X0 lab60

lab59:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab57
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X23 48 X2
BEQ X23 X0 lab50
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab48
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab49

lab48:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab49:

lab50:
//      check child 2 for erasure
LW X23 32 X2
BEQ X23 X0 lab53
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab51
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab52

lab51:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab52:

lab53:
//      check child 1 for erasure
LW X23 16 X2
BEQ X23 X0 lab56
//       check refcount
LW X1 0 X23
BEQ X1 X0 lab54
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X23
JAL X0 lab55

lab54:
//       ... or add block to lazy free list
SW X3 0 X23
MV X3 X23

lab55:

lab56:
JAL X0 lab58

lab57:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab58:

lab60:
//  load tag
LI X23 0
// substitute (bb2 !-> bb2);
//  erase dd1
BEQ X22 X0 lab63
//       check refcount
LW X1 0 X22
BEQ X1 X0 lab61
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X22
JAL X0 lab62

lab61:
//       ... or add block to lazy free list
SW X3 0 X22
MV X3 X22

lab62:

lab63:
//  erase bb3
BEQ X18 X0 lab66
//       check refcount
LW X1 0 X18
BEQ X1 X0 lab64
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X18
JAL X0 lab65

lab64:
//       ... or add block to lazy free list
SW X3 0 X18
MV X3 X18

lab65:

lab66:
//  move variables
MV X4 X20
MV X5 X21
// lit y <- 4;
LI X7 4
// leta a1: Box = B(y);
//  allocate memory
//   store values
SW X7 56 X2
SW X0 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X6 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab78
//     initialize refcount of just acquired block
SW X0 0 X6
JAL X0 lab79

lab78:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab76
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X7 48 X2
BEQ X7 X0 lab69
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab67
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab68

lab67:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab68:

lab69:
//      check child 2 for erasure
LW X7 32 X2
BEQ X7 X0 lab72
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab70
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab71

lab70:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab71:

lab72:
//      check child 1 for erasure
LW X7 16 X2
BEQ X7 X0 lab75
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab73
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab74

lab73:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab74:

lab75:
JAL X0 lab77

lab76:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab77:

lab79:
//  load tag
LI X7 0
// substitute (a1 !-> a1)(bb2 !-> bb2);
//  move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// switch bb2 \{ ... \};
LA X1 BoxBox80
JALR X0 X1 0

BoxBox80:

BoxBox80BB:
//  load from memory
LW X1 0 X6
//   check refcount
BEQ X1 X0 lab82
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
//    load values
LW X7 56 X6
LW X6 48 X6
BEQ X6 X0 lab81
//     increment refcount
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab81:
JAL X0 lab83

lab82:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X6
MV X2 X6
//    load values
LW X7 56 X6
LW X6 48 X6

lab83:
// switch b2 \{ ... \};
LA X1 Box84
JALR X0 X1 0

Box84:

Box84B:
//  load from memory
LW X1 0 X6
//   check refcount
BEQ X1 X0 lab85
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
//    load values
LW X7 56 X6
JAL X0 lab86

lab85:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X6
MV X2 X6
//    load values
LW X7 56 X6

lab86:
// leta a2: Box = B(x2);
//  allocate memory
//   store values
SW X7 56 X2
SW X0 48 X2
//   mark unused fields with null
SW X0 32 X2
SW X0 16 X2
//   acquire free block from heap register
MV X6 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab98
//     initialize refcount of just acquired block
SW X0 0 X6
JAL X0 lab99

lab98:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab96
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X7 48 X2
BEQ X7 X0 lab89
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab87
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab88

lab87:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab88:

lab89:
//      check child 2 for erasure
LW X7 32 X2
BEQ X7 X0 lab92
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab90
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab91

lab90:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab91:

lab92:
//      check child 1 for erasure
LW X7 16 X2
BEQ X7 X0 lab95
//       check refcount
LW X1 0 X7
BEQ X1 X0 lab93
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab94

lab93:
//       ... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab94:

lab95:
JAL X0 lab97

lab96:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab97:

lab99:
//  load tag
LI X7 0
// switch a2 \{ ... \};
LA X1 Box100
JALR X0 X1 0

Box100:

Box100B:
//  load from memory
LW X1 0 X6
//   check refcount
BEQ X1 X0 lab101
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
//    load values
LW X7 56 X6
JAL X0 lab102

lab101:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X6
MV X2 X6
//    load values
LW X7 56 X6

lab102:
// substitute (x2 !-> x2)(a1 !-> a1);
//  move variables
MV X6 X4
MV X1 X7
MV X7 X5
MV X5 X1
// switch a1 \{ ... \};
LA X1 Box103
JALR X0 X1 0

Box103:

Box103B:
//  load from memory
LW X1 0 X6
//   check refcount
BEQ X1 X0 lab104
//   either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
//    load values
LW X7 56 X6
JAL X0 lab105

lab104:
//   ... or release blocks onto linear free list when loading
//    release block
SW X2 0 X6
MV X2 X6
//    load values
LW X7 56 X6

lab105:
// res <- x1 + x2;
ADD X9 X7 X5
// return res
MV X11 X9
JAL X0 cleanup

cleanup: