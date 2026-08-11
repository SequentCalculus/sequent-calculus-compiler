data Nested[A] {
    Base(x: A),
    Grow[E](tag: E, inner: Nested[Nested[A]])
}

data Bool {
    True,
    False
}

data Box {
    Pack[V](val: V)
}

data List[D] {
    Nil,
    Cons(x: D, xs: List[D])
}

codata Holder[H] {
    put[S](val: S, tag: H): H
}

def identity[X](x: X): X {
    x
}

def sumNested[A](n: Nested[A]): i64 {
    n.case[A] {
        Base(x) => 1,
        Grow[E](tag, inner) => 1 + sumNested[Nested[A]](inner)
    }
}

def unpackBox(): i64 {
    let b: Box = Pack[i64](7);
    b.case {
        Pack[V](v) => identity[i64](0)
    }
}

def sumList(l: List[i64]): i64 {
    l.case[i64] {
        Nil => 0,
        Cons(x, xs) => x + sumList(xs)
    }
}

def main(): i64 {
    let b0: Nested[i64] = Base(1);
    let b1: Nested[Nested[i64]] = Base(b0);
    let outer1: Nested[i64] = Grow[Bool](True, b1);
    let nestedResult1: i64 = sumNested[i64](outer1);

  
    let c0: Nested[i64] = Base(5);
    let c1: Nested[Nested[i64]] = Base(c0);
    let c2: Nested[Nested[Nested[i64]]] = Base(c1);
    let outer2: Nested[Nested[i64]] = Grow[i64](7, c2);
    let nestedResult2: i64 = sumNested[Nested[i64]](outer2);

    let boxResult: i64 = unpackBox();

    let l: List[i64] = Cons(1, Cons(2, Nil));
    let listResult: i64 = sumList(l);

    let holder: Holder[i64] = new {
        put[S](val, tag) => tag
    };
    let holderResult: i64 = holder.put[i64, Bool](True, 3);

    let idInt: i64 = identity[i64](7);
    let idList: List[i64] = identity[List[i64]](l);
    let idBool: Bool = identity[Bool](True);

    println_i64(((((nestedResult1 + nestedResult2) + boxResult) + listResult) + holderResult) + idInt);
    0
}