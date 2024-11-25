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
MOV X4, 1
MOV X6, 9
STR X6, [ X0, 56 ]
MOV X2, 0
STR X2, [ X0, 48 ]
MOV X2, 0
STR X2, [ X0, 32 ]
MOV X2, 0
STR X2, [ X0, 16 ]
MOV X5, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab12
MOV X2, 0
STR X2, [ X5, 0 ]
B lab13

lab12:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab10
MOV X2, 0
STR X2, [ X0, 0 ]
LDR X6, [ X0, 48 ]
CMP X6, 0
BEQ lab3
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab1
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab2

lab1:
STR X1, [ X6, 0 ]
MOV X1, X6

lab2:

lab3:
LDR X6, [ X0, 32 ]
CMP X6, 0
BEQ lab6
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab4
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab5

lab4:
STR X1, [ X6, 0 ]
MOV X1, X6

lab5:

lab6:
LDR X6, [ X0, 16 ]
CMP X6, 0
BEQ lab9
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab7
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab8

lab7:
STR X1, [ X6, 0 ]
MOV X1, X6

lab8:

lab9:
B lab11

lab10:
ADD X1, X0, 64

lab11:

lab13:
MOV X6, 4
ADR X2, Either14
ADD X2, X2, X6
BR X2

Either14:
B Either14Left
B Either14Right

Either14Left:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab15
ADD X2, X2, -1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab16

lab15:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab16:
B cleanup

Either14Right:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab17
ADD X2, X2, -1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
B lab18

lab17:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]

lab18:
ADD X8, X6, X4
MOV X1, X8
B cleanup

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