//def fibonacci(i: Int): Int := ifz(i, i, ifz(i - 1, i, (fibonacci(i - 1)) + (fibonacci(i - 2))));

def fibonacci(i: Int): Int := ifz(i, i, let j: Int = i - 1 in ifz(j, i, let a: Int = i - 1 in let x: Int = fibonacci(a) in let b: Int = i - 2 in x + (fibonacci(b))));
