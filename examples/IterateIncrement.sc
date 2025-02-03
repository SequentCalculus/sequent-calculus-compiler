codata FunI64I64 { Ap(x:i64) : i64 }

def iterate(i: i64, f: FunI64I64, a: i64): i64 { ifz(i, a, iterate(i - 1, f, f.Ap(a))) }

def main(n: i64): i64 { println_i64(iterate(n, cocase { Ap(x: i64) => x + 1}, 0));
                        0 }
