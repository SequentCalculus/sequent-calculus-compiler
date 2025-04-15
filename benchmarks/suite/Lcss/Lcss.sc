data List[A] { Nil, Cons(a: A, as: List[A]) }
data Pair[A, B] { Pair(a: A, b: B) }
data Bool { True, False }
data Option[A] { None, Some(a: A) }
codata Fun[A, B] { Apply(a: A): B }

def int_max(i1: i64, i2: i64): i64 {
  if i1 < i2 {
    i2
  } else {
    i1
  }
}

def algb2(x: i64, k0j1: i64, k1j1: i64, yss: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  yss.case[Pair[i64, i64]] {
    Nil => Nil,
    Cons(p, ys) => p.case[i64, i64] {
      Pair(y, k0j) =>
        let kjcurr: i64 = if x == y {
          k0j1 + 1
        } else {
          int_max(k1j1, k0j)
        };
        Cons(Pair(y, kjcurr), algb2(x, k0j, kjcurr, ys))
    }
  }
}

def snd_ii(p: Pair[i64, i64]): i64 {
  p.case[i64, i64] {
    Pair(i1, i2) => i2
  }
}

def rev_loop(l1: List[i64], l2: List[i64]): List[i64] {
  l1.case[i64] {
    Nil => l2,
    Cons(is, iss) => rev_loop(iss, Cons(is, l2))
  }
}

def rev(l: List[i64]): List[i64] {
  rev_loop(l, Nil)
}

def map_loop(f: Fun[Pair[i64, i64], i64], l: List[Pair[i64, i64]], acc: List[i64]): List[i64] {
  l.case[Pair[i64, i64]] {
    Nil => rev(acc),
    Cons(p, ps) => map_loop(f, ps, Cons(f.Apply[Pair[i64, i64], i64](p), acc))
  }
}

def map(f: Fun[Pair[i64, i64], i64], l: List[Pair[i64, i64]]): List[i64] {
  map_loop(f, l, Nil)
}

def algb1(xss: List[i64], yss: List[Pair[i64, i64]]): List[i64] {
  xss.case[i64] {
    Nil => map(new { Apply(x) => snd_ii(x) }, yss),
    Cons(x, xs) => algb1(xs, algb2(x, 0, 0, yss))
  }
}
def algb_listcomp_fun(listcomp_fun_para: List[i64]): List[Pair[i64, i64]] {
  listcomp_fun_para.case[i64] {
    Nil => Nil,
    Cons(listcomp_fun_ls_h, listcomp_fun_ls_t) =>
      Cons(Pair(listcomp_fun_ls_h, 0), algb_listcomp_fun(listcomp_fun_ls_t))
  }
}

def algb(xs: List[i64], ys: List[i64]): List[i64] {
    Cons(0, algb1(xs, algb_listcomp_fun(ys)))
}

def findk(k: i64, km: i64, m: i64, ls: List[Pair[i64, i64]]): i64 {
  ls.case[Pair[i64, i64]] {
    Nil => km,
    Cons(p, xys) => p.case[i64, i64] {
      Pair(x, y) =>
        if m <= (x + y) {
          findk(k + 1, k, x + y, xys)
        } else {
          findk(k + 1, km, m, xys)
        }
    }
  }
}

def is_nil(ls: List[i64]): Bool {
  ls.case[i64] {
    Nil => True,
    Cons(x, xs) => False
  }
}

def is_singleton(ls: List[i64]): Option[i64] {
  ls.case[i64] {
    Nil => None,
    Cons(x, xs) => xs.case[i64] {
      Nil => Some(x),
      Cons(x, xs) => None
    }
  }
}

def take(n: i64, ls: List[i64]): List[i64] {
  ls.case[i64] {
    Nil => Nil,
    Cons(i, is) => if n == 0 { Nil } else { Cons(i, take(n - 1, is)) }
  }
}

def drop(n: i64, ls: List[i64]): List[i64] {
  if n == 0 {
    ls
  } else {
    ls.case[i64] {
      Nil => Nil,
      Cons(i, is) => drop(n - 1, is)
    }
  }
}

def zip(l1: List[i64], l2: List[i64]): List[Pair[i64, i64]] {
  l1.case[i64] {
    Nil => Nil,
    Cons(i1, is1) => l2.case[i64] {
      Nil => Nil,
      Cons(i2, is2) => Cons(Pair(i1, i2), zip(is1, is2))
    }
  }
}

def in_list(x: i64, ls: List[i64]): Bool {
  ls.case[i64] {
    Nil => False,
    Cons(i, is) =>
      if i == x {
        True
      } else {
        in_list(x, is)
      }
  }
}

def algc(m: i64, n: i64, xs: List[i64], ys: List[i64]): Fun[List[i64], List[i64]] {
  is_nil(ys).case {
    True => new { Apply(x) => x },
    False => is_singleton(xs).case[i64] {
      Some(x) => in_list(x, ys).case {
        True => new { Apply(t) => Cons(x, t) },
        False => new { Apply(x) => x }
      },
      None =>
        let m2: i64 = m/2;
        let xs1: List[i64] = take(m2, xs);
        let xs2: List[i64] = drop(m2, xs);
        let l1: List[i64] = algb(xs1, ys);
        let l2: List[i64] = rev(algb(rev(xs2), rev(ys)));
        let k: i64 = findk(0, 0, -1, zip(l1, l2));
        new { Apply(x) =>
          let f1 : Fun[List[i64], List[i64]] = (algc(m - m2, n - k, xs2, drop(k, ys)));
          let f2 : Fun[List[i64], List[i64]] = algc(m2, k, xs1, take(k, ys));
          f2.Apply[List[i64], List[i64]](f1.Apply[List[i64], List[i64]](x))
        }
    }
  }
}

def list_len(l: List[i64]): i64 {
  l.case[i64] {
    Nil => 0,
    Cons(i, is) => 1 + list_len(is)
  }
}

def lcss(xs: List[i64], ys: List[i64]): List[i64] {
  algc(list_len(xs), list_len(ys), xs, ys).Apply[List[i64], List[i64]](Nil)
}

def enum_from_then_to(from: i64, then: i64, t: i64): List[i64] {
  if from <= t {
    Cons(from, enum_from_then_to(then, (2 * then) - from, t))
  } else {
    Nil
  }
}

def lcss_main(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64): List[i64] {
  lcss(enum_from_then_to(a, b, c), enum_from_then_to(d, e, f))
}

def test_lcss_nofib(): List[i64] {
  lcss_main(1, 2, 200, 100, 101, 300)
}

def head(l: List[i64]): i64 {
  l.case[i64] {
    Nil => -1,
    Cons(i, is) => i
  }
}

def main_loop(iters: i64): i64 {
  if iters == 1 {
    let res: List[i64] = test_lcss_nofib();
    println_i64(head(res));
    0
  } else {
    let res: List[i64] = test_lcss_nofib();
    main_loop(iters - 1)
  }
}

def main(iters: i64): i64 {
  main_loop(iters)
}
