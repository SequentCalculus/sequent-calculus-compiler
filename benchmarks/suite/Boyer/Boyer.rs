#[derive(PartialEq, Eq, Clone, Copy)]
enum Id {
    A,
    B,
    C,
    D,
    X,
    Y,
    Z,
    U,
    W,
    ADD1,
    AND,
    APPEND,
    CONS,
    CONSP,
    DIFFERENCE,
    DIVIDES,
    EQUAL,
    EVEN,
    EXP,
    F,
    FALSE,
    FOUR,
    GCD,
    GREATEREQP,
    GREATERP,
    IF,
    IFF,
    IMPLIES,
    LENGTH,
    LESSEQP,
    LESSP,
    LISTP,
    MEMBER,
    NIL,
    NILP,
    NLISTP,
    NOT,
    ODD,
    ONE,
    OR,
    PLUS,
    QUOTIENT,
    REMAINDER,
    REVERSE,
    SUB1,
    TIMES,
    TRUE,
    TWO,
    ZERO,
    ZEROP,
}

#[derive(Clone)]
enum Term<'a> {
    Var(Id),
    Fun(Id, Vec<Term<'a>>, &'a dyn Fn() -> Vec<(Term<'a>, Term<'a>)>),
    ERROR,
}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Term<'a>) -> bool {
        match (self, other) {
            (Term::Var(id1), Term::Var(id2)) => id1 == id2,
            (Term::Fun(id1, ts1, _), Term::Fun(id2, ts2, _)) => id1 == id2 && ts1 == ts2,
            _ => false,
        }
    }
}

impl<'a> Eq for Term<'a> {}

fn a<'a>() -> Term<'a> {
    Term::Var(Id::A)
}

fn b<'a>() -> Term<'a> {
    Term::Var(Id::B)
}

fn c<'a>() -> Term<'a> {
    Term::Var(Id::C)
}

fn d<'a>() -> Term<'a> {
    Term::Var(Id::D)
}

fn x<'a>() -> Term<'a> {
    Term::Var(Id::X)
}
fn y<'a>() -> Term<'a> {
    Term::Var(Id::Y)
}
fn z<'a>() -> Term<'a> {
    Term::Var(Id::Z)
}
fn u<'a>() -> Term<'a> {
    Term::Var(Id::U)
}
fn w<'a>() -> Term<'a> {
    Term::Var(Id::W)
}

fn boyer_false<'a>() -> Term<'a> {
    Term::Fun(Id::FALSE, vec![], &|| vec![])
}

fn nil<'a>() -> Term<'a> {
    Term::Fun(Id::NIL, vec![], &|| vec![])
}

fn boyer_true<'a>() -> Term<'a> {
    Term::Fun(Id::TRUE, vec![], &|| vec![])
}

fn zero<'a>() -> Term<'a> {
    Term::Fun(Id::ZERO, vec![], &|| vec![])
}

fn one<'a>() -> Term<'a> {
    Term::Fun(Id::ONE, vec![], &|| vec![(one(), add1(zero()))])
}

fn two<'a>() -> Term<'a> {
    Term::Fun(Id::TWO, vec![], &|| vec![(two(), add1(one()))])
}

fn four<'a>() -> Term<'a> {
    Term::Fun(Id::FOUR, vec![], &|| vec![(four(), add1(add1(two())))])
}

fn add1<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::ADD1, vec![a], &|| vec![])
}

fn if_<'a>(a: Term<'a>, b: Term<'a>, c: Term<'a>) -> Term<'a> {
    Term::Fun(Id::IF, vec![a, b, c], &|| {
        vec![(
            if_(if_(x(), y(), z()), u(), w()),
            if_(x(), if_(y(), u(), w()), if_(z(), u(), w())),
        )]
    })
}

fn not_<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::NOT, vec![a], &|| {
        vec![(not_(x()), if_(x(), boyer_true(), boyer_false()))]
    })
}

fn and_<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::AND, vec![a, b], &|| {
        vec![(
            and_(x(), y()),
            if_(x(), if_(y(), boyer_true(), boyer_false()), boyer_false()),
        )]
    })
}

fn append_<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::APPEND, vec![a, b], &|| {
        vec![(
            append_(append_(x(), y()), z()),
            append_(x(), append_(y(), z())),
        )]
    })
}

fn cons<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::CONS, vec![a, b], &|| vec![])
}

fn consp<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::CONSP, vec![a], &|| {
        vec![(consp(cons(x(), y())), boyer_true())]
    })
}

fn difference<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::DIFFERENCE, vec![a, b], &|| {
        vec![
            (difference(x(), x()), zero()),
            (difference(plus(x(), y()), x()), y()),
            (difference(plus(y(), x()), x()), y()),
            (
                difference(plus(x(), y()), plus(x(), z())),
                difference(y(), z()),
            ),
            (difference(plus(y(), plus(x(), z())), x()), plus(y(), z())),
            (difference(add1(plus(y(), z())), z()), add1(y())),
            (difference(add1(add1(x())), two()), x()),
        ]
    })
}

fn divides<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::DIVIDES, vec![a, b], &|| {
        vec![(divides(x(), y()), zerop(remainder(y(), x())))]
    })
}

fn equal<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::EQUAL, vec![a, b], &|| {
        vec![
            (equal(plus(x(), y()), zero()), and_(zerop(x()), zerop(y()))),
            (equal(plus(x(), y()), plus(x(), z())), equal(y(), z())),
            (equal(zero(), difference(x(), y())), not_(lessp(y(), z()))),
            (
                equal(x(), difference(x(), y())),
                or_(equal(x(), zero()), zerop(y())),
            ),
            (
                equal(x(), difference(x(), y())),
                or_(equal(x(), zero()), zerop(y())),
            ),
            (equal(times(x(), y()), zero()), or_(zerop(x()), zerop(y()))),
            (equal(append_(x(), y()), append_(x(), z())), equal(y(), z())),
            (
                equal(y(), times(x(), y())),
                or_(equal(y(), zero()), equal(x(), one())),
            ),
            (
                equal(x(), times(x(), y())),
                or_(equal(x(), zero()), equal(y(), one())),
            ),
            (
                equal(times(x(), y()), one()),
                and_(equal(x(), one()), equal(y(), one())),
            ),
            (
                equal(difference(x(), y()), difference(z(), y())),
                if_(
                    lessp(x(), y()),
                    not_(lessp(y(), z())),
                    if_(lessp(z(), y()), not_(lessp(y(), x())), equal(x(), z())),
                ),
            ),
            (
                equal(lessp(x(), y()), z()),
                if_(
                    lessp(x(), y()),
                    equal(boyer_true(), z()),
                    equal(boyer_false(), z()),
                ),
            ),
        ]
    })
}

fn even_<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::EVEN, vec![a], &|| {
        vec![(even_(x()), if_(zerop(x()), boyer_true(), odd_(sub1(x()))))]
    })
}

fn exp_<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::EXP, vec![a, b], &|| {
        vec![
            (
                exp_(x(), plus(y(), z())),
                times(exp_(x(), y()), exp_(x(), z())),
            ),
            (
                exp_(x(), plus(y(), z())),
                times(exp_(x(), y()), exp_(x(), z())),
            ),
            (exp_(x(), times(y(), z())), exp_(exp_(x(), y()), z())),
        ]
    })
}

fn f<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::F, vec![a], &|| vec![])
}

fn gcd_<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::GCD, vec![a, b], &|| {
        vec![
            (gcd_(x(), y()), gcd_(y(), x())),
            (
                gcd_(times(x(), z()), times(y(), z())),
                times(z(), gcd_(x(), y())),
            ),
        ]
    })
}

fn greatereqp<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::GREATEREQP, vec![a, b], &|| {
        vec![(greatereqp(x(), y()), not_(lessp(x(), y())))]
    })
}

fn implies<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::IMPLIES, vec![a, b], &|| {
        vec![(
            implies(x(), y()),
            if_(x(), if_(y(), boyer_true(), boyer_false()), boyer_true()),
        )]
    })
}

fn iff<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::IFF, vec![a, b], &|| {
        vec![(iff(x(), y()), and_(implies(x(), y()), implies(y(), x())))]
    })
}

fn length_<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::LENGTH, vec![a], &|| {
        vec![
            (length_(reverse_(x())), length_(x())),
            (
                length_(cons(x(), cons(y(), cons(z(), cons(y(), w()))))),
                plus(four(), length_(w())),
            ),
        ]
    })
}

fn lesseqp<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::LESSEQP, vec![a, b], &|| {
        vec![(lesseqp(x(), y()), not_(lessp(y(), x())))]
    })
}

fn lessp<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::LESSP, vec![a, b], &|| {
        vec![
            (lessp(remainder(x(), y()), y()), not_(zerop(y()))),
            (
                lessp(quotient(x(), y()), x()),
                and_(not_(zerop(x())), lessp(one(), y())),
            ),
            (lessp(plus(x(), y()), plus(x(), z())), lessp(y(), z())),
            (
                lessp(times(x(), z()), times(y(), z())),
                and_(not_(zerop(z())), lessp(x(), y())),
            ),
            (lessp(y(), plus(x(), y())), not_(zerop(x()))),
        ]
    })
}

fn nilp<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::NILP, vec![a], &|| vec![(nilp(x()), equal(x(), nil()))])
}

fn listp<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::LISTP, vec![a], &|| {
        vec![(listp(x()), or_(nilp(x()), consp(x())))]
    })
}

fn member<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::MEMBER, vec![a, b], &|| {
        vec![
            (
                member(x(), append_(y(), z())),
                or_(member(x(), y()), member(x(), z())),
            ),
            (member(x(), reverse_(y())), member(x(), y())),
        ]
    })
}

fn nlistp<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::NLISTP, vec![a], &|| {
        vec![(nlistp(x()), not_(listp(x())))]
    })
}

fn odd_<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::ODD, vec![a], &|| vec![(odd_(x()), even_(sub1(x())))])
}

fn or_<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::OR, vec![a, b], &|| {
        vec![(
            or_(x(), y()),
            if_(x(), boyer_true(), if_(y(), boyer_true(), boyer_false())),
        )]
    })
}

fn plus<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::PLUS, vec![a, b], &|| {
        vec![
            (plus(plus(x(), y()), z()), plus(x(), plus(y(), z()))),
            (
                plus(remainder(x(), y()), times(y(), quotient(x(), y()))),
                x(),
            ),
            (plus(x(), add1(y())), add1(plus(x(), y()))),
        ]
    })
}

fn quotient<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::QUOTIENT, vec![a, b], &|| {
        vec![
            (
                quotient(plus(x(), plus(x(), y())), two()),
                plus(x(), quotient(y(), two())),
            ),
            (quotient(times(y(), x()), y()), if_(zerop(y()), zero(), x())),
        ]
    })
}

fn remainder<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::REMAINDER, vec![a, b], &|| {
        vec![
            (remainder(x(), one()), zero()),
            (remainder(x(), x()), zero()),
            (remainder(times(x(), y()), x()), zero()),
            (remainder(times(x(), y()), y()), zero()),
        ]
    })
}

fn reverse_<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::REVERSE, vec![a], &|| {
        vec![(
            reverse_(append_(x(), y())),
            append_(reverse_(y()), reverse_(x())),
        )]
    })
}

fn sub1<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::SUB1, vec![a], &|| vec![(sub1(add1(x())), x())])
}

fn times<'a>(a: Term<'a>, b: Term<'a>) -> Term<'a> {
    Term::Fun(Id::TIMES, vec![a, b], &|| {
        vec![
            (
                times(x(), plus(y(), z())),
                plus(times(x(), y()), times(x(), z())),
            ),
            (times(times(x(), y()), z()), times(x(), times(y(), z()))),
            (
                times(x(), difference(y(), z())),
                difference(times(y(), x()), times(z(), x())),
            ),
            (times(x(), add1(y())), plus(x(), times(x(), y()))),
        ]
    })
}

fn zerop<'a>(a: Term<'a>) -> Term<'a> {
    Term::Fun(Id::ZEROP, vec![a], &|| {
        vec![(zerop(x()), equal(x(), zero()))]
    })
}

fn one_way_unify1<'a>(
    t1: Term<'a>,
    t2: Term<'a>,
    subst: Vec<(Id, Term<'a>)>,
) -> (bool, Vec<(Id, Term<'a>)>) {
    match (t1, t2) {
        (t1, Term::Var(vid2)) => {
            if let Some((_, v2)) = subst.iter().find(|(id, _)| *id == vid2) {
                (t1 == *v2, subst)
            } else {
                let mut new_subst = subst;
                new_subst.insert(0, (vid2, t1));
                (true, new_subst)
            }
        }
        (Term::Fun(f1, as1, _), Term::Fun(f2, as2, _)) => {
            if f1 == f2 {
                one_way_unify1_lst(as1, as2, subst)
            } else {
                (false, vec![])
            }
        }
        _ => (false, vec![]),
    }
}

fn one_way_unify1_lst<'a>(
    mut tts1: Vec<Term<'a>>,
    mut tts2: Vec<Term<'a>>,
    subst: Vec<(Id, Term<'a>)>,
) -> (bool, Vec<(Id, Term<'a>)>) {
    if tts1.is_empty() && tts2.is_empty() {
        return (true, subst);
    }
    if tts1.is_empty() || tts2.is_empty() {
        return (false, subst);
    }

    let t1 = tts1.remove(0);
    let t2 = tts2.remove(0);
    let (hd_ok, subst_) = one_way_unify1(t1, t2, subst);
    let (tl_ok, subst__) = one_way_unify1_lst(tts1, tts2, subst_);
    (hd_ok && tl_ok, subst__)
}

fn one_way_unify<'a>(t1: Term<'a>, t2: Term<'a>) -> (bool, Vec<(Id, Term<'a>)>) {
    one_way_unify1(t1, t2, vec![])
}

fn rewrite_with_lemmas<'a>(t: Term<'a>, mut lss: Vec<(Term<'a>, Term<'a>)>) -> Term<'a> {
    if lss.is_empty() {
        return t;
    }
    let (lhs, rhs) = lss.remove(0);

    let (unified, subst) = one_way_unify(t.clone(), lhs);
    if unified {
        rewrite(apply_subst(subst, rhs))
    } else {
        rewrite_with_lemmas(t, lss)
    }
}

fn rewrite<'a>(t: Term<'a>) -> Term<'a> {
    match t {
        Term::Var(v) => Term::Var(v),
        Term::Fun(f, args, lemmas) => rewrite_with_lemmas(
            Term::Fun(f, args.into_iter().map(rewrite).collect(), lemmas),
            lemmas(),
        ),
        Term::ERROR => Term::ERROR,
    }
}

fn truep<'a>(x: &Term<'a>, l: &[Term<'a>]) -> bool {
    match x {
        Term::Fun(Id::TRUE, _, _) => true,
        _ => l.iter().find(|t| *t == x).is_some(),
    }
}

fn falsep<'a>(x: &Term<'a>, l: &[Term<'a>]) -> bool {
    match x {
        Term::Fun(Id::FALSE, _, _) => true,
        _ => l.iter().find(|t| *t == x).is_some(),
    }
}

fn tautologyp<'a>(x: Term<'a>, true_lst: Vec<Term<'a>>, false_lst: Vec<Term<'a>>) -> bool {
    if truep(&x, &true_lst) {
        true
    } else if falsep(&x, &false_lst) {
        false
    } else {
        match x {
            Term::Fun(Id::IF, mut args, _) => {
                if args.len() != 3 {
                    return false;
                }

                let cond = args.remove(0);
                let t = args.remove(0);
                let e = args.remove(0);

                if truep(&cond, &true_lst) {
                    return tautologyp(t, true_lst, false_lst);
                }

                if falsep(&cond, &false_lst) {
                    return tautologyp(t, true_lst, false_lst);
                }

                let mut new_tru = true_lst.clone();
                new_tru.insert(0, cond.clone());

                let mut new_fls = false_lst.clone();
                new_fls.insert(0, cond);

                tautologyp(t, new_tru, false_lst) && tautologyp(e, true_lst, new_fls)
            }
            _ => false,
        }
    }
}

fn apply_subst<'a>(subst: Vec<(Id, Term<'a>)>, t: Term<'a>) -> Term<'a> {
    match t {
        Term::Var(vid) => {
            if let Some((_, value)) = subst.into_iter().find(|(id, _)| vid == *id) {
                value
            } else {
                Term::Var(vid)
            }
        }
        Term::Fun(f, args, ls) => Term::Fun(
            f,
            args.into_iter()
                .map(|t| apply_subst(subst.clone(), t))
                .collect(),
            ls,
        ),
        Term::ERROR => Term::ERROR,
    }
}

fn tautp<'a>(x: Term<'a>) -> bool {
    tautologyp(rewrite(x), vec![], vec![])
}

fn test0<'a>(xxxx: Term<'a>) -> bool {
    let subst0 = vec![
        (Id::X, f(plus(plus(a(), b()), plus(c(), zero())))),
        (Id::Y, f(times(times(a(), b()), plus(c(), d())))),
        (Id::Z, f(reverse_(append_(append_(a(), b()), nil())))),
        (Id::U, equal(plus(a(), b()), difference(x(), y()))),
        (Id::W, lessp(remainder(a(), b()), member(a(), length_(b())))),
    ];
    let theorem = implies(
        and_(
            implies(xxxx, y()),
            and_(
                implies(y(), z()),
                and_(implies(z(), u()), implies(u(), w())),
            ),
        ),
        implies(x(), w()),
    );
    tautp(apply_subst(subst0, theorem))
}

fn replicate_term<'a>(n: u64, t: Term<'a>) -> Vec<Term<'a>> {
    let mut ts = Vec::with_capacity(n as usize);
    for _ in 0..n {
        ts.push(t.clone());
    }
    ts
}

fn test_boyer_nofib(n: u64) -> bool {
    let ts = replicate_term(n, Term::Var(Id::X));
    ts.into_iter().all(|t| test0(t))
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = test_boyer_nofib(n);
    if iters == 1 {
        println!("{}", if res { 1 } else { -1 });
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
