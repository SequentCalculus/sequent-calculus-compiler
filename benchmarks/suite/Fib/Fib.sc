def fib(n:i64) : i64 := ifz(n,0,ifz(n-1,1,fib(n-1)+fib(n-2)));

def main(n:i64) : i64 := fib(n);
