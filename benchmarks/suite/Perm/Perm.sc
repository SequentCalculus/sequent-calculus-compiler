data Unit { Unit }
data Bool { True, False }
data List[A] { Nil, Cons(a: A, as: List[A]) }
data Pair[A, B] { Tup(a: A, b: B) }
codata Fun[A, B] { Apply(a: A): B }

def empty_i(l: List[i64]): Bool {
  l.case[i64] {
    Nil => True,
    Cons(i, is) => False
  }
}

def tail_i(l: List[i64]): List[i64] {
  l.case[i64] {
    Nil => exit -1,
    Cons(i, is) => is
  }
}

def head_i(l: List[i64]): i64 {
  l.case[i64] {
    Nil => exit -1,
    Cons(i, is) => i
  }
}

def len_i_loop(l: List[i64], acc: i64): i64 {
  l.case[i64] {
    Nil => acc,
    Cons(is, iss) => len_i_loop(iss, acc + 1)
  }
}

def len_i(l: List[i64]): i64 {
  len_i_loop(l, 0)
}

def empty_l(l: List[List[i64]]): Bool {
  l.case[List[i64]] {
    Nil => True,
    Cons(is, iss) => False
  }
}

def tail_l(l: List[List[i64]]): List[List[i64]] {
  l.case[List[i64]] {
    Nil => exit -1,
    Cons(is, iss) => iss
  }
}

def head_l(l: List[List[i64]]): List[i64] {
  l.case[List[i64]] {
    Nil => exit -1,
    Cons(is, iss) => is
  }
}

def loop_p(j: i64, perms: List[List[i64]], x: List[i64], n: i64): Pair[List[List[i64]], List[i64]] {
  if j == 0 {
    p(n - 1, perms, x)
  } else {
    let pair_perms_x: Pair[List[List[i64]], List[i64]] = p(n - 1, perms, x);
    pair_perms_x.case[List[List[i64]], List[i64]] {
      Tup(perms, x) =>
        let pair_perms_x: Pair[List[List[i64]], List[i64]] = f(n, perms, x);
        pair_perms_x.case[List[List[i64]], List[i64]] {
          Tup(perms, x) => loop_p(j - 1, perms, x, n)
        }
    }
  }
}

def p(n: i64, perms: List[List[i64]], x: List[i64]): Pair[List[List[i64]], List[i64]] {
  if 1 < n {
    loop_p(n - 1, perms, x, n)
  } else {
    Tup(perms, x)
  }
}

def f(n: i64, perms: List[List[i64]], x: List[i64]): Pair[List[List[i64]], List[i64]] {
  let x: List[i64] = rev_loop(x, n, list_tail(x, n));
  let perms: List[List[i64]] = Cons(x, perms);
  Tup(perms, x)
}

def rev_loop(x: List[i64], n: i64, y: List[i64]): List[i64] {
  if n == 0 {
    y
  } else {
    rev_loop(tail_i(x), n - 1, Cons(head_i(x), y))
  }
}

def list_tail(x: List[i64], n: i64): List[i64] {
  if n == 0 {
    x
  } else {
    list_tail(tail_i(x), n - 1)
  }
}

def permutations(x0: List[i64]): List[List[i64]] {
  p(len_i(x0), Cons(x0, Nil), x0).case[List[List[i64]], List[i64]] {
    Tup(final_perms, x) => final_perms
  }
}

def loop_sum2(y: List[i64], sum: i64): i64 {
  empty_i(y).case {
    True => sum,
    False => loop_sum2(tail_i(y), sum + head_i(y))
  }
}

def loop_sum1(x: List[List[i64]], sum: i64): i64 {
  empty_l(x).case {
    True => sum,
    False => loop_sum1(tail_l(x), loop_sum2(head_l(x), sum))
  }
}

def sumlists(x: List[List[i64]]): i64 {
  loop_sum1(x, 0)
}

def loop_one2n(n: i64, p: List[i64]): List[i64] {
  if n == 0 {
    p
  } else {
    loop_one2n(n - 1, Cons(n, p))
  }
}

def one2n(n: i64): List[i64] {
  loop_one2n(n, Nil)
}

def loop_run(iters: i64, work: Fun[Unit, List[List[i64]]], result: Fun[List[List[i64]], Bool]): Bool {
  let res: Bool = result.Apply[List[List[i64]], Bool](work.Apply[Unit, List[List[i64]]](Unit));
  if iters == 0 {
    res
  } else {
    loop_run(iters - 1, work, result)
  }
}

def run_benchmark(iters: i64, work: Fun[Unit, List[List[i64]]], result: Fun[List[List[i64]], Bool]): Bool {
  loop_run(iters, work, result)
}

def factorial(n: i64): i64 {
  if n == 1 {
    1
  } else {
    n * factorial(n - 1)
  }
}

def loop_work(m: i64, perms: List[List[i64]]): List[List[i64]] {
  if m == 0 {
    perms
  } else {
    loop_work(m - 1, permutations(head_l(perms)))
  }
}

def perm9(m: i64, n: i64): Bool {
  run_benchmark(
    1,
    new { Apply(u) =>  loop_work(m, permutations(one2n(n))) },
    new { Apply(result) =>
      if sumlists(result) == (((n * (n + 1)) * factorial(n))/2) {
        True
      } else {
        False
      }
    })
}

def main_loop(iters: i64, m: i64, n: i64): i64 {
  let res: Bool = perm9(m, n);
  if iters == 1 {
    res.case {
      True => println_i64(1);
              0,
      False => println_i64(0);
               0
    }
  } else {
    main_loop(iters - 1, m, n)
  }
}

def main(iters: i64, m: i64, n: i64): i64 {
  main_loop(iters, m, n)
}
