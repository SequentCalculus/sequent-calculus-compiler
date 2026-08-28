data Box[V+] {
    Wrap(val: V)
}

data Bag[V+] {
    Put(val: V)
}

def grow[C+](n: i64, x: C): i64 {
    if n == 0 {
        0
    } else {
        if n == 1 {
            grow[Box[C]](n - 1, Wrap(x))
        } else {
            grow[Bag[C]](n - 1, Put(x))
        }
    }
}

def main(): i64 {
    println_i64(grow[i64](2, 0));
    0
}
