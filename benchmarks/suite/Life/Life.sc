data Unit { Unit }
data Pair[A, B] { Tup(fst: A, snd: B) }
data List[A] { Nil, Cons(x: A, xs: List[A]) }
data Gen { Gen(coordslist: List[Pair[i64, i64]]) }
data Bool { True, False }
codata Fun[A, B] { Apply(x: A): B }
codata Fun2[A, B, C] { Apply2(x: A, y: B): C }

// Boolean Functions

def pair_eq(p1: Pair[i64, i64], p2: Pair[i64, i64]): Bool {
  p1.case[i64, i64] {
    Tup(fst1, snd1) => p2.case[i64, i64] {
      Tup(fst2, snd2) =>
        if fst1 == fst2 {
          if snd1 == snd2 { True } else { False }
        } else {
          False
        }
    }
  }
}

def or(b1: Bool, b2: Bool): Bool {
  b1.case {
    True => True,
    False => b2
  }
}

def not(b: Bool): Bool {
  b.case {
    True => False,
    False => True
  }
}


// List Functions

def fold(
  a: List[Pair[i64, i64]],
  xs: List[Pair[i64, i64]],
  f: Fun2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]]
): List[Pair[i64, i64]] {
  xs.case[Pair[i64, i64]] {
    Nil => a,
    Cons(b, x) =>
      fold(f.Apply2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]](a, b), x, f)
  }
}

def accumulate(
  a: List[Pair[i64, i64]],
  xs: List[Pair[i64, i64]],
  f: Fun2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]]
): List[Pair[i64, i64]] {
  fold(a, xs, f)
}

def revonto(x: List[Pair[i64, i64]], y: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  accumulate(x, y, new { Apply2(t, h) => Cons(h, t) })
}

def collect_accum(sofar: List[Pair[i64, i64]], xs: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], List[Pair[i64, i64]]]): List[Pair[i64, i64]] {
  xs.case[Pair[i64, i64]] {
    Nil => sofar,
    Cons(p, xs) => collect_accum(revonto(sofar, f.Apply[Pair[i64, i64], List[Pair[i64, i64]]](p)), xs, f)
  }
}

def collect(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], List[Pair[i64, i64]]]): List[Pair[i64, i64]] {
  collect_accum(Nil, l, f)
}

def exists(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Bool]): Bool {
  l.case[Pair[i64, i64]] {
    Nil => False,
    Cons(p, ps) => or(f.Apply[Pair[i64, i64], Bool](p), exists(ps, f))
  }
}

def appendRev(l1: List[Pair[i64, i64]], l2: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  l1.case[Pair[i64, i64]] {
    Nil => l2,
    Cons(is, iss) => appendRev(iss, Cons(is, l2))
  }
}

def rev(l: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  appendRev(l, Nil)
}

def append(l1: List[Pair[i64, i64]], l2: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  l2.case[Pair[i64, i64]] {
    Nil => l1,
    Cons(is, iss) => appendRev(rev(l1), Cons(is, iss))
  }
}

def map_loop(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Pair[i64, i64]], acc: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  l.case[Pair[i64, i64]] {
    Nil => rev(acc),
    Cons(p, ps) => map_loop(ps, f, Cons(f.Apply[Pair[i64, i64], Pair[i64, i64]](p), acc))
  }
}

def map(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Pair[i64, i64]]): List[Pair[i64, i64]] {
  map_loop(l, f, Nil)
}

def member(l: List[Pair[i64, i64]], p: Pair[i64, i64]): Bool {
  exists(l, new { Apply(p1) => pair_eq(p, p1) })
}

def len_loop(l: List[Pair[i64, i64]], acc: i64): i64 {
  l.case[Pair[i64, i64]] {
    Nil => acc,
    Cons(p, ps) => len_loop(ps, acc + 1)
  }
}

def len(l: List[Pair[i64, i64]]): i64 {
  len_loop(l, 0)
}

def filter_loop(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Bool], acc: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  l.case[Pair[i64, i64]] {
    Nil => rev(acc),
    Cons(p, ps) => filter_loop(ps, f,
      f.Apply[Pair[i64, i64], Bool](p).case {
        True => Cons(p, acc),
        False => acc
      })
  }
}

def filter(l: List[Pair[i64, i64]], p: Fun[Pair[i64, i64], Bool]): List[Pair[i64, i64]] {
  filter_loop(l, p, Nil)
}

def lexordset(xs: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  xs.case[Pair[i64, i64]] {
    Nil => Nil,
    Cons(a, x) =>
      append(
        lexordset(filter(x, new { Apply(b) => lexless(a, b) })),
        append(
          Cons(a, Nil),
          lexordset(filter(x, new { Apply(b) => lexgreater(a, b) }))))
  }
}

def lexless(a: Pair[i64, i64], b: Pair[i64, i64]): Bool {
  a.case[i64, i64] {
    Tup(fst1, snd1) => b.case[i64, i64] {
      Tup(fst2, snd2) =>
        if fst2 < fst1 {
          True
        } else {
          if fst2 == fst1 {
            if snd2 < snd1 { True } else { False }
          } else {
            False
          }
        }
    }
  }
}

def lexgreater(a: Pair[i64, i64], b: Pair[i64, i64]): Bool {
    lexless(b, a)
}

def diff(x: List[Pair[i64, i64]], y: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  filter(x, new { Apply(p) => not(member(y, p)) })
}

// Gen Functions

def collect_neighbors(xover: List[Pair[i64, i64]], x3: List[Pair[i64, i64]], x2: List[Pair[i64, i64]], x1: List[Pair[i64, i64]], xs: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  xs.case[Pair[i64, i64]] {
    Nil => diff(x3, xover),
    Cons(a, x) => member(xover, a).case {
      True => collect_neighbors(xover, x3, x2, x1, x),
      False => member(x3, a).case {
        True => collect_neighbors(Cons(a, xover), x3, x2, x1, x),
        False => member(x2, a).case {
          True => collect_neighbors(xover, Cons(a, x3), x2, x1, x),
          False => member(x1, a).case {
            True => collect_neighbors(xover, x3, Cons(a, x2), x1, x),
            False => collect_neighbors(xover, x3, x2, Cons(a, x1), x)
          }
        }
      }
    }
  }
}

def occurs3(l: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
  collect_neighbors(Nil, Nil, Nil, Nil, l)
}

def neighbours(p: Pair[i64, i64]): List[Pair[i64, i64]] {
  p.case[i64, i64] {
    Tup(fst, snd) =>
      Cons(Tup(fst - 1, snd - 1),
        Cons(Tup(fst - 1, snd),
          Cons(Tup(fst - 1, snd + 1),
            Cons(Tup(fst, snd - 1),
              Cons(Tup(fst, snd + 1),
                Cons(Tup(fst + 1, snd - 1),
                  Cons(Tup(fst + 1, snd),
                    Cons(Tup(fst + 1, snd + 1),
                      Nil))))))))
  }
}

def alive(g: Gen): List[Pair[i64, i64]] {
  g.case { Gen(livecoords) => livecoords }
}

def mkgen(coordlist: List[Pair[i64, i64]]): Gen {
  Gen(lexordset(coordlist))
}

def twoorthree(n: i64): Bool {
  if n == 2 { True } else { if n == 3 { True } else { False } }
}

def mk_nextgen_fn(gen: Gen): Gen {
  let living: List[Pair[i64, i64]] = alive(gen);
  let isalive: Fun[Pair[i64, i64], Bool] = new { Apply(p) => member(living, p) };
  let liveneighbours: Fun[Pair[i64, i64], i64] = new { Apply(p) => len(filter(neighbours(p), isalive)) };
  let survivors: List[Pair[i64, i64]] = filter(living, new { Apply(p) => twoorthree(liveneighbours.Apply[Pair[i64, i64], i64](p)) });
  let newnbrlist: List[Pair[i64, i64]] = collect(
    living,
    new { Apply(p) =>
      filter(neighbours(p), new { Apply(n) => not(isalive.Apply[Pair[i64, i64], Bool](n))}) });
  let newborn: List[Pair[i64, i64]] = occurs3(newnbrlist);
  mkgen(append(survivors, newborn))
}

def nthgen(g: Gen, i: i64): Gen {
  if i == 0 { g } else { nthgen(mk_nextgen_fn(g), i - 1) }
}

def gun(): Gen {
  let r9: List[Pair[i64, i64]] = Cons(Tup(9, 29), Cons(Tup(9, 30), Cons(Tup(9, 31), Cons(Tup(9, 32), Nil))));
  let r8: List[Pair[i64, i64]] = Cons(Tup(8, 20), Cons(Tup(8, 28), Cons(Tup(8, 29), Cons(Tup(8, 30),
                                   Cons(Tup(8, 31), Cons(Tup(8, 40), Cons(Tup(8, 41), r9)))))));
  let r7: List[Pair[i64, i64]] = Cons(Tup(7, 19), Cons(Tup(7, 21), Cons(Tup(7, 28), Cons(Tup(7, 31),
                                   Cons(Tup(7, 40), Cons(Tup(7, 41), r8))))));
  let r6: List[Pair[i64, i64]] = Cons(Tup(6, 7), Cons(Tup(6, 8), Cons(Tup(6, 18), Cons(Tup(6, 22),
                                   Cons(Tup(6, 23), Cons(Tup(6, 28), Cons(Tup(6, 29), Cons(Tup(6, 30),
                                     Cons(Tup(6, 31), Cons(Tup(6, 36), r7))))))))));
  let r5: List[Pair[i64, i64]] = Cons(Tup(5, 7), Cons(Tup(5, 8), Cons(Tup(5, 18), Cons(Tup(5, 22),
                                   Cons(Tup(5, 23), Cons(Tup(5, 29), Cons(Tup(5, 30), Cons(Tup(5, 31),
                                     Cons(Tup(5, 32), Cons(Tup(5, 36), r6))))))))));
  let r4: List[Pair[i64, i64]] = Cons(Tup(4, 18), Cons(Tup(4, 22), Cons(Tup(4, 23), Cons(Tup(4, 32), r5))));
  let r3: List[Pair[i64, i64]] = Cons(Tup(3, 19), Cons(Tup(3, 21), r4));
  let r2: List[Pair[i64, i64]] = Cons(Tup(2, 20), r3);
  mkgen(r2)
}

def go_gun(): Fun[i64, Unit] {
  new { Apply(steps) =>
    let gen: Gen = nthgen(gun(), steps);
    Unit
  }
}

def centerLine(): i64 {
  5
}

def bail(): List[Pair[i64, i64]] {
  Cons(Tup(0, 0), Cons(Tup(0, 1), Cons(Tup(1, 0), Cons(Tup(1, 1), Nil))))
}

def shuttle(): List[Pair[i64, i64]] {
  let r4: List[Pair[i64, i64]] = Cons(Tup(4, 1), Cons(Tup(4, 0), Cons(Tup(4, 5), Cons(Tup(4, 6), Nil))));
  let r3: List[Pair[i64, i64]] = Cons(Tup(3, 2), Cons(Tup(3, 3), Cons(Tup(3, 4), r4)));
  let r2: List[Pair[i64, i64]] = Cons(Tup(2, 1), Cons(Tup(2, 5), r3));
  let r1: List[Pair[i64, i64]] = Cons(Tup(1, 2), Cons(Tup(1, 4), r2));
  Cons(Tup(0, 3), r1)
}

def at_pos(coordlist: List[Pair[i64, i64]], p: Pair[i64, i64]): List[Pair[i64, i64]] {
  let move: Fun[Pair[i64, i64], Pair[i64, i64]] = new { Apply(a) =>
    a.case[i64, i64] { Tup(fst1, snd1) =>
      p.case[i64, i64] { Tup(fst2, snd2) => Tup(fst1 + fst2, snd1 + snd2) }
    }
  };
  map(coordlist, move)
}

def non_steady(): Gen {
  mkgen(append(append(
    at_pos(bail(), Tup(1, centerLine())),
    at_pos(bail(), Tup(21, centerLine()))),
    at_pos(shuttle(), Tup(6, centerLine() - 2))))
}

def go_shuttle(): Fun[i64, Unit] {
  new { Apply(steps) =>
    let gen: Gen = nthgen(non_steady(), steps);
    Unit
  }
}

def go_loop(iters: i64, steps: i64, go: Fun[i64, Unit]): i64 {
  if iters == 0 {
    0
  } else {
    let res: Unit = go.Apply[i64, Unit](steps);
    go_loop(iters - 1, steps, go)
  }
}

def main(iters: i64, steps: i64): i64 {
  let gun_res: i64 = go_loop(iters, steps, go_gun());
  print_i64(gun_res);
  let shuttle_res: i64 = go_loop(iters, steps, go_shuttle());
  println_i64(shuttle_res);
  0
}
