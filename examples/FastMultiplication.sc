data ListInt { Nil, Cons(x:Int,xs:ListInt) }
// Fast multiplication function from the introduction of the paper.
def fmult(l : ListInt) : Int := label 'a { mult(l,'a) };
def mult(l : ListInt, 'a :cnt Int) : Int := l.case { Nil => 1,
                              Cons(x :Int , xs :ListInt) => ifz(x, goto(0; 'a), x * (mult(xs, 'a))) };
def main() : Int := fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil)))));
