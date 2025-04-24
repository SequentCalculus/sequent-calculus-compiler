fn ack(m: i64, n: i64) -> i64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ack(m - 1, 1)
    } else {
        ack(m - 1, ack(m, n - 1))
    }
}

fn main_loop(iters: u64, m: i64, n: i64) -> i64 {
    let res = ack(m, n);
    if iters == 1 {
        println!("{}", res);
        0
    } else {
        main_loop(iters - 1, m, n)
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
    let m = args
        .next()
        .expect("Missing Argument m")
        .parse::<i64>()
        .expect("m must be a number");
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<i64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, m, n) as i32)
}
