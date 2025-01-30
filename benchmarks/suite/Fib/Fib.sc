def fib(n:i64) : i64 := ifl(n,2,n,fib(n-1)+fib(n-2));

def main(n:i64) : i64 := fib(n);
