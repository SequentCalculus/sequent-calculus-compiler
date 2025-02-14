data List[A] { Nil, Cons(a:A,as:List[A]) }
codata Fun[A,B] { Ap(a:A) : B }

def merge(l1:List[i64],l2:List[i64]) : List[i64] {
  l1.case[i64] {
    Nil => l2,
    Cons(x1,xs1) => l2.case[i64]{
      Nil => l1,
      Cons(x2,xs2) => 
        if x1<=x2 {
          Cons(x1,merge(xs1,l2))
        } else { 
          Cons(x2,merge(l1,xs2))
        }
    },
  }
}

def tabulate_loop(n:i64,len:i64,f:Fun[i64,i64]) : List[i64]{
  if n==len{
    Nil
  }else{
    Cons(f.Ap[i64,i64](n),tabulate_loop(n+1,len,f))
  }
}

def tabulate(n:i64, f:Fun[i64,i64]) : List[i64] { 
  if n < 0 {
    Nil // this should raise a runtime error
  } else { 
    tabulate_loop(0,n,f)
  }
}

def head(l:List[i64]) : i64 {
  l.case[i64] { 
    Nil => -1, // should raise a runtime error 
    Cons(x,xs) => x 
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==0{
    0
  } else{
    let l1: List[i64] = tabulate(n,cocase{Ap(x) => 2*x});
    let l2: List[i64] = tabulate(n,cocase{Ap(x) => (2*x)+1});
    let res: List[i64] = merge(l1,l2);
    main_loop(iters-1,n)
  }
}

def main(iters:i64, n:i64) : i64 {
  main_loop(iters,n)
}
