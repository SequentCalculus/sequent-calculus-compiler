data List[A] { Nil, Cons(x: A, xs: List[A]) }

def list_append(x: List[i64], y: List[i64]): List[i64] {
    x.case {
        Nil => y,
        Cons(c, cs) => list_append(cs, y)
    }
}


def main(): i64 {
    let list_list = Cons(Cons(5, Nil), Cons(Cons(4, Nil), Nil));
    let simple_list = Cons(1, Nil);
    let appended_list = list_append(simple_list, list_list);
    0
}