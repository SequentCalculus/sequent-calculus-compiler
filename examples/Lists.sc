data List[A] { Nil, Cons(x: A, xs: List[A]) }
codata Fun[A, B] { Apply(x: A) : B }

def map(f : Fun[i64, i64] , l : List[i64]) : List[i64] {
    l.case[i64] { Nil => Nil,
                  Cons(x, xs) => Cons(f.Apply[i64, i64](x), map(f, xs)) } }

def mult(x : List[i64]) : i64 {
    x.case[i64] { Nil => 1,
                  Cons(y, ys) => y * mult(ys) } }

codata Fun2[A, B, C] { Apply2(x: A, y: B): C }

def foldr(f : Fun2[i64, i64, i64], st : i64 , l : List[i64]) : i64 {
    l.case[i64] { Nil => st,
                  Cons(y , ys) => f.Apply2[i64, i64,i64](y, foldr(f, st, ys)) }}

def len(l : List[i64]) : i64 {
    l.case[i64] { Nil => 0,
                  Cons(x,xs) => 1 + len(xs) }}

def main() : i64  { println_i64(len(Cons(1 + 2, Cons(2, Cons(3, Cons(4, Nil))))));
                    0 }
