def sudan(n: i64, x: i64, y: i64): i64 {
    if n == 0 {
        x + y
    } else {
        if y == 0 {
            x
        } else {
            let inner: i64 = sudan(n, x, y - 1);
            sudan(n - 1, inner, inner + y)
        }
    }
}

def main_loop(iters: i64, n: i64, x: i64, y: i64): i64 {
    let res: i64 = sudan(n, x, y);
    if iters == 1 {
        println_i64(res);
        0
    } else {
        main_loop(iters - 1, n, x, y)
    }
}

def main(iters: i64, n: i64, x: i64, y: i64): i64 {
    main_loop(iters, n, x, y)
}