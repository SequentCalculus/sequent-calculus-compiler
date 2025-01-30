data ListI64 { Nil, Cons(x:i64,xs:ListI64) }

// this should be n<=m or m>n
def interval_list(m:i64,n:i64) : ListI64 := ifl(n,m,Nil,Cons(m, interval_list(m+1,n)));

// Top-level definition instead of local let binding since we don't have term-level recursion
def remove_multiples(n:i64,l:ListI64) : ListI64 := l.case{
  Nil => Nil,
  Cons(x:i64,xs:ListI64) => ifz(x % n,remove_multiples(n,xs),Cons(x,remove_multiples(n,xs)))
};

def sieve(l:ListI64) : ListI64 := l.case{
  Nil => Nil,
  Cons(x:i64, xs:ListI64) => Cons(x,sieve((remove_multiples(x,xs))))
};

def len(l : ListI64) : i64 := l.case { Nil => 0,
             Cons(x:i64,xs:ListI64) => 1 + len(xs) 
};

def main(n:i64) : i64 := let x : ListI64 = sieve(interval_list(2,n)) in 0;
