fn quot_rem(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
}

fn g((u1, u2, u3): (i64, i64, i64), (v1, v2, v3): (i64, i64, i64)) -> (i64, i64, i64) {
    if v3 == 0 {
        (u3, u1, u2)
    } else {
        let (q, r) = quot_rem(u3, v3);
        g((v1, v2, v3), (u1 - (q * v1), u2 - (q * v2), r))
    }
}

fn gcd_e(x: i64, y: i64) -> (i64, i64, i64) {
    if x == 0 {
        (y, 0, 1)
    } else {
        g((1, 0, x), (0, 1, y))
    }
}

fn test_gcd_nofib(d: i64) -> i64 {
    let ns: Vec<i64> = (5000..=5000 + d).collect();
    let ms: Vec<i64> = (10000..=10000 + d).collect();
    let tripls: Vec<(i64, i64, (i64, i64, i64))> = ns
        .into_iter()
        .zip(ms.into_iter())
        .map(|(x, y)| (x, y, gcd_e(x, y)))
        .collect();
    let rs: Vec<i64> = tripls
        .into_iter()
        .map(|(_, _, (gg, u, v))| (gg + u).abs() + v)
        .collect();
    rs.into_iter().max().unwrap()
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = test_gcd_nofib(n);
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
        .parse::<i64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
