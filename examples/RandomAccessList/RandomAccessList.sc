// Okasaki's nested-type ("polymorphic recursion") binary random-access list: the list is a binary
// counter over "digits" Zero/One, and each successive digit represents a complete pairing-up of
// the previous level's elements. `RList[A]`'s own constructors recurse at `RList[Pair[A]]`, a
// strictly larger type, so `Pair` itself never shows up nested more than once per level, it's
// the *type argument* to `RList` that keeps doubling, exactly the pattern `cons`/`lookup` must
// follow polymorphically to stay well-typed at every depth.

data Pair[A+] {
    MkPair(fst: A, snd: A)
}

data RList[A+] {
    Nil,
    Zero(rest: RList[Pair[A]]),
    One(x: A, rest: RList[Pair[A]])
}

// Push a new element to the front, structurally identical to incrementing a binary counter:
// an empty or already-vacated digit just takes the new element, while a filled digit carries over
// by pairing the new element with the old one and recursing one level deeper, at `Pair[A]`.
def cons[A+](x: A, l: RList[A]): RList[A] {
    l.case[A] {
        Nil => One(x, Nil),
        Zero(rest) => One(x, rest),
        One(y, rest) => Zero(cons[Pair[A]](MkPair(x, y), rest))
    }
}

// Indexed access, 0 at the front (the most recently `cons`ed element). At a `Zero` digit every
// slot at this level holds a `Pair[A]` covering two real elements, so the index is halved and the
// parity picks the half; at a `One` digit index 0 is the extra element itself, everything else
// shifts by one before the same halving applies to the paired-up rest.
def lookup[A+](i: i64, l: RList[A]): A {
    l.case[A] {
        Nil => exit -1,
        Zero(rest) =>
            lookup[Pair[A]](i / 2, rest).case[A] {
                MkPair(a, b) => if i % 2 == 0 { a } else { b }
            },
        One(x, rest) =>
            if i == 0 {
                x
            } else {
                let j: i64 = i - 1;
                lookup[Pair[A]](j / 2, rest).case[A] {
                    MkPair(a, b) => if j % 2 == 0 { a } else { b }
                }
            }
    }
}

def main(): i64 {
    let l0: RList[i64] = Nil;
    let l1: RList[i64] = cons[i64](10, l0);
    let l2: RList[i64] = cons[i64](20, l1);
    let l3: RList[i64] = cons[i64](30, l2);
    let l4: RList[i64] = cons[i64](40, l3);
    let l5: RList[i64] = cons[i64](50, l4);
    let l6: RList[i64] = cons[i64](60, l5);
    let l7: RList[i64] = cons[i64](70, l6);
    let l8: RList[i64] = cons[i64](80, l7);
    let l9: RList[i64] = cons[i64](90, l8);

    // front-to-back is the reverse push order: 90,80,70,60,50,40,30,20,10
    println_i64(lookup[i64](0, l9));
    println_i64(lookup[i64](3, l9));
    println_i64(lookup[i64](8, l9));
    0
}
