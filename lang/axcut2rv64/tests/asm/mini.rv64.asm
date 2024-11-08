mini

// actual code
main:
JAL X0 l

l:
LI X5 1
LI X7 9
JAL X0 j

j:
ADD X9 X7 X5
MV X11 X9
JAL X0 cleanup

cleanup: