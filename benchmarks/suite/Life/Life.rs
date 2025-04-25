use std::rc::Rc;

const CENTER_LINE: i64 = 5;

#[derive(Clone)]
enum List<T> {
    Nil,
    Cons(Rc<T>, Rc<List<T>>),
}

impl<T> List<T> {
    fn cons(t: T, tl: List<T>) -> List<T> {
        List::Cons(Rc::new(t), Rc::new(tl))
    }

    fn contains(&self, t: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            List::Nil => false,
            List::Cons(hd, tl) => {
                if **hd == *t {
                    true
                } else {
                    tl.contains(t)
                }
            }
        }
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tl) => 1 + tl.len(),
        }
    }

    fn append(self, other: List<T>) -> List<T>
    where
        T: Clone,
    {
        match self {
            List::Nil => other,
            List::Cons(hd, tl) => List::Cons(hd, Rc::new(Rc::unwrap_or_clone(tl).append(other))),
        }
    }

    fn map<U>(self, f: Box<dyn Fn(T) -> U>) -> List<U>
    where
        T: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(hd, tl) => List::Cons(
                Rc::new(f(Rc::unwrap_or_clone(hd))),
                Rc::new(Rc::unwrap_or_clone(tl).map(f)),
            ),
        }
    }

    fn filter(self, f: &dyn for<'a> Fn(&'a T) -> bool) -> List<T>
    where
        T: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(hd, tl) => {
                if f(&(*hd)) {
                    List::Cons(hd, Rc::new(Rc::unwrap_or_clone(tl).filter(f)))
                } else {
                    Rc::unwrap_or_clone(tl).filter(f)
                }
            }
        }
    }

    fn diff(self, y: List<T>) -> List<T>
    where
        T: Clone + PartialEq,
    {
        self.filter(&|p| !y.contains(&p))
    }

    fn fold<U>(self, start: U, f: Box<dyn Fn(T, U) -> U>) -> U
    where
        T: Clone,
    {
        match self {
            List::Nil => start,
            List::Cons(hd, tl) => {
                Rc::unwrap_or_clone(tl).fold(f(Rc::unwrap_or_clone(hd), start), f)
            }
        }
    }

    fn accumulate(self, xs: List<T>, f: Box<dyn Fn(T, List<T>) -> List<T>>) -> List<T>
    where
        T: Clone,
    {
        self.fold(xs, f)
    }

    fn revonto(self, y: List<T>) -> List<T>
    where
        T: Clone,
    {
        self.accumulate(y, Box::new(|h, t| List::Cons(Rc::new(h), Rc::new(t))))
    }

    fn collect_accum(self, sofar: List<T>, f: Box<dyn Fn(T) -> List<T>>) -> List<T>
    where
        T: Clone,
    {
        match self {
            List::Nil => sofar,
            List::Cons(p, xs) => sofar
                .revonto(f(Rc::unwrap_or_clone(p)))
                .collect_accum(Rc::unwrap_or_clone(xs), f),
        }
    }

    fn collect(self, f: Box<dyn Fn(T) -> List<T>>) -> List<T>
    where
        T: Clone,
    {
        List::Nil.collect_accum(self, f)
    }
}

struct Gen {
    coordslist: List<(i64, i64)>,
}

impl Gen {
    fn new(coordslist: List<(i64, i64)>) -> Gen {
        Gen { coordslist }
    }

    fn non_steady() -> Gen {
        Gen::new(
            at_pos(bail(), (1, CENTER_LINE))
                .append(at_pos(bail(), (21, CENTER_LINE)))
                .append(at_pos(shuttle(), (6, CENTER_LINE - 2))),
        )
    }

    fn alive(self) -> List<(i64, i64)> {
        self.coordslist
    }

    fn next(self) -> Gen {
        let living = self.alive();
        let living_ = living.clone();
        let living__ = living.clone();
        let is_alive: Box<dyn for<'a> Fn(&'a (i64, i64)) -> bool> =
            Box::new(move |&p| living_.contains(&p));
        let live_neighbors = move |p| neighbors(p).filter(&(*is_alive)).len();
        let survivors = living.clone().filter(&|&p| twoorthree(live_neighbors(p)));
        let not_is_alive: Box<dyn for<'a> Fn(&'a (i64, i64)) -> bool> =
            Box::new(move |&p| !living__.contains(&p));
        let newbrnlist = living.collect(Box::new(move |p| neighbors(p).filter(&not_is_alive)));
        let newborn = occurs3(newbrnlist);
        Gen::new(survivors.append(newborn))
    }

    fn nth(self, i: u64) -> Gen {
        if i == 0 {
            self
        } else {
            self.next().nth(i - 1)
        }
    }
}

fn collect_neighbors(
    xover: List<(i64, i64)>,
    x3: List<(i64, i64)>,
    x2: List<(i64, i64)>,
    x1: List<(i64, i64)>,
    xs: List<(i64, i64)>,
) -> List<(i64, i64)> {
    match xs {
        List::Nil => x3.diff(xover),
        List::Cons(a, x) => {
            if xover.contains(&a) {
                collect_neighbors(xover, x3, x2, x1, Rc::unwrap_or_clone(x))
            } else if x3.contains(&a) {
                collect_neighbors(
                    List::Cons(a, Rc::new(xover)),
                    x3,
                    x2,
                    x1,
                    Rc::unwrap_or_clone(x),
                )
            } else if x2.contains(&a) {
                collect_neighbors(
                    xover,
                    List::Cons(a, Rc::new(x3)),
                    x2,
                    x1,
                    Rc::unwrap_or_clone(x),
                )
            } else if x1.contains(&a) {
                collect_neighbors(
                    xover,
                    x3,
                    List::Cons(a, Rc::new(x2)),
                    x1,
                    Rc::unwrap_or_clone(x),
                )
            } else {
                collect_neighbors(
                    xover,
                    x3,
                    x2,
                    List::Cons(a, Rc::new(x1)),
                    Rc::unwrap_or_clone(x),
                )
            }
        }
    }
}

fn occurs3(l: List<(i64, i64)>) -> List<(i64, i64)> {
    collect_neighbors(List::Nil, List::Nil, List::Nil, List::Nil, l)
}

fn twoorthree(n: usize) -> bool {
    n == 2 || n == 3
}

fn neighbors((fst, snd): (i64, i64)) -> List<(i64, i64)> {
    List::cons(
        (fst - 1, snd - 1),
        List::cons(
            (fst - 1, snd),
            List::cons(
                (fst - 1, snd + 1),
                List::cons(
                    (fst, snd - 1),
                    List::cons(
                        (fst, snd + 1),
                        List::cons(
                            (fst + 1, snd - 1),
                            List::cons((fst + 1, snd), List::cons((fst + 1, snd + 1), List::Nil)),
                        ),
                    ),
                ),
            ),
        ),
    )
}

fn at_pos(coordslist: List<(i64, i64)>, (fst2, snd2): (i64, i64)) -> List<(i64, i64)> {
    let move_ = Box::new(move |(fst1, snd1)| (fst1 + snd1, fst2 + snd2));
    coordslist.map(move_)
}

fn bail() -> List<(i64, i64)> {
    List::cons(
        (0, 0),
        List::cons((0, 1), List::cons((1, 0), List::cons((1, 1), List::Nil))),
    )
}

fn shuttle() -> List<(i64, i64)> {
    let r4 = List::cons(
        (4, 1),
        List::cons((4, 0), List::cons((4, 5), List::cons((4, 6), List::Nil))),
    );
    let r3 = List::cons((3, 2), List::cons((3, 3), List::cons((3, 4), r4)));
    let r2 = List::cons((2, 1), List::cons((2, 5), r3));
    let r1 = List::cons((1, 2), List::cons((1, 4), r2));
    List::cons((0, 3), r1)
}

fn gun() -> Gen {
    let r9 = List::cons(
        (9, 29),
        List::cons((9, 30), List::cons((9, 31), List::cons((9, 32), List::Nil))),
    );
    let r8 = List::cons(
        (8, 20),
        List::cons(
            (8, 28),
            List::cons(
                (8, 29),
                List::cons(
                    (8, 30),
                    List::cons((8, 31), List::cons((8, 40), List::cons((8, 41), r9))),
                ),
            ),
        ),
    );
    let r7 = List::cons(
        (7, 19),
        List::cons(
            (7, 21),
            List::cons(
                (7, 28),
                List::cons((7, 31), List::cons((7, 40), List::cons((7, 41), r8))),
            ),
        ),
    );
    let r6 = List::cons(
        (6, 7),
        List::cons(
            (6, 8),
            List::cons(
                (6, 18),
                List::cons(
                    (6, 22),
                    List::cons(
                        (6, 23),
                        List::cons(
                            (6, 28),
                            List::cons(
                                (6, 29),
                                List::cons((6, 30), List::cons((6, 31), List::cons((6, 36), r7))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    let r5 = List::cons(
        (5, 7),
        List::cons(
            (5, 8),
            List::cons(
                (5, 18),
                List::cons(
                    (5, 22),
                    List::cons(
                        (5, 23),
                        List::cons(
                            (5, 29),
                            List::cons(
                                (5, 30),
                                List::cons((5, 31), List::cons((5, 32), List::cons((5, 36), r6))),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );
    let r4 = List::cons(
        (4, 18),
        List::cons((4, 22), List::cons((4, 23), List::cons((4, 32), r5))),
    );
    let r3 = List::cons((3, 19), List::cons((3, 21), r4));
    let r2 = List::cons((2, 20), r3);
    Gen::new(r2)
}

fn go_shuttle() -> Box<dyn Fn(u64)> {
    Box::new(|steps| {
        Gen::non_steady().nth(steps);
    })
}

fn go_gun() -> Box<dyn Fn(u64)> {
    Box::new(|steps| {
        gun().nth(steps);
    })
}

fn go_loop(iters: u64, steps: u64, go: Box<dyn Fn(u64)>) -> i64 {
    if iters == 0 {
        0
    } else {
        go(steps);
        go_loop(iters - 1, steps, go)
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let iters = args
        .next()
        .expect("Missing Argument iterations")
        .parse::<u64>()
        .expect("Iterations must be a number");
    let steps = args
        .next()
        .expect("Missing Argument steps")
        .parse::<u64>()
        .expect("steps must be a number");
    let gun_res = go_loop(iters, steps, go_gun());
    println!("{}", gun_res);
    go_loop(iters, steps, go_shuttle());
}
