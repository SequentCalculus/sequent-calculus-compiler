// Factorial function
def fac(n : i64)  : i64 { ifz(n, 1, n * fac(n -1 )) }
def prod(n : i64, m : i64) : i64 { ifz(n, 0, ifz(n - 1, m, m + prod(n - 1, m ))) }

// Subtraction with cut-off, i.e. monus(4,2) = 2 and monus(2,4) = 0
def monus(n : i64, m : i64) : i64  { ifz(m, n, ifz(n, 0, monus(n - 1, m - 1))) }

def main() : i64  { monus(14, 5) / 2 }
