data List[A] { Nil, Cons(a: A, xs: List[A]) }

def range(i: i64, n: i64): List[i64] {
    if i < n { Cons(i, range(i + 1, n)) } else { Nil }
}

def sum(xs: List[i64]): i64 {
    xs.case[i64] {
        Nil => 0,
        Cons(y, ys) => y + sum(ys)
    }
}

def main_loop(iters: i64, n: i64): i64 {
    let res: i64 = sum(range(0, n));
    if iters == 1 {
        println_i64(res);
        0
    } else {
        main_loop(iters - 1, n)
    }
}

def main(iters: i64, n: i64): i64 {
    main_loop(iters, n)
}