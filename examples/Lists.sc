data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
codata FunI64I64 { Ap(x:i64) : i64 }

def map(f : FunI64I64 , l : ListI64) : ListI64 :=
    l.case { Nil => Nil,
             Cons(x : i64, xs : ListI64) => Cons(f.Ap(x), map(f, xs)) };

def mult(x : ListI64) : i64 :=
    x.case { Nil => 1,
             Cons(y :i64, ys : ListI64) => y * mult(ys) };

codata FunI64I64I64 { Ap2(x: i64, y: i64): i64 }

def foldr(f : FunI64I64I64, st : i64 , l : ListI64) : i64 :=
    l.case { Nil => st,
             Cons(y : i64 , ys : ListI64) => f.Ap2(y, foldr(f, st, ys)) };

def len(l : ListI64) : i64 :=
    l.case { Nil => 0,
             Cons(x:i64,xs:ListI64) => 1 + len(xs) };

def main() : i64 := len(Cons(1 + 2, Cons(2, Cons(3, Cons(4, Nil)))));
