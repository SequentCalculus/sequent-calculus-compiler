codata Fun[A, B] { Apply(x: A): B }

def sum_loop(i: i64, tot: i64, stop: i64, f: Fun[i64, i64]): i64 {
  if stop < i {
    tot
  } else {
    sum_loop(i + 1, (f.Apply[i64, i64](i)) + tot, stop, f)
  }
}

def sum(f: Fun[i64, i64], start: i64, stop: i64): i64 {
  sum_loop(start, 0, stop, f)
}

def motz(n: i64): i64 {
  if n <= 1 {
    1
  } else {
    let limit: i64 = n - 2;
    let product: Fun[i64, i64] = new { Apply(i) => motz(i) * motz(limit - i) };
    motz(n - 1) + sum(product, 0, limit)
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 0 {
    0
  } else {
    let res: i64 = motz(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
