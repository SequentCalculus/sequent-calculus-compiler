fn sum_loop(i: i64, tot: i64, stop: i64, f: Box<impl Fn(i64) -> i64>) -> i64 {
    if stop < i {
        tot
    } else {
        sum_loop(i + 1, f(i) + tot, stop, f)
    }
}

fn sum(f: Box<impl Fn(i64) -> i64>, start: i64, stop: i64) -> i64 {
    sum_loop(start, 0, stop, f)
}

fn motz(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        let limit = n - 2;
        let product = Box::new(|i| motz(i) * motz(limit - 1));
        motz(n - 1) + sum(product, 0, limit)
    }
}

fn main_loop(iters: u64, n: i64) -> i64 {
    let res = motz(n);
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
