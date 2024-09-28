def map(f : FunIntInt , l : ListInt) : ListInt := case l of { Nil => Nil,
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs)) };
def mult(x : ListInt) : Int := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys)) };
def foldr(f : FunIntInt, st : Int , l : ListInt) : Int := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.Ap(y).Ap(st), ys)};
def len(l : ListInt) : Int := case l of { Nil => 0, Cons(x:Int,xs:ListInt) => len(xs) };

def main() : Int := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))));
