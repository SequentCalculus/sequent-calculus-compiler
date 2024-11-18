//def factorial(a: Int, i: Int): Int := ifz(i, a, factorial(((i * a) - 1000000007), (i - 1)));

def factorial(a: Int, i: Int): Int := ifz(i, a, let j: Int = i - 1 in let c: Int = i * a in let d: Int = c - 1000000007 in factorial(d, j));

def main(n: Int): Int := factorial(1, n);
