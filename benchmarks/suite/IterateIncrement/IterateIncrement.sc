codata Fun[A, B] { Apply(x: A): B }

def iterate(i: i64, f: Fun[i64, i64], a: i64): i64 {
  if i == 0 {
    a
  } else {
    iterate(i - 1, f, f.Apply[i64, i64](a))
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 0 {
    0
  } else {
    let res: i64 = iterate(n, new { Apply(x) => x + 1 }, 0);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
