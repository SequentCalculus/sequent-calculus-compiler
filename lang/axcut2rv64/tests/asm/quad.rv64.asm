// actual code
main:
// lit z <- 8;
LI X5 8
// lit y <- 6;
LI X7 6
// lit x <- 4;
LI X9 4
// lit w <- 2;
LI X11 2
// leta q: Quad = Q(z, y, x, w);
// #allocate memory
// ##store values
SW X11 56 X2
SW X0 48 X2
SW X9 40 X2
SW X0 32 X2
SW X7 24 X2
SW X0 16 X2
// ##acquire free block from heap register
MV X6 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab12
// ####initialize refcount of just acquired block
SW X0 0 X6
JAL X0 lab13

lab12:
// ###(2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab10
// ####mark linear free list empty
SW X0 0 X2
// ####erase children of next block
// #####check child 1 for erasure
LW X7 16 X2
BEQ X7 X0 lab3
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab1
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab2

lab1:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab2:

lab3:
// #####check child 2 for erasure
LW X7 32 X2
BEQ X7 X0 lab6
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab4
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab5

lab4:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab5:

lab6:
// #####check child 3 for erasure
LW X7 48 X2
BEQ X7 X0 lab9
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab7
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab8

lab7:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab8:

lab9:
JAL X0 lab11

lab10:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab11:

lab13:
// ##store link to previous block
SW X6 48 X2
// ##store values
SW X5 40 X2
SW X0 32 X2
// ##mark unused fields with null
SW X0 16 X2
// ##acquire free block from heap register
MV X4 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab25
// ####initialize refcount of just acquired block
SW X0 0 X4
JAL X0 lab26

lab25:
// ###(2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab23
// ####mark linear free list empty
SW X0 0 X2
// ####erase children of next block
// #####check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab16
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab14
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab15

lab14:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab15:

lab16:
// #####check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab19
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab17
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab18

lab17:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab18:

lab19:
// #####check child 3 for erasure
LW X5 48 X2
BEQ X5 X0 lab22
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab20
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab21

lab20:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab21:

lab22:
JAL X0 lab24

lab23:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab24:

lab26:
// #load tag
LI X5 0
// switch q \{ ... \};
LA X1 Quad_27
JALR X0 X1 0

Quad_27:

Quad_27_Q:
// #load from memory
LW X1 0 X4
// ##check refcount
BEQ X1 X0 lab28
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X4
// ###load link to next block
LW X6 48 X4
// ###load values
LW X5 40 X4
// ###load values
LW X11 56 X6
LW X9 40 X6
LW X7 24 X6
JAL X0 lab29

lab28:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X4
MV X2 X4
// ###load link to next block
LW X6 48 X4
// ###load values
LW X5 40 X4
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X11 56 X6
LW X9 40 X6
LW X7 24 X6

lab29:
// lit z <- 7;
LI X13 7
// e <- d + z;
ADD X15 X5 X13
// return e
MV X10 X15
JAL X0 cleanup

cleanup: