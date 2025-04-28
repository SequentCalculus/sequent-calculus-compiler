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
            List::Cons(_, as_) => 1 + as_.len(),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
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

    fn map<B>(self, f: &impl Fn(A) -> B) -> List<B>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => List::Cons(f(a), Rc::new(Rc::unwrap_or_clone(as_).map(f))),
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

fn addj(j: i64, ls: List<i64>) -> List<List<i64>> {
    if ls.is_empty() {
        return List::Cons(List::Cons(j, Rc::new(List::Nil)), Rc::new(List::Nil));
    }
    let (k, ls) = ls.split_head();
    let lscomp = |p1: List<List<i64>>| p1.map(&|h1| List::Cons(k, Rc::new(h1)));
    let res: List<List<i64>> = List::Cons(
        List::Cons(k, Rc::new(List::Cons(j, Rc::new(ls.clone())))),
        Rc::new(lscomp(addj(j, ls))),
    );
    res
}

fn perm_lscomp1(p1: List<List<i64>>, j: i64) -> List<List<i64>> {
    if p1.is_empty() {
        return List::Nil;
    }
    let (pjs, p1) = p1.split_head();
    perm_lscomp2(addj(j, pjs), p1, j)
}

fn perm_lscomp2(p2: List<List<i64>>, t1: List<List<i64>>, j: i64) -> List<List<i64>> {
    if p2.is_empty() {
        return perm_lscomp1(t1, j);
    }

    let (r, p2) = p2.split_head();
    List::Cons(r, Rc::new(perm_lscomp2(p2, t1, j)))
}

fn permutations(ls: List<i64>) -> List<List<i64>> {
    if ls.is_empty() {
        return List::Cons(List::Nil, Rc::new(List::Nil));
    }
    let (j, ls) = ls.split_head();
    perm_lscomp1(permutations(ls), j)
}

fn test_cryptarithm_nofib(n: i64) -> List<List<List<i64>>> {
    List::from_iterator((1..=n).map(&|i| {
        let p0: List<i64> = List::from_iterator((0..=(9 + i)).take(10));
        permutations(p0).filter(&|l| condition(l))
    }))
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_cryptarithm_nofib(n);
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
    std::process::exit(main_loop(iters, n) as i32)
}
