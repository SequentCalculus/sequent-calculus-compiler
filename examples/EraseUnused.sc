data ListInt { Nil, Cons(x: Int, xs: ListInt) }

//def useless(i: Int, n: Int, b: ListInt): Int := ifl(i, n, useless(i + 1, n, replicate(0, i, Nil)), i);
//def replicate(v: Int, n: Int, a: ListInt): ListInt := ifz(n, a, replicate(v, n - 1, Cons(v, a)));

def useless(i: Int, n: Int, b: ListInt): Int := ifl(i, n, let j: Int = i + 1 in useless(j, n, replicate(0, i, Nil)),i);
def replicate(v: Int, n: Int, a: ListInt): ListInt := ifz(n, a, let m: Int = n - 1 in replicate(v, m, Cons(v, a)));

def main(n: Int): Int := useless(0, n, Nil);
