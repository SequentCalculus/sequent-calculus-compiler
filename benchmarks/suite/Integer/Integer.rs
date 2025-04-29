use std::rc::Rc;

#[derive(Debug)]
enum Either<T, U> {
    Left(T),
    Right(U),
}

#[derive(Clone, Debug)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T> List<T> {
    fn head(self) -> T {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(t, _) => t,
        }
    }
}

fn enum_from_then_to(from: i64, then: i64, t: i64) -> List<i64> {
    if from <= t {
        List::Cons(from, Rc::new(enum_from_then_to(then, (2 * then) - from, t)))
    } else {
        List::Nil
    }
}

fn bench_lscomp2(
    ls: List<i64>,
    t1: List<i64>,
    a: i64,
    op: Box<dyn Fn(i64, i64) -> Either<i64, bool>>,
    bstart: i64,
    bstep: i64,
    blim: i64,
) -> List<Either<i64, bool>> {
    match ls {
        List::Nil => bench_lscomp1(t1, bstart, bstep, blim, op),
        List::Cons(b, t2) => List::Cons(
            op(a, b),
            Rc::new(bench_lscomp2(
                t1,
                Rc::unwrap_or_clone(t2),
                a,
                op,
                bstart,
                bstep,
                blim,
            )),
        ),
    }
}

fn bench_lscomp1(
    ls: List<i64>,
    bstart: i64,
    bstep: i64,
    blim: i64,
    op: Box<dyn Fn(i64, i64) -> Either<i64, bool>>,
) -> List<Either<i64, bool>> {
    match ls {
        List::Nil => List::Nil,
        List::Cons(a, t1) => bench_lscomp2(
            enum_from_then_to(bstart, bstart + bstep, blim),
            Rc::unwrap_or_clone(t1),
            a,
            op,
            bstart,
            bstep,
            blim,
        ),
    }
}

fn integerbench(
    op: Box<dyn Fn(i64, i64) -> Either<i64, bool>>,
    astart: i64,
    astep: i64,
    alim: i64,
    bstart: i64,
    bstep: i64,
    blim: i64,
) -> List<Either<i64, bool>> {
    bench_lscomp1(
        enum_from_then_to(astart, astart + astep, alim),
        bstart,
        bstep,
        blim,
        op,
    )
}

fn runbench(
    jop: Box<dyn Fn(i64, i64) -> Either<i64, bool>>,
    iop: Box<dyn Fn(i64, i64) -> Either<i64, bool>>,
    astart: i64,
    astep: i64,
    alim: i64,
    _: i64,
    _: i64,
    _: i64,
) -> List<Either<i64, bool>> {
    let _ = integerbench(iop, astart, astep, alim, astart, astep, alim);
    integerbench(jop, astart, astep, alim, astart, astep, alim)
}

fn runalltests(
    astart: i64,
    astep: i64,
    alim: i64,
    _: i64,
    _: i64,
    _: i64,
) -> List<Either<i64, bool>> {
    let z_add: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Left(a + b));
    let z_sub: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Left(a - b));
    let z_mul: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Left(a * b));
    let z_div: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Left(a / b));
    let z_mod: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Left(a % b));
    let z_equal: Box<dyn Fn(i64, i64) -> Either<i64, bool>> =
        Box::new(|a, b| Either::Right(a == b));
    let z_lt: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Right(a < b));
    let z_gt: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Right(a > b));
    let z_leq: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Right(a <= b));
    let z_geq: Box<dyn Fn(i64, i64) -> Either<i64, bool>> = Box::new(|a, b| Either::Right(a >= b));

    let _ = runbench(
        z_add,
        Box::new(|a, b| Either::Left(a + b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_sub,
        Box::new(|a, b| Either::Left(a - b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_mul,
        Box::new(|a, b| Either::Left(a * b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_div,
        Box::new(|a, b| Either::Left(a / b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_mod,
        Box::new(|a, b| Either::Left(a % b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_equal,
        Box::new(|a, b| Either::Right(a == b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_lt,
        Box::new(|a, b| Either::Right(a < b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_leq,
        Box::new(|a, b| Either::Right(a <= b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    let _ = runbench(
        z_gt,
        Box::new(|a, b| Either::Right(a > b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    );
    runbench(
        z_geq,
        Box::new(|a, b| Either::Right(a >= b)),
        astart,
        astep,
        alim,
        astart,
        astep,
        alim,
    )
}

fn test_integer_nofib(n: i64) -> List<Either<i64, bool>> {
    runalltests(-2100000000, n, 2100000000, -2100000000, n, 2100000000)
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_integer_nofib(n);
    if iters == 1 {
        println!(
            "{}",
            match res.head() {
                Either::Left(l) => l,
                Either::Right(b) =>
                    if b {
                        -2
                    } else {
                        -3
                    },
            }
        );
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
