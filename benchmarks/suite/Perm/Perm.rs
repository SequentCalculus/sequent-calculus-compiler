use std::{ops::Add, rc::Rc};

#[derive(Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, as_) => 1 + as_.len(),
        }
    }

    fn head(self) -> A {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(a, _) => a,
        }
    }

    fn tail(self) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take tail of empty list"),
            List::Cons(_, as_) => Rc::unwrap_or_clone(as_),
        }
    }

    fn split_head(self) -> (A, List<A>)
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(a, as_) => (a, Rc::unwrap_or_clone(as_)),
        }
    }

    fn tail_n(self, n: u64) -> List<A>
    where
        A: Clone,
    {
        if n == 0 {
            self
        } else {
            self.tail().tail_n(n - 1)
        }
    }

    fn rev_loop(self, n: u64, y: List<A>) -> List<A>
    where
        A: Clone,
    {
        if n == 0 {
            y
        } else {
            let (hd, tl) = self.split_head();
            tl.rev_loop(n - 1, List::Cons(hd, Rc::new(y)))
        }
    }

    fn sum(self) -> A
    where
        A: Add<Output = A> + Default + Clone,
    {
        match self {
            List::Nil => A::default(),
            List::Cons(a, as_) => a + Rc::unwrap_or_clone(as_).sum(),
        }
    }

    fn map<B>(self, f: Box<impl Fn(A) -> B>) -> List<B>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => List::Cons(f(a), Rc::new(Rc::unwrap_or_clone(as_).map(f))),
        }
    }
}

impl List<u64> {
    fn one2n(n: u64) -> List<u64> {
        List::loop_one2n(n, List::Nil)
    }

    fn loop_one2n(n: u64, p: List<u64>) -> List<u64> {
        if n == 0 {
            p
        } else {
            List::loop_one2n(n - 1, List::Cons(n, Rc::new(p)))
        }
    }
}

fn f(n: u64, perms: List<List<u64>>, x: List<u64>) -> (List<List<u64>>, List<u64>) {
    let x = x.clone().rev_loop(n, x.tail_n(n));
    let perms = List::Cons(x.clone(), Rc::new(perms));
    (perms, x)
}

fn loop_p(j: u64, perms: List<List<u64>>, x: List<u64>, n: u64) -> (List<List<u64>>, List<u64>) {
    if j == 0 {
        p(n - 1, perms, x)
    } else {
        let (perms, x) = p(n - 1, perms, x);
        let (perms, x) = f(n, perms, x);
        loop_p(j - 1, perms, x, n)
    }
}

fn p(n: u64, perms: List<List<u64>>, x: List<u64>) -> (List<List<u64>>, List<u64>) {
    if 1 < n {
        loop_p(n - 1, perms, x, n)
    } else {
        (perms, x)
    }
}

fn permutations(x0: List<u64>) -> List<List<u64>> {
    let (final_perms, _) = p(
        x0.len() as u64,
        List::Cons(x0.clone(), Rc::new(List::Nil)),
        x0,
    );
    final_perms
}

fn loop_run(
    iters: u64,
    work: Box<impl Fn() -> List<List<u64>>>,
    result: Box<impl Fn(List<List<u64>>) -> bool>,
) -> bool {
    let res = result(work());
    if iters == 0 {
        res
    } else {
        loop_run(iters - 1, work, result)
    }
}

fn run_benchmark(
    iters: u64,
    work: Box<impl Fn() -> List<List<u64>>>,
    result: Box<impl Fn(List<List<u64>>) -> bool>,
) -> bool {
    loop_run(iters, work, result)
}

fn loop_work(m: u64, perms: List<List<u64>>) -> List<List<u64>> {
    if m == 0 {
        perms
    } else {
        loop_work(m - 1, permutations(perms.head()))
    }
}

fn factorial(n: u64) -> u64 {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn perm9(m: u64, n: u64) -> bool {
    run_benchmark(
        1,
        Box::new(|| loop_work(m, permutations(List::one2n(n)))),
        Box::new(|result: List<List<u64>>| {
            result.map(Box::new(|is_: List<u64>| is_.sum())).sum()
                == (((n * (n + 1)) * factorial(n)) / 2)
        }),
    )
}

fn main_loop(iters: u64, n: u64, m: u64) -> i64 {
    let res = perm9(m, n);
    if iters == 1 {
        println!("{}", if res { 1 } else { 0 });
        0
    } else {
        main_loop(iters - 1, n, m)
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
    let m = args
        .next()
        .expect("Missing Argument n")
        .parse::<u64>()
        .expect("m must be a number");
    std::process::exit(main_loop(iters, n, m) as i32)
}
