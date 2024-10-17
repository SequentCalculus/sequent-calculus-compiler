data ListInt { Nil, Cons(x: Int, xs: ListInt) }

//def range(i: Int, n: Int): ListInt := ifz(n - i, Cons(i, range(i + 1, n)), Nil);
//
//def sum(xs: ListInt): Int := case xs of { Nil => 0,
//                                          Cons(y: Int, ys: ListInt) => y + (sum(ys)) };
//
//def main(n: Int): Int := sum(range(0, n));

def range(i: Int, n: Int): ListInt := let b: Int = n - i in ifz(b, Cons(i, let c: Int = i + 1 in range(c, n)), Nil);

def sum(xs: ListInt): Int := xs.case { Nil => 0,
                                       Cons(y: Int, ys: ListInt) => y + (sum(ys)) };

def main(n: Int): Int := sum(range(0, n));
