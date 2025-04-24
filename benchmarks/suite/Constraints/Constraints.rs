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

struct Node<T> {
    lab: T,
    children: Vec<Node<T>>,
}

#[derive(Clone)]
enum ConflictSet {
    Known(Vec<i64>),
    Unknown,
}

fn map_tree<T, U>(f: &dyn Fn(T) -> U, n: Node<T>) -> Node<U> {
    Node {
        lab: f(n.lab),
        children: n.children.into_iter().map(|x| map_tree(f, x)).collect(),
    }
}

fn fold_tree<T, U>(f: &dyn Fn(T, Vec<Node<U>>) -> Node<U>, n: Node<T>) -> Node<U> {
    f(
        n.lab,
        n.children.into_iter().map(|x| fold_tree(f, x)).collect(),
    )
}

fn filter_tree<T: Clone>(p: &dyn Fn(T) -> bool, t: Node<T>) -> Node<T> {
    let f1 = |a, cs: Vec<Node<T>>| Node {
        lab: a,
        children: cs.into_iter().filter(|x| p(x.lab.clone())).collect(),
    };
    fold_tree(&f1, t)
}

fn prune<T: Clone>(p: &dyn Fn(T) -> bool, t: Node<T>) -> Node<T> {
    filter_tree(&|x| !(p(x)), t)
}

fn leaves<T>(t: Node<T>) -> Vec<T> {
    if t.children.is_empty() {
        vec![t.lab]
    } else {
        t.children.into_iter().fold(vec![], |ls, t| {
            ls.into_iter().chain(leaves(t).into_iter()).collect()
        })
    }
}

fn zip_with<T, U, V>(f: &dyn Fn(T, U) -> V, x: Vec<T>, y: Vec<U>) -> Vec<V> {
    x.into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| f(x, y))
        .collect()
}

fn union<T: PartialEq>(mut ls1: Vec<T>, ls2: Vec<T>) -> Vec<T> {
    let ls2_filtered: Vec<T> = ls2.into_iter().filter(|t| !ls1.contains(t)).collect();
    ls1.extend(ls2_filtered.into_iter());
    ls1
}

fn max_level(s: &[Assign]) -> i64 {
    match s.first() {
        None => 0,
        Some(a) => a.varr,
    }
}

fn complete(csp: &CSP, s: &[Assign]) -> bool {
    max_level(s) == csp.vars
}

fn check_complete(csp: &CSP, s: &[Assign]) -> ConflictSet {
    if complete(csp, s) {
        ConflictSet::Known(vec![])
    } else {
        ConflictSet::Unknown
    }
}

fn earliest_inconsistency(csp: &CSP, mut aas: Vec<Assign>) -> Option<(i64, i64)> {
    if aas.is_empty() {
        return None;
    }
    let a = aas.remove(0);
    match aas.iter().filter(|x| !((csp.rel)(&a, x))).next() {
        None => None,
        Some(b) => Some((a.varr, b.varr)),
    }
}

fn lookup_cache(
    csp: &CSP,
    t: Node<(Vec<Assign>, Vec<Vec<ConflictSet>>)>,
) -> Node<((Vec<Assign>, ConflictSet), Vec<Vec<ConflictSet>>)> {
    let f5 = |csp, (as_, tbl): (Vec<Assign>, Vec<Vec<ConflictSet>>)| match as_.first() {
        None => ((vec![], ConflictSet::Unknown), tbl),
        Some(a) => {
            let table_entry = tbl
                .first()
                .unwrap()
                .iter()
                .nth((a.value - 1) as usize)
                .unwrap();
            let cs = match table_entry {
                ConflictSet::Unknown => check_complete(csp, &as_),
                ConflictSet::Known(_) => table_entry.clone(),
            };
            ((as_, cs), tbl)
        }
    };
    map_tree(&|x| f5(csp, x), t)
}

fn cache_checks(
    csp: &CSP,
    tbl: Vec<Vec<ConflictSet>>,
    n: Node<Vec<Assign>>,
) -> Node<(Vec<Assign>, Vec<Vec<ConflictSet>>)> {
    let mut tbl_tl = tbl.clone();
    tbl_tl.remove(0);
    let s = n.lab.clone();
    Node {
        lab: (n.lab, tbl),
        children: n
            .children
            .into_iter()
            .map(|x| cache_checks(csp, fill_table(&s, csp, tbl_tl.clone()), x))
            .collect(),
    }
}

fn fill_table(s: &[Assign], csp: &CSP, tbl: Vec<Vec<ConflictSet>>) -> Vec<Vec<ConflictSet>> {
    match s.first() {
        None => tbl,
        Some(as_) => {
            let f4 = |cs, (varr, vall)| match cs {
                ConflictSet::Unknown => {
                    if !(csp.rel)(as_, &Assign { varr, value: vall }) {
                        ConflictSet::Known(vec![as_.varr, varr])
                    } else {
                        cs
                    }
                }
                ConflictSet::Known(_) => cs,
            };
            let lscomp2 = |ls: Vec<i64>, varrr| ls.iter().map(|valll| (varrr, *valll)).collect();
            let lscomp1 = |ls: Vec<i64>| {
                ls.iter()
                    .map(|varrr| lscomp2((1..=csp.vals).collect(), *varrr))
                    .collect()
            };

            zip_with(
                &|x, y| zip_with(&f4, x, y),
                tbl,
                lscomp1(((as_.varr + 1)..=csp.vars).collect()),
            )
        }
    }
}
fn empty_table(csp: &CSP) -> Vec<Vec<ConflictSet>> {
    let lscomp2 = |ls: Vec<i64>| ls.iter().map(|_| ConflictSet::Unknown).collect();
    let lscomp1 = |ls: Vec<i64>| {
        ls.iter()
            .map(|_| lscomp2((1..=csp.vals).collect()))
            .collect()
    };
    let mut res: Vec<Vec<ConflictSet>> = lscomp1((1..=csp.vars).collect());
    res.insert(0, vec![]);
    res
}

fn combine(mut ls: Vec<(Vec<Assign>, ConflictSet)>, acc: Vec<i64>) -> Vec<i64> {
    if ls.is_empty() {
        return acc;
    }

    let (s, cs) = ls.remove(0);
    let cs = match cs {
        ConflictSet::Unknown => return acc,
        ConflictSet::Known(cs) => cs,
    };

    if !cs.contains(&max_level(&s)) {
        cs
    } else {
        combine(ls, union(cs, acc))
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

fn collect_conflict(ls: Vec<ConflictSet>) -> Vec<i64> {
    ls.into_iter().fold(vec![], |css, cs| match cs {
        ConflictSet::Unknown => css,
        ConflictSet::Known(cs_) => union(cs_, css),
    })
}

fn domain_wipeout(
    _: &CSP,
    t: Node<((Vec<Assign>, ConflictSet), Vec<Vec<ConflictSet>>)>,
) -> Node<(Vec<Assign>, ConflictSet)> {
    let f8 = |((as_, cs), tbl)| {
        let lscomp1 = |ls: Vec<Vec<ConflictSet>>| {
            ls.into_iter()
                .filter(|vs| vs.iter().all(known_conflict))
                .collect()
        };
        let mut wiped_domains: Vec<Vec<ConflictSet>> = lscomp1(tbl);
        let cs_ = if wiped_domains.is_empty() {
            cs
        } else {
            let hd = wiped_domains.remove(0);
            ConflictSet::Known(collect_conflict(hd))
        };
        (as_, cs_)
    };
    map_tree(&f8, t)
}

fn init_tree(f: &dyn Fn(Vec<Assign>) -> Vec<Vec<Assign>>, x: Vec<Assign>) -> Node<Vec<Assign>> {
    let children = f(x.clone()).into_iter().map(|y| init_tree(f, y)).collect();
    Node { lab: x, children }
}

fn mk_tree(csp: &CSP) -> Node<Vec<Assign>> {
    let next = |ss: Vec<Assign>| {
        if max_level(&ss) < csp.vars {
            let lscomp1 = |ls: Vec<i64>| {
                ls.into_iter()
                    .map(|j| {
                        let mut new_ss = ss.clone();
                        new_ss.insert(
                            0,
                            Assign {
                                varr: max_level(&ss) + 1,
                                value: j,
                            },
                        );
                        new_ss
                    })
                    .collect()
            };
            lscomp1((1..=csp.vals).collect())
        } else {
            vec![]
        }
    };
    init_tree(&next, vec![])
}

fn search(
    labeler: Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
    csp: CSP,
) -> Vec<Vec<Assign>> {
    let tree = mk_tree(&csp);
    let labeled = labeler(csp, tree);
    let pruned = prune(&|(_, x)| known_conflict(&x), labeled);
    leaves(pruned)
        .into_iter()
        .filter(|(_, x)| known_solution(x))
        .map(|(x, _)| x)
        .collect()
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

fn bt(csp: &CSP, t: Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)> {
    let f3 = |s: Vec<Assign>| {
        (
            s.clone(),
            match earliest_inconsistency(csp, s.clone()) {
                Some((a, b)) => ConflictSet::Known(vec![a, b]),
                None => check_complete(csp, &s),
            },
        )
    };
    map_tree(&f3, t)
}

fn bm(csp: CSP, t: Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)> {
    map_tree(
        &|(x, _)| x,
        lookup_cache(&csp, cache_checks(&csp, empty_table(&csp), t)),
    )
}

fn bjbt(csp: CSP, t: Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)> {
    bj(&csp, bt(&csp, t))
}

fn bj(_: &CSP, t: Node<(Vec<Assign>, ConflictSet)>) -> Node<(Vec<Assign>, ConflictSet)> {
    let f6 = |lp2, chs| match lp2 {
        (a, ConflictSet::Known(cs)) => Node {
            lab: (a, ConflictSet::Known(cs)),
            children: chs,
        },
        (a, ConflictSet::Unknown) => Node {
            lab: (
                a,
                ConflictSet::Known(combine(chs.iter().map(|n| n.lab.clone()).collect(), vec![])),
            ),
            children: chs,
        },
    };
    fold_tree(&f6, t)
}

fn bjbt_(csp: CSP, t: Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)> {
    bj_(&csp, bt(&csp, t))
}

fn bj_(_: &CSP, t: Node<(Vec<Assign>, ConflictSet)>) -> Node<(Vec<Assign>, ConflictSet)> {
    let f7 = |tp2, chs| match tp2 {
        (a, ConflictSet::Known(cs)) => Node {
            lab: (a, ConflictSet::Known(cs)),
            children: chs,
        },
        (a, ConflictSet::Unknown) => {
            let cs_ =
                ConflictSet::Known(combine(chs.iter().map(|n| n.lab.clone()).collect(), vec![]));
            if known_conflict(&cs_) {
                Node {
                    lab: (a, cs_),
                    children: vec![],
                }
            } else {
                Node {
                    lab: (a, cs_),
                    children: chs,
                }
            }
        }
    };
    fold_tree(&f7, t)
}

fn fc(csp: CSP, t: Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)> {
    domain_wipeout(
        &csp,
        lookup_cache(&csp, cache_checks(&csp, empty_table(&csp), t)),
    )
}

fn try_(
    n: i64,
    algorithm: Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
) -> i64 {
    search(algorithm, queens(n)).len() as i64
}

fn test_constraints_nofib(n: i64) -> Vec<i64> {
    vec![
        Box::new(|csp, n| bt(&csp, n))
            as Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bm(csp, n))
            as Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bjbt(csp, n))
            as Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
        Box::new(|csp, n| bjbt_(csp, n))
            as Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
        Box::new(|csp, n| fc(csp, n))
            as Box<dyn FnOnce(CSP, Node<Vec<Assign>>) -> Node<(Vec<Assign>, ConflictSet)>>,
    ]
    .into_iter()
    .map(|x| try_(n, x))
    .collect()
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
