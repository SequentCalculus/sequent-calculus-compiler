codata Stream[A] { head : A, tail : Stream[A] }
data List[A] { Nil, Cons(x: A, xs: List[A]) }

def repeat(x: i64) : Stream[i64] { new { tail => repeat(x), head => x } }
def const1() : Stream[i64] { new { head => 1, tail => const1() } }

def take(n:i64,x:Stream[i64]) : List[i64] { if 0 >= n {Nil} else {Cons(x.head[i64],take(n-1,x.tail[i64]))} }
def sumList(ls:List[i64]) : i64 { ls.case[i64] { Nil=>0, Cons(x, xs) => x+(sumList(xs)) } }

def main(n: i64) : i64 { println_i64(sumList(take(n,repeat(n))));
                   0 }
