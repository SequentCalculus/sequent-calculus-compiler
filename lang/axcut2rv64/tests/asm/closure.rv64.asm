// actual code
main:
// lit a <- 9;
LI X5 9
// new f: Func = (a)\{ ... \};
// #allocate memory
// ##store values
SW X5 56 X2
SW X0 48 X2
// ##mark unused fields with null
SW X0 32 X2
SW X0 16 X2
// ##acquire free block from heap register
MV X4 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab12
// ####initialize refcount of just acquired block
SW X0 0 X4
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
LW X5 48 X2
BEQ X5 X0 lab3
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab1
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab2

lab1:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab2:

lab3:
// #####check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab6
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab4
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab5

lab4:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab5:

lab6:
// #####check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab9
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab7
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab8

lab7:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab8:

lab9:
JAL X0 lab11

lab10:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab11:

lab13:
// #load tag
LA X5 Func14
// new k: Cont = ()\{ ... \};
// #mark no allocation
MV X6 X0
// #load tag
LA X7 Cont15
// lit y <- 1;
LI X9 1
// substitute (y !-> y)(k !-> k)(f !-> f);
// #move variables
MV X8 X4
MV X1 X9
MV X9 X5
MV X5 X1
// invoke f Ap
JALR X0 X9 0

Cont15:

Cont15Ret:
// return r
MV X10 X5
JAL X0 cleanup

Func14:

Func14Ap:
// #load from memory
LW X1 0 X8
// ##check refcount
BEQ X1 X0 lab16
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X8
// ###load values
LW X9 56 X8
JAL X0 lab17

lab16:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X8
MV X2 X8
// ###load values
LW X9 56 X8

lab17:
// b <- a + x;
ADD X11 X9 X5
// substitute (b !-> b)(k !-> k);
// #move variables
MV X5 X11
// invoke k Ret
JALR X0 X7 0

cleanup: