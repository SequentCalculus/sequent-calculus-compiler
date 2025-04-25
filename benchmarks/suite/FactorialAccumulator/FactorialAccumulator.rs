fn factorial(a: i64, i: i64) -> i64 {
    if i == 0 {
        a
    } else {
        factorial((i * a) % 1000000007, i - 1)
    }
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = factorial(1, n);
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
