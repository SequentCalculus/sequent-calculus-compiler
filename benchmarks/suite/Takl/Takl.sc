data List[A] { Nil, Cons(x: A, xs: List[A]) }

data Bool { True, False }

def not(b: Bool): Bool {
    b.case {
        True => False,
        False => True
    }
}

def null(x: List[i64]): Bool {
    x.case[i64] {
        Nil => True,
        Cons(x, xs) => False
    }
}

def tail(x: List[i64]): List[i64] {
    x.case[i64] {
        Nil => Nil,
        Cons(x, xs) => xs
    }
}

def len(l: List[i64]): i64 {
    l.case[i64] {
        Nil => 0,
        Cons(x, xs) => 1 + len(xs)
    }
}

def list_n(n: i64): List[i64] {
    if n == 0 { Nil } else { Cons(n, list_n(n - 1)) }
}

def shorterp(x: List[i64], y: List[i64]): Bool {
    null(y).case {
        True => False,
        False =>
            null(x).case {
                True => True,
                False => shorterp(tail(x), tail(y))
            }
    }
}

def mas(x: List[i64], y: List[i64], z: List[i64]): List[i64] {
    not(shorterp(y, x)).case {
        True => z,
        False => mas(mas(tail(x), y, z), mas(tail(y), z, x), mas(tail(z), x, y))
    }
}

def main_loop(iters: i64, x: i64, y: i64, z: i64): i64 {
    let res: i64 = len(mas(list_n(x), list_n(y), list_n(z)));
    if iters == 1 {
        println_i64(res);
        0
    } else {
        main_loop(iters - 1, x, y, z)
    }
}

def main(iters: i64, x: i64, y: i64, z: i64): i64 {
    main_loop(iters, x, y, z)
}