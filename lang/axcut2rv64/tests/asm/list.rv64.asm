// actual code
main:
// leta ws: List = Nil();
//  nothing to store
MV X4 X0
//  load tag
LI X5 0
// lit z <- 5;
LI X7 5
// leta zs: List = Cons(z, ws);
//  allocate memory
//   store values
SW X7 56 X2
SW X0 48 X2
SW X5 40 X2
SW X4 32 X2
//   mark unused fields with null
SW X0 16 X2
//   acquire free block from heap register
MV X4 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab12
//     initialize refcount of just acquired block
SW X0 0 X4
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
LW X5 48 X2
BEQ X5 X0 lab3
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab1
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab2

lab1:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab2:

lab3:
//      check child 2 for erasure
LW X5 32 X2
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
//      check child 1 for erasure
LW X5 16 X2
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
JAL X0 lab11

lab10:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab11:

lab13:
//  load tag
LI X5 4
// lit y <- 7;
LI X7 7
// leta ys: List = Cons(y, zs);
//  allocate memory
//   store values
SW X7 56 X2
SW X0 48 X2
SW X5 40 X2
SW X4 32 X2
//   mark unused fields with null
SW X0 16 X2
//   acquire free block from heap register
MV X4 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab25
//     initialize refcount of just acquired block
SW X0 0 X4
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
LW X5 48 X2
BEQ X5 X0 lab16
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab14
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab15

lab14:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab15:

lab16:
//      check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab19
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab17
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab18

lab17:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab18:

lab19:
//      check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab22
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab20
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab21

lab20:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab21:

lab22:
JAL X0 lab24

lab23:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab24:

lab26:
//  load tag
LI X5 4
// lit x <- 9;
LI X7 9
// leta xs: List = Cons(x, ys);
//  allocate memory
//   store values
SW X7 56 X2
SW X0 48 X2
SW X5 40 X2
SW X4 32 X2
//   mark unused fields with null
SW X0 16 X2
//   acquire free block from heap register
MV X4 X2
//   get next free block into heap register
//    (1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab38
//     initialize refcount of just acquired block
SW X0 0 X4
JAL X0 lab39

lab38:
//    (2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab36
//     mark linear free list empty
SW X0 0 X2
//     erase children of next block
//      check child 3 for erasure
LW X5 48 X2
BEQ X5 X0 lab29
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab27
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab28

lab27:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab28:

lab29:
//      check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab32
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab30
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab31

lab30:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab31:

lab32:
//      check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab35
//       check refcount
LW X1 0 X5
BEQ X1 X0 lab33
//       either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab34

lab33:
//       ... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab34:

lab35:
JAL X0 lab37

lab36:
//    (3) fall back to bump allocation
ADD X3 X2 64

lab37:

lab39:
//  load tag
LI X5 4
// switch xs \{ ... \};
LA X1 List40
ADD X1 X1 X5
JALR X0 X1 0

List40:
JAL X0 List40Nil
JAL X0 List40Cons

List40Nil:
// Done
JAL X0 cleanup

List40Cons:
LW X1 0 X4
BEQ X1 X0 lab42
ADD X1 X1 -1
SW X1 0 X4
LW X7 56 X4
LW X5 40 X4
LW X4 32 X4
BEQ X4 X0 lab41
LW X1 0 X4
ADD X1 X1 1
SW X1 0 X4

lab41:
JAL X0 lab43

lab42:
SW X2 0 X4
MV X2 X4
LW X7 56 X4
LW X5 40 X4
LW X4 32 X4

lab43:
// return a
MV X11 X7
JAL X0 cleanup

cleanup: