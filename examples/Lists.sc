<<<<<<< HEAD
<<<<<<< HEAD
def map(f : FunIntInt , l : ListInt) : ListInt := case l of { Nil => Nil,
=======
def map(f : FunIntInt , l : ListInt) : ListIntInt := case l of { Nil => Nil,
>>>>>>> d1a7180 (fixed list exmaple)
=======
def map(f : FunIntInt , l : ListInt) : ListInt := case l of { Nil => Nil,
>>>>>>> 06381c8 (fixed list again)
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs;)) };
def mult(x : ListInt) : Int := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys;)) };
def foldr(f : FunIntInt, st : Int , l : ListInt) : Int := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : ListInt) : Int := case l of { Nil => 0,
def map(f : FunIntInt , l : ListInt) : ListIntINt := case l of { Nil => Nil,
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs;)) };
def mult(x : ListInt) : Int := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys;)) };
def foldr(f : FunIntInt, st : Int , l : ListInt) : Int := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : ListInt) := case l of { Nil => 0,
                           Cons(y : Int , ys :ListInt) => 1 + (len(ys;))};

def main() : Int := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))););
