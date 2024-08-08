// Factorial function
def fac(n;) := ifz(n, 1, n * (fac(n - 1;)));
def prod(n, m;) := ifz(n, 0, ifz(n - 1, m, m + (prod(n - 1, m;))));

// Subtraction with cut-off, i.e. monus(4,2) = 2 and monus(2,4) = 0
def monus(n, m;) := ifz(m, n, ifz(n, 0, monus(n - 1, m - 1;)));

def main(;) := monus(10, 5;);
