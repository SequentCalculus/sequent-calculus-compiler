// Fast multiplication function from the introduction of the paper.
def fmult(l;) := label a { mult2(l; a) };
def mult(l; a) := case l of { Nil => 1,
                           Cons(x, xs) => ifz(x, goto(0; a), x * (mult(xs; a))) };
def main(;) := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))););
