data Nested[A+] {
    Base(x: A),
    Grow[E+](tag: E, inner: Nested[Nested[A]])
}

data Bool {
    True,
    False
}

data Box {
    Pack[V+](val: V, tag: i64)
}

data List[D+] {
    Nil,
    Cons(x: D, xs: List[D])
}

codata Holder[H+] {
    put[S+](val: S, tag: H): H
}

def identity[X+](x: X): X {
    x
}

def sumNested[A+](n: Nested[A]): i64 {
    n.case[A] {
        Base(x) => 1,
        Grow[E](tag, inner) => 1 + sumNested[Nested[A]](inner)
    }
}

def sumList(l: List[i64]): i64 {
    l.case[i64] {
        Nil => 0,
        Cons(x, xs) => x + sumList(xs)
    }
}

def main(): i64 {
    let inner: Nested[i64] = Base(1);
    let nested: Nested[Nested[i64]] = Base(inner);
    let grown: Nested[i64] = Grow[Bool](True, nested);
    let nestedResult: i64 = sumNested[i64](grown);

    let boxed: Box = Pack[i64](7, 7);
    let boxResult: i64 = boxed.case {
        Pack[V](val, tag) =>
            let discarded: V = identity[V](val);
            tag
    };

    let l: List[i64] = Cons(1, Cons(2, Nil));
    let listResult: i64 = sumList(identity[List[i64]](l));

    let holder: Holder[i64] = new {
        put[S](val, tag) => tag
    };
    let holderResult: i64 = holder.put[i64, Bool](True, 3);

    println_i64(((nestedResult + boxResult) + listResult) + holderResult);
    0
}
