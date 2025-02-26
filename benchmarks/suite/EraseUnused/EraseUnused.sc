data List[A] { Nil, Cons(x: A, xs: List[A]) }

def useless(i: i64, n: i64, b: List[i64]): i64 {
  if i < n {
    useless(i + 1, n, replicate(0, i, Nil))
  } else {
    i
  }
}

def replicate(v: i64, n: i64, a: List[i64]): List[i64] {
  if n == 0 {
    a
  } else {
    replicate(v, n - 1, Cons(v, a))
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: i64 = useless(0, n, Nil);
    println_i64(res);
    0
  } else {
    let res: i64 = useless(0, n, Nil);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
