data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
codata FunI64I64 { Ap(x:i64) : i64 }

def merge(l1:ListI64,l2:ListI64) : ListI64 := l1.case {
  Nil => l2,
  Cons(x1:i64,xs1:ListI64) => l2.case{
    Nil => l1,
      Cons(x2:i64,xs2:ListI64) => ifl(x2,x1,Cons(x2,merge(l1,xs2)),Cons(x1,merge(xs1,l2)))
  },
};

def tabulate(n:i64, f:FunI64I64) : ListI64 := ifz(n,Nil,Cons(f.Ap(n),tabulate(n-1,f)));

def rev(lold:ListI64,lnew:ListI64) : ListI64 := lold.case{
  Nil => lnew,
  Cons(x:i64,xs:ListI64) => rev(xs,Cons(x,lnew))
};

def head(l:ListI64) : i64 := l.case { Nil => -1, Cons(x:i64,xs:ListI64) => x };

def main(n:i64) : i64 := 
  let l1 : ListI64 = rev(tabulate(n,cocase{Ap(x:i64) => 2*x}),Nil) in 
  let l2 : ListI64 = rev(tabulate(n,cocase{Ap(x:i64) => (2*x)+1}),Nil) in 
  head(merge(l1,l2));
