data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
// Fast multiplication function from the introduction of the paper.
def fmult(l : ListI64) : i64 { label a { mult(l,a) } }
def mult(l : ListI64, a :cns i64) : i64 { l.case { Nil => println_i64(21); 1,
                                                    Cons(x :i64 , xs :ListI64) => println_i64(42); ifz(x) { goto(0; a) } else { x * mult(xs, a) }}}
def main() : i64 { println_i64(fmult(Cons(2, Cons(0, Cons(3, Cons(3, Nil))))));
                   0 }
