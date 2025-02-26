data List[A] { Nil, Cons(x: A, xs: List[A]) }
data Bool { True, False }

def not(b: Bool): Bool {
  b.case {
    True => False,
    False => True
  }
}

def null(x: List[i64]): Bool {
  x.case[i64] {
    Nil => True,
    Cons(x, xs) => False
  }
}

def tail(x: List[i64]): List[i64] {
  x.case[i64] {
    Nil => Nil, // should give a runtime error
    Cons(x, xs) => xs
  }
}

def len_loop(l: List[i64], acc: i64): i64 {
  l.case[i64] {
    Nil => acc,
    Cons(x, xs) => len_loop(xs, acc + 1)
  }
}

def len(l: List[i64]): i64 {
  len_loop(l, 0)
}

def list_n_loop(n: i64, a: List[i64]): List[i64] {
  if n == 0 {
    a
  } else {
    list_n_loop(n - 1, Cons(n, a))
  }
}

def list_n(n: i64): List[i64] {
  list_n_loop(n, Nil)
}

def shorterp(x: List[i64], y: List[i64]): Bool {
  not(null(y)).case {
    True => null(x).case {
      True => True,
      False => shorterp(tail(x), tail(y))
    },
    False => False
  }
}

def mas(x: List[i64], y: List[i64], z: List[i64]): List[i64] {
  not(shorterp(y, x)).case {
    True => z,
    False =>
      mas(
        mas(tail(x), y, z),
        mas(tail(y), z, x),
        mas(tail(z), x, y))
  }
}

def main_loop(iters: i64, x: i64, y: i64, z: i64): i64 {
  let res: i64 = len(mas(list_n(x), list_n(y), list_n(z)));
  if iters == 1 {
    println_i64(res);
    0
  } else {
    main_loop(iters - 1, x, y, z)
  }
}

def main(iters: i64, x: i64, y: i64, z: i64): i64 {
  main_loop(iters, x, y, z)
}
