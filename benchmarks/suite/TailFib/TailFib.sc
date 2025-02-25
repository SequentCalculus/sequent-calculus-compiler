def tfib(n: i64, a: i64, b: i64): i64 {
  if n == 0 { a } else { tfib(n - 1, a + b, a) }
}

def fib(n: i64): i64 {
  tfib(n, 0, 1)
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: i64 = fib(n);
    println_i64(res);
    0
  } else {
    let res: i64 = fib(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
