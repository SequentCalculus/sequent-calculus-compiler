data List[A] { Nil, Cons(a: A, as: List[A]) }

data Pair[A, B] { Tup(a: A, b: B) }

data Option[A] { None, Some(a: A) }

data Bool { True, False }

codata Fun[A, B] { apply(a: A): B }

codata Fun2[A, B, C] { apply2(a: A, b: B): C }

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
    a.case { Assign(varr, value) => varr }
}

def value(a: Assign): i64 {
    a.case { Assign(varr, value) => value }
}

def max_level(ls: List[Assign]): i64 {
    ls.case[Assign] {
        Nil => 0,
        Cons(a, t) => a.case { Assign(v, value) => v }
    }
}

def complete(csp: CSP, s: List[Assign]): Bool {
    csp.case { CSP(v, vals, rel) => eq(max_level(s), v) }
}

def enum_from_to(from: i64, to_: i64): List[i64] {
    if from <= to_ { Cons(from, enum_from_to(from + 1, to_)) } else { Nil }
}

def safe(as1: Assign, as2: Assign): Bool {
    as1.case {
        Assign(i, m) =>
            as2.case {
                Assign(j, n) =>
                    not(eq(m, n)).case {
                        True => not(eq(abs(i - j), abs(m - n))),
                        False => False
                    }
            }
    }
}

def queens(n: i64): CSP {
    CSP(n, n, new { apply2(x, y) => safe(x, y) })
}

def mk_map(
    f: Fun[List[Assign], Node[List[Assign]]],
    l: List[List[Assign]]
): List[Node[List[Assign]]] {
    l.case[List[Assign]] {
        Nil => Nil,
        Cons(p, ps) => Cons(f.apply[List[Assign], Node[List[Assign]]](p), mk_map(f, ps))
    }
}

def mk_init_tree(f: Fun[List[Assign], List[List[Assign]]], x: List[Assign]): Node[List[Assign]] {
    Node(
        x,
        mk_map(new { apply(y) => mk_init_tree(f, y) }, f.apply[List[Assign], List[List[Assign]]](x))
    )
}

def to_assign(ls: List[i64], ss: List[Assign]): List[List[Assign]] {
    ls.case[i64] {
        Nil => Nil,
        Cons(j, t1) => Cons(Cons(Assign(max_level(ss) + 1, j), ss), to_assign(t1, ss))
    }
}

def mk_tree(csp: CSP): Node[List[Assign]] {
    csp.case {
        CSP(vars, vals, rel) =>
            let next: Fun[List[Assign], List[List[Assign]]] = new {
                apply(ss) =>
                    if max_level(ss) < vars { to_assign(enum_from_to(1, vals), ss) } else { Nil }
            };
            mk_init_tree(next, Nil)
    }
}

def ear_inc_filter(f: Fun[Assign, Bool], ls: List[Assign]): List[Assign] {
    ls.case[Assign] {
        Nil => Nil,
        Cons(a, as) =>
            f.apply[Assign, Bool](a)
                .case {
                    True => Cons(a, ear_inc_filter(f, as)),
                    False => ear_inc_filter(f, as)
                }
    }
}

def earliest_inconsistency(csp: CSP, aas: List[Assign]): Option[Pair[i64, i64]] {
    csp.case {
        CSP(vars, vals, rel) =>
            aas.case[Assign] {
                Nil => None,
                Cons(a, as_) =>
                    ear_inc_filter(
                        new { apply(x) => not(rel.apply2[Assign, Assign, Bool](a, x)) },
                        reverse(as_)
                    ).case[Assign] {
                        Nil => None,
                        Cons(b, bs_) => Some(Tup(level(a), level(b)))
                    }
            }
    }
}

def known_conflict(c: ConflictSet): Bool {
    c.case {
        Known(vs) =>
            vs.case[i64] {
                Nil => False,
                Cons(v, vs) => True
            },
        Unknown => False
    }
}

def known_solution(c: ConflictSet): Bool {
    c.case {
        Known(vs) =>
            vs.case[i64] {
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

def search_map(
    f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
    l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[List[Assign], ConflictSet]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[List[Assign], ConflictSet]],
                    Node[Pair[List[Assign], ConflictSet]]
                ](p),
                search_map(f, ps)
            )
    }
}

def search_filter(
    f: Fun[Node[Pair[List[Assign], ConflictSet]], Bool],
    l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[List[Assign], ConflictSet]]] {
        Nil => Nil,
        Cons(p, ps) =>
            f.apply[Node[Pair[List[Assign], ConflictSet]], Bool](p)
                .case {
                    True => Cons(p, search_filter(f, ps)),
                    False => search_filter(f, ps)
                }
    }
}

def search_label(n: Node[Pair[List[Assign], ConflictSet]]): Pair[List[Assign], ConflictSet] {
    n.case[Pair[List[Assign], ConflictSet]] { Node(p, cs) => p }
}

def search_fold_tree(
    f: Fun2[
        Pair[List[Assign], ConflictSet],
        List[Node[Pair[List[Assign], ConflictSet]]],
        Node[Pair[List[Assign], ConflictSet]]
    ],
    n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
    n.case[Pair[List[Assign], ConflictSet]] {
        Node(l, c) =>
            f.apply2[
                Pair[List[Assign], ConflictSet],
                List[Node[Pair[List[Assign], ConflictSet]]],
                Node[Pair[List[Assign], ConflictSet]]
            ](l, search_map(new { apply(x) => search_fold_tree(f, x) }, c))
    }
}

def search_filter_tree(
    p: Fun[Pair[List[Assign], ConflictSet], Bool],
    n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
    let f: Fun2[
        Pair[List[Assign], ConflictSet],
        List[Node[Pair[List[Assign], ConflictSet]]],
        Node[Pair[List[Assign], ConflictSet]]
    ] = new {
        apply2(a, cs) =>
            Node(
                a,
                search_filter(
                    new {
                        apply(x) => p.apply[Pair[List[Assign], ConflictSet], Bool](search_label(x))
                    },
                    cs
                )
            )
    };
    search_fold_tree(f, n)
}

def search_prune(
    f: Fun[Pair[List[Assign], ConflictSet], Bool],
    n: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
    search_filter_tree(
        new { apply(x) => not(f.apply[Pair[List[Assign], ConflictSet], Bool](x)) },
        n
    )
}

def search_map2(
    f: Fun[Node[Pair[List[Assign], ConflictSet]], List[Pair[List[Assign], ConflictSet]]],
    l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[List[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[List[Assign], ConflictSet]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[List[Assign], ConflictSet]],
                    List[Pair[List[Assign], ConflictSet]]
                ](p),
                search_map2(f, ps)
            )
    }
}

def search_rev_loop(
    l1: List[Pair[List[Assign], ConflictSet]],
    l2: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
    l1.case[Pair[List[Assign], ConflictSet]] {
        Nil => l2,
        Cons(p, ps) => search_rev_loop(ps, Cons(p, l2))
    }
}

def search_rev(l: List[Pair[List[Assign], ConflictSet]]): List[Pair[List[Assign], ConflictSet]] {
    search_rev_loop(l, Nil)
}

def search_concat_loop(
    ls: List[List[Pair[List[Assign], ConflictSet]]],
    acc: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
    ls.case[List[Pair[List[Assign], ConflictSet]]] {
        Nil => search_rev(acc),
        Cons(l, ls) => search_concat_loop(ls, search_rev_loop(l, acc))
    }
}

def search_concat(
    ls: List[List[Pair[List[Assign], ConflictSet]]]
): List[Pair[List[Assign], ConflictSet]] {
    search_concat_loop(ls, Nil)
}

def search_leaves(n: Node[Pair[List[Assign], ConflictSet]]): List[Pair[List[Assign], ConflictSet]] {
    n.case[Pair[List[Assign], ConflictSet]] {
        Node(leaf, cs) =>
            cs.case[Node[Pair[List[Assign], ConflictSet]]] {
                Nil => Cons(leaf, Nil),
                Cons(c, cs) =>
                    search_concat(search_map2(new { apply(x) => search_leaves(x) }, Cons(c, cs)))
            }
    }
}

def search_map3(
    f: Fun[Pair[List[Assign], ConflictSet], List[Assign]],
    l: List[Pair[List[Assign], ConflictSet]]
): List[List[Assign]] {
    l.case[Pair[List[Assign], ConflictSet]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(f.apply[Pair[List[Assign], ConflictSet], List[Assign]](p), search_map3(f, ps))
    }
}

def search_filter2(
    f: Fun[Pair[List[Assign], ConflictSet], Bool],
    l: List[Pair[List[Assign], ConflictSet]]
): List[Pair[List[Assign], ConflictSet]] {
    l.case[Pair[List[Assign], ConflictSet]] {
        Nil => Nil,
        Cons(p, ps) =>
            f.apply[Pair[List[Assign], ConflictSet], Bool](p)
                .case {
                    True => Cons(p, search_filter2(f, ps)),
                    False => search_filter2(f, ps)
                }
    }
}

def search_fst(p: Pair[List[Assign], ConflictSet]): List[Assign] {
    p.case[List[Assign], ConflictSet] { Tup(l, c) => l }
}

def search_snd(p: Pair[List[Assign], ConflictSet]): ConflictSet {
    p.case[List[Assign], ConflictSet] { Tup(l, c) => c }
}

def search(
    labeler: Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]],
    csp: CSP
): List[List[Assign]] {
    search_map3(
        new { apply(x) => search_fst(x) },
        search_filter2(
            new { apply(x) => known_solution(search_snd(x)) },
            search_leaves(
                search_prune(
                    new { apply(x) => known_conflict(search_snd(x)) },
                    labeler
                        .apply2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]](
                            csp,
                            mk_tree(csp)
                        )
                )
            )
        )
    )
}

def bt_map(
    f: Fun[Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]],
    l: List[Node[List[Assign]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[List[Assign]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]](p),
                bt_map(f, ps)
            )
    }
}

def bt_map_tree(
    f: Fun[List[Assign], Pair[List[Assign], ConflictSet]],
    n: Node[List[Assign]]
): Node[Pair[List[Assign], ConflictSet]] {
    n.case[List[Assign]] {
        Node(l, ls) =>
            Node(
                f.apply[List[Assign], Pair[List[Assign], ConflictSet]](l),
                bt_map(new { apply(x) => bt_map_tree(f, x) }, ls)
            )
    }
}

def bt(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
    let f3: Fun[List[Assign], Pair[List[Assign], ConflictSet]] = new {
        apply(s) =>
            Tup(
                s,
                (
                    earliest_inconsistency(csp, s).case[Pair[i64, i64]] {
                        Some(p) => p.case[i64, i64] { Tup(a, b) => Known(Cons(a, Cons(b, Nil))) },
                        None => check_complete(csp, s)
                    }
                )
            )
    };
    bt_map_tree(f3, t)
}

def to_unknown(ls: List[i64]): List[ConflictSet] {
    ls.case[i64] {
        Nil => Nil,
        Cons(m, t2) => Cons(Unknown, to_unknown(t2))
    }
}

def n_unknown(ls: List[i64], n: i64): List[List[ConflictSet]] {
    ls.case[i64] {
        Nil => Nil,
        Cons(n, t1) => Cons(to_unknown(enum_from_to(1, n)), n_unknown(t1, n))
    }
}

def empty_table(csp: CSP): List[List[ConflictSet]] {
    csp.case { CSP(vars, vals, rel) => Cons(Nil, n_unknown(enum_from_to(1, vars), vals)) }
}

def to_pair(ls: List[i64], varrr: i64): List[Pair[i64, i64]] {
    ls.case[i64] {
        Nil => Nil,
        Cons(valll, t2) => Cons(Tup(varrr, valll), to_pair(t2, varrr))
    }
}

def n_pairs(ls: List[i64], n: i64): List[List[Pair[i64, i64]]] {
    ls.case[i64] {
        Nil => Nil,
        Cons(varrr, t1) => Cons(to_pair(enum_from_to(1, n), varrr), n_pairs(t1, n))
    }
}

def fill_zip_with(
    f: Fun2[ConflictSet, Pair[i64, i64], ConflictSet],
    x: List[ConflictSet],
    y: List[Pair[i64, i64]]
): List[ConflictSet] {
    x.case[ConflictSet] {
        Nil => Nil,
        Cons(c, cs) =>
            y.case[Pair[i64, i64]] {
                Nil => Nil,
                Cons(p, ps) =>
                    Cons(
                        f.apply2[ConflictSet, Pair[i64, i64], ConflictSet](c, p),
                        fill_zip_with(f, cs, ps)
                    )
            }
    }
}

def fill_zip_with2(
    f: Fun2[List[ConflictSet], List[Pair[i64, i64]], List[ConflictSet]],
    tbl: List[List[ConflictSet]],
    ls: List[List[Pair[i64, i64]]]
): List[List[ConflictSet]] {
    tbl.case[List[ConflictSet]] {
        Nil => Nil,
        Cons(cs, css) =>
            ls.case[List[Pair[i64, i64]]] {
                Nil => Nil,
                Cons(ps, pss) =>
                    Cons(
                        f.apply2[List[ConflictSet], List[Pair[i64, i64]], List[ConflictSet]](
                            cs,
                            ps
                        ),
                        fill_zip_with2(f, css, pss)
                    )
            }
    }
}

def fill_table(s: List[Assign], csp: CSP, tbl: List[List[ConflictSet]]): List[List[ConflictSet]] {
    s.case[Assign] {
        Nil => tbl,
        Cons(as, as_) =>
            as.case {
                Assign(var_, val_) =>
                    csp.case {
                        CSP(vars, vals, rel) =>
                            let f4: Fun2[ConflictSet, Pair[i64, i64], ConflictSet] = new {
                                apply2(cs, varval) =>
                                    varval.case[i64, i64] {
                                        Tup(varr, vall) =>
                                            cs.case {
                                                Known(vs) => cs,
                                                Unknown =>
                                                    not(
                                                        rel.apply2[Assign, Assign, Bool](
                                                            Assign(var_, val_),
                                                            Assign(varr, vall)
                                                        )
                                                    ).case {
                                                        True => Known(Cons(var_, Cons(varr, Nil))),
                                                        False => cs
                                                    }
                                            }
                                    }
                            };
                            fill_zip_with2(
                                new { apply2(x, y) => fill_zip_with(f4, x, y) },
                                tbl,
                                n_pairs(enum_from_to(var_ + 1, vars), vals)
                            )
                    }
            }
    }
}

def lookup_map(
    f: Fun[
        Node[Pair[List[Assign], List[List[ConflictSet]]]],
        Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
    ],
    l: List[Node[Pair[List[Assign], List[List[ConflictSet]]]]]
): List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
    l.case[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[List[Assign], List[List[ConflictSet]]]],
                    Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
                ](p),
                lookup_map(f, ps)
            )
    }
}

def lookup_map_tree(
    f: Fun[
        Pair[List[Assign], List[List[ConflictSet]]],
        Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]
    ],
    t: Node[Pair[List[Assign], List[List[ConflictSet]]]]
): Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
    t.case[Pair[List[Assign], List[List[ConflictSet]]]] {
        Node(p, ps) =>
            Node(
                f.apply[
                    Pair[List[Assign], List[List[ConflictSet]]],
                    Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]
                ](p),
                lookup_map(new { apply(x) => lookup_map_tree(f, x) }, ps)
            )
    }
}

def lookup_at_index(ind: i64, ls: List[ConflictSet]): ConflictSet {
    ls.case[ConflictSet] {
        Nil => Unknown,
        Cons(c, cs) => if ind == 0 { c } else { lookup_at_index(ind - 1, cs) }
    }
}

def lookup_head(tbl: List[List[ConflictSet]]): List[ConflictSet] {
    tbl.case[List[ConflictSet]] {
        Nil => Nil,
        Cons(cs, css) => cs
    }
}

def lookup_cache(
    csp: CSP,
    t: Node[Pair[List[Assign], List[List[ConflictSet]]]]
): Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
    let f5: Fun2[
        CSP,
        Pair[List[Assign], List[List[ConflictSet]]],
        Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]
    ] = new {
        apply2(csp, tp) =>
            tp.case[List[Assign], List[List[ConflictSet]]] {
                Tup(ls, tbl) =>
                    ls.case[Assign] {
                        Nil => Tup(Tup(Nil, Unknown), tbl),
                        Cons(a, as_) =>
                            let table_entry: ConflictSet = lookup_at_index(
                                value(a) - 1,
                                lookup_head(tbl)
                            );
                            let cs: ConflictSet = table_entry.case {
                                Unknown => check_complete(csp, Cons(a, as_)),
                                Known(vals) => table_entry
                            };
                            Tup(Tup(Cons(a, as_), cs), tbl)
                    }
            }
    };
    lookup_map_tree(
        new {
            apply(x) =>
                f5.apply2[
                    CSP,
                    Pair[List[Assign], List[List[ConflictSet]]],
                    Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]
                ](csp, x)
        },
        t
    )
}

def checks_map(
    f: Fun[Node[List[Assign]], Node[Pair[List[Assign], List[List[ConflictSet]]]]],
    l: List[Node[List[Assign]]]
): List[Node[Pair[List[Assign], List[List[ConflictSet]]]]] {
    l.case[Node[List[Assign]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[Node[List[Assign]], Node[Pair[List[Assign], List[List[ConflictSet]]]]](p),
                checks_map(f, ps)
            )
    }
}

def checks_tail(ls: List[List[ConflictSet]]): List[List[ConflictSet]] {
    ls.case[List[ConflictSet]] {
        Nil => Nil,
        Cons(l, ls) => ls
    }
}

def cache_checks(
    csp: CSP,
    tbl: List[List[ConflictSet]],
    n: Node[List[Assign]]
): Node[Pair[List[Assign], List[List[ConflictSet]]]] {
    n.case[List[Assign]] {
        Node(s, cs) =>
            Node(
                Tup(s, tbl),
                checks_map(
                    new { apply(x) => cache_checks(csp, fill_table(s, csp, checks_tail(tbl)), x) },
                    cs
                )
            )
    }
}

def bm_fst(
    x: Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]
): Pair[List[Assign], ConflictSet] {
    x.case[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]] { Tup(p, ls) => p }
}

def bm_map(
    f: Fun[
        Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]],
        Node[Pair[List[Assign], ConflictSet]]
    ],
    l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]],
                    Node[Pair[List[Assign], ConflictSet]]
                ](p),
                bm_map(f, ps)
            )
    }
}

def bm_map_tree(
    f: Fun[
        Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]],
        Pair[List[Assign], ConflictSet]
    ],
    t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
): Node[Pair[List[Assign], ConflictSet]] {
    t.case[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
        Node(p, ps) =>
            Node(
                f.apply[
                    Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]],
                    Pair[List[Assign], ConflictSet]
                ](p),
                bm_map(new { apply(x) => bm_map_tree(f, x) }, ps)
            )
    }
}

def bm(csp: CSP, t: Node[List[Assign]]): Node[Pair[List[Assign], ConflictSet]] {
    bm_map_tree(
        new { apply(x) => bm_fst(x) },
        lookup_cache(csp, cache_checks(csp, empty_table(csp), t))
    )
}

def in_list(i: i64, ls: List[i64]): Bool {
    ls.case[i64] {
        Nil => False,
        Cons(j, js) => if i == j { True } else { in_list(i, js) }
    }
}

def not_elem(i: i64, ls: List[i64]): Bool {
    not(in_list(i, ls))
}

def append(l1: List[i64], l2: List[i64]): List[i64] {
    l1.case[i64] {
        Nil => l2,
        Cons(is, iss) => Cons(is, append(iss, l2))
    }
}

def delete_by(f: Fun[i64, Fun[i64, Bool]], x: i64, ys: List[i64]): List[i64] {
    ys.case[i64] {
        Nil => Nil,
        Cons(y, ys) =>
            f.apply[i64, Fun[i64, Bool]](x)
                .apply[i64, Bool](y)
                .case {
                    True => ys,
                    False => Cons(y, delete_by(f, x, ys))
                }
    }
}

def nub_by(f: Fun[i64, Fun[i64, Bool]], ls: List[i64]): List[i64] {
    ls.case[i64] {
        Nil => Nil,
        Cons(h, t) =>
            Cons(
                h,
                nub_by(
                    f,
                    filter_union(
                        new {
                            apply(y) => not(f.apply[i64, Fun[i64, Bool]](h).apply[i64, Bool](y))
                        },
                        t
                    )
                )
            )
    }
}

def filter_union(f: Fun[i64, Bool], ls: List[i64]): List[i64] {
    ls.case[i64] {
        Nil => Nil,
        Cons(i, is) =>
            f.apply[i64, Bool](i)
                .case {
                    True => Cons(i, filter_union(f, is)),
                    False => filter_union(f, is)
                }
    }
}

def foldl(f: Fun[List[i64], Fun[i64, List[i64]]], a: List[i64], xs: List[i64]): List[i64] {
    xs.case[i64] {
        Nil => a,
        Cons(h, t) =>
            foldl(f, f.apply[List[i64], Fun[i64, List[i64]]](a).apply[i64, List[i64]](h), t)
    }
}

def union_by(f: Fun[i64, Fun[i64, Bool]], l1: List[i64], l2: List[i64]): List[i64] {
    append(
        l1,
        foldl(new { apply(acc) => new { apply(y) => delete_by(f, y, acc) } }, nub_by(f, l2), l1)
    )
}

def union(l1: List[i64], l2: List[i64]): List[i64] {
    union_by(new { apply(x) => new { apply(y) => eq(x, y) } }, l1, l2)
}

def combine(ls: List[Pair[List[Assign], ConflictSet]], acc: List[i64]): List[i64] {
    ls.case[Pair[List[Assign], ConflictSet]] {
        Nil => acc,
        Cons(p, css) =>
            p.case[List[Assign], ConflictSet] {
                Tup(s, cs) =>
                    cs.case {
                        Known(cs) =>
                            not_elem(max_level(s), cs).case {
                                True => cs,
                                False => combine(css, union(cs, acc))
                            },
                        Unknown => acc
                    }
            }
    }
}

def bj_(csp: CSP, t: Node[Pair[List[Assign], ConflictSet]]): Node[Pair[List[Assign], ConflictSet]] {
    let f7: Fun2[
        Pair[List[Assign], ConflictSet],
        List[Node[Pair[List[Assign], ConflictSet]]],
        Node[Pair[List[Assign], ConflictSet]]
    ] = new {
        apply2(tp2, chs) =>
            tp2.case[List[Assign], ConflictSet] {
                Tup(a, conf) =>
                    conf.case {
                        Known(cs) => Node(Tup(a, Known(cs)), chs),
                        Unknown =>
                            let cs_: ConflictSet = Known(
                                combine(bj_map(new { apply(x) => bj_label(x) }, chs), Nil)
                            );
                            known_conflict(cs_).case {
                                True => Node(Tup(a, cs_), Nil),
                                False => Node(Tup(a, cs_), chs)
                            }
                    }
            }
    };
    bj_fold_tree(f7, t)
}

def bj_label(n: Node[Pair[List[Assign], ConflictSet]]): Pair[List[Assign], ConflictSet] {
    n.case[Pair[List[Assign], ConflictSet]] { Node(l, cs) => l }
}

def bj_map(
    f: Fun[Node[Pair[List[Assign], ConflictSet]], Pair[List[Assign], ConflictSet]],
    l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Pair[List[Assign], ConflictSet]] {
    l.case[Node[Pair[List[Assign], ConflictSet]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[Node[Pair[List[Assign], ConflictSet]], Pair[List[Assign], ConflictSet]](p),
                bj_map(f, ps)
            )
    }
}

def bj_map2(
    f: Fun[Node[Pair[List[Assign], ConflictSet]], Node[Pair[List[Assign], ConflictSet]]],
    l: List[Node[Pair[List[Assign], ConflictSet]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[List[Assign], ConflictSet]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[List[Assign], ConflictSet]],
                    Node[Pair[List[Assign], ConflictSet]]
                ](p),
                bj_map2(f, ps)
            )
    }
}

def bj_fold_tree(
    f: Fun2[
        Pair[List[Assign], ConflictSet],
        List[Node[Pair[List[Assign], ConflictSet]]],
        Node[Pair[List[Assign], ConflictSet]]
    ],
    t: Node[Pair[List[Assign], ConflictSet]]
): Node[Pair[List[Assign], ConflictSet]] {
    t.case[Pair[List[Assign], ConflictSet]] {
        Node(l, c) =>
            f.apply2[
                Pair[List[Assign], ConflictSet],
                List[Node[Pair[List[Assign], ConflictSet]]],
                Node[Pair[List[Assign], ConflictSet]]
            ](l, bj_map2(new { apply(x) => bj_fold_tree(f, x) }, c))
    }
}

def bj(csp: CSP, t: Node[Pair[List[Assign], ConflictSet]]): Node[Pair[List[Assign], ConflictSet]] {
    let f6: Fun2[
        Pair[List[Assign], ConflictSet],
        List[Node[Pair[List[Assign], ConflictSet]]],
        Node[Pair[List[Assign], ConflictSet]]
    ] = new {
        apply2(tp2, chs) =>
            tp2.case[List[Assign], ConflictSet] {
                Tup(a, conf) =>
                    conf.case {
                        Known(cs) => Node(Tup(a, Known(cs)), chs),
                        Unknown =>
                            Node(
                                Tup(
                                    a,
                                    Known(
                                        combine(bj_map(new { apply(x) => bj_label(x) }, chs), Nil)
                                    )
                                ),
                                chs
                            )
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
        Nil => Nil,
        Cons(conf, css) =>
            conf.case {
                Known(cs) => union(cs, collect(css)),
                Unknown => Nil
            }
    }
}

def wipe_all(f: Fun[ConflictSet, Bool], ls: List[ConflictSet]): Bool {
    ls.case[ConflictSet] {
        Nil => True,
        Cons(c, cs) =>
            f.apply[ConflictSet, Bool](c)
                .case {
                    True => wipe_all(f, cs),
                    False => False
                }
    }
}

def filter_known(
    f: Fun[List[ConflictSet], Bool],
    ls: List[List[ConflictSet]]
): List[List[ConflictSet]] {
    ls.case[List[ConflictSet]] {
        Nil => Nil,
        Cons(vs, t1) =>
            f.apply[List[ConflictSet], Bool](vs)
                .case {
                    True => Cons(vs, filter_known(f, t1)),
                    False => filter_known(f, t1)
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
        Nil => Nil,
        Cons(l, ls) => l
    }
}

def wipe_map(
    f: Fun[
        Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]],
        Node[Pair[List[Assign], ConflictSet]]
    ],
    l: List[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]]
): List[Node[Pair[List[Assign], ConflictSet]]] {
    l.case[Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[
                    Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]],
                    Node[Pair[List[Assign], ConflictSet]]
                ](p),
                wipe_map(f, ps)
            )
    }
}

def wipe_map_tree(
    f: Fun[
        Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]],
        Pair[List[Assign], ConflictSet]
    ],
    t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
): Node[Pair[List[Assign], ConflictSet]] {
    t.case[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]] {
        Node(l, c) =>
            Node(
                f.apply[
                    Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]],
                    Pair[List[Assign], ConflictSet]
                ](l),
                wipe_map(new { apply(x) => wipe_map_tree(f, x) }, c)
            )
    }
}

def domain_wipeout(
    csp: CSP,
    t: Node[Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]]]
): Node[Pair[List[Assign], ConflictSet]] {
    csp.case {
        CSP(vars, vals, rel) =>
            let f8: Fun[
                Pair[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]],
                Pair[List[Assign], ConflictSet]
            ] = new {
                apply(tp2) =>
                    tp2.case[Pair[List[Assign], ConflictSet], List[List[ConflictSet]]] {
                        Tup(p, tbl) =>
                            p.case[List[Assign], ConflictSet] {
                                Tup(as_, cs) =>
                                    let wiped_domains: List[List[ConflictSet]] = filter_known(
                                        new {
                                            apply(vs) =>
                                                wipe_all(new { apply(x) => known_conflict(x) }, vs)
                                        },
                                        tbl
                                    );
                                    let cs_: ConflictSet = wipe_null_(wiped_domains).case {
                                        True => cs,
                                        False => Known(collect(wipe_head(wiped_domains)))
                                    };
                                    Tup(as_, cs_)
                            }
                    }
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

def try_(
    n: i64,
    algorithm: Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]
): i64 {
    list_len(search(algorithm, queens(n)))
}

def test_map(
    f: Fun[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], i64],
    l: List[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]]
): List[i64] {
    l.case[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]]] {
        Nil => Nil,
        Cons(p, ps) =>
            Cons(
                f.apply[Fun2[CSP, Node[List[Assign]], Node[Pair[List[Assign], ConflictSet]]], i64](
                    p
                ),
                test_map(f, ps)
            )
    }
}

def test_constraints_nofib(n: i64): List[i64] {
    test_map(
        new { apply(x) => try_(n, x) },
        Cons(
            new { apply2(csp, n) => bt(csp, n) },
            Cons(
                new { apply2(csp, n) => bm(csp, n) },
                Cons(
                    new { apply2(csp, n) => bjbt(csp, n) },
                    Cons(
                        new { apply2(csp, n) => bjbt_(csp, n) },
                        Cons(new { apply2(csp, n) => fc(csp, n) }, Nil)
                    )
                )
            )
        )
    )
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