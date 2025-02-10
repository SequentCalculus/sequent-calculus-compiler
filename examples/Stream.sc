codata Stream[A] { Hd : A, Tl : Stream[A] }
data List[A] { Nil, Cons(x: A, xs: List[A]) }

def repeat(x: i64) : Stream[i64] { cocase { Hd => x, Tl => repeat(x) } }
def const1() : Stream[i64] { cocase { Hd => 1, Tl => const1() } }

def take(n:i64,x:Stream[i64]) : List[i64] { if n == 0 {Nil} else {Cons(x.Hd[i64],take(n-1,x.Tl[i64]))} }
def sumList(ls:List[i64]) : i64 { ls.case[i64] { Nil=>0, Cons(x:i64, xs:List[i64]) => x+(sumList(xs)) } }

def main() : i64 { println_i64(sumList(take(5,repeat(5))));
                   0 }
