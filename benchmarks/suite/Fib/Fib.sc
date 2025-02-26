def fib(n: i64): i64 {
  if n == 0 {
    0
  } else {
    if n == 1 {
      1
    } else {
      fib(n - 1)+ fib(n - 2)
    }
  }
}

def main_loop(iters: i64, n: i64): i64 {
  let res: i64 = fib(n);
  if iters == 1 {
    println_i64(res);
    0
  } else {
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
