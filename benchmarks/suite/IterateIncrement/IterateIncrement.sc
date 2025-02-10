codata Fun[A, B] { Apply(x: A) : B }

def iterate(i: i64, f: Fun[i64,i64], a: i64): i64 { if i == 0 {a} else {iterate(i - 1, f, f.Apply[i64,i64](a))} }

def main(n: i64): i64 { println_i64(iterate(n, cocase { Apply(x: i64) => x + 1}, 0));
                        0 }
