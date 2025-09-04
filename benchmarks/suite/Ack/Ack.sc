def ack(m: i64, n: i64): i64 {
    if m == 0 { n + 1 } else { if n == 0 { ack(m - 1, 1) } else { ack(m - 1, ack(m, n - 1)) } }
}

def main_loop(iters: i64, m: i64, n: i64): i64 {
    let res: i64 = ack(m, n);
    if iters == 1 {
        println_i64(res);
        0
    } else {
        main_loop(iters - 1, m, n)
    }
}

def main(iters: i64, m: i64, n: i64): i64 {
    main_loop(iters, m, n)
}