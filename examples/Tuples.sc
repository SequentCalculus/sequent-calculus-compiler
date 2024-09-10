def swap(x : TupInt) := case x of { Tup(a : Int, b : Int) => Tup(b, a) };
def diag(x : Int) := Tup(x, x);
def first(x : TupInt) := case x of { Tup(a : Int, b : Int) => a };
def second(x : TupInt) := case x of { Tup(a : Int, b : Int) => b };
def toList(x : TupInt ) := case x of { Tup(a : Int, b : Int) => Cons(a, Cons(b, Nil)) };

def main() := toList(Tup(1, 2););
