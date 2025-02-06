data Unit { Unit }
data ListUnit { Nil, Cons(x:Unit,xs:ListUnit) }

def create_n_loop(n:i64,acc:ListUnit) : ListUnit {
  if n==0{
    acc
  } else {
    create_n_loop(n-1,Cons(Unit,acc))
  }
}

def create_n(n:i64) : ListUnit { 
  create_n_loop(n,Nil)
}

def len_loop(l:ListUnit,acc:i64) : i64 {
  l.case{
    Nil => acc,
    Cons(u:Unit,us:ListUnit) => len_loop(us,acc+1)
  }
}

def len(l:ListUnit) : i64 {
  len_loop(l,0)
}

def rec_div2(l:ListUnit) : ListUnit {
  l.case { 
    Nil => Nil, 
    Cons(u:Unit,us:ListUnit) => us.case {
      Nil => Nil, // should raise a runtime error  
      Cons(u:Unit,us:ListUnit) => Cons(Unit,rec_div2(us))
  }}
}

def main(n:i64) : i64 {
  len(rec_div2(create_n(n)))
}
