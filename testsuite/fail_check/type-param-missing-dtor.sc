codata T[A, B] { c1: A, c2(x: B): i64 }

def foo(x: T[i64, i64]): i64 { x.c2(3) }
