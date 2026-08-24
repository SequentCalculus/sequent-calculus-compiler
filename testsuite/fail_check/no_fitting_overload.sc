data List[A] { Nil, Cons(x: A, xs: List[A]) }

def fun(x: i64, y:i64):i64 {
    x + y
}

def fun(x: List[i64], y:List[i64]): List[i64] {
    x.case {
        Nil => 0,
        Cons(a, xs) => y.case {
            Nil => 0,
            Cons(b, ys) => (a + b) + fun(xs, ys)
        }
    }
}