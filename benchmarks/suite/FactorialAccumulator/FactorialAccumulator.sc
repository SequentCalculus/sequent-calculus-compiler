def factorial(a: i64, i: i64): i64 {
  if i == 0 {
    a
  } else {
    factorial((i * a) % 1000000007, i - 1)
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res : i64 = factorial(1,n);
    println_i64(res);
    0
  } else {
    let res: i64 = factorial(1, n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
