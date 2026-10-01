data Pair[A, B] { Tup(x:A,y:B) }
data List[A] { Nil, Cons(x: A, xs: List[A]) }

def swap(x : Pair[i64, i64]) : Pair[i64, i64] { x.case[i64, i64] { Tup(a, b) => Tup(b, a) } }
def diag(x : i64) : Pair[i64, i64] { Tup(x, x) }
def first(x : Pair[i64, i64]) : i64 { x.case[i64, i64] { Tup(a, b ) => a } }
def second(x : Pair[i64, i64]) : i64 { x.case[i64, i64] { Tup(a, b) => b } }
def toList(x : Pair[i64, i64] ) : List[i64] { x.case[i64, i64] { Tup(a, b) => Cons(a, Cons(b, Nil)) } }

def main() : i64 { println_i64(second(Tup(1, 2)));
                   0 }
