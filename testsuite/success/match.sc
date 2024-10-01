data ListInt { Nil, Cons(x: Int, xs: ListInt) }

def isEmpty(xs: ListInt): Int := case xs of { Nil => 0, Cons(x: Int,xs: ListInt) => 1 };
