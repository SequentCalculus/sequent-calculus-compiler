data Unit { Unit }

data Pair[A, B] { Tup(fst: A, snd: B) }

data List[A] { Nil, Cons(x: A, xs: List[A]) }

data Gen { Gen(coordslist: List[Pair[i64, i64]]) }

data Bool { True, False }

codata Fun[A, B] { apply(x: A): B }

codata Fun2[A, B, C] { apply2(x: A, y: B): C }

def pair_eq(p1: Pair[i64, i64], p2: Pair[i64, i64]): Bool {
    p1.case[i64, i64] {
        Tup(fst1, snd1) =>
            p2.case[i64, i64] {
                Tup(fst2, snd2) =>
                    if fst1 == fst2 { if snd1 == snd2 { True } else { False } } else { False }
            }
    }
}

def not(b: Bool): Bool {
    b.case {
        True => False,
        False => True
    }
}

def fold(
    a: List[Pair[i64, i64]],
    xs: List[Pair[i64, i64]],
    f: Fun2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]]
): List[Pair[i64, i64]] {
    xs.case[Pair[i64, i64]] {
        Nil => a,
        Cons(b, x) =>
            fold(f.apply2[List[Pair[i64, i64]], Pair[i64, i64], List[Pair[i64, i64]]](a, b), x, f)
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
    accumulate(x, y, new { apply2(t, h) => Cons(h, t) })
}

def collect_accum(
    sofar: List[Pair[i64, i64]],
    xs: List[Pair[i64, i64]],
    f: Fun[Pair[i64, i64], List[Pair[i64, i64]]]
): List[Pair[i64, i64]] {
    xs.case[Pair[i64, i64]] {
        Nil => sofar,
        Cons(p, xs) =>
            collect_accum(revonto(sofar, f.apply[Pair[i64, i64], List[Pair[i64, i64]]](p)), xs, f)
    }
}

def collect(
    l: List[Pair[i64, i64]],
    f: Fun[Pair[i64, i64], List[Pair[i64, i64]]]
): List[Pair[i64, i64]] {
    collect_accum(Nil, l, f)
}

def append(l1: List[Pair[i64, i64]], l2: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
    l1.case[Pair[i64, i64]] {
        Nil => l2,
        Cons(is, iss) => Cons(is, append(iss, l2))
    }
}

def map(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Pair[i64, i64]]): List[Pair[i64, i64]] {
    l.case[Pair[i64, i64]] {
        Nil => Nil,
        Cons(p, ps) => Cons(f.apply[Pair[i64, i64], Pair[i64, i64]](p), map(ps, f))
    }
}

def exists(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Bool]): Bool {
    l.case[Pair[i64, i64]] {
        Nil => False,
        Cons(p, ps) =>
            f.apply[Pair[i64, i64], Bool](p)
                .case {
                    True => True,
                    False => exists(ps, f)
                }
    }
}

def len(l: List[Pair[i64, i64]]): i64 {
    l.case[Pair[i64, i64]] {
        Nil => 0,
        Cons(p, ps) => 1 + len(ps)
    }
}

def member(l: List[Pair[i64, i64]], p: Pair[i64, i64]): Bool {
    exists(l, new { apply(p1) => pair_eq(p, p1) })
}

def filter(l: List[Pair[i64, i64]], f: Fun[Pair[i64, i64], Bool]): List[Pair[i64, i64]] {
    l.case[Pair[i64, i64]] {
        Nil => Nil,
        Cons(p, ps) =>
            f.apply[Pair[i64, i64], Bool](p)
                .case {
                    True => Cons(p, filter(ps, f)),
                    False => filter(ps, f)
                }
    }
}

def diff(x: List[Pair[i64, i64]], y: List[Pair[i64, i64]]): List[Pair[i64, i64]] {
    filter(x, new { apply(p) => not(member(y, p)) })
}

def alive(g: Gen): List[Pair[i64, i64]] {
    g.case { Gen(livecoords) => livecoords }
}

def neighbours(p: Pair[i64, i64]): List[Pair[i64, i64]] {
    p.case[i64, i64] {
        Tup(fst, snd) =>
            Cons(
                Tup(fst - 1, snd - 1),
                Cons(
                    Tup(fst - 1, snd),
                    Cons(
                        Tup(fst - 1, snd + 1),
                        Cons(
                            Tup(fst, snd - 1),
                            Cons(
                                Tup(fst, snd + 1),
                                Cons(
                                    Tup(fst + 1, snd - 1),
                                    Cons(Tup(fst + 1, snd), Cons(Tup(fst + 1, snd + 1), Nil))
                                )
                            )
                        )
                    )
                )
            )
    }
}

def twoorthree(n: i64): Bool {
    if n == 2 { True } else { if n == 3 { True } else { False } }
}

def collect_neighbors(
    xover: List[Pair[i64, i64]],
    x3: List[Pair[i64, i64]],
    x2: List[Pair[i64, i64]],
    x1: List[Pair[i64, i64]],
    xs: List[Pair[i64, i64]]
): List[Pair[i64, i64]] {
    xs.case[Pair[i64, i64]] {
        Nil => diff(x3, xover),
        Cons(a, x) =>
            member(xover, a).case {
                True => collect_neighbors(xover, x3, x2, x1, x),
                False =>
                    member(x3, a).case {
                        True => collect_neighbors(Cons(a, xover), x3, x2, x1, x),
                        False =>
                            member(x2, a).case {
                                True => collect_neighbors(xover, Cons(a, x3), x2, x1, x),
                                False =>
                                    member(x1, a).case {
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

def nextgen(gen: Gen): Gen {
    let living: List[Pair[i64, i64]] = alive(gen);
    let isalive: Fun[Pair[i64, i64], Bool] = new { apply(p) => member(living, p) };
    let liveneighbours: Fun[Pair[i64, i64], i64] = new {
        apply(p) => len(filter(neighbours(p), isalive))
    };
    let survivors: List[Pair[i64, i64]] = filter(
        living,
        new { apply(p) => twoorthree(liveneighbours.apply[Pair[i64, i64], i64](p)) }
    );
    let newbrnlist: List[Pair[i64, i64]] = collect(
        living,
        new {
            apply(p) =>
                filter(
                    neighbours(p),
                    new { apply(n) => not(isalive.apply[Pair[i64, i64], Bool](n)) }
                )
        }
    );
    let newborn: List[Pair[i64, i64]] = occurs3(newbrnlist);
    Gen(append(survivors, newborn))
}

def nthgen(g: Gen, i: i64): Gen {
    if i == 0 { g } else { nthgen(nextgen(g), i - 1) }
}

def gun(): Gen {
    let r9: List[Pair[i64, i64]] = Cons(
        Tup(9, 29),
        Cons(Tup(9, 30), Cons(Tup(9, 31), Cons(Tup(9, 32), Nil)))
    );
    let r8: List[Pair[i64, i64]] = Cons(
        Tup(8, 20),
        Cons(
            Tup(8, 28),
            Cons(
                Tup(8, 29),
                Cons(Tup(8, 30), Cons(Tup(8, 31), Cons(Tup(8, 40), Cons(Tup(8, 41), r9))))
            )
        )
    );
    let r7: List[Pair[i64, i64]] = Cons(
        Tup(7, 19),
        Cons(Tup(7, 21), Cons(Tup(7, 28), Cons(Tup(7, 31), Cons(Tup(7, 40), Cons(Tup(7, 41), r8)))))
    );
    let r6: List[Pair[i64, i64]] = Cons(
        Tup(6, 7),
        Cons(
            Tup(6, 8),
            Cons(
                Tup(6, 18),
                Cons(
                    Tup(6, 22),
                    Cons(
                        Tup(6, 23),
                        Cons(
                            Tup(6, 28),
                            Cons(
                                Tup(6, 29),
                                Cons(Tup(6, 30), Cons(Tup(6, 31), Cons(Tup(6, 36), r7)))
                            )
                        )
                    )
                )
            )
        )
    );
    let r5: List[Pair[i64, i64]] = Cons(
        Tup(5, 7),
        Cons(
            Tup(5, 8),
            Cons(
                Tup(5, 18),
                Cons(
                    Tup(5, 22),
                    Cons(
                        Tup(5, 23),
                        Cons(
                            Tup(5, 29),
                            Cons(
                                Tup(5, 30),
                                Cons(Tup(5, 31), Cons(Tup(5, 32), Cons(Tup(5, 36), r6)))
                            )
                        )
                    )
                )
            )
        )
    );
    let r4: List[Pair[i64, i64]] = Cons(
        Tup(4, 18),
        Cons(Tup(4, 22), Cons(Tup(4, 23), Cons(Tup(4, 32), r5)))
    );
    let r3: List[Pair[i64, i64]] = Cons(Tup(3, 19), Cons(Tup(3, 21), r4));
    let r2: List[Pair[i64, i64]] = Cons(Tup(2, 20), r3);
    Gen(r2)
}

def at_pos(coordlist: List[Pair[i64, i64]], p: Pair[i64, i64]): List[Pair[i64, i64]] {
    let move: Fun[Pair[i64, i64], Pair[i64, i64]] = new {
        apply(a) =>
            a.case[i64, i64] {
                Tup(fst1, snd1) =>
                    p.case[i64, i64] { Tup(fst2, snd2) => Tup(fst1 + fst2, snd1 + snd2) }
            }
    };
    map(coordlist, move)
}

def centerLine(): i64 {
    5
}

def bail(): List[Pair[i64, i64]] {
    Cons(Tup(0, 0), Cons(Tup(0, 1), Cons(Tup(1, 0), Cons(Tup(1, 1), Nil))))
}

def shuttle(): List[Pair[i64, i64]] {
    let r4: List[Pair[i64, i64]] = Cons(
        Tup(4, 1),
        Cons(Tup(4, 0), Cons(Tup(4, 5), Cons(Tup(4, 6), Nil)))
    );
    let r3: List[Pair[i64, i64]] = Cons(Tup(3, 2), Cons(Tup(3, 3), Cons(Tup(3, 4), r4)));
    let r2: List[Pair[i64, i64]] = Cons(Tup(2, 1), Cons(Tup(2, 5), r3));
    let r1: List[Pair[i64, i64]] = Cons(Tup(1, 2), Cons(Tup(1, 4), r2));
    Cons(Tup(0, 3), r1)
}

def non_steady(): Gen {
    Gen(
        append(
            append(at_pos(bail(), Tup(1, centerLine())), at_pos(bail(), Tup(21, centerLine()))),
            at_pos(shuttle(), Tup(6, centerLine() - 2))
        )
    )
}

def go_gun(): Fun[i64, Gen] {
    new { apply(steps) => nthgen(gun(), steps) }
}

def go_shuttle(): Fun[i64, Gen] {
    new { apply(steps) => nthgen(non_steady(), steps) }
}

def go_loop(iters: i64, steps: i64, go: Fun[i64, Gen]): i64 {
    let res: Gen = go.apply[i64, Gen](steps);
    if iters == 1 { len(alive(res)) } else { go_loop(iters - 1, steps, go) }
}

def main(iters: i64, steps: i64): i64 {
    let gun_res: i64 = go_loop(iters, steps, go_gun());
    let shuttle_res: i64 = go_loop(iters, steps, go_shuttle());
    print_i64(gun_res);
    println_i64(shuttle_res);
    0
}