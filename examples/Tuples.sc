<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 27de75f (added parsing definition types)
def swap(x : TupIntInt) : TupIntInt := case x of { Tup(a : Int, b : Int) => Tup(b, a) };
def diag(x : Int) : TupIntInt := Tup(x, x);
def first(x : TupIntInt) : Int := case x of { Tup(a : Int, b : Int) => a };
def second(x : TupIntInt) : Int := case x of { Tup(a : Int, b : Int) => b };
def toList(x : TupIntInt ) : ListInt := case x of { Tup(a : Int, b : Int) => Cons(a, Cons(b, Nil)) };
<<<<<<< HEAD

def main() : ListInt := toList(Tup(1, 2););
=======
def swap(x : TupInt) := case x of { Tup(a : Int, b : Int) => Tup(b, a) };
=======
def swap(x : TupIntInt) := case x of { Tup(a : Int, b : Int) => Tup(b, a) };
>>>>>>> 8e5d518 (renamed tupint to tupintint)
def diag(x : Int) := Tup(x, x);
def first(x : TupIntInt) := case x of { Tup(a : Int, b : Int) => a };
def second(x : TupIntInt) := case x of { Tup(a : Int, b : Int) => b };
def toList(x : TupIntInt ) := case x of { Tup(a : Int, b : Int) => Cons(a, Cons(b, Nil)) };

<<<<<<< HEAD
def main() := toList(Tup(1, 2););
>>>>>>> 8eb76bc (fixed integration tests)
=======
def main() := toList(Tup(1, 2));
>>>>>>> 7b89b63 (fixed integration tests)
=======

def main() : ListInt := toList(Tup(1, 2););
>>>>>>> 27de75f (added parsing definition types)
