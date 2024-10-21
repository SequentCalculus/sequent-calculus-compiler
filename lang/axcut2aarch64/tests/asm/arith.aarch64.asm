// To create an executable:
// $ as -o arith.aarch64.o arith.aarch64.asm
// $ gcc -o arith path/to/AARCH64-infrastructure/driver$MODE.c arith.aarch64.o
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
MOV X4, 1
MOV X6, 3
SUB X8, X4, X6
MOV X10, 8
MOV X12, -1
MUL X14, X12, X10
ADD X16, X14, X8
MOV X18, -6
MUL X20, X18, X16
MOV X1, X20
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