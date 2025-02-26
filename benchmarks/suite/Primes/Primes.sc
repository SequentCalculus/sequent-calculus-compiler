data List[A] { Nil, Cons(a: A, as: List[A]) }

def interval_list(m: i64, n: i64): List[i64] {
  if n < m {
    Nil
  } else {
    Cons(m, interval_list(m + 1, n))
  }
}

def remove_multiples(n: i64, l: List[i64]): List[i64] {
  l.case[i64] {
    Nil => Nil,
    Cons(x, xs) =>
      if x % n == 0 {
        remove_multiples(n, xs)
      } else {
        Cons(x, remove_multiples(n, xs))
      }
  }
}

def sieve(l: List[i64]): List[i64] {
  l.case[i64] {
    Nil => Nil,
    Cons(x, xs) => Cons(x, sieve(remove_multiples(x, xs)))
  }
}

def len_loop(l: List[i64], acc: i64): i64 {
  l.case[i64] {
    Nil => acc,
    Cons(p, ps) => len_loop(ps, acc + 1)
  }
}

def len(l: List[i64]): i64 {
  len_loop(l, 0)
}

def main_loop(iters: i64, n: i64): i64 {
  let x: List[i64] = sieve(interval_list(2, n));
  if iters == 1 {
    println_i64(len(x));
    0
  } else {
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
