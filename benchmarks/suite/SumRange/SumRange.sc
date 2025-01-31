data ListI64 { Nil, Cons(x: i64, xs: ListI64) }

def range(i: i64, n: i64): ListI64 { ifl(i, n, Cons(i, range(i + 1, n)), Nil) }

def sum(xs: ListI64): i64 { xs.case { Nil => 0,
                                       Cons(y: i64, ys: ListI64) => y + sum(ys) }}

def main(n: i64): i64 { sum(range(0, n)) }
