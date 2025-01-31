data ListI64 { Nil, Cons(x: i64, xs: ListI64) }

def useless(i: i64, n: i64, b: ListI64): i64 { ifl(i, n, useless(i + 1, n, replicate(0, i, Nil)), i) }
def replicate(v: i64, n: i64, a: ListI64): ListI64 { ifz(n, a, replicate(v, n - 1, Cons(v, a))) }

def main(n: i64): i64 { useless(0, n, Nil) }
