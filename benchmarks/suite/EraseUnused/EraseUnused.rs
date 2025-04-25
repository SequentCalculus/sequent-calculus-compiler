fn useless(i: i64, n: i64, _: Vec<i64>) -> i64 {
    if i < n {
        useless(i + 1, n, replicate(0, i, vec![]))
    } else {
        i
    }
}

fn replicate(v: i64, n: i64, mut a: Vec<i64>) -> Vec<i64> {
    if n == 0 {
        a
    } else {
        a.insert(0, v);
        replicate(v, n - 1, a)
    }
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = useless(0, n, vec![]);
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
