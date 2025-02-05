def tfib(n:i64,a:i64,b:i64) : i64 {
  if n==0{
    a
  }else{
    tfib(n-1,a+b,a)
  }
}

def fib(n:i64) : i64 {
  tfib(n,0,1)
}

def main(n:i64) : i64 {
  fib(n)
}
