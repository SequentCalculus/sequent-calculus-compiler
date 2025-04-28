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

    fn is_empty(&self) -> bool {
        matches!(self, List::Nil)
    }

    fn contains(&self, a: &A) -> bool
    where
        A: PartialEq,
    {
        match self {
            List::Nil => false,
            List::Cons(a_, as_) => {
                if a_ == a {
                    true
                } else {
                    as_.contains(a)
                }
            }
        }
    }

    fn all(&self, f: &impl Fn(&A) -> bool) -> bool {
        match self {
            List::Nil => true,
            List::Cons(a, as_) => {
                if f(a) {
                    as_.all(f)
                } else {
                    false
                }
            }
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

    fn nth(self, n: usize) -> A
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take nth element of empty list"),
            List::Cons(a, as_) => {
                if n == 0 {
                    a
                } else {
                    Rc::unwrap_or_clone(as_).nth(n - 1)
                }
            }
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

    fn append(self, other: List<A>) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => other,
            List::Cons(a, as_) => List::Cons(a, Rc::new(Rc::unwrap_or_clone(as_).append(other))),
        }
    }

    fn union(self, other: List<A>) -> List<A>
    where
        A: Clone + PartialEq,
    {
        let other_filtered = other.filter(&|t| !self.contains(&t));
        self.append(other_filtered)
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

    fn zip_with<B, C>(self, other: List<B>, f: &impl Fn(A, B) -> C) -> List<C>
    where
        A: Clone,
        B: Clone,
    {
        self.zip(other).map(&|(x, y)| f(x, y))
    }
}

impl<T, A> From<T> for List<A>
where
    T: Iterator<Item = A>,
{
    fn from(it: T) -> List<A> {
        let mut ls = List::Nil;
        for a in it {
            ls = List::Cons(a, Rc::new(ls));
        }
        ls
    }
}

#[derive(Clone)]
struct Assign {
    varr: i64,
    value: i64,
}

struct CSP {
    vars: i64,
    vals: i64,
    rel: Box<dyn Fn(&Assign, &Assign) -> bool>,
}

#[derive(Clone)]
struct Node<T> {
    lab: T,
    children: List<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn leaves(self) -> List<T>
    where
        T: Clone,
    {
        if self.children.is_empty() {
            List::Cons(self.lab, Rc::new(List::Nil))
        } else {
            self.children
                .fold(List::Nil, &|t: Rc<Node<T>>, ls: List<T>| {
                    ls.append(Rc::unwrap_or_clone(t).leaves())
                })
        }
    }
    fn map<U>(self, f: &impl Fn(T) -> U) -> Node<U>
    where
        T: Clone,
    {
        Node {
            lab: f(self.lab),
            children: self
                .children
                .map(&|x| Rc::new(Rc::unwrap_or_clone(x).map(f))),
        }
    }

    fn fold<U>(self, f: &impl Fn(T, List<Node<U>>) -> Node<U>) -> Node<U>
    where
        T: Clone,
    {
        f(
            self.lab,
            self.children.map(&|x| Rc::unwrap_or_clone(x).fold(f)),
        )
    }

    fn filter(self, p: &impl Fn(&T) -> bool) -> Node<T>
    where
        T: Clone,
    {
        let f1 = |a, cs: List<Node<T>>| Node {
            lab: a,
            children: cs.filter(&|x| p(&x.lab)).map(&Rc::new),
        };
        self.fold(&f1)
    }

    fn prune(self, p: &impl Fn(&T) -> bool) -> Node<T>
    where
        T: Clone,
    {
        self.filter(&|x| !(p(x)))
    }
}

#[derive(Clone)]
enum ConflictSet {
    Known(List<i64>),
    Unknown,
}

fn max_level(s: &List<Assign>) -> i64 {
    match s {
        List::Nil => 0,
        List::Cons(a, _) => a.varr,
    }
}

fn complete(csp: &CSP, s: &List<Assign>) -> bool {
    max_level(s) == csp.vars
}

fn check_complete(csp: &CSP, s: &List<Assign>) -> ConflictSet {
    if complete(csp, s) {
        ConflictSet::Known(List::Nil)
    } else {
        ConflictSet::Unknown
    }
}

fn earliest_inconsistency(csp: &CSP, aas: List<Assign>) -> Option<(i64, i64)> {
    match aas {
        List::Nil => None,
        List::Cons(a, aas_) => match Rc::unwrap_or_clone(aas_).filter(&|x| !((csp.rel)(&a, x))) {
            List::Nil => None,
            List::Cons(b, _) => Some((a.varr, b.varr)),
        },
    }
}

fn lookup_cache(
    csp: &CSP,
    t: Node<(List<Assign>, List<List<ConflictSet>>)>,
) -> Node<((List<Assign>, ConflictSet), List<List<ConflictSet>>)> {
    let f5 = |csp, (as_, tbl): (List<Assign>, List<List<ConflictSet>>)| match as_ {
        List::Nil => ((List::Nil, ConflictSet::Unknown), tbl),
        List::Cons(ref a, _) => {
            let table_entry = tbl.clone().head().nth((a.value - 1) as usize);
            let cs = match table_entry {
                ConflictSet::Unknown => check_complete(csp, &as_),
                ConflictSet::Known(_) => table_entry.clone(),
            };
            ((as_, cs), tbl)
        }
    };
    t.map(&|x| f5(csp, x))
}

fn cache_checks(
    csp: &CSP,
    tbl: List<List<ConflictSet>>,
    n: Node<List<Assign>>,
) -> Node<(List<Assign>, List<List<ConflictSet>>)> {
    let tbl_tl = tbl.clone().tail();
    let s = n.lab.clone();
    Node {
        lab: (n.lab, tbl),
        children: n.children.map(&|x| {
            Rc::new(cache_checks(
                csp,
                fill_table(&s, csp, tbl_tl.clone()),
                Rc::unwrap_or_clone(x),
            ))
        }),
    }
}

fn fill_table(
    s: &List<Assign>,
    csp: &CSP,
    tbl: List<List<ConflictSet>>,
) -> List<List<ConflictSet>> {
    match s {
        List::Nil => tbl,
        List::Cons(as_, _) => {
            let f4 = |cs, (varr, vall)| match cs {
                ConflictSet::Unknown => {
                    if !(csp.rel)(as_, &Assign { varr, value: vall }) {
                        ConflictSet::Known(List::Cons(
                            as_.varr,
                            Rc::new(List::Cons(varr, Rc::new(List::Nil))),
                        ))
                    } else {
                        cs
                    }
                }
                ConflictSet::Known(_) => cs,
            };
            let lscomp2 = |ls: List<i64>, varrr| ls.map(&|valll| (varrr, valll));
            let lscomp1 = |ls: List<i64>| ls.map(&|varrr| lscomp2((1..=csp.vals).into(), varrr));

            tbl.zip_with(lscomp1(((as_.varr + 1)..=csp.vars).into()), &|x, y| {
                x.zip_with(y, &f4)
            })
        }
    }
}
fn empty_table(csp: &CSP) -> List<List<ConflictSet>> {
    let lscomp2 = |ls: List<i64>| ls.map(&|_| ConflictSet::Unknown);
    let lscomp1 = |ls: List<i64>| ls.map(&|_| lscomp2((1..=csp.vals).into()));
    List::Cons(List::Nil, Rc::new(lscomp1((1..=csp.vars).into())))
}

fn combine(ls: List<(List<Assign>, ConflictSet)>, acc: List<i64>) -> List<i64> {
    if ls.is_empty() {
        return acc;
    }

    let (s, cs) = ls.clone().head();
    let cs = match cs {
        ConflictSet::Unknown => return acc,
        ConflictSet::Known(cs) => cs,
    };

    if !cs.contains(&max_level(&s)) {
        cs
    } else {
        combine(ls, cs.union(acc))
    }
}

fn known_conflict(c: &ConflictSet) -> bool {
    match c {
        ConflictSet::Known(cs) => !cs.is_empty(),
        ConflictSet::Unknown => false,
    }
}

fn known_solution(c: &ConflictSet) -> bool {
    match c {
        ConflictSet::Known(cs) => cs.is_empty(),
        ConflictSet::Unknown => false,
    }
}

fn collect_conflict(ls: List<ConflictSet>) -> List<i64> {
    ls.fold(List::Nil, &|cs, css| match cs {
        ConflictSet::Unknown => css,
        ConflictSet::Known(cs_) => cs_.union(css),
    })
}

fn domain_wipeout(
    _: &CSP,
    t: Node<((List<Assign>, ConflictSet), List<List<ConflictSet>>)>,
) -> Node<(List<Assign>, ConflictSet)> {
    let f8 = |((as_, cs), tbl)| {
        let lscomp1 = |ls: List<List<ConflictSet>>| ls.filter(&|vs| vs.all(&known_conflict));
        let wiped_domains: List<List<ConflictSet>> = lscomp1(tbl);
        let cs_ = if wiped_domains.is_empty() {
            cs
        } else {
            let hd = wiped_domains.head();
            ConflictSet::Known(collect_conflict(hd))
        };
        (as_, cs_)
    };
    t.map(&f8)
}

fn init_tree(
    f: &dyn Fn(List<Assign>) -> List<List<Assign>>,
    x: List<Assign>,
) -> Node<List<Assign>> {
    let children = f(x.clone()).map(&|y| Rc::new(init_tree(f, y)));
    Node { lab: x, children }
}

fn mk_tree(csp: &CSP) -> Node<List<Assign>> {
    let next = |ss: List<Assign>| {
        if max_level(&ss) < csp.vars {
            (1..=csp.vals)
                .map({
                    let ss_ = ss.clone();
                    move |j| {
                        List::Cons(
                            Assign {
                                varr: max_level(&ss_) + 1,
                                value: j,
                            },
                            Rc::new(ss_.clone()),
                        )
                    }
                })
                .into()
        } else {
            List::Nil
        }
    };
    init_tree(&next, List::Nil)
}

fn search(
    labeler: Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
    csp: CSP,
) -> List<List<Assign>> {
    let tree = mk_tree(&csp);
    let labeled = labeler(csp, tree);
    let pruned = labeled.prune(&|(_, x)| known_conflict(&x));
    pruned
        .leaves()
        .filter(&|(_, x)| known_solution(x))
        .map(&|(x, _)| x)
}

fn safe(as1: &Assign, as2: &Assign) -> bool {
    !(as1.value == as2.value) && !((as1.varr - as2.varr).abs() == (as1.value - as2.value).abs())
}

fn queens(n: i64) -> CSP {
    CSP {
        vars: n,
        vals: n,
        rel: Box::new(safe),
    }
}

fn bt(csp: &CSP, t: Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)> {
    let f3 = |s: List<Assign>| {
        (
            s.clone(),
            match earliest_inconsistency(csp, s.clone()) {
                Some((a, b)) => {
                    ConflictSet::Known(List::Cons(a, Rc::new(List::Cons(b, Rc::new(List::Nil)))))
                }
                None => check_complete(csp, &s),
            },
        )
    };
    t.map(&f3)
}

fn bm(csp: CSP, t: Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)> {
    lookup_cache(&csp, cache_checks(&csp, empty_table(&csp), t)).map(&|(x, _)| x)
}

fn bjbt(csp: CSP, t: Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)> {
    bj(&csp, bt(&csp, t))
}

fn bj(_: &CSP, t: Node<(List<Assign>, ConflictSet)>) -> Node<(List<Assign>, ConflictSet)> {
    let f6 = |lp2, chs: List<Node<(List<Assign>, ConflictSet)>>| match lp2 {
        (a, ConflictSet::Known(cs)) => Node {
            lab: (a, ConflictSet::Known(cs)),
            children: chs.map(&Rc::new),
        },
        (a, ConflictSet::Unknown) => Node {
            lab: (
                a,
                ConflictSet::Known(combine(chs.clone().map(&|n| n.lab.clone()), List::Nil)),
            ),
            children: chs.map(&Rc::new),
        },
    };
    t.fold(&f6)
}

fn bjbt_(csp: CSP, t: Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)> {
    bj_(&csp, bt(&csp, t))
}

fn bj_(_: &CSP, t: Node<(List<Assign>, ConflictSet)>) -> Node<(List<Assign>, ConflictSet)> {
    let f7 = |tp2, chs: List<Node<(List<Assign>, ConflictSet)>>| match tp2 {
        (a, ConflictSet::Known(cs)) => Node {
            lab: (a, ConflictSet::Known(cs)),
            children: chs.map(&Rc::new),
        },
        (a, ConflictSet::Unknown) => {
            let cs_ = ConflictSet::Known(combine(chs.clone().map(&|n| n.lab.clone()), List::Nil));
            if known_conflict(&cs_) {
                Node {
                    lab: (a, cs_),
                    children: List::Nil,
                }
            } else {
                Node {
                    lab: (a, cs_),
                    children: chs.map(&Rc::new),
                }
            }
        }
    };
    t.fold(&f7)
}

fn fc(csp: CSP, t: Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)> {
    domain_wipeout(
        &csp,
        lookup_cache(&csp, cache_checks(&csp, empty_table(&csp), t)),
    )
}

fn try_(
    n: i64,
    algorithm: Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
) -> i64 {
    search(algorithm, queens(n)).len() as i64
}

fn test_constraints_nofib(n: i64) -> List<i64> {
    [
        Box::new(|csp, n| bt(&csp, n))
            as Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bm(csp, n))
            as Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bjbt(csp, n))
            as Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bjbt_(csp, n))
            as Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
        Box::new(|csp, n| fc(csp, n))
            as Box<dyn FnOnce(CSP, Node<List<Assign>>) -> Node<(List<Assign>, ConflictSet)>>,
    ]
    .map(&|x| try_(n, x))
    .iter()
    .map(|x| *x)
    .into()
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_constraints_nofib(n);
    if iters == 1 {
        println!("{:?}", res);
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
