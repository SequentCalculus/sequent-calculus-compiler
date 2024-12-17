data TupI64I64 { Tup(x:i64, y:i64) }
data ListI64 { Nil, Cons(x:i64, xs:ListI64) }

def swap(x : TupI64I64) : TupI64I64 :=x.case { Tup(a : i64, b : i64) => Tup(b, a) };
def diag(x : i64) : TupI64I64 := Tup(x, x);
def first(x : TupI64I64) : i64 := x.case { Tup(a : i64, b : i64) => a };
def second(x : TupI64I64) : i64 := x.case { Tup(a : i64, b : i64) => b };
def toList(x : TupI64I64 ) : ListI64 := x.case { Tup(a : i64, b : i64) => Cons(a, Cons(b, Nil)) };

def main() : i64 := second(Tup(1, 2));
