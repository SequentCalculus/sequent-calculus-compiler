data List[A] { Nil, Cons(x: A, xs: List[A]) }

def range(i: i64, n: i64): List[i64] { if i < n { Cons(i, range(i + 1, n)) } else { Nil } }

def sum(xs: List[i64]): i64 { xs.case[i64] { Nil => 0,
                                             Cons(y, ys) => y + sum(ys) } }

def main(n: i64): i64 { println_i64(sum(range(0, n)));
                        0 }
