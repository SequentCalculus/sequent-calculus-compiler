codata Fun[A, B] { apply(x: A): B }

def iterate(i: i64, f: Fun[i64, i64], a: i64): i64 {
  if i == 0 {
    a
  } else {
    iterate(i - 1, f, f.apply[i64, i64](a))
  }
}

def main_loop(iters: i64, n: i64): i64 {
  let res: i64 = iterate(n, new { apply(x) => x + 1 }, 0);
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
