codata T[A, B] { C1: A, C2(x: B): i64 }

def foo(x: T[i64, i64]): i64 { x.C2(3) }
