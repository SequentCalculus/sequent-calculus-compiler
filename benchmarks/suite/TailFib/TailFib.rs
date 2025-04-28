fn tfib(n: u64, a: u64, b: u64) -> u64 {
    if n == 0 {
        a
    } else {
        tfib(n - 1, a + b, a)
    }
}

fn fib(n: u64) -> u64 {
    tfib(n, 0, 1)
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = fib(n);
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
        .parse::<u64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
