data List[A] { Nil, Cons(a: A, as: List[A]) }
data Pair[A, B] { Tup(a: A, b: B) }
data Option[A] { None, Some(a: A) }
data Bool { True, False }
codata Fun[A, B] { Apply(a: A): B }
codata Fun2[A, B, C] { Apply2(a: A,b: B): C }

data Assign { Assign(varr: i64, value: i64) }
data CSP { CSP(vars: i64, vals: i64, rel: Fun2[Assign, Assign, Bool]) }
data Node[T] { Node(lab: T, children: List[Node[T]]) }
data ConflictSet { Known(vs: List[i64]), Unknown }

def abs(i: i64): i64 {
  if i < 0 { -1 * i } else { i }
}

def eq(i1: i64, i2: i64): Bool {
  if i1 == i2 { True } else { False }
}

def and(b1: Bool, b2: Bool): Bool {
  b1.case {
    True => b2,
    False => False
  }
}

def not(b: Bool): Bool {
  b.case {
    True => False,
    False => True
  }
}

def reverse_loop(ls: List[Assign], acc: List[Assign]): List[Assign] {
  ls.case[Assign] {
    Nil => acc,
    Cons(a, as) => reverse_loop(as, Cons(a, acc))
  }
}

def reverse(ls: List[Assign]): List[Assign] {
  reverse_loop(ls, Nil)
}

def level(a: Assign): i64 {
  a.case {
    Assign(varr, value) => varr
  }
}

def value(a: Assign): i64 {
  a.case {
    Assign(varr, value) => value
  }
}

def max_level(ls: List[Assign]): i64 {
  ls.case[Assign] {
    Nil => 0,
    Cons(a, t) => a.case {
      Assign(v, value) => v
    }
  }
}

def complete(csp: CSP, s: List[Assign]): Bool {
  csp.case {
    CSP(v, vals, rel) => eq(max_level(s), v)
  }
}

def enum_from_to(from: i64, to_: i64): List[i64] {
  if from <= to_{
    Cons(from, enum_from_to(from + 1, to_))
  } else {
    Nil
  }
}

def safe(as1: Assign, as2: Assign): Bool {
  as1.case {
    Assign(i, m) => as2.case {
      Assign(j, n) => and(not(eq(m, n)), not(eq(abs(i - j), abs(m - n))))
    }
  }
}

def queens(n: i64): CSP {
  CSP(n, n, new { Apply2(x, y) => safe(x, y)})
}


// mk_tree

def mk_rev_loop(l1: List[Node[List[Assign]]], l2: List[Node[List[Assign]]]): List[Node[List[Assign]]] {
  l1.case[Node[List[Assign]]] {
    Nil => l2,
    Cons(is, iss) => mk_rev_loop(iss, Cons(is, l2))
  }
}

def mk_rev(l: List[Node[List[Assign]]]): List[Node[List[Assign]]] {
  mk_rev_loop(l, Nil)
}

def mk_map_loop(f: Fun[List[Assign], Node[List[Assign]]], l: List[List[Assign]], acc: List[Node[List[Assign]]]): List[Node[List[Assign]]] {
  l.case[List[Assign]] {
    Nil => mk_rev(acc),
    Cons(p, ps) => mk_map_loop(f, ps, Cons(f.Apply[List[Assign], Node[List[Assign]]](p), acc))
  }
}

def mk_map(f: Fun[List[Assign], Node[List[Assign]]], l: List[List[Assign]]): List[Node[List[Assign]]] {
  mk_map_loop(f, l, Nil)
}

def mk_init_tree(f: Fun[List[Assign], List[List[Assign]]], x: List[Assign]): Node[List[Assign]] {
  Node(x, mk_map(new { Apply(y) => mk_init_tree(f, y) }, f.Apply[List[Assign], List[List[Assign]]](x)))
}

def mk_lscomp1(ls: List[i64], ss: List[Assign]): List[List[Assign]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(j, t1) => Cons(Cons(Assign(max_level(ss) + 1, j), ss), mk_lscomp1(t1, ss))
  }
}

def mk_tree(csp: CSP): Node[List[Assign]] {
  csp.case {
    CSP(vars, vals, rel) =>
      let next: Fun[List[Assign], List[List[Assign]]] =
        new { Apply(ss) =>
          if max_level(ss) < vars {
            mk_lscomp1(enum_from_to(1, vals), ss)
          } else {
            Nil
          }
        };
      mk_init_tree(next, Nil)
  }
}


// earilest_inconsistency

def ear_inc_filter(f: Fun[Assign, Bool], ls: List[Assign]): List[Assign] {
  ls.case[Assign] {
    Nil => Nil,
    Cons(a, as) => f.Apply[Assign, Bool](a).case {
      True => Cons(a, ear_inc_filter(f, as)),
      False => ear_inc_filter(f, as)
    }
  }
}

def earliest_inconsistency(csp: CSP, aas: List[Assign]): Option[Pair[i64, i64]] {
  csp.case {
    CSP(vars, vals, rel) => aas.case[Assign] {
      Nil => None,
      Cons(a, as_) =>  ear_inc_filter(
        new { Apply(x) => not(rel.Apply2[Assign, Assign, Bool](a, x)) },
        reverse(as_)).case[Assign] {
          Nil => None,
          Cons(b, bs_) => Some(Tup(level(a), level(b)))
        }
    }
  }
}


// search

def known_conflict(c: ConflictSet): Bool {
  c.case {
    Known(vs) => vs.case[i64] {
      Nil => False,
      Cons(v, vs) => True
    },
    Unknown => False
  }
}

def known_solution(c: ConflictSet): Bool {
  c.case {
    Known(vs) => vs.case[i64] {
      Nil => True,
      Cons(v, vs) => False
    },
    Unknown => False
  }
}

def check_complete(csp: CSP, s: List[Assign]): ConflictSet {
  complete(csp, s).case {
    True => Known(Nil),
    False => Unknown
  }
}

def search_rev_loop(
  l1: List[Node[Pair[List[Assign], ConflictSet]]],
  l2: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l1.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => l2,
    Cons(is, iss) => search_rev_loop(iss, Cons(is, l2))
  }
}

def search_rev(l: List[Node[Pair[List[Assign], ConflictSet]]]): List[Node[Pair[List[Assign], ConflictSet]]] {
  search_rev_loop(l, Nil)
}

def search_map_loop(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => search_rev(acc),
    Cons(p, ps) => search_map_loop(f, ps, Cons(f.Apply[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def search_map(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  search_map_loop(f, l, Nil)
}

def search_filter_loop(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Bool],
  l: List[Node[Pair[List[Assign], ConflictSet]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => search_rev(acc),
    Cons(p, ps) => f.Apply[Node[Pair[List[Assign], ConflictSet]], Bool](p).case {
      True => search_filter_loop(f, ps, Cons(p, acc)),
      False => search_filter_loop(f, ps, acc)
    }
  }
}

def search_filter(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Bool],
  l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  search_filter_loop(f, l, Nil)
}

def search_label(n: Node[Pair[List[Assign], ConflictSet]]): Pair[List[Assign], ConflictSet] {
  n.case[Pair[List[Assign], ConflictSet]] {
    Node(p, cs) => p
  }
}

def search_fold_tree(
  f: Fun2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]],
  n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
  n.case[Pair[List[Assign], ConflictSet]] {
    Node(l, c) =>
      f.Apply2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]]
      (l, search_map(new { Apply(x) => search_fold_tree(f, x)}, c))
  }
}

def search_filter_tree(
  p: Fun[Pair[List[Assign], ConflictSet], Bool],
  n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
  let f: Fun2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]] =
    new { Apply2(a, cs) =>
      Node(a, search_filter(new { Apply(x) => p.Apply[Pair[List[Assign], ConflictSet], Bool](search_label(x)) }, cs))
    };
  search_fold_tree(f, n)
}

def search_prune(
  f: Fun[Pair[List[Assign], ConflictSet], Bool],
  n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
  search_filter_tree(new { Apply(x) => not(f.Apply[Pair[List[Assign], ConflictSet], Bool](x)) }, n)
}

def search_rev2_loop(
  l1: List[List[Pair[List[Assign], ConflictSet]]],
  l2: List[List[Pair[List[Assign], ConflictSet]]]
): List[List[Pair[List[Assign], ConflictSet]]] {
  l1.case[List[Pair[List[Assign], ConflictSet]]] {
    Nil => l2,
    Cons(is, iss) => search_rev2_loop(iss, Cons(is, l2))
  }
}

def search_rev2(l: List[List[Pair[List[Assign], ConflictSet]]]): List[List[Pair[List[Assign], ConflictSet]]] {
  search_rev2_loop(l, Nil)
}

def search_map2_loop(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], List[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]],
  acc: List[List[Pair[List[Assign], ConflictSet]]]
): List[List[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => search_rev2(acc),
    Cons(p, ps) => search_map2_loop(f, ps, Cons(f.Apply[Node[Pair[List[Assign], ConflictSet]], List[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def search_map2(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], List[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[List[Pair[List[Assign], ConflictSet]]] {
  search_map2_loop(f, l, Nil)
}

def search_rev3_loop(
  l1: List[Pair[List[Assign], ConflictSet]],
  l2: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  l1.case[Pair[List[Assign], ConflictSet]] {
    Nil => l2,
    Cons(p, ps) => search_rev3_loop(ps, Cons(p, l2))
  }
}

def search_rev3(l: List[Pair[List[Assign], ConflictSet]]): List[Pair[List[Assign], ConflictSet]] {
  search_rev3_loop(l, Nil)
}

def search_concat_loop(
  ls: List[List[Pair[List[Assign], ConflictSet]]],
  acc: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  ls.case[List[Pair[List[Assign], ConflictSet]]] {
    Nil => search_rev3(acc),
    Cons(l, ls) => search_concat_loop(ls, search_rev3_loop(l, acc))
  }
}

def search_concat(ls: List[List[Pair[List[Assign], ConflictSet]]]): List[Pair[List[Assign], ConflictSet]] {
  search_concat_loop(ls, Nil)
}

def search_leaves(n: Node[Pair[List[Assign], ConflictSet]]): List[Pair[List[Assign], ConflictSet]] {
  n.case[Pair[List[Assign], ConflictSet]] {
    Node(leaf, cs) => cs.case[Node[Pair[List[Assign], ConflictSet]]] {
      Nil => Cons(leaf, Nil),
      Cons(c, cs) => search_concat(search_map2(new { Apply(x) => search_leaves(x) }, Cons(c, cs)))
    }
  }
}

def search_rev4_loop(
  l1: List[List[Assign]],
  l2: List[List[Assign]]
): List[List[Assign]] {
  l1.case[List[Assign]] {
    Nil => l2,
    Cons(is, iss) => search_rev4_loop(iss, Cons(is, l2))
  }
}

def search_rev4(l: List[List[Assign]]): List[List[Assign]] {
  search_rev4_loop(l, Nil)
}

def search_map3_loop(
  f: Fun[Pair[List[Assign], ConflictSet], List[Assign]],
  l: List[Pair[List[Assign], ConflictSet]],
  acc: List[List[Assign]]
): List[List[Assign]] {
  l.case[Pair[List[Assign], ConflictSet]] {
    Nil => search_rev4(acc),
    Cons(p, ps) => search_map3_loop(f, ps, Cons(f.Apply[Pair[List[Assign], ConflictSet], List[Assign]](p), acc))
  }
}

def search_map3(
  f: Fun[Pair[List[Assign], ConflictSet], List[Assign]],
  l: List[Pair[List[Assign], ConflictSet]]
): List[List[Assign]] {
  search_map3_loop(f, l, Nil)
}

def search_filter2_loop(
  f: Fun[Pair[List[Assign], ConflictSet], Bool],
  l: List[Pair[List[Assign], ConflictSet]],
  acc: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  l.case[Pair[List[Assign], ConflictSet]] {
    Nil => search_rev3(acc),
    Cons(p, ps) => f.Apply[Pair[List[Assign], ConflictSet], Bool](p).case {
      True => search_filter2_loop(f, ps, Cons(p, acc)),
      False => search_filter2_loop(f, ps, acc)
    }
  }
}

def search_filter2(
  f: Fun[Pair[List[Assign], ConflictSet], Bool],
  l: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  search_filter2_loop(f, l, Nil)
}

def search_fst(p: Pair[List[Assign], ConflictSet]): List[Assign] {
  p.case[List[Assign], ConflictSet] {
    Tup(l, c) => l
  }
}

def search_snd(p: Pair[List[Assign], ConflictSet]): ConflictSet {
  p.case[List[Assign], ConflictSet] {
    Tup(l, c) => c
  }
}

def search(labeler: Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], csp: CSP): List[List[Assign]] {
  search_map3(new { Apply(x) => search_fst(x)},
    search_filter2(new { Apply(x) => known_solution(search_snd(x)) },
      search_leaves(
        search_prune(new { Apply(x) => known_conflict(search_snd(x)) },
          labeler.Apply2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]](csp, mk_tree(csp))))))
}

// bt
def bt_map_loop(
  f: Fun[Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[List[Assign]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[List[Assign]]] {
    Nil => search_rev(acc),
    Cons(p, ps) => bt_map_loop(f, ps, Cons(f.Apply[Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def bt_map(
  f: Fun[Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[List[Assign]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  bt_map_loop(f, l, Nil)
}

def bt_map_tree(f: Fun[List[Assign], Pair[List[Assign], ConflictSet]], n: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
  n.case[List[Assign]] {
    Node(l, ls) => Node(f.Apply[List[Assign], Pair[List[Assign], ConflictSet]](l),
      bt_map(new { Apply(x) => bt_map_tree(f, x) }, ls))
  }
}

def bt(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
  let f3: Fun[List[Assign], Pair[List[Assign], ConflictSet]] =
    new { Apply(s) =>
      Tup(s, (earliest_inconsistency(csp, s).case[Pair[i64, i64]] {
        Some(p) => p.case[i64, i64] {
          Tup(a, b) => Known(Cons(a, Cons(b, Nil)))
        },
        None => check_complete(csp, s)
      }))
    };
  bt_map_tree(f3, t)
}

//empty_table

def empt_lscomp2(ls: List[i64]): List[ConflictSet] {
  ls.case[i64] {
    Nil => Nil,
    Cons(m, t2) => Cons(Unknown, empt_lscomp2(t2))
  }
}

def empt_lscomp1(ls: List[i64], vals: i64): List[List[ConflictSet]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(n, t1) =>  Cons(empt_lscomp2(enum_from_to(1, vals)), empt_lscomp1(t1, vals))
  }
}

def empty_table(csp: CSP): List[List[ConflictSet]] {
  csp.case {
    CSP(vars, vals, rel) => Cons(Nil, empt_lscomp1(enum_from_to(1, vars), vals))
  }
}

//fill_table
def fill_lscomp2(ls: List[i64], varrr: i64): List[Pair[i64, i64]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(valll, t2) => Cons(Tup(varrr, valll), fill_lscomp2(t2, varrr))
  }
}

def fill_lscomp1(ls: List[i64], vals: i64): List[List[Pair[i64, i64]]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(varrr, t1) => Cons(fill_lscomp2(enum_from_to(1, vals), varrr), fill_lscomp1(t1, vals))
  }
}

def fill_zip_with(f: Fun2[ConflictSet, Pair[i64, i64], ConflictSet], x: List[ConflictSet], y: List[Pair[i64, i64]]):
  List[ConflictSet] {
  x.case[ConflictSet] {
    Nil => Nil,
    Cons(c, cs) => y.case[Pair[i64, i64]] {
      Nil => Nil,
      Cons(p, ps) => Cons(f.Apply2[ConflictSet, Pair[i64, i64], ConflictSet](c, p), fill_zip_with(f, cs, ps))
    }
  }
}

def fill_zip_with2(f: Fun2[List[ConflictSet], List[Pair[i64, i64]], List[ConflictSet]], tbl: List[List[ConflictSet]],
    ls: List[List[Pair[i64, i64]]]): List[List[ConflictSet]] {
      tbl.case[List[ConflictSet]] {
        Nil => Nil,
        Cons(cs, css) => ls.case[List[Pair[i64, i64]]] {
          Nil => Nil,
          Cons(ps, pss) => Cons(f.Apply2[List[ConflictSet], List[Pair[i64, i64]], List[ConflictSet]](cs, ps), fill_zip_with2(f, css, pss))
        }
      }
}

def fill_table(s: List[Assign], csp: CSP, tbl: List[List[ConflictSet]]): List[List[ConflictSet]] {
  s.case[Assign] {
    Nil => tbl,
    Cons(as, as_) => as.case { Assign(var_, val_) =>
      csp.case {
        CSP(vars, vals, rel) =>
          let f4: Fun2[ConflictSet, Pair[i64, i64], ConflictSet] = new { Apply2(cs, varval) =>
            varval.case[i64, i64] {
              Tup(varr, vall) => cs.case {
                Known(vs) => cs,
                Unknown => not(rel.Apply2[Assign, Assign, Bool](Assign(var_, val_), Assign(varr, vall))).case {
                  True => Known(Cons(var_, Cons(varr, Nil))),
                  False => cs
                }
              }
          }};
          fill_zip_with2(new { Apply2(x, y) => fill_zip_with(f4, x, y) }, tbl, fill_lscomp1(enum_from_to(var_ + 1, vars), vals))
      }
    }
  }
}

// lookup_cache

def lookup_rev_loop(
  l1: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]],
  l2: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
  l1.case[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
    Nil => l2,
    Cons(is, iss) => lookup_rev_loop(iss, Cons(is, l2))
  }
}

def lookup_rev(
  l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
  lookup_rev_loop(l, Nil)
}

def lookup_map_loop(
  f: Fun[Node[Pair[List[Assign], List[List[ConflictSet]]]], Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]],
  l: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]],
  acc: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
  l.case[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
    Nil => lookup_rev(acc),
    Cons(p, ps) =>
      lookup_map_loop(f, ps, Cons(f.Apply[Node[Pair[List[Assign], List[List[ConflictSet]]]], Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]](p), acc))
  }
}

def lookup_map(
  f: Fun[Node[Pair[List[Assign], List[List[ConflictSet]]]], Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]],
  l: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]]
): List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
  lookup_map_loop(f, l, Nil)
}

def lookup_map_tree(
  f: Fun[Pair[List[Assign], List[List[ConflictSet]]], Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]],
  t: Node[Pair[List[Assign], List[List[ConflictSet]]]]
): Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
  t.case[Pair[List[Assign], List[List[ConflictSet]]]] {
    Node(p, ps) =>
      Node(f.Apply[Pair[List[Assign], List[List[ConflictSet]]], Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]](p),
        lookup_map(new { Apply(x) => lookup_map_tree(f, x) }, ps))
  }
}

def lookup_at_index(ind: i64, ls: List[ConflictSet]): ConflictSet {
  ls.case[ConflictSet] {
    Nil => exit -1,
    Cons(c, cs) => if ind == 0 { c } else { lookup_at_index(ind - 1, cs) }
  }
}

def lookup_head(tbl: List[List[ConflictSet]]): List[ConflictSet] {
  tbl.case[List[ConflictSet]] {
    Nil => exit -1,
    Cons(cs, css) => cs
  }
}

def lookup_cache(
  csp: CSP,
  t: Node[Pair[List[Assign], List[List[ConflictSet]]]]
): Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
  let f5: Fun2[CSP, Pair[List[Assign], List[List[ConflictSet]]], Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] =
    new { Apply2(csp, tp) =>
      tp.case[List[Assign], List[List[ConflictSet]]] {
        Tup(ls, tbl) => ls.case[Assign] {
          Nil => Tup(Tup(Nil, Unknown), tbl),
          Cons(a, as_) =>
            let table_entry: ConflictSet = lookup_at_index(value(a) - 1, lookup_head(tbl));
            let cs: ConflictSet = table_entry.case {
              Unknown => check_complete(csp, Cons(a, as_)),
              Known(vals) => table_entry
            };
            Tup(Tup(Cons(a, as_), cs), tbl)
        }
      }
    };
  lookup_map_tree(
    new { Apply(x) =>
      f5.Apply2[CSP, Pair[List[Assign], List[List[ConflictSet]]], Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
      (csp, x)
    },
    t
  )
}


// cache_checks

def checks_rev_loop(
  l1: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]],
  l2: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
  l1.case[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
    Nil => l2,
    Cons(is, iss) => checks_rev_loop(iss, Cons(is, l2))
  }
}

def checks_rev(l: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]]): List[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
  checks_rev_loop(l, Nil)
}

def checks_map_loop(
  f: Fun[Node[List[Assign]], Node[Pair[List[Assign], List[List[ConflictSet]]]]],
  l: List[Node[List[Assign]]],
  acc: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
  l.case[Node[List[Assign]]] {
    Nil => checks_rev(acc),
    Cons(p, ps) => checks_map_loop(f, ps, Cons(f.Apply[Node[List[Assign]], Node[Pair[List[Assign], List[List[ConflictSet]]]]](p), acc))
  }
}

def checks_map(
  f: Fun[Node[List[Assign]], Node[Pair[List[Assign], List[List[ConflictSet]]]]],
  l: List[Node[List[Assign]]]
): List[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
  checks_map_loop(f, l, Nil)
}

def checks_tail(ls: List[List[ConflictSet]]): List[List[ConflictSet]] {
  ls.case[List[ConflictSet]] {
    Nil => exit -1,
    Cons(l, ls) => ls
  }
}

def cache_checks(csp: CSP, tbl: List[List[ConflictSet]], n: Node[List[Assign]]): Node[Pair[List[Assign], List[List[ConflictSet]]]] {
  n.case[List[Assign]] {
    Node(s, cs) => Node(Tup(s, tbl), checks_map(new { Apply(x) => cache_checks(csp, fill_table(s, csp, checks_tail(tbl)), x) }, cs))
  }
}

//bm
def bm_fst(x: Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]): Pair[List[Assign], ConflictSet] {
  x.case[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]] {
    Tup(p, ls) => p
  }
}

def bm_rev_loop(
  l1: List[Node[Pair[List[Assign], ConflictSet]]],
  l2: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l1.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => l2,
    Cons(is, iss) => bm_rev_loop(iss, Cons(is, l2))
  }
}

def bm_rev(l: List[Node[Pair[List[Assign], ConflictSet]]]): List[Node[Pair[List[Assign], ConflictSet]]] {
  bm_rev_loop(l, Nil)
}

def bm_map_loop(
  f: Fun[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
    Nil => bm_rev(acc),
    Cons(p, ps) => bm_map_loop(f, ps, Cons(f.Apply[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def bm_map(
  f: Fun[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  bm_map_loop(f, l, Nil)
}

def bm_map_tree(
  f: Fun[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]], Pair[List[Assign], ConflictSet]],
  t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
): Node[Pair[List[Assign], ConflictSet]] {
  t.case[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
    Node(p, ps) =>
      Node(f.Apply[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]], Pair[List[Assign], ConflictSet]](p),
        bm_map(new { Apply(x) => bm_map_tree(f, x) }, ps))
  }
}

def bm(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
  bm_map_tree(new { Apply(x) => bm_fst(x) }, lookup_cache(csp, cache_checks(csp, empty_table(csp), t)))
}

// combine

def in_list(i: i64, ls: List[i64]): Bool {
  ls.case[i64] {
    Nil => False,
    Cons(j, js) => if i == j { True } else { in_list(i, js) }
  }
}

def not_elem(i: i64, ls: List[i64]): Bool {
  not(in_list(i, ls))
}

def appendRev(l1: List[i64], l2: List[i64]): List[i64] {
  l1.case[i64] {
    Nil => l2,
    Cons(is, iss) => appendRev(iss, Cons(is, l2))
  }
}

def rev(l: List[i64]): List[i64] {
  appendRev(l, Nil)
}

def append(l1: List[i64], l2: List[i64]): List[i64] {
  l2.case[i64] {
    Nil => l1,
    Cons(is, iss) => appendRev(rev(l1), Cons(is, iss))
  }
}

def delete_by(f: Fun[i64, Fun[i64, Bool]], x: i64, ys: List[i64]): List[i64] {
  ys.case[i64] {
    Nil => Nil,
    Cons(y, ys) => f.Apply[i64, Fun[i64, Bool]](x).Apply[i64, Bool](y).case {
      True => ys,
      False => Cons(y, delete_by(f, x, ys))
    }
  }
}

def nub_by(f: Fun[i64, Fun[i64, Bool]], ls: List[i64]): List[i64] {
  ls.case[i64] {
    Nil => Nil,
    Cons(h, t) => Cons(h, nub_by(f, filter_union(new { Apply(y) => not(f.Apply[i64, Fun[i64, Bool]](h).Apply[i64, Bool](y)) }, t)))
  }
}

def filter_union(f: Fun[i64, Bool], ls: List[i64]): List[i64] {
  ls.case[i64] {
    Nil => Nil,
    Cons(i, is) => f.Apply[i64, Bool](i).case {
      True => Cons(i, filter_union(f, is)),
      False => filter_union(f, is)
    }
  }
}

def foldl(f: Fun[List[i64], Fun[i64, List[i64]]], a: List[i64], xs: List[i64]): List[i64] {
  xs.case[i64] {
    Nil => a,
    Cons(h, t) => foldl(f, f.Apply[List[i64], Fun[i64, List[i64]]](a).Apply[i64, List[i64]](h), t)
  }
}

def union_by(f: Fun[i64, Fun[i64, Bool]], l1: List[i64], l2: List[i64]): List[i64] {
  append(l1, foldl(new { Apply(acc) => new { Apply(y) => delete_by(f, y, acc) } }, nub_by(f, l2), l1))
}

def union(l1: List[i64], l2: List[i64]): List[i64] {
  union_by(new { Apply(x) => new { Apply(y) => eq(x, y) } }, l1, l2)
}

def combine(ls: List[Pair[List[Assign], ConflictSet]], acc: List[i64]): List[i64] {
  ls.case[Pair[List[Assign], ConflictSet]] {
    Nil => acc,
    Cons(p, css) => p.case[List[Assign], ConflictSet] {
      Tup(s, cs) => cs.case {
        Known(cs) => not_elem(max_level(s), cs).case {
          True => cs,
          False => combine(css, union(cs, acc))
        },
        Unknown => acc
      }
    }
  }
}

//bj_

def bj_(csp: CSP, t: Node[Pair[List[Assign], ConflictSet]]): Node[Pair[List[Assign], ConflictSet]] {
  let f7: Fun2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]] =
    new { Apply2(tp2, chs) =>
      tp2.case[List[Assign], ConflictSet] { Tup(a, conf) => conf.case {
        Known(cs) => Node(Tup(a, Known(cs)), chs),
        Unknown =>
          let cs_: ConflictSet = Known(combine(bj_map(new { Apply(x) => bj_label(x) }, chs), Nil));
         known_conflict(cs_).case {
            True => Node(Tup(a, cs_), Nil),
            False => Node(Tup(a, cs_), chs)
          }
      }}
    };
  bj_fold_tree(f7, t)
}

// bj
def bj_label(n: Node[Pair[List[Assign], ConflictSet]]): Pair[List[Assign], ConflictSet] {
  n.case[Pair[List[Assign], ConflictSet]] {
    Node(l, cs) => l
  }
}

def bj_rev_loop(
  l1: List[Pair[List[Assign], ConflictSet]],
  l2: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  l1.case[Pair[List[Assign], ConflictSet]] {
    Nil => l2,
    Cons(is, iss) => bj_rev_loop(iss, Cons(is, l2))
  }
}

def bj_rev(l: List[Pair[List[Assign], ConflictSet]]): List[Pair[List[Assign], ConflictSet]] {
  bj_rev_loop(l, Nil)
}

def bj_map_loop(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Pair[List[Assign], ConflictSet]],
  l: List[Node[Pair[List[Assign], ConflictSet]]],
  acc: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
  l.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => bj_rev(acc),
    Cons(p, ps) => bj_map_loop(f, ps, Cons(f.Apply[Node[Pair[List[Assign], ConflictSet]], Pair[List[Assign], ConflictSet]](p), acc))
  }
}

def bj_map(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Pair[List[Assign], ConflictSet]],
  l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Pair[List[Assign], ConflictSet]] {
  bj_map_loop(f, l, Nil)
}

def bj_rev2_loop(
  l1: List[Node[Pair[List[Assign], ConflictSet]]],
  l2: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l1.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => l2,
    Cons(is, iss) => bj_rev2_loop(iss, Cons(is, l2))
  }
}

def bj_rev2(l: List[Node[Pair[List[Assign], ConflictSet]]]): List[Node[Pair[List[Assign], ConflictSet]]] {
  bj_rev2_loop(l, Nil)
}

def bj_map2_loop(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => bj_rev2(acc),
    Cons(p, ps) => bj_map2_loop(f, ps, Cons(f.Apply[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def bj_map2(
  f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  bj_map2_loop(f, l, Nil)
}

def bj_fold_tree(
  f: Fun2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]],
  t: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
  t.case[Pair[List[Assign], ConflictSet]] {
    Node(l, c) =>
      f.Apply2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]]
      (l, bj_map2(new { Apply(x) => bj_fold_tree(f, x)}, c))
  }
}

def bj(csp: CSP, t: Node[Pair[List[Assign], ConflictSet]]): Node[Pair[List[Assign], ConflictSet]] {
  let f6: Fun2[Pair[List[Assign], ConflictSet], List[Node[Pair[List[Assign], ConflictSet]]], Node[Pair[List[Assign], ConflictSet]]] =
    new { Apply2(tp2, chs) =>
      tp2.case[List[Assign], ConflictSet] {
        Tup(a, conf) => conf.case {
          Known(cs) => Node(Tup(a, Known(cs)), chs),
          Unknown =>  Node(Tup(a, Known(combine(bj_map(new { Apply(x) => bj_label(x) }, chs), Nil))), chs)
        }
      }
    };
  bj_fold_tree(f6, t)
}

def bjbt(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
  bj(csp, bt(csp, t))
}


def bjbt_(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
 bj_(csp, bt(csp, t))
}

def collect(ls: List[ConflictSet]): List[i64] {
  ls.case[ConflictSet] {
    Nil=>Nil,
    Cons(conf, css) => conf.case {
      Known(cs) => union(cs, collect(css)),
      Unknown => Nil
    }
  }
}

//domain_wipeout

def wipe_all(f: Fun[ConflictSet, Bool], ls: List[ConflictSet]): Bool {
  ls.case[ConflictSet] {
    Nil => True,
    Cons(c, cs) => f.Apply[ConflictSet, Bool](c).case {
      True => wipe_all(f, cs),
      False => False
    }
  }
}

def wipe_lscomp1(ls: List[List[ConflictSet]]): List[List[ConflictSet]] {
  ls.case[List[ConflictSet]] {
    Nil => Nil,
    Cons(vs, t1) => wipe_all(new { Apply(x) => known_conflict(x)}, vs).case {
      True => Cons(vs, wipe_lscomp1(t1)),
      False => wipe_lscomp1(t1)
    }
  }
}

def wipe_null_(ls: List[List[ConflictSet]]): Bool {
  ls.case[List[ConflictSet]] {
    Nil => True,
    Cons(l, ls) => False
  }
}

def wipe_head(ls: List[List[ConflictSet]]): List[ConflictSet] {
  ls.case[List[ConflictSet]] {
    Nil => exit -1,
    Cons(l, ls)=>l
  }
}

def wipe_rev_loop(
  l1: List[Node[Pair[List[Assign], ConflictSet]]],
  l2: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l1.case[Node[Pair[List[Assign], ConflictSet]]] {
    Nil => l2,
    Cons(is, iss) => wipe_rev_loop(iss, Cons(is, l2))
  }
}

def wipe_rev(l: List[Node[Pair[List[Assign], ConflictSet]]]): List[Node[Pair[List[Assign], ConflictSet]]] {
  wipe_rev_loop(l, Nil)
}

def wipe_map_loop(
  f: Fun[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]],
  acc: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  l.case[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
    Nil => wipe_rev(acc),
    Cons(p, ps) => wipe_map_loop(f, ps, Cons(f.Apply[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]](p), acc))
  }
}

def wipe_map(
  f: Fun[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]], Node[Pair[List[Assign], ConflictSet]]],
  l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
  wipe_map_loop(f, l, Nil)
}

def wipe_map_tree(
  f: Fun[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]], Pair[List[Assign], ConflictSet]],
  t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
): Node[Pair[List[Assign], ConflictSet]] {
  t.case[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
    Node(l, c) =>
      Node(f.Apply[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]], Pair[List[Assign], ConflictSet]](l),
        wipe_map(new { Apply(x) => wipe_map_tree(f, x) }, c))
  }
}

def domain_wipeout(csp: CSP, t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]): Node[Pair[List[Assign], ConflictSet]] {
  csp.case {
    CSP(vars, vals, rel) =>
      let f8: Fun[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]], Pair[List[Assign], ConflictSet]] =
        new { Apply(tp2) =>
          tp2.case[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]] {
            Tup(p, tbl) => p.case[List[Assign], ConflictSet] {
              Tup(as_, cs) =>
                let wiped_domains: List[List[ConflictSet]]= wipe_lscomp1(tbl);
                let cs_: ConflictSet = wipe_null_(wiped_domains).case {
                  True => cs,
                  False => Known(collect(wipe_head(wiped_domains)))
                };
                Tup(as_, cs_)
          }}
        };
        wipe_map_tree(f8, t)
  }
}

def fc(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
  domain_wipeout(csp, lookup_cache(csp, cache_checks(csp, empty_table(csp), t)))
}

def list_len(l: List[List[Assign]]): i64 {
  l.case[List[Assign]] {
    Nil => 0,
    Cons(l, ls) => 1 + list_len(ls)
  }
}

def try_(n: i64, algorithm: Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]): i64 {
  list_len(search(algorithm, queens(n)))
}

def test_rev_loop(l1: List[i64], l2: List[i64]): List[i64] {
  l1.case[i64] {
    Nil => l2,
    Cons(is, iss) => test_rev_loop(iss, Cons(is, l2))
  }
}

def test_rev(l: List[i64]): List[i64] {
  test_rev_loop(l, Nil)
}

def test_map_loop(
  f: Fun[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], i64],
  l: List[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]],
  acc: List[i64]
): List[i64] {
  l.case[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]] {
    Nil => test_rev(acc),
    Cons(p, ps) => test_map_loop(f, ps, Cons(f.Apply[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], i64](p), acc))
  }
}

def test_map(
  f: Fun[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], i64],
  l: List[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]]
): List[i64] {
  test_map_loop(f, l, Nil)
}

def test_constraints_nofib(n: i64): List[i64] {
  test_map(new { Apply(x) => try_(n, x) },
    Cons(new { Apply2(csp, n) => bt(csp, n) },
      Cons(new { Apply2(csp, n) => bm(csp, n) },
        Cons(new { Apply2(csp, n) => bjbt(csp, n) },
          Cons(new { Apply2(csp, n) => bjbt_(csp, n) },
            Cons(new { Apply2(csp, n) => fc(csp, n) },
              Nil))))))
}

def head(l: List[i64]): i64 {
  l.case[i64] {
    Nil => -1,
    Cons(x, xs) => x
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: List[i64] = test_constraints_nofib(n);
    println_i64(head(res));
    0
  } else {
    let res: List[i64] = test_constraints_nofib(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
