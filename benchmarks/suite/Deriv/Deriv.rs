#[derive(PartialEq, Eq, Clone)]
enum Expr {
    Add(Vec<Expr>),
    Sub(Vec<Expr>),
    Mul(Vec<Expr>),
    Div(Vec<Expr>),
    Num(i64),
    X,
}

fn deriv(e: Expr) -> Expr {
    match e {
        Expr::Add(sums) => Expr::Add(sums.into_iter().map(deriv).collect()),
        Expr::Sub(subs) => Expr::Sub(subs.into_iter().map(deriv).collect()),
        Expr::Mul(muls) => Expr::Mul(vec![
            Expr::Mul(muls.clone()),
            Expr::Add(
                muls.into_iter()
                    .map(|x| Expr::Div(vec![deriv(x.clone()), x]))
                    .collect(),
            ),
        ]),
        Expr::Div(mut divs) if divs.len() == 2 => {
            let x = divs.remove(0);
            let y = divs.remove(0);
            Expr::Sub(vec![
                Expr::Div(vec![deriv(x.clone()), y.clone()]),
                Expr::Div(vec![x, Expr::Mul(vec![y.clone(), y.clone(), deriv(y)])]),
            ])
        }
        Expr::Num(_) => Expr::Num(0),
        Expr::X => Expr::Num(1),
        _ => panic!("Invalid Expression"),
    }
}

fn mk_ans(a: Expr, b: Expr) -> Expr {
    Expr::Add(vec![
        Expr::Mul(vec![
            Expr::Mul(vec![Expr::Num(3), Expr::X, Expr::X]),
            Expr::Add(vec![
                Expr::Div(vec![Expr::Num(0), Expr::Num(3)]),
                Expr::Div(vec![Expr::Num(1), Expr::X]),
                Expr::Div(vec![Expr::Num(1), Expr::X]),
            ]),
        ]),
        Expr::Mul(vec![
            Expr::Mul(vec![a.clone(), Expr::X, Expr::X]),
            Expr::Add(vec![
                Expr::Div(vec![Expr::Num(0), a]),
                Expr::Div(vec![Expr::Num(1), Expr::X]),
                Expr::Div(vec![Expr::Num(1), Expr::X]),
            ]),
        ]),
        Expr::Mul(vec![
            Expr::Mul(vec![b.clone(), Expr::X]),
            Expr::Add(vec![
                Expr::Div(vec![Expr::Num(0), b]),
                Expr::Div(vec![Expr::Num(1), Expr::X]),
            ]),
        ]),
        Expr::Num(0),
    ])
}

fn mk_exp(a: Expr, b: Expr) -> Expr {
    Expr::Add(vec![
        Expr::Mul(vec![Expr::Num(3), Expr::X, Expr::X]),
        Expr::Mul(vec![a, Expr::X, Expr::X]),
        Expr::Mul(vec![b, Expr::X]),
        Expr::Num(5),
    ])
}

fn main_loop(iters: u64, n: i64, m: i64) -> i64 {
    let res = deriv(mk_exp(Expr::Num(n), Expr::Num(m)));
    let expected = mk_ans(Expr::Num(n), Expr::Num(m));
    if iters == 1 {
        println!("{}", if res == expected { 1 } else { 0 });
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
        .parse::<i64>()
        .expect("n must be a number");
    let m = args
        .next()
        .expect("Missing Argument m")
        .parse::<i64>()
        .expect("m must be a number");

    std::process::exit(main_loop(iters, n, m) as i32)
}
