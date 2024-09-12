<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
def map(f : FunIntInt , l : ListInt) : ListInt := case l of { Nil => Nil,
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs;)) };
def mult(x : ListInt) : Int := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys;)) };
def foldr(f : FunIntInt, st : Int , l : ListInt) : Int := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : ListInt) : Int := case l of { Nil => 0,
=======
def map(f : Fun , l : ListInt) := case l of { Nil => Nil,
=======
def map(f : FunIntInt , l : ListInt) := case l of { Nil => Nil,
>>>>>>> d217cba (renamed fun to funintint)
                              Cons(x : Int, xs : ListInt) => Cons(f, map(f, xs;)) };
def mult(x : ListInt) := case x of { Nil => 1,
                            Cons(y :Int, ys : ListInt) => y * (mult(ys;)) };
def foldr(f : FunIntInt, st : Int , l : ListInt) := case l of { Nil => st,
                                    Cons(y : Int , ys : ListInt) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : ListInt) := case l of { Nil => 0,
>>>>>>> d5525c7 (renamed Listint to ListInt)
                           Cons(y : Int , ys :ListInt) => 1 + (len(ys;))};

def main() : Int := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))););
=======
def map(f : Fun , l : Listint) := case l of { Nil => Nil,
                              Cons(x : Int, xs : Listint) => Cons(f, map(f, xs)) };
def mult(x : Listint) := case x of { Nil => 1,
                            Cons(y :Int, ys : Listint) => y * (mult(ys)) };
def foldr(f : Fun, st : Int , l : Listint) := case l of { Nil => st,
                                    Cons(y : Int , ys : Listint) => foldr(f, f.ap(y).ap(st), ys)};
def len(l : Listint) := case l of { Nil => 0,
                           Cons(y : Int , ys :Listint) => 1 + (len(ys))};

<<<<<<< HEAD
def main() := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))););
>>>>>>> 8eb76bc (fixed integration tests)
=======
def main() := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))));
>>>>>>> 7b89b63 (fixed integration tests)
