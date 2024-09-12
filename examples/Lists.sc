def map(f : Fun , l : ListInt) := case l of { Nil => Nil,
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs;)) };
def mult(x : ListInt) := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys;)) };
def foldr(f : Fun, st : Int , l : ListInt) := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : ListInt) := case l of { Nil => 0,
                           Cons(y : Int , ys :ListInt) => 1 + (len(ys;))};

def main() := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))););
