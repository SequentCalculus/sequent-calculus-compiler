// Factorial function
def fac(n : i64)  : i64 { if n == 0 {1} else {n * fac(n -1 )} }
def prod(n : i64, m : i64) : i64 { if 0!= n { if n - 1 == 0 {m} else { m + prod(n - 1, m )}} else {0} }

// Subtraction with cut-off, i.e. monus(4, 2) = 2 and monus(2, 4) = 0
def monus(n : i64, m : i64) : i64  { if 0 == m {n} else { if n!=0 {monus(n - 1, m - 1)} else {0}} }

def main(n : i64,m : i64) : i64  {
  println_i64(monus(n, m) / -2);
  0 }
