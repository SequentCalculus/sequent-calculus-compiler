codata Fun[A, B] { apply(x: A): B }

def sum(f: Fun[i64, i64], start: i64, stop: i64): i64 {
  if stop < start { 
    0
  } else {
    (f.apply[i64,i64](start)) + sum(f,start + 1,stop)
  }
}

def motz(n: i64): i64 {
  if n <= 1 {
    1
  } else {
    let limit: i64 = n - 2;
    let product: Fun[i64, i64] = new { apply(i) => motz(i) * motz(limit - i) };
    motz(n - 1) + sum(product, 0, limit)
  }
}

def main_loop(iters: i64, n: i64): i64 {
  let res: i64 = motz(n);
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
