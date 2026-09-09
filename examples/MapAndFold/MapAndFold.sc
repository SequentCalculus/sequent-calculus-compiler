data List[A+] { Nil, Cons(x: A, xs: List[A]) }
data Option[A+] { None, Some(value: A) }

codata Fun[A+, B+] { apply(x: A): B }
codata Fun2[A+, B+, C+] { apply2(x: A, y: B): C }

def map[A+, B+](f: Fun[A, B], l: List[A]): List[B] {
    l.case[A] {
        Nil => Nil,
        Cons(x, xs) => Cons(f.apply[A, B](x), map[A, B](f, xs))
    }
}

def foldr[A+, B+](f: Fun2[A, B, B], st: B, l: List[A]): B {
    l.case[A] {
        Nil => st,
        Cons(y, ys) => f.apply2[A, B, B](y, foldr[A, B](f, st, ys))
    }
}

def main(): i64 {
    let l: List[i64] = Cons(1, Cons(2, Cons(3, Nil)));

    let optList: List[Option[i64]] = map[i64, Option[i64]](
        new { apply(x) => if x > 1 { Some(x) } else { None } },
        l
    );

    let totalSum: i64 = foldr[Option[i64], i64](
        new { 
            apply2(opt, acc) => 
                opt.case[i64] {
                    None => acc + 10,
                    Some(v) => acc + v
                }
        },
        0,
        optList
    );

    println_i64(totalSum);
    0
}