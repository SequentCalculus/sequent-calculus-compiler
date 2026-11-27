data Pair { Tup(a: i64, b: i64) }

def main(): i64 { Tup(1, 2).case { Tup(x, x) => x } }
