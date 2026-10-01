data List[A] { Nil, Cons(x: A, xs: List[A]) }
codata Fun[A, B] { apply(x: A): B }

def map(f: Fun[i64, i64] , l: List[i64]): List[i64] {
    l.case[i64] { Nil => Nil,
                  Cons(x, xs) => Cons(f.apply[i64, i64](x), map(f, xs)) } }

def len(l: List[i64]): i64 {
    l.case[i64] { Nil => 0,
                  Cons(x,xs) => 1 + len(xs) }}

codata Fun2[A, B, C] { apply2(x: A, y: B): C }

def foldr(f: Fun2[i64, i64, i64], st: i64, l: List[i64]): i64 {
    l.case[i64] { Nil => st,
                  Cons(y , ys) => f.apply2[i64, i64, i64](y, foldr(f, st, ys)) }}

def mult(l: List[i64]): i64 { foldr(new { apply2(x, y) => x * y }, 1, l) }

def main(): i64 {
  let l: List[i64] = Cons(1 + 2, Cons(2, Cons(3, Cons(4, Nil))));
  let x: i64 = if len(l) > 0 {
    len(l) + 1
  } else {
    0
  };
  let l1: List[i64] = l.case[i64] {
    Nil => Nil,
    Cons(z, zs) => map(new { apply(n) => (x + n) - z }, zs),
  };
  println_i64(mult(l1));
  0
}
