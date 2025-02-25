data List[A] { Nil, Cons(a: A, as: List[A]) }
data Bool { True, False }

def and(b1: Bool, b2: Bool): Bool {
  b1.case {
    True => b2,
    False => False
  }
}

def neq_i(i1: i64, i2: i64): Bool {
  if i1 == i2 { False } else { True }
}

def length_loop(l: List[List[i64]], acc: i64): i64 {
  l.case[List[i64]] {
    Nil => acc,
    Cons(is, iss) => length_loop(iss, acc + 1)
  }
}

def length(l: List[List[i64]]): i64 {
  length_loop(l, 0)
}

def appendRev(l1: List[List[i64]], l2: List[List[i64]]): List[List[i64]] {
  l1.case[List[i64]] {
    Nil => l2,
    Cons(is, iss) => appendRev(iss, Cons(is, l2))
  }
}

def rev(l: List[List[i64]]): List[List[i64]] {
  appendRev(l, Nil)
}

def append(l1: List[List[i64]], l2: List[List[i64]]): List[List[i64]] {
  l2.case[List[i64]] {
    Nil => l1,
    Cons(is, iss) => appendRev(rev(l1), Cons(is, iss))
  }
}

def safe(x: i64, d: i64, l: List[i64]): Bool {
  l.case[i64] {
    Nil => True,
    Cons(q, l) =>
      and(neq_i(x, q),
        and(neq_i(x, q + d),
          and(neq_i(x, q - d),
            safe(x, d + 1, l))))
  }
}

def check(l: List[List[i64]], acc: List[List[i64]], q: i64): List[List[i64]] {
  l.case[List[i64]] {
    Nil => acc,
    Cons(b, bs) => safe(q, 1, b).case {
      True => check(bs, Cons(Cons(q, b), acc), q),
      False => check(bs, acc, q)
    }
  }
}

def enumerate(q: i64, acc: List[List[i64]], bs: List[List[i64]]): List[List[i64]] {
  if q == 0 {
    acc
  } else {
    let res: List[List[i64]] = check(bs, Nil, q);
    enumerate(q - 1, append(res, acc), bs)
  }
}

def gen(n: i64, nq: i64): List[List[i64]] {
  if n == 0 {
    Cons(Nil, Nil)
  } else {
    let bs: List[List[i64]] = gen(n - 1, nq);
    enumerate(nq, Nil, bs)
  }
}

def nsoln(n: i64): i64 {
  length(gen(n, n))
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res : i64 = nsoln(n);
    println_i64(res);
    0
  } else {
    let res: i64 = nsoln(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
