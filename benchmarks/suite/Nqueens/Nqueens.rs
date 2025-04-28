use std::rc::Rc;

#[derive(Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, as_) => 1 + (*as_).len(),
        }
    }

    fn append(self, other: List<A>) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => other,
            List::Cons(a, as_) => List::Cons(a, Rc::new(Rc::unwrap_or_clone(as_).append(other))),
        }
    }
}

fn safe(x: u64, d: u64, l: &List<u64>) -> bool {
    match l {
        List::Nil => true,
        List::Cons(q, l) => x != *q && x != (*q + d) && x != (*q - d) && safe(x, d + 1, l),
    }
}

fn check(l: &List<List<u64>>, acc: List<List<u64>>, q: u64) -> List<List<u64>> {
    match l {
        List::Nil => acc,
        List::Cons(b, bs) => {
            if safe(q, 1, &b) {
                check(
                    bs,
                    List::Cons(List::Cons(q, Rc::new(b.clone())), Rc::new(acc)),
                    q,
                )
            } else {
                check(bs, acc, q)
            }
        }
    }
}

fn enumerate(q: u64, acc: List<List<u64>>, bs: List<List<u64>>) -> List<List<u64>> {
    if q == 0 {
        acc
    } else {
        let res = check(&bs, List::Nil, q);
        enumerate(q - 1, res.append(acc), bs)
    }
}

fn gen(n: u64, nq: u64) -> List<List<u64>> {
    if n == 0 {
        List::Cons(List::Nil, Rc::new(List::Nil))
    } else {
        let bs = gen(n - 1, nq);
        enumerate(nq, List::Nil, bs)
    }
}

fn nsoln(n: u64) -> usize {
    gen(n, n).len()
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = nsoln(n);
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
