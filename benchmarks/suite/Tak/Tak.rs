fn tak(x: i64, y: i64, z: i64) -> i64 {
    if y < x {
        tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y))
    } else {
        z
    }
}

fn main_loop(iters: u64, x: i64, y: i64, z: i64) -> i64 {
    let res = tak(x, y, z);
    if iters == 1 {
        println!("{}", res);
        0
    } else {
        main_loop(iters - 1, x, y, z)
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
    let x = args
        .next()
        .expect("Missing Argument x")
        .parse::<i64>()
        .expect("x must be a number");
    let y = args
        .next()
        .expect("Missing Argument y")
        .parse::<i64>()
        .expect("y must be a number");
    let z = args
        .next()
        .expect("Missing Argument z")
        .parse::<i64>()
        .expect("z must be a number");
    std::process::exit(main_loop(iters, x, y, z) as i32)
}
