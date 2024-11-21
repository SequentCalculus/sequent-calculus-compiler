def fibonacci(i: Int): Int := ifz(i, i, ife(i, 1, i, (fibonacci(i - 1)) + (fibonacci(i - 2))));

def main(n: Int): Int := fibonacci(n);
