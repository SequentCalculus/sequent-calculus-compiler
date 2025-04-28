use std::rc::Rc;

#[derive(Debug, Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl List<u64> {
    fn tabulate_loop(n: u64, len: u64, f: Box<impl Fn(u64) -> u64>, acc: List<u64>) -> List<u64> {
        if n < len {
            let next = f(n);
            List::tabulate_loop(n + 1, len, f, List::Cons(next, Rc::new(acc)))
        } else {
            acc.rev()
        }
    }

    fn tabulate(n: u64, f: Box<impl Fn(u64) -> u64>) -> List<u64> {
        List::tabulate_loop(0, n, f, List::Nil)
    }
}

impl<A> List<A> {
    fn merge(self, other: List<A>) -> List<A>
    where
        A: PartialOrd + Clone,
    {
        match (self, other) {
            (List::Nil, other) => other,
            (s, List::Nil) => s,
            (List::Cons(x1, xs1), List::Cons(x2, xs2)) if x1 <= x2 => List::Cons(
                x1,
                Rc::new(Rc::unwrap_or_clone(xs1).merge(List::Cons(x2, xs2))),
            ),
            (List::Cons(x1, xs1), List::Cons(x2, xs2)) => List::Cons(
                x2,
                Rc::new(List::Cons(x1, xs1).merge(Rc::unwrap_or_clone(xs2))),
            ),
        }
    }

    fn rev_loop(self, acc: List<A>) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => acc,
            List::Cons(p, ps) => Rc::unwrap_or_clone(ps).rev_loop(List::Cons(p, Rc::new(acc))),
        }
    }

    fn rev(self) -> List<A>
    where
        A: Clone,
    {
        self.rev_loop(List::Nil)
    }
}

fn main_loop(iters: u64, n: u64, l1: List<u64>, l2: List<u64>) -> i64 {
    let res = l1.clone().merge(l2.clone());
    if iters == 1 {
        println!("{:?}", res);
        0
    } else {
        main_loop(iters - 1, n, l1, l2)
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
        .parse::<u64>()
        .expect("n must be a number");
    let l1 = List::tabulate(n, Box::new(|x| 2 * x));
    let l2 = List::tabulate(n, Box::new(|x| (2 * x) + 1));
    std::process::exit(main_loop(iters, n, l1, l2) as i32)
}
