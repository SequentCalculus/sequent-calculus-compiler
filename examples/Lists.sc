data ListInt { Nil, Cons(x:Int,xs:ListInt) }
codata FunIntInt { Ap(x:Int) : Int }

def map(f : FunIntInt , l : ListInt) : ListInt :=
    case l of { Nil => Nil,
                Cons(x : Int, xs : ListInt) => Cons(f.Ap(x), map(f, xs)) };

def mult(x : ListInt) : Int :=
    case x of { Nil => 1,
                Cons(y :Int, ys : ListInt) => y * (mult(ys)) };

codata FunIntIntInt { Ap2(x: Int, y: Int): Int }

def foldr(f : FunIntIntInt, st : Int , l : ListInt) : Int :=
    case l of { Nil => st,
                Cons(y : Int , ys : ListInt) => f.Ap2(y, foldr(f, st, ys)) };

def len(l : ListInt) : Int :=
    case l of { Nil => 0,
                Cons(x:Int,xs:ListInt) => 1 + (len(xs)) };

def main() : Int := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))));
