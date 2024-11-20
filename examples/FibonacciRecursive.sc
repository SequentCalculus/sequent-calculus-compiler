//def fibonacci(i: Int): Int := ifz(i, i, ife(i, 1, i, (fibonacci(i - 1)) + (fibonacci(i - 2))));

def fibonacci(i: Int): Int := ifz(i, i, ife(i, 1, i, let a: Int = i - 1 in let x: Int = fibonacci(a) in let b: Int = i - 2 in x + (fibonacci(b))));

def main(n: Int): Int := fibonacci(n);
