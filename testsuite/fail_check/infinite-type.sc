data List[A] { Nil, Cons(x: A, xs: List[A]) }

def main(): i64 {
    let a = Nil;
    let b = Cons(a, a);
    0
}
