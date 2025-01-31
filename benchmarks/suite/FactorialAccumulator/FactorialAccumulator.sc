def factorial(a: i64, i: i64): i64 { ifz(i, a, factorial((i * a) % 1000000007, i - 1)) }

def main(n: i64): i64 { factorial(1, n) }
