// Both growing cycles below run through a node bundling two type variables of which only the
// first one grows, while the second one merely flows onto itself. Erasure must therefore erase
// only the first parameter of `Tag` and `Pair`, not both. Since `grow` is called with two
// different types for `L`, the partially erased `Tag` is still duplicated per kept parameter
// (`Tag[i64]` and `Tag[Bool]`), each copy with only its own xtor variants, and every `case` in
// `depth` covers exactly the variants of its own copy.
data Tag[V, W] {
    MkTag(val: V, mark: W)
}

data Bool {
    True,
    False
}

data Pair[X, Y] {
    MkPair(x: X, y: Y)
}

// the cycle runs through the node of this declaration's own parameters `[A, B]`
data Foo[A, B] {
    Leaf(a: A, b: B),
    Grow(next: Foo[Pair[A, B], B])
}

def depth[V+, W+](t: Tag[V, W]): i64 {
    t.case[V, W] {
        MkTag(val, mark) => 1
    }
}

// the cycle runs through the node of this def's parameters `[C, L]`
def grow[C+, L+](n: i64, x: C, l: L): i64 {
    if n == 0 {
        0
    } else {
        let t: Tag[C, L] = MkTag(x, l);
        depth[C, L](t) + grow[Tag[C, L], L](n - 1, t, l)
    }
}

def size[A+, B+](f: Foo[A, B]): i64 {
    f.case[A, B] {
        Leaf(a, b) => 1,
        Grow(next) => 1 + size[Pair[A, B], B](next)
    }
}

def main(): i64 {
    let leaf: Foo[Pair[i64, i64], i64] = Leaf(MkPair(1, 2), 3);
    let f: Foo[i64, i64] = Grow(leaf);
    println_i64((grow[i64, i64](3, 0, 7) + grow[i64, Bool](2, 0, True)) + size[i64, i64](f));
    0
}
