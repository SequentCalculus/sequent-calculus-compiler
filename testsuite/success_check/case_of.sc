data List[A] { Nil, Cons(x: A, xs: List[A]) }

def isEmpty(xs: List[i64]): i64 { xs.case[i64] { Nil => 0, Cons(x,xs) => 1 } }

def safeHead(xs: List[i64]): i64 { xs.case[i64] { Nil => 0, Cons(y, ys) => y}}
