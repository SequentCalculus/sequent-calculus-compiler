// Factorial function
<<<<<<< HEAD
<<<<<<< HEAD
=======
>>>>>>> 27de75f (added parsing definition types)
def fac(n : Int)  : Int := ifz(n, 1, n * (fac(n - 1; )));
def prod(n : Int, m : Int) : Int := ifz(n, 0, ifz(n - 1, m, m + (prod(n - 1, m; ))));

// Subtraction with cut-off, i.e. monus(4,2) = 2 and monus(2,4) = 0
def monus(n : Int, m : Int) : Int := ifz(m, n, ifz(n, 0, monus(n - 1, m - 1;)));

def main() : Int := monus(10, 5;);
<<<<<<< HEAD
=======
def fac(n : Int) := ifz(n, 1, n * (fac(n - 1)));
def prod(n : Int, m : Int) := ifz(n, 0, ifz(n - 1, m, m + (prod(n - 1, m))));

// Subtraction with cut-off, i.e. monus(4,2) = 2 and monus(2,4) = 0
def monus(n : Int, m : Int) := ifz(m, n, ifz(n, 0, monus(n - 1, m - 1)));

def main() := monus(10, 5);
>>>>>>> 7b89b63 (fixed integration tests)
=======
>>>>>>> 27de75f (added parsing definition types)
