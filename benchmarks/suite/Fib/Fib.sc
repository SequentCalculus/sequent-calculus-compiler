def fib(n:i64) : i64 {
  if n<2 {
    n
  } else {
    fib(n-1)+fib(n-2)
  }
}

def main(n:i64) : i64 {
  fib(n)
}
