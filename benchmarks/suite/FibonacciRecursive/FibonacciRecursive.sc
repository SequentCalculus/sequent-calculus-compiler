def fibonacci(i: i64): i64 {
  if i == 0 {
    i
  } else {
    if i == 1 {
      i
    } else {
      fibonacci(i - 1) + fibonacci(i - 2)
    }
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: i64 = fibonacci(n);
    println_i64(res);
    0
  } else {
    let res: i64 = fibonacci(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
