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
STR X16, [sp, -16]!
STR X17, [sp, -16]!
STR X18, [sp, -16]!
STR X19, [sp, -16]!
STR X20, [sp, -16]!
STR X21, [sp, -16]!
STR X22, [sp, -16]!
STR X23, [sp, -16]!
STR X24, [sp, -16]!
STR X25, [sp, -16]!
STR X26, [sp, -16]!
STR X27, [sp, -16]!
STR X28, [sp, -16]!
STR X29, [sp, -16]!
STR X30, [sp, -16]!

// move parameters into place

// initialize free pointer
MOV X1, X0
ADD X1, X1, 64

// actual code
main:
B l

l:
MOVZ X4, 1, LSL 0
MOVZ X6, 9, LSL 0
B j

j:
ADD X8, X6, X4
MOV X1, X8
B cleanup

// cleanup
cleanup:
// restore registers
LDR X30, [sp], 16
LDR X29, [sp], 16
LDR X28, [sp], 16
LDR X27, [sp], 16
LDR X26, [sp], 16
LDR X25, [sp], 16
LDR X24, [sp], 16
LDR X23, [sp], 16
LDR X22, [sp], 16
LDR X21, [sp], 16
LDR X20, [sp], 16
LDR X19, [sp], 16
LDR X18, [sp], 16
LDR X17, [sp], 16
LDR X16, [sp], 16
RET