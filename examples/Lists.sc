def map(f : Fun , l : Listint) := case l of { Nil => Nil,
                              Cons(x : Int, xs : Listint) => Cons(f, map(f, xs;)) };
def mult(x : Listint) := case x of { Nil => 1,
                            Cons(y :Int, ys : Listint) => y * (mult(ys;)) };
def foldr(f : Fun, st : Int , l : Listint) := case l of { Nil => st,
                                    Cons(y : Int , ys : Listint) => foldr(f, f.ap(y).ap(st), ys;)};
def len(l : Listint) := case l of { Nil => 0,
                           Cons(y : Int , ys :Listint) => 1 + (len(ys;))};

def main() := len(Cons(1, Cons(2, Cons(3, Cons(4, Nil)))););
