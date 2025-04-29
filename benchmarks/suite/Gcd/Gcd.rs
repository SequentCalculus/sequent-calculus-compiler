use std::rc::Rc;

#[derive(Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn from_iterator<T>(t: T) -> List<A>
    where
        T: Iterator<Item = A>,
    {
        let mut ls = List::Nil;
        for it in t {
            ls = List::Cons(it, Rc::new(ls));
        }
        ls
    }

    fn zip<B>(self, other: List<B>) -> List<(A, B)>
    where
        A: Clone,
        B: Clone,
    {
        match (self, other) {
            (List::Nil, _) => List::Nil,
            (_, List::Nil) => List::Nil,
            (List::Cons(a, as_), List::Cons(b, bs_)) => List::Cons(
                (a, b),
                Rc::new(Rc::unwrap_or_clone(as_).zip(Rc::unwrap_or_clone(bs_))),
            ),
        }
    }

    fn map<B>(self, f: &impl Fn(A) -> B) -> List<B>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => List::Cons(f(a), Rc::new(Rc::unwrap_or_clone(as_).map(f))),
        }
    }

    fn fold<B>(self, start: B, f: &impl Fn(A, B) -> B) -> B
    where
        A: Clone,
    {
        match self {
            List::Nil => start,
            List::Cons(a, as_) => {
                let new_start = f(a, start);
                Rc::unwrap_or_clone(as_).fold(new_start, f)
            }
        }
    }

    fn max(self) -> A
    where
        A: PartialOrd + Default + Clone,
    {
        self.fold(A::default(), &|a, mx| if a > mx { a } else { mx })
    }
}

fn quot_rem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

fn g((u1, u2, u3): (i64, i64, i64), (v1, v2, v3): (i64, i64, i64)) -> (i64, i64, i64) {
    if v3 == 0 {
        (u3, u1, u2)
    } else {
        let (q, r) = quot_rem(u3, v3);
        g((v1, v2, v3), (u1 - (q * v1), u2 - (q * v2), r))
    }
}

fn gcd_e(x: i64, y: i64) -> (i64, i64, i64) {
    if x == 0 {
        (y, 0, 1)
    } else {
        g((1, 0, x), (0, 1, y))
    }
}

fn lscomp2(p2: List<i64>, t1: List<i64>, ms: List<i64>, h1: i64) -> List<(i64, i64)> {
    match p2 {
        List::Nil => lscomp1(t1, ms),
        List::Cons(h2, t2) => List::Cons(
            (h1, h2),
            Rc::new(lscomp2(Rc::unwrap_or_clone(t2), t1, ms, h1)),
        ),
    }
}

fn lscomp1(p1: List<i64>, ms: List<i64>) -> List<(i64, i64)> {
    match p1 {
        List::Nil => List::Nil,
        List::Cons(h1, t1) => lscomp2(ms.clone(), Rc::unwrap_or_clone(t1), ms, h1),
    }
}

fn test_gcd_nofib(d: i64) -> i64 {
    let ns: List<i64> = List::from_iterator(5000..=(5000 + d));
    let ms: List<i64> = List::from_iterator(10000..=(10000 + d));
    let tripls: List<(i64, i64, (i64, i64, i64))> =
        lscomp1(ns, ms).map(&|(x, y)| (x, y, gcd_e(x, y)));
    let rs: List<i64> = tripls.map(&|(_, _, (gg, u, v))| (gg + u).abs() + v);
    rs.max()
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_gcd_nofib(n);
    if iters == 1 {
        println!("{}", res);
        0
    } else {
        main_loop(iters - 1, n)
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
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<i64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
