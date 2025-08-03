data List[A] { Nil, Cons(a: A, as: List[A]) }
data Bool { True, False }

def neq_i(i1: i64, i2: i64): Bool {
  if i1 == i2 { False } else { True }
}

def length(l: List[List[i64]]): i64 {
  l.case[List[i64]]{
    Nil => 0,
    Cons(i,ls) => 1+length(ls)
  }
}

def append(l1: List[List[i64]], l2: List[List[i64]]): List[List[i64]] {
  l1.case[List[i64]] {
    Nil => l2,
    Cons(is, iss) => Cons(is,append(iss,l2))
  }
}

def safe(x: i64, d: i64, l: List[i64]): Bool {
  l.case[i64] {
    Nil => True,
    Cons(q, l) => neq_i(x, q).case {
      True => neq_i(x, q + d).case {
        True => neq_i(x, q - d).case {
          True => safe(x, d + 1, l),
          False => False
        },
        False => False
      },
      False => False
    }
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
  let res: i64 = nsoln(n);
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
