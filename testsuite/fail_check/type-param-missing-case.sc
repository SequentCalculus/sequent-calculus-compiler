data T[A, B] { C1(x: A), C2(y: B) }

def foo(): i64 { C1.case[i64] { C1 => 1, C2 => 2 } }
