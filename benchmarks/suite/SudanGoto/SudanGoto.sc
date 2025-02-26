def sudan(n: i64, x: i64, y: i64, k:cns i64): i64 {
  if n == 0 {
    return x + y to k
  } else {
    if y == 0 {
      return x to k
    } else {
      let inner: i64 = label a { sudan(n, x, y - 1, a) };
      sudan(n - 1, inner, inner + y, k)
    }
  }
}

def main_loop(iters: i64, n: i64, x: i64, y: i64): i64 {
  if iters == 1 {
    let res: i64 = label a { sudan(n, x, y, a) };
    println_i64(res);
    0
  } else {
    let res: i64 = label a { sudan(n, x, y, a) };
    main_loop(iters - 1, n, x, y)
  }
}

def main(iters: i64, n: i64, x: i64, y: i64): i64 {
  main_loop(iters, n, x, y)
}
