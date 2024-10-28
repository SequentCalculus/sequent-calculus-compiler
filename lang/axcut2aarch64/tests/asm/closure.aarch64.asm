// To create an executable:
// $ as -o closure.aarch64.o closure.aarch64.asm
// $ gcc -o closure path/to/AARCH64-infrastructure/driver$MODE.c closure.aarch64.o
// where $MODE = Args | Debug

.text
  .global asm_main0, _asm_main0
  .global asm_main1, _asm_main1
  .global asm_main2, _asm_main2
  .global asm_main3, _asm_main3
  .global asm_main4, _asm_main4
  .global asm_main5, _asm_main5
  .global asm_main6, _asm_main6
  .global asm_main7, _asm_main7
asm_main0:
_asm_main0:
asm_main1:
_asm_main1:
asm_main2:
_asm_main2:
asm_main3:
_asm_main3:
asm_main4:
_asm_main4:
asm_main5:
_asm_main5:
asm_main6:
_asm_main6:
asm_main7:
_asm_main7:
// setup
// save registers
str X16, [sp, -16]!
str X17, [sp, -16]!
str X18, [sp, -16]!
str X19, [sp, -16]!
str X20, [sp, -16]!
str X21, [sp, -16]!
str X22, [sp, -16]!
str X23, [sp, -16]!
str X24, [sp, -16]!
str X25, [sp, -16]!
str X26, [sp, -16]!
str X27, [sp, -16]!
str X28, [sp, -16]!
str X29, [sp, -16]!
str X30, [sp, -16]!

// move parameters into place

// initialize free pointer
MOV X1, X0
ADD X1, X1, 64

// actual code
main:
MOV X4, 9
STR X4, [ X0, 56 ]
MOV X2, 0
STR X2, [ X0, 48 ]
MOV X2, 0
STR X2, [ X0, 32 ]
MOV X2, 0
STR X2, [ X0, 16 ]
MOV X3, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab12
MOV X2, 0
STR X2, [ X3, 0 ]
B lab13

lab12:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab10
MOV X2, 0
STR X2, [ X0, 0 ]
LDR X4, [ X0, 48 ]
CMP X4, 0
BEQ lab3
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab1
ADD X2, X2, -1
STR X2, [ X4, 0 ]
B lab2

lab1:
STR X1, [ X4, 0 ]
MOV X1, X4

lab2:

lab3:
LDR X4, [ X0, 32 ]
CMP X4, 0
BEQ lab6
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab4
ADD X2, X2, -1
STR X2, [ X4, 0 ]
B lab5

lab4:
STR X1, [ X4, 0 ]
MOV X1, X4

lab5:

lab6:
LDR X4, [ X0, 16 ]
CMP X4, 0
BEQ lab9
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab7
ADD X2, X2, -1
STR X2, [ X4, 0 ]
B lab8

lab7:
STR X1, [ X4, 0 ]
MOV X1, X4

lab8:

lab9:
B lab11

lab10:
ADD X1, X0, 64

lab11:

lab13:
ADR X4, Func14
MOV X5, 0
ADR X6, Cont15
MOV X8, 1
MOV X7, X3
MOV X2, X8
MOV X8, X4
MOV X4, X2
BR X8

Cont15:

Cont15Ret:
MOV X1, X4
B cleanup

Func14:

Func14Ap:
LDR X2, [ X7, 0 ]
CMP X2, 0
BEQ lab16
ADD X2, X2, -1
STR X2, [ X7, 0 ]
LDR X8, [ X7, 56 ]
B lab17

lab16:
STR X0, [ X7, 0 ]
MOV X0, X7
LDR X8, [ X7, 56 ]

lab17:
ADD X10, X8, X4
MOV X4, X10
BR X6

// cleanup
cleanup:
// restore registers
ldr X30, [sp], 16
ldr X29, [sp], 16
ldr X28, [sp], 16
ldr X27, [sp], 16
ldr X26, [sp], 16
ldr X25, [sp], 16
ldr X24, [sp], 16
ldr X23, [sp], 16
ldr X22, [sp], 16
ldr X21, [sp], 16
ldr X20, [sp], 16
ldr X19, [sp], 16
ldr X18, [sp], 16
ldr X17, [sp], 16
ldr X16, [sp], 16
ret