use std::rc::Rc;

#[derive(Debug, Clone)]
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

    fn split_head(self) -> (A, List<A>)
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(a, as_) => (a, Rc::unwrap_or_clone(as_)),
        }
    }

    fn take(self, n: usize) -> List<A>
    where
        A: Clone,
    {
        if n == 0 {
            return List::Nil;
        }

        match self {
            List::Nil => panic!("Cannot take from empty list"),
            List::Cons(a, as_) => List::Cons(a, Rc::new(Rc::unwrap_or_clone(as_).take(n - 1))),
        }
    }

    fn filter(self, f: &impl Fn(&A) -> bool) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => {
                if f(&a) {
                    List::Cons(a, Rc::new(Rc::unwrap_or_clone(as_).filter(f)))
                } else {
                    Rc::unwrap_or_clone(as_).filter(f)
                }
            }
        }
    }

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
}

fn expand(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> i64 {
    f + 10 * e + 100 * d + 1000 * c + 10000 * b + 100000 * a
}

fn condition(thirywelvn: &List<i64>) -> bool {
    if thirywelvn.len() != 10 {
        return false;
    }
    let (t, ls) = thirywelvn.clone().split_head();
    let (h, ls) = ls.split_head();
    let (i, ls) = ls.split_head();
    let (r, ls) = ls.split_head();
    let (y, ls) = ls.split_head();
    let (w, ls) = ls.split_head();
    let (e, ls) = ls.split_head();
    let (l, ls) = ls.split_head();
    let (v, ls) = ls.split_head();
    let (n, _) = ls.split_head();
    expand(t, h, i, r, t, y) + 5 * expand(t, w, e, l, v, e) == expand(n, i, n, e, t, y)
}

fn add_lscomp(p1: List<List<i64>>, k: i64) -> List<List<i64>> {
    match p1 {
        List::Nil => List::Nil,
        List::Cons(h1, t1) => List::Cons(
            List::Cons(k, Rc::new(h1)),
            Rc::new(add_lscomp(Rc::unwrap_or_clone(t1), k)),
        ),
    }
}

fn addj(j: i64, ls: List<i64>) -> List<List<i64>> {
    match ls {
        List::Nil => List::Cons(List::Cons(j, Rc::new(List::Nil)), Rc::new(List::Nil)),
        List::Cons(k, ks) => List::Cons(
            List::Cons(j, Rc::new(List::Cons(k, ks.clone()))),
            Rc::new(add_lscomp(addj(j, Rc::unwrap_or_clone(ks)), k)),
        ),
    }
}

fn perm_lscomp2(p2: List<List<i64>>, t1: List<List<i64>>, j: i64) -> List<List<i64>> {
    //println!("perm lscomp2 of {p2:?}");
    match p2 {
        List::Nil => perm_lscomp1(t1, j),
        List::Cons(r, t2) => List::Cons(r, Rc::new(perm_lscomp2(Rc::unwrap_or_clone(t2), t1, j))),
    }
}

fn perm_lscomp1(p1: List<List<i64>>, j: i64) -> List<List<i64>> {
    println!("perm lscomp1 of {p1:?}");
    match p1 {
        List::Nil => List::Nil,
        List::Cons(pjs, t1) => perm_lscomp2(addj(j, pjs), Rc::unwrap_or_clone(t1), j),
    }
}

fn permutations(ls: List<i64>) -> List<List<i64>> {
    println!("calculating permutations of {ls:?}");
    match ls {
        List::Nil => List::Cons(List::Nil, Rc::new(List::Nil)),
        List::Cons(j, ls) => perm_lscomp1(permutations(Rc::unwrap_or_clone(ls)), j),
    }
}

fn test_cryptarithm_nofib(n: i64) -> List<List<List<i64>>> {
    println!("testing {n}");
    List::from_iterator((1..=n).map(&|i| {
        let p0: List<i64> = List::from_iterator(0..=(9 + i)).take(10);
        permutations(p0).filter(&|l| condition(l))
    }))
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_cryptarithm_nofib(n);
    println!("got res {res:?}");
    if iters == 1 {
        println!("{}", res.head().head().head());
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
        .expect("m must be a number");
    println!("iters {iters}");
    println!("n {n}");
    std::process::exit(main_loop(iters, n) as i32)
}
