fn fibonacci(i: u64) -> u64 {
    if i == 0 || i == 1 {
        i
    } else {
        fibonacci(i - 1) + fibonacci(i - 2)
    }
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = fibonacci(n);
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
