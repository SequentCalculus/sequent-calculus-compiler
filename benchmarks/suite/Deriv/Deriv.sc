data List[A] { Nil, Cons(x: A, xs: List[A]) }

data Unit { Unit }

data Bool { True, False }

codata Fun[A, B] { apply(x: A): B }

data Expr {
    Add(sums: List[Expr]),
    Sub(subs: List[Expr]),
    Mul(muls: List[Expr]),
    Div(divs: List[Expr]),
    Num(i: i64),
    X
}

def map_list(f: Fun[Expr, Expr], l: List[Expr]): List[Expr] {
    l.case[Expr] {
        Nil => Nil,
        Cons(x, xs) => Cons(f.apply[Expr, Expr](x), map_list(f, xs))
    }
}

def equal_list(l1: List[Expr], l2: List[Expr]): Bool {
    l1.case[Expr] {
        Nil =>
            l2.case[Expr] {
                Nil => True,
                Cons(e, es) => False
            },
        Cons(e1, es1) =>
            l2.case[Expr] {
                Nil => False,
                Cons(e2, es2) =>
                    equal(e1, e2).case {
                        True => equal_list(es1, es2),
                        False => False
                    }
            }
    }
}

def equal(exp1: Expr, exp2: Expr): Bool {
    exp1.case {
        Add(sums1) =>
            exp2.case {
                Add(sums2) => equal_list(sums1, sums2),
                Sub(subs) => False,
                Mul(muls) => False,
                Div(divs) => False,
                Num(i) => False,
                X => False
            },
        Sub(subs1) =>
            exp2.case {
                Add(sums) => False,
                Sub(subs2) => equal_list(subs1, subs2),
                Mul(muls) => False,
                Div(divs) => False,
                Num(i) => False,
                X => False
            },
        Mul(muls1) =>
            exp2.case {
                Add(sums) => False,
                Sub(subs) => False,
                Mul(muls2) => equal_list(muls1, muls2),
                Div(divs) => False,
                Num(i) => False,
                X => False
            },
        Div(divs1) =>
            exp2.case {
                Add(sums) => False,
                Sub(subs) => False,
                Mul(muls) => False,
                Div(divs2) => equal_list(divs1, divs2),
                Num(i) => False,
                X => False
            },
        Num(i1) =>
            exp2.case {
                Add(sums) => False,
                Sub(subs) => False,
                Mul(muls) => False,
                Div(divs) => False,
                Num(i2) => if i1 == i2 { True } else { False },
                X => False
            },
        X =>
            exp2.case {
                Add(sums) => False,
                Sub(subs) => False,
                Mul(muls) => False,
                Div(divs) => False,
                Num(i) => False,
                X => True
            }
    }
}

def deriv(e: Expr): Expr {
    e.case {
        Add(sums) => Add(map_list(new { apply(x) => deriv(x) }, sums)),
        Sub(subs) => Sub(map_list(new { apply(x) => deriv(x) }, subs)),
        Mul(muls) =>
            Mul(
                Cons(
                    e,
                    Cons(
                        Add(map_list(new { apply(x) => Div(Cons(deriv(x), Cons(x, Nil))) }, muls)),
                        Nil
                    )
                )
            ),
        Div(divs) =>
            divs.case[Expr] {
                Nil => X,
                Cons(x, xs) =>
                    xs.case[Expr] {
                        Nil => X,
                        Cons(y, ys) =>
                            ys.case[Expr] {
                                Nil =>
                                    Sub(
                                        Cons(
                                            Div(Cons(deriv(x), Cons(y, Nil))),
                                            Cons(
                                                Div(
                                                    Cons(
                                                        x,
                                                        Cons(
                                                            Mul(
                                                                Cons(
                                                                    y,
                                                                    Cons(y, Cons(deriv(y), Nil))
                                                                )
                                                            ),
                                                            Nil
                                                        )
                                                    )
                                                ),
                                                Nil
                                            )
                                        )
                                    ),
                                Cons(z, zs) => X
                            }
                    }
            },
        Num(i) => Num(0),
        X => Num(1)
    }
}

def mk_exp(a: Expr, b: Expr): Expr {
    Add(
        Cons(
            Mul(Cons(Num(3), Cons(X, Cons(X, Nil)))),
            Cons(
                Mul(Cons(a, Cons(X, Cons(X, Nil)))),
                Cons(Mul(Cons(b, Cons(X, Nil))), Cons(Num(5), Nil))
            )
        )
    )
}

def mk_ans(a: Expr, b: Expr): Expr {
    Add(
        Cons(
            Mul(
                Cons(
                    Mul(Cons(Num(3), Cons(X, Cons(X, Nil)))),
                    Cons(
                        Add(
                            Cons(
                                Div(Cons(Num(0), Cons(Num(3), Nil))),
                                Cons(
                                    Div(Cons(Num(1), Cons(X, Nil))),
                                    Cons(Div(Cons(Num(1), Cons(X, Nil))), Nil)
                                )
                            )
                        ),
                        Nil
                    )
                )
            ),
            Cons(
                Mul(
                    Cons(
                        Mul(Cons(a, Cons(X, Cons(X, Nil)))),
                        Cons(
                            Add(
                                Cons(
                                    Div(Cons(Num(0), Cons(a, Nil))),
                                    Cons(
                                        Div(Cons(Num(1), Cons(X, Nil))),
                                        Cons(Div(Cons(Num(1), Cons(X, Nil))), Nil)
                                    )
                                )
                            ),
                            Nil
                        )
                    )
                ),
                Cons(
                    Mul(
                        Cons(
                            Mul(Cons(b, Cons(X, Nil))),
                            Cons(
                                Add(
                                    Cons(
                                        Div(Cons(Num(0), Cons(b, Nil))),
                                        Cons(Div(Cons(Num(1), Cons(X, Nil))), Nil)
                                    )
                                ),
                                Nil
                            )
                        )
                    ),
                    Cons(Num(0), Nil)
                )
            )
        )
    )
}

def main_loop(iters: i64, n: i64, m: i64): i64 {
    let res: Expr = deriv(mk_exp(Num(n), Num(m)));
    let expected: Expr = mk_ans(Num(n), Num(m));
    if iters == 1 {
        equal(expected, res).case {
            True =>
                println_i64(1);
                0,
            False =>
                println_i64(0);
                0
        }
    } else {
        main_loop(iters - 1, n, m)
    }
}

def main(iters: i64, n: i64, m: i64): i64 {
    main_loop(iters, n, m)
}