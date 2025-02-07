data ListI64 { Nil, Cons(x:i64,xs:ListI64) }

def interval_list(m:i64,n:i64) : ListI64 {
  if n<=m{
    Nil
  }else {
    Cons(m, interval_list(m+1,n))
  }
}

def remove_multiples(n:i64,l:ListI64) : ListI64 {
  l.case{
    Nil => Nil,
    Cons(x:i64,xs:ListI64) => 
      if x % n==0{
        remove_multiples(n,xs)
      } else {
        Cons(x,remove_multiples(n,xs))
      }
  }
}

def sieve(l:ListI64) : ListI64 {
  l.case{
    Nil => Nil,
    Cons(x:i64, xs:ListI64) => Cons(x,sieve((remove_multiples(x,xs))))
  }
}

def len(l : ListI64) : i64 {
  l.case { Nil => 0,
    Cons(x:i64,xs:ListI64) => 1 + len(xs) 
  }
}

def main(n:i64) : i64 {
  let x : ListI64 = sieve(interval_list(2,n));
  0
}
