codata Fun[A, B] { 
    apply(x: A) : B 
}

codata Id {
    applyId[C](x: C) : C
}

codata Container[T] {
    wrap[S](x: S, tag: T) : T
}

data List[D] {
    Nil,
    Cons(x: D, xs: List[D])
}

data Box {
    Pack[E](x: E, transform: Fun[E, i64])
}

data Sealed[K] {
    Seal[V](val: V, key: K)
}

data Bool {
    True,
    False
}

def identity[X](x: X) : X {
    x
}

def sum(l: List[i64]) : i64 {
    l.case[i64] {
        Nil => 0,
        Cons(x, xs) => x + sum(xs)
    }
}

def negate(b: Bool) : Bool {
    b.case {
        True => False,
        False => True
    }
}

def main(): i64 {
    let l1: List[i64] = Cons(1, Cons(2, Nil));
    let l2: List[Bool] = Cons(True, Cons(False, Nil));

    let sumFn: Fun[List[i64], i64] = new { apply(x) => sum(x) };
    let countFn: Fun[Bool, i64] = new { apply(b) => 1 };

    let boxedList: Box = Pack[List[i64]](l1, sumFn);
    let boxedBool: Box = Pack[Bool](True, countFn);
    let boxedInt: Box = Pack[i64](7, new { apply(n) => n });

    let idFn: Id = new { applyId[F](x) => x };
    let keptInt: i64 = idFn.applyId[i64](42);
    let keptList: List[Bool] = idFn.applyId[List[Bool]](l2);

    let containerInt: Container[i64] = new { wrap[S](x, tag) => tag };
    let a: i64 = containerInt.wrap[i64, i64](5, 100);
    let b: i64 = containerInt.wrap[i64, List[i64]](l1, 200);

    let containerBool: Container[Bool] = new { wrap[S](x, tag) => tag };
    let c: Bool = containerBool.wrap[Bool, i64](9, True);
    let d: Bool = containerBool.wrap[Bool, Bool](False, False);

    let sealedListInInt: Sealed[i64] = Seal[List[i64]](l1, 11);
    let sealedBoolInInt: Sealed[i64] = Seal[Bool](True, 22);
    let sealedIntInBool: Sealed[Bool] = Seal[i64](33, False);

    let n: i64 = identity[i64](keptInt);
    let l3: List[i64] = identity[List[i64]](l1);
    let boolVal: Bool = identity[Bool](True);

    let r1: i64 = boxedList.case {
        Pack[G](x, transform) => transform.apply[G, i64](idFn.applyId[G](x))
    };
    let r2: i64 = boxedBool.case {
        Pack[G](x, transform) => transform.apply[G, i64](x)
    };
    let r3: i64 = boxedInt.case {
        Pack[G](x, transform) => transform.apply[G, i64](x)
    };

    let s1: i64 = sealedListInInt.case[i64] {
        Seal[V](val, key) =>
            let discarded: V = identity[V](val);
            key
    };
    let s2: i64 = sealedBoolInInt.case[i64] {
        Seal[V](val, key) =>
            let discarded: V = identity[V](val);
            key
    };

    let s3: i64 = if 0 >= 1 { 0 } else { 1 };

    println_i64((((((((r1 + r2) + r3) + s1) + s2) + s3) + a) + b) + n);
    0
}