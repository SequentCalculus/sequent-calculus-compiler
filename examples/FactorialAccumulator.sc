def factorial(a: Int, i: Int): Int := ifz(i, a, factorial(((i * a) % 1000000007), i - 1));

def main(n: Int): Int := factorial(1, n);
