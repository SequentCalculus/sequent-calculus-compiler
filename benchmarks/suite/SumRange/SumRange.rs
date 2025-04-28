use std::{ops::Add, rc::Rc};

#[derive(Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn sum(self) -> A
    where
        A: Add<Output = A> + Default + Clone,
    {
        match self {
            List::Nil => A::default(),
            List::Cons(a, as_) => a + Rc::unwrap_or_clone(as_).sum(),
        }
    }
}

impl List<u64> {
    fn range(i: u64, n: u64) -> List<u64> {
        if i < n {
            List::Cons(i, Rc::new(List::range(i + 1, n)))
        } else {
            List::Nil
        }
    }
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = List::range(0, n).sum();
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
        .parse::<u64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
