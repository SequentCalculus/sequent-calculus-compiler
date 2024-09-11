def swap(x : TupIntInt) : TupIntInt := case x of { Tup(a : Int, b : Int) => Tup(b, a) };
def diag(x : Int) : TupIntInt := Tup(x, x);
def first(x : TupIntInt) : Int := case x of { Tup(a : Int, b : Int) => a };
def second(x : TupIntInt) : Int := case x of { Tup(a : Int, b : Int) => b };
def toList(x : TupIntInt ) : ListInt := case x of { Tup(a : Int, b : Int) => Cons(a, Cons(b, Nil)) };

<<<<<<< HEAD
def main() : ListInt := toList(Tup(1, 2));
=======
def main() := toList(Tup(1, 2));
>>>>>>> 7b89b63 (fixed integration tests)
