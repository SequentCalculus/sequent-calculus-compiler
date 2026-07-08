data List[A] { Nil, Cons(x: A, xs: List[A]) }

def main(): i64 {
    let correct_list = Cons(5, Nil);
    let false_list = Cons(Cons(3, Nil), Cons(3, Nil));
    0
}