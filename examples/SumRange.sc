data ListInt { Nil, Cons(x: Int, xs: ListInt) }

def range(i: Int, n: Int): ListInt := ifl(i, n, Cons(i, range(i + 1, n)), Nil);

def sum(xs: ListInt): Int := xs.case { Nil => 0,
                                       Cons(y: Int, ys: ListInt) => y + sum(ys) };

def main(n: Int): Int := sum(range(0, n));
