// actual code
main_:
// create t_1: ContInt = ()\{ ... \};
// #mark no allocation
MV X4 X0
// #load tag
LA X5 ContInt_1
// create k_4: ContList = (t_1)\{ ... \};
// #allocate memory
// ##store values
SW X5 56 X2
SW X4 48 X2
// ##mark unused fields with null
SW X0 16 X2
SW X0 32 X2
// ##acquire free block from heap register
MV X4 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab13
// ####initialize refcount of just acquired block
SW X0 0 X4
JAL X0 lab14

lab13:
// ###(2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab11
// ####mark linear free list empty
SW X0 0 X2
// ####erase children of next block
// #####check child 1 for erasure
LW X5 16 X2
BEQ X5 X0 lab4
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab2
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab3

lab2:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab3:

lab4:
// #####check child 2 for erasure
LW X5 32 X2
BEQ X5 X0 lab7
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab5
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab6

lab5:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab6:

lab7:
// #####check child 3 for erasure
LW X5 48 X2
BEQ X5 X0 lab10
// ######check refcount
LW X1 0 X5
BEQ X1 X0 lab8
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X5
JAL X0 lab9

lab8:
// ######... or add block to lazy free list
SW X3 0 X5
MV X3 X5

lab9:

lab10:
JAL X0 lab12

lab11:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab12:

lab14:
// #load tag
LA X5 ContList_15
// let zs_6: List = Nil();
// #mark no allocation
MV X6 X0
// #load tag
LI X7 0
// lit n_7 <- 3;
LI X9 3
// substitute (k_4 := k_4)(zs_6 := zs_6)(n_7 := n_7);
// range(...)
JAL X0 range_

ContList_15:

ContList_15_Retl:
// #load from memory
LW X1 0 X6
// ##check refcount
BEQ X1 X0 lab17
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
// ###load values
LW X7 56 X6
LW X6 48 X6
BEQ X6 X0 lab16
// ####increment refcount
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab16:
JAL X0 lab18

lab17:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X7 56 X6
LW X6 48 X6

lab18:
// substitute (t_1 := t_1)(as_5 := as_5);
// #move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// sum(...)
JAL X0 sum_

ContInt_1:

ContInt_1_Reti:
// exit r_2
MV X10 X5
JAL X0 cleanup

range_:
// if i_10 == 0 \{ ... \}
BEQ X9 X0 lab19
// else branch
// substitute (n_11 := i_10)(k_8 := k_8)(xs_9 := xs_9)(i_10 := i_10);
// #move variables
MV X8 X6
MV X6 X4
MV X1 X9
MV X11 X9
MV X9 X7
MV X7 X5
MV X5 X1
// let ys_12: List = Cons(xs_9, i_10);
// #allocate memory
// ##store values
SW X11 56 X2
SW X0 48 X2
SW X9 40 X2
SW X8 32 X2
// ##mark unused fields with null
SW X0 16 X2
// ##acquire free block from heap register
MV X8 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab31
// ####initialize refcount of just acquired block
SW X0 0 X8
JAL X0 lab32

lab31:
// ###(2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab29
// ####mark linear free list empty
SW X0 0 X2
// ####erase children of next block
// #####check child 1 for erasure
LW X9 16 X2
BEQ X9 X0 lab22
// ######check refcount
LW X1 0 X9
BEQ X1 X0 lab20
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab21

lab20:
// ######... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab21:

lab22:
// #####check child 2 for erasure
LW X9 32 X2
BEQ X9 X0 lab25
// ######check refcount
LW X1 0 X9
BEQ X1 X0 lab23
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab24

lab23:
// ######... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab24:

lab25:
// #####check child 3 for erasure
LW X9 48 X2
BEQ X9 X0 lab28
// ######check refcount
LW X1 0 X9
BEQ X1 X0 lab26
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X9
JAL X0 lab27

lab26:
// ######... or add block to lazy free list
SW X3 0 X9
MV X3 X9

lab27:

lab28:
JAL X0 lab30

lab29:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab30:

lab32:
// #load tag
LI X9 4
// lit o_13 <- -1;
LI X11 -1
// j_14 <- n_11 + o_13;
ADD X13 X5 X11
// substitute (k_8 := k_8)(ys_12 := ys_12)(j_14 := j_14);
// #move variables
MV X4 X6
MV X5 X7
MV X6 X8
MV X7 X9
MV X9 X13
// range(...)
JAL X0 range_

lab19:
// then branch
// substitute (xs_9 := xs_9)(k_8 := k_8);
// #move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// invoke k_8 Retl
// #there is only one clause, so we can jump there directly
JALR X0 X7 0

sum_:
// switch xs_16 \{ ... \};
LA X1 List_33
ADD X1 X1 X7
JALR X0 X1 0

List_33:
JAL X0 List_33_Nil
JAL X0 List_33_Cons

List_33_Nil:
// lit z_17 <- 0;
LI X7 0
// substitute (z_17 := z_17)(k_15 := k_15);
// #move variables
MV X6 X4
MV X1 X7
MV X7 X5
MV X5 X1
// invoke k_15 Reti
// #there is only one clause, so we can jump there directly
JALR X0 X7 0

List_33_Cons:
// #load from memory
LW X1 0 X6
// ##check refcount
BEQ X1 X0 lab35
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
// ###load values
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6
BEQ X6 X0 lab34
// ####increment refcount
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab34:
JAL X0 lab36

lab35:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6

lab36:
// substitute (ys_18 := ys_18)(k_15 := k_15)(y_19 := y_19);
// #move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// create j_20: ContInt = (k_15, y_19)\{ ... \};
// #allocate memory
// ##store values
SW X9 56 X2
SW X0 48 X2
SW X7 40 X2
SW X6 32 X2
// ##mark unused fields with null
SW X0 16 X2
// ##acquire free block from heap register
MV X6 X2
// ##get next free block into heap register
// ###(1) check linear free list for next block
LW X2 0 X2
BEQ X2 X0 lab48
// ####initialize refcount of just acquired block
SW X0 0 X6
JAL X0 lab49

lab48:
// ###(2) check non-linear lazy free list for next block
MV X2 X3
LW X3 0 X3
BEQ X3 X0 lab46
// ####mark linear free list empty
SW X0 0 X2
// ####erase children of next block
// #####check child 1 for erasure
LW X7 16 X2
BEQ X7 X0 lab39
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab37
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab38

lab37:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab38:

lab39:
// #####check child 2 for erasure
LW X7 32 X2
BEQ X7 X0 lab42
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab40
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab41

lab40:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab41:

lab42:
// #####check child 3 for erasure
LW X7 48 X2
BEQ X7 X0 lab45
// ######check refcount
LW X1 0 X7
BEQ X1 X0 lab43
// ######either decrement refcount ...
ADD X1 X1 -1
SW X1 0 X7
JAL X0 lab44

lab43:
// ######... or add block to lazy free list
SW X3 0 X7
MV X3 X7

lab44:

lab45:
JAL X0 lab47

lab46:
// ###(3) fall back to bump allocation
ADD X3 X2 64

lab47:

lab49:
// #load tag
LA X7 ContInt_50
// substitute (j_20 := j_20)(ys_18 := ys_18);
// #move variables
MV X1 X6
MV X6 X4
MV X4 X1
MV X1 X7
MV X7 X5
MV X5 X1
// sum(...)
JAL X0 sum_

ContInt_50:

ContInt_50_Reti:
// #load from memory
LW X1 0 X6
// ##check refcount
BEQ X1 X0 lab52
// ##either decrement refcount and share children...
ADD X1 X1 -1
SW X1 0 X6
// ###load values
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6
BEQ X6 X0 lab51
// ####increment refcount
LW X1 0 X6
ADD X1 X1 1
SW X1 0 X6

lab51:
JAL X0 lab53

lab52:
// ##... or release blocks onto linear free list when loading
// ###release block
SW X2 0 X6
MV X2 X6
// ###load values
LW X9 56 X6
LW X7 40 X6
LW X6 32 X6

lab53:
// s_22 <- y_19 + r_21;
ADD X11 X9 X5
// substitute (s_22 := s_22)(k_15 := k_15);
// #move variables
MV X5 X11
// invoke k_15 Reti
// #there is only one clause, so we can jump there directly
JALR X0 X7 0

cleanup: