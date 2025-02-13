codata Stream[A] { Hd : A, Tl : Stream[A] }
data List[A] { Nil, Cons(x: A, xs: List[A]) }

def repeat(x: i64) : Stream[i64] { cocase { Hd => x, Tl => repeat(x) } }
def const1() : Stream[i64] { cocase { Hd => 1, Tl => const1() } }

def take(n:i64,x:Stream[i64]) : List[i64] { if n == 0 {Nil} else {Cons(x.Hd[i64],take(n-1,x.Tl[i64]))} }
def sumList(ls:List[i64]) : i64 { ls.case[i64] { Nil=>0, Cons(x, xs) => x+(sumList(xs)) } }

def main(n: i64) : i64 { println_i64(sumList(take(n,repeat(n))));
                   0 }
