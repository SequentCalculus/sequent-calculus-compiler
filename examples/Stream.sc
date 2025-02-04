codata StreamI64 { Hd : i64, Tl : StreamI64 }
data ListI64 { Nil, Cons(x:i64,xs:ListI64) }

def repeat(x: i64) : StreamI64 { cocase { Hd => x, Tl => repeat(x) } }
def const1() : StreamI64 { cocase { Hd => 1, Tl => const1() } }

def take(n:i64,x:StreamI64) : ListI64 { if n == 0 {Nil} else {Cons(x.Hd,take(n-1,x.Tl))} }
def sumList(ls:ListI64) : i64 { ls.case { Nil=>0, Cons(x:i64, xs:ListI64) => x+(sumList(xs)) } }

def main() : i64 { println_i64(sumList(take(5,repeat(5))));
                   0 }
