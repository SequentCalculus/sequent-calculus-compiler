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
MOV X3, 0
ADR X4, ContInt3
STR X4, [ X0, 56 ]
STR X3, [ X0, 48 ]
MOV X2, 0
STR X2, [ X0, 32 ]
MOV X2, 0
STR X2, [ X0, 16 ]
MOV X3, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab15
MOV X2, 0
STR X2, [ X3, 0 ]
B lab16

lab15:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab13
MOV X2, 0
STR X2, [ X0, 0 ]
LDR X4, [ X0, 48 ]
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
LDR X4, [ X0, 32 ]
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
LDR X4, [ X0, 16 ]
CMP X4, 0
BEQ lab12
LDR X2, [ X4, 0 ]
CMP X2, 0
BEQ lab10
ADD X2, X2, -1
STR X2, [ X4, 0 ]
B lab11

lab10:
STR X1, [ X4, 0 ]
MOV X1, X4

lab11:

lab12:
B lab14

lab13:
ADD X1, X0, 64

lab14:

lab16:
ADR X4, ContList17
MOV X5, 0
MOV X6, 0
MOV X8, 3
B range

ContList17:

ContList17Retl:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab20
ADD X2, X2, -1
STR X2, [ X5, 0 ]
LDR X6, [ X5, 56 ]
LDR X5, [ X5, 48 ]
CMP X5, 0
BEQ lab19
LDR X2, [ X5, 0 ]
ADD X2, X2, 1
STR X2, [ X5, 0 ]

lab19:
B lab21

lab20:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X6, [ X5, 56 ]
LDR X5, [ X5, 48 ]

lab21:
MOV X2, X5
MOV X5, X3
MOV X3, X2
MOV X2, X6
MOV X6, X4
MOV X4, X2
B sum

ContInt3:

ContInt3Reti:
MOV X1, X4
B cleanup

range:
CMP X8, 0
BEQ lab22
MOV X7, X5
MOV X5, X3
MOV X2, X8
MOV X10, X8
MOV X8, X6
MOV X6, X4
MOV X4, X2
STR X10, [ X0, 56 ]
MOV X2, 0
STR X2, [ X0, 48 ]
STR X8, [ X0, 40 ]
STR X7, [ X0, 32 ]
MOV X2, 0
STR X2, [ X0, 16 ]
MOV X7, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab34
MOV X2, 0
STR X2, [ X7, 0 ]
B lab35

lab34:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab32
MOV X2, 0
STR X2, [ X0, 0 ]
LDR X8, [ X0, 48 ]
CMP X8, 0
BEQ lab25
LDR X2, [ X8, 0 ]
CMP X2, 0
BEQ lab23
ADD X2, X2, -1
STR X2, [ X8, 0 ]
B lab24

lab23:
STR X1, [ X8, 0 ]
MOV X1, X8

lab24:

lab25:
LDR X8, [ X0, 32 ]
CMP X8, 0
BEQ lab28
LDR X2, [ X8, 0 ]
CMP X2, 0
BEQ lab26
ADD X2, X2, -1
STR X2, [ X8, 0 ]
B lab27

lab26:
STR X1, [ X8, 0 ]
MOV X1, X8

lab27:

lab28:
LDR X8, [ X0, 16 ]
CMP X8, 0
BEQ lab31
LDR X2, [ X8, 0 ]
CMP X2, 0
BEQ lab29
ADD X2, X2, -1
STR X2, [ X8, 0 ]
B lab30

lab29:
STR X1, [ X8, 0 ]
MOV X1, X8

lab30:

lab31:
B lab33

lab32:
ADD X1, X0, 64

lab33:

lab35:
MOV X8, 4
MOV X10, -1
ADD X12, X4, X10
MOV X3, X5
MOV X4, X6
MOV X5, X7
MOV X6, X8
MOV X8, X12
B range

lab22:
MOV X2, X5
MOV X5, X3
MOV X3, X2
MOV X2, X6
MOV X6, X4
MOV X4, X2
BR X6

sum:
ADR X2, List36
ADD X2, X2, X6
BR X2

List36:
B List36Nil
B List36Cons

List36Nil:
MOV X6, 0
MOV X5, X3
MOV X2, X6
MOV X6, X4
MOV X4, X2
BR X6

List36Cons:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab39
ADD X2, X2, -1
STR X2, [ X5, 0 ]
LDR X8, [ X5, 56 ]
LDR X6, [ X5, 40 ]
LDR X5, [ X5, 32 ]
CMP X5, 0
BEQ lab38
LDR X2, [ X5, 0 ]
ADD X2, X2, 1
STR X2, [ X5, 0 ]

lab38:
B lab40

lab39:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X8, [ X5, 56 ]
LDR X6, [ X5, 40 ]
LDR X5, [ X5, 32 ]

lab40:
MOV X2, X5
MOV X5, X3
MOV X3, X2
MOV X2, X6
MOV X6, X4
MOV X4, X2
STR X8, [ X0, 56 ]
MOV X2, 0
STR X2, [ X0, 48 ]
STR X6, [ X0, 40 ]
STR X5, [ X0, 32 ]
MOV X2, 0
STR X2, [ X0, 16 ]
MOV X5, X0
LDR X0, [ X0, 0 ]
CMP X0, 0
BEQ lab52
MOV X2, 0
STR X2, [ X5, 0 ]
B lab53

lab52:
MOV X0, X1
LDR X1, [ X1, 0 ]
CMP X1, 0
BEQ lab50
MOV X2, 0
STR X2, [ X0, 0 ]
LDR X6, [ X0, 48 ]
CMP X6, 0
BEQ lab43
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab41
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab42

lab41:
STR X1, [ X6, 0 ]
MOV X1, X6

lab42:

lab43:
LDR X6, [ X0, 32 ]
CMP X6, 0
BEQ lab46
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab44
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab45

lab44:
STR X1, [ X6, 0 ]
MOV X1, X6

lab45:

lab46:
LDR X6, [ X0, 16 ]
CMP X6, 0
BEQ lab49
LDR X2, [ X6, 0 ]
CMP X2, 0
BEQ lab47
ADD X2, X2, -1
STR X2, [ X6, 0 ]
B lab48

lab47:
STR X1, [ X6, 0 ]
MOV X1, X6

lab48:

lab49:
B lab51

lab50:
ADD X1, X0, 64

lab51:

lab53:
ADR X6, ContInt54
MOV X2, X5
MOV X5, X3
MOV X3, X2
MOV X2, X6
MOV X6, X4
MOV X4, X2
B sum

ContInt54:

ContInt54Reti:
LDR X2, [ X5, 0 ]
CMP X2, 0
BEQ lab57
ADD X2, X2, -1
STR X2, [ X5, 0 ]
LDR X8, [ X5, 56 ]
LDR X6, [ X5, 40 ]
LDR X5, [ X5, 32 ]
CMP X5, 0
BEQ lab56
LDR X2, [ X5, 0 ]
ADD X2, X2, 1
STR X2, [ X5, 0 ]

lab56:
B lab58

lab57:
STR X0, [ X5, 0 ]
MOV X0, X5
LDR X8, [ X5, 56 ]
LDR X6, [ X5, 40 ]
LDR X5, [ X5, 32 ]

lab58:
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