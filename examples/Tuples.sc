data TupIntInt { Tup(x:Int, y:Int) }
data ListInt { Nil, Cons(x:Int, xs:ListInt) }

def swap(x : TupIntInt) : TupIntInt :=x.case { Tup(a : Int, b : Int) => Tup(b, a) };
def diag(x : Int) : TupIntInt := Tup(x, x);
def first(x : TupIntInt) : Int := x.case { Tup(a : Int, b : Int) => a };
def second(x : TupIntInt) : Int := x.case { Tup(a : Int, b : Int) => b };
def toList(x : TupIntInt ) : ListInt := x.case { Tup(a : Int, b : Int) => Cons(a, Cons(b, Nil)) };

def main() : ListInt := toList(Tup(1, 2));
