data Unit { Unit }
data List[A] { Nil, Cons(x:A,xs:List[A]) }

def create_n_loop(n:i64,acc:List[Unit]) : List[Unit] {
  if n==0{
    acc
  } else {
    create_n_loop(n-1,Cons(Unit,acc))
  }
}

def create_n(n:i64) : List[Unit] { 
  create_n_loop(n,Nil)
}

def len_loop(l:List[Unit],acc:i64) : i64 {
  l.case[Unit]{
    Nil => acc,
    Cons(u,us) => len_loop(us,acc+1)
  }
}

def len(l:List[Unit]) : i64 {
  len_loop(l,0)
}

def rec_div2(l:List[Unit]) : List[Unit] {
  l.case[Unit] { 
    Nil => Nil, 
    Cons(u,us) => us.case[Unit] {
      Nil => Nil, // should raise a runtime error  
      Cons(u,us) => Cons(Unit,rec_div2(us))
  }}
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==0{
    0
  }else{
    let res : i64 = len(rec_div2(create_n(n)));
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64 {
  main_loop(iters,n)
}
