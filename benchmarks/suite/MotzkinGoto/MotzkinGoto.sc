codata Fun[A, B] { apply(a: A): B }

def sum_loop(i: i64, tot: i64, stop: i64, f: Fun[i64, i64], k:cns i64): i64 {
  if stop < i {
    goto k (tot)
  } else {
    sum_loop(i + 1, (f.apply[i64, i64](i)) + tot, stop, f, k)
  }
}

def sum(f: Fun[i64, i64], start: i64, stop: i64, k:cns i64): i64 {
  sum_loop(start, 0, stop, f, k)
}

def motz(n: i64, k:cns i64): i64 {
  if n <= 1 {
    goto k (1)
  } else {
    let limit: i64 = n - 2;
    let product: Fun[i64, i64] = new { apply(i) => (label a { motz(i, a) }) * (label b { motz(limit - i, b) }) };
    goto k ((label a { motz(n - 1, a) }) + (label b { sum(product, 0, limit, b)}))
  }
}

def main_loop(iters: i64, n: i64): i64 {
  let res: i64 = label k { motz(n, k) };
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
