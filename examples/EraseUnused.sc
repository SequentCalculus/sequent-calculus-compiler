data ListInt { Nil, Cons(x: Int, xs: ListInt) }

//def useless(i: Int, n: Int, b: ListInt): Int := ifz(n - i, useless(i + 1, n, replicate(0, i, Nil)), i);
//def replicate(v: Int, n: Int, a: ListInt): ListInt := ifz(n, a, replicate(v, n - 1, Cons(v, a)));

def useless(i: Int, n: Int, b: ListInt): Int := let m: Int = n - i in ifz(m, let j: Int = i + 1 in useless(j, n, replicate(0, i, Nil)), i);
def replicate(v: Int, n: Int, a: ListInt): ListInt := ifz(n, a, let m: Int = n - 1 in replicate(v, m, Cons(v, a)));
