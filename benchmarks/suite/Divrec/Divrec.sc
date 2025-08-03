data Unit { Unit }
data List[A] { Nil, Cons(x: A, xs: List[A]) }

def create_n(n: i64): List[Unit] {
  if n == 0 {
    Nil
  } else {
    Cons(Unit,create_n(n - 1))
  }
}

def len(l: List[Unit]): i64 {
  l.case[Unit] {
    Nil => 0,
    Cons(x, xs) => 1+len(xs)
  }

}

def rec_div2(l: List[Unit]): List[Unit] {
  l.case[Unit] {
    Nil => Nil,
    Cons(x, xs) => xs.case[Unit] {
      Nil => Nil, // should raise a runtime error
      Cons(y, ys) => Cons(x, rec_div2(ys))
  }}
}

def main_loop(iters: i64, n: i64): i64 {
  let res: i64 = len(rec_div2(create_n(n)));
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
