// actual code
main:
// lit z <- 1;
LI X5 1
// lit x <- 9;
LI X7 9
// leta p: Either = Right(x);
// #allocate memory
// ##store values
SW X7 56 X2
SW X0 48 X2
// ##mark unused fields with null
SW X0 32 X2
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
// #####check child 3 for erasure
LW X7 48 X2
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
// #####check child 1 for erasure
LW X7 16 X2
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
// #load tag
LI X7 4
// switch p \{ ... \};
LA X1 Either14
ADD X1 X1 X7
JALR X0 X1 0

Either14:
JAL X0 Either14Left
JAL X0 Either14Right

Either14Left:
// #load from memory
LW X1 0 X6
// ##check refcount
BEQ X1 X0 lab15
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
// ###load values
LW X7 56 X6
JAL X0 lab16

lab15:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X7 56 X6

lab16:
// Done
JAL X0 cleanup

Either14Right:
// #load from memory
LW X1 0 X6
// ##check refcount
BEQ X1 X0 lab17
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
// ###load values
LW X7 56 X6
JAL X0 lab18

lab17:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X7 56 X6

lab18:
// c <- b + z;
ADD X9 X7 X5
// return c
MV X11 X9
JAL X0 cleanup

cleanup: