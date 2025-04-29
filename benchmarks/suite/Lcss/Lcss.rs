use std::rc::Rc;

#[derive(Debug, Clone)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> List<T> {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tl) => 1 + tl.len(),
        }
    }

    fn is_nil(&self) -> bool {
        matches!(self, List::Nil)
    }

    fn is_singleton(&self) -> Option<&T> {
        match self {
            List::Nil => None,
            List::Cons(t, tl) => {
                if matches!(**tl, List::Nil) {
                    Some(t)
                } else {
                    None
                }
            }
        }
    }

    fn contains(&self, t: &T) -> bool
    where
        T: PartialEq,
    {
        match self {
            List::Nil => false,
            List::Cons(t1, ts) => {
                if *t == *t1 {
                    true
                } else {
                    ts.contains(t)
                }
            }
        }
    }

    fn head(self) -> T {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(a, _) => a,
        }
    }

    fn split_at(self, n: usize) -> (List<T>, List<T>)
    where
        T: Clone,
    {
        if n == 0 {
            return (List::Nil, self);
        }
        match self {
            List::Nil => panic!("Cannot split empty list"),
            List::Cons(hd, tl) => {
                let (fst, snd) = Rc::unwrap_or_clone(tl).split_at(n - 1);
                (List::Cons(hd, Rc::new(fst)), snd)
            }
        }
    }

    fn rev_loop(self, acc: List<T>) -> List<T>
    where
        T: Clone,
    {
        match self {
            List::Nil => acc,
            List::Cons(hd, tl) => Rc::unwrap_or_clone(tl).rev_loop(List::Cons(hd, Rc::new(acc))),
        }
    }

    fn rev(self) -> List<T>
    where
        T: Clone,
    {
        self.rev_loop(List::Nil)
    }

    fn zip<U>(self, other: List<U>) -> List<(T, U)>
    where
        T: Clone,
        U: Clone,
    {
        match (self, other) {
            (List::Nil, _) => List::Nil,
            (_, List::Nil) => List::Nil,
            (List::Cons(t1, ts), List::Cons(u1, us)) => List::Cons(
                (t1, u1),
                Rc::new(Rc::unwrap_or_clone(ts).zip(Rc::unwrap_or_clone(us))),
            ),
        }
    }

    fn take(self, n: usize) -> List<T>
    where
        T: Clone,
    {
        if n == 0 {
            return List::Nil;
        }
        match self {
            List::Nil => panic!("cannot take from empty list"),
            List::Cons(hd, tl) => List::Cons(hd, Rc::new(Rc::unwrap_or_clone(tl).take(n - 1))),
        }
    }

    fn drop(self, n: usize) -> List<T>
    where
        T: Clone,
    {
        if n == 0 {
            return List::Nil;
        }
        match self {
            List::Nil => panic!("Cannot drop from empty list"),
            List::Cons(_, tl) => Rc::unwrap_or_clone(tl).drop(n - 1),
        }
    }

    fn map<U>(self, f: &impl Fn(T) -> U) -> List<U>
    where
        T: Clone,
        U: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(hd, tl) => List::Cons(f(hd), Rc::new(Rc::unwrap_or_clone(tl).map(f))),
        }
    }
}

fn findk(k: usize, km: usize, m: i64, ls: List<(i64, i64)>) -> usize {
    match ls {
        List::Nil => km,
        List::Cons((x, y), xys) => {
            if m <= x + y {
                findk(k + 1, k, x + y, Rc::unwrap_or_clone(xys))
            } else {
                findk(k + 1, km, m, Rc::unwrap_or_clone(xys))
            }
        }
    }
}

fn enum_from_then_to(from: i64, then: i64, t: i64) -> List<i64> {
    if from <= t {
        List::Cons(from, Rc::new(enum_from_then_to(then, (2 * then) - from, t)))
    } else {
        List::Nil
    }
}

fn algb2(x: i64, k0j1: i64, k1j1: i64, yss: List<(i64, i64)>) -> List<(i64, i64)> {
    match yss {
        List::Nil => List::Nil,
        List::Cons((y, k0j), ys) => {
            let kjcurr = if x == y { k0j1 + 1 } else { k1j1.max(k0j) };
            List::Cons(
                (y, kjcurr),
                Rc::new(algb2(x, k0j, kjcurr, Rc::unwrap_or_clone(ys))),
            )
        }
    }
}

fn algb1(xss: List<i64>, yss: List<(i64, i64)>) -> List<i64> {
    match xss {
        List::Nil => yss.map(&|(_, x)| x),
        List::Cons(x, xs) => algb1(Rc::unwrap_or_clone(xs), algb2(x, 0, 0, yss)),
    }
}

fn algb(xs: List<i64>, ys: List<i64>) -> List<i64> {
    List::Cons(0, Rc::new(algb1(xs, ys.map(&|x| (x, 0)))))
}

fn algc(m: usize, n: usize, xs: List<i64>, ys: List<i64>) -> Box<dyn Fn(List<i64>) -> List<i64>> {
    if ys.is_nil() {
        return Box::new(|x| x);
    }

    if let Some(x) = xs.is_singleton() {
        if ys.contains(x) {
            let x_ = *x;
            return Box::new(move |t| List::Cons(x_, Rc::new(t)));
        } else {
            return Box::new(|x| x);
        }
    }

    let m2 = m / 2;
    let (xs1, xs2) = xs.split_at(m2);
    let l1 = algb(xs1.clone(), ys.clone());
    let l2 = algb(xs2.clone().rev(), ys.clone().rev()).rev();
    let k = findk(0, 0, -1, l1.zip(l2));
    Box::new(move |x| {
        let f1 = algc(m - m2, n - k, xs2.clone(), ys.clone().drop(k));
        let f2 = algc(m2, k, xs1.clone(), ys.clone().take(k));
        f2(f1(x))
    })
}

fn lcss(xs: List<i64>, ys: List<i64>) -> List<i64> {
    algc(xs.len(), ys.len(), xs, ys)(List::Nil)
}

fn lcss_main(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> List<i64> {
    lcss(enum_from_then_to(a, b, c), enum_from_then_to(d, e, f))
}

fn test_lcss_nofib() -> List<i64> {
    lcss_main(1, 2, 200, 100, 101, 300)
}

fn main_loop(iters: u64) -> i64 {
    let res = test_lcss_nofib();
    if iters == 1 {
        println!("{}", res.head());
        0
    } else {
        main_loop(iters - 1)
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
    std::process::exit(main_loop(iters) as i32)
}
