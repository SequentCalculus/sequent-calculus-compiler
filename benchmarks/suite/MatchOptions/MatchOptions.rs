fn attempt(i: u64) -> Option<u64> {
    if i == 0 {
        Some(i)
    } else {
        match attempt(i - 1) {
            None => None,
            Some(x) => Some(x + 1),
        }
    }
}

fn main_loop(iters: u64, n: u64) -> u64 {
    let res = attempt(n);
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
        .parse::<u64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
