// A naive 2-3 finger tree (Hinze/Paterson), the textbook example of polymorphic recursion: each
// `Deep` node's inner tree holds `Node[A]`, not `A`, so `FingerTree[A]` recurses at a strictly
// larger type on every level. No amortized O(1) tricks here (no explicit spine caching), just the
// nested-type structure itself plus `pushFront`/`pushBack`, both of which have to call themselves
// polymorphically whenever a boundary digit overflows.

data Digit[A+] {
    One(a: A),
    Two(a: A, b: A),
    Three(a: A, b: A, c: A),
    Four(a: A, b: A, c: A, d: A)
}

data Node[A+] {
    Node2(a: A, b: A),
    Node3(a: A, b: A, c: A)
}

data FingerTree[A+] {
    Empty,
    Single(a: A),
    Deep(prefix: Digit[A], deeper: FingerTree[Node[A]], suffix: Digit[A])
}

def pushFront[A+](x: A, t: FingerTree[A]): FingerTree[A] {
    t.case[A] {
        Empty => Single(x),
        Single(y) => Deep(One(x), Empty, One(y)),
        Deep(pre, deeper, suf) =>
            pre.case[A] {
                One(a) => Deep(Two(x, a), deeper, suf),
                Two(a, b) => Deep(Three(x, a, b), deeper, suf),
                Three(a, b, c) => Deep(Four(x, a, b, c), deeper, suf),
                Four(a, b, c, d) =>
                    Deep(Two(x, a), pushFront[Node[A]](Node3(b, c, d), deeper), suf)
            }
    }
}

def pushBack[A+](t: FingerTree[A], x: A): FingerTree[A] {
    t.case[A] {
        Empty => Single(x),
        Single(y) => Deep(One(y), Empty, One(x)),
        Deep(pre, deeper, suf) =>
            suf.case[A] {
                One(a) => Deep(pre, deeper, Two(a, x)),
                Two(a, b) => Deep(pre, deeper, Three(a, b, x)),
                Three(a, b, c) => Deep(pre, deeper, Four(a, b, c, x)),
                Four(a, b, c, d) =>
                    Deep(pre, pushBack[Node[A]](deeper, Node3(a, b, c)), Two(d, x))
            }
    }
}

// Counts digit-list *slots* summed across every level of nesting -- e.g. one `Node3` at the
// second level counts as 1 slot, not the 3 real base elements it ultimately stands for. Getting
// the *true* element count right would need a fold parameterized over "how many real elements
// does one A represent here", rebuilt one level bigger at each `Deep` (the same trick as
// `pushFront`/`pushBack`, just for a consuming closure instead of a constructor) -- real finger
// tree implementations do exactly this via a higher-order fold, but that is more machinery than a
// naive example needs. `digitSlots` still recurses through the same nested type, at the same
// growing instantiation, so it exercises the identical polymorphic-recursive path.
def digitSlotsInDigit[A+](d: Digit[A]): i64 {
    d.case[A] {
        One(a) => 1,
        Two(a, b) => 2,
        Three(a, b, c) => 3,
        Four(a, b, c, d) => 4
    }
}

def digitSlots[A+](t: FingerTree[A]): i64 {
    t.case[A] {
        Empty => 0,
        Single(a) => 1,
        Deep(pre, deeper, suf) =>
            (digitSlotsInDigit[A](pre) + digitSlots[Node[A]](deeper)) + digitSlotsInDigit[A](suf)
    }
}

// How many times `deeper` had to be unwrapped before hitting `Empty`/`Single` -- again the same
// polymorphically-recursive traversal, this time just counting levels instead of slots.
def depth[A+](t: FingerTree[A]): i64 {
    t.case[A] {
        Empty => 0,
        Single(a) => 0,
        Deep(pre, deeper, suf) => 1 + depth[Node[A]](deeper)
    }
}

def main(): i64 {
    // pushFront six times: after four, the front digit is Four(4,3,2,1); the fifth push
    // overflows it into the deeper tree as a Node3, exercising the polymorphic-recursive branch.
    let t0: FingerTree[i64] = Empty;
    let t1: FingerTree[i64] = pushFront[i64](1, t0);
    let t2: FingerTree[i64] = pushFront[i64](2, t1);
    let t3: FingerTree[i64] = pushFront[i64](3, t2);
    let t4: FingerTree[i64] = pushFront[i64](4, t3);
    let t5: FingerTree[i64] = pushFront[i64](5, t4);
    let t6: FingerTree[i64] = pushFront[i64](6, t5);

    // pushBack five more times, overflowing the back digit the same way on the fifth push.
    let t7: FingerTree[i64] = pushBack[i64](t6, 7);
    let t8: FingerTree[i64] = pushBack[i64](t7, 8);
    let t9: FingerTree[i64] = pushBack[i64](t8, 9);
    let t10: FingerTree[i64] = pushBack[i64](t9, 10);
    let t11: FingerTree[i64] = pushBack[i64](t10, 11);

    println_i64(digitSlots[i64](t11));
    println_i64(depth[i64](t11));
    0
}
