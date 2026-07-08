data List[A] { Nil, Cons(a: A, as: List[A]) }
data Pair[A, B] { Tup(a: A, b: B) }
data Option[A] { None, Some(a: A) }
data Bool { True, False }

def fun(a: Bool): Bool {
    a.case {
        True => False,
        False => True
    }
}

def fun(a: List[i64], b: List[i64]): List[i64] {
    a
}

def fun(a: Option[i64], b: Option[i64]): Option[i64] {
    a
}

def fun(a: Pair[List[i64], i64], b: Bool): Bool {
    b
}

def fun(a: List[i64]): i64 {
    a.case {
        Cons(x, xs) => x,
        Nil => 0
    }
}


def main(): i64 {
    let x = True;
    let y = fun(x);
    let l1 = Cons(14, Cons(9, Nil));
    let l2 = Cons(2, Nil);

    let z = fun(l1, l2);
    println_i64(fun(z));
    0
}