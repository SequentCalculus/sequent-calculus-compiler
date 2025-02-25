def ack(m: i64, n: i64, k:cns i64): i64 {
  if m == 0 {
    return n + 1 to k
  } else {
    if n == 0 {
      ack(m - 1, 1, k)
    } else {
      ack(m - 1, label a { ack(m, n - 1, a) }, k)
    }
  }
}

def main_loop(iters: i64, m: i64, n: i64): i64 {
  if iters == 1 {
    let res : i64 = label a { ack(m,n,a) };
    println_i64(res);
    0
  } else {
    let res: i64 = label a { ack(m, n, a) };
    main_loop(iters - 1, m, n)
  }
}

def main(iters: i64, m: i64, n: i64): i64 {
  main_loop(iters, m, n)
}
