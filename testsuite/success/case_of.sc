data List[A] { Nil, Cons(x: A, xs: List[A]) }

def isEmpty(xs: List[i64]): i64 { xs.case[i64] { Nil => 0, Cons(x: i64,xs: List[i64]) => 1 } }

def safeHead(xs: List[i64]): i64 { xs.case[i64] { Nil => 0, Cons(y: i64, ys: List[i64]) => y}}
