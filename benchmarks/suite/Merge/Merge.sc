data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
codata FunI64I64 { Ap(x:i64) : i64 }

def merge(l1:ListI64,l2:ListI64) : ListI64 {
  l1.case {
    Nil => l2,
    Cons(x1:i64,xs1:ListI64) => l2.case{
      Nil => l1,
      Cons(x2:i64,xs2:ListI64) => 
        if x1<=x2 {
          Cons(x1,merge(xs1,l2))
        } else { 
          Cons(x2,merge(l1,xs2))
        }
    },
  }
}

def tabulate_loop(n:i64,len:i64,f:FunI64I64) : ListI64{
  if n==len{
    Nil
  }else{
    Cons(f.Ap(n),tabulate_loop(n+1,len,f))
  }
}

def tabulate(n:i64, f:FunI64I64) : ListI64 { 
  if n < 0 {
    Nil // this should raise a runtime error
  } else { 
    tabulate_loop(0,n,f)
  }
}

def head(l:ListI64) : i64 {
  l.case { 
    Nil => -1, // should raise a runtime error 
    Cons(x:i64,xs:ListI64) => x 
  }
}

def main(n:i64) : i64 {
  let l1 : ListI64 = tabulate(n,cocase{Ap(x:i64) => 2*x});
  let l2 : ListI64 = tabulate(n,cocase{Ap(x:i64) => (2*x)+1});
  head(merge(l1,l2))
}
