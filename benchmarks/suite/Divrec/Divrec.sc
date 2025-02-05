data Unit { Unit }
data ListUnit { Nil, Cons(x:Unit,xs:ListUnit) }

def create_n(n:i64) : ListUnit { 
  if n==0{
    Nil
  }else {
    Cons(Unit,create_n(n-1))
  }
}

def len(l:ListUnit):i64 {
  l.case {
    Nil => 0,
    Cons(u:Unit,us:ListUnit) => 1+len(us)
  }
}

def rec_div2(l:ListUnit) : ListUnit {
  l.case { 
    Nil => Nil, 
    Cons(u:Unit,us:ListUnit) => us.case {
      Nil => Nil, 
      Cons(u:Unit,us:ListUnit) => Cons(Unit,rec_div2(us))
  }}
}

def main(n:i64) : i64 {
  let x : ListUnit = rec_div2(create_n(n));
  0
}
