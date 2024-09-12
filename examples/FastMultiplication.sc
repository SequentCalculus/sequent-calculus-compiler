// Fast multiplication function from the introduction of the paper.
def fmult(l : ListInt) := label 'a { mult(l; 'a) };
def mult(l : ListInt, 'a :cnt Int) := case l of { Nil => 1,
                              Cons(x :Int , xs :ListInt) => ifz(x, goto(0; 'a), x * (mult(xs; 'a))) };
def main() := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))););
