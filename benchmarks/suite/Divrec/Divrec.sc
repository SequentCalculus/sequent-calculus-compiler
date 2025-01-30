data Unit { Unit }
data ListUnit { Nil, Cons(x:Unit,xs:ListUnit) }

def create_n(n:i64) : ListUnit := ifz(n,Nil,Cons(Unit,create_n(n-1)));

def len(l:ListUnit):i64 := l.case {
  Nil => 0,
  Cons(u:Unit,us:ListUnit) => 1+len(us)};

def rec_div2(l:ListUnit) : ListUnit := l.case { 
  Nil => Nil, 
  Cons(u:Unit,us:ListUnit) => us.case {
      Nil => Nil, 
      Cons(u:Unit,us:ListUnit) => Cons(Unit,rec_div2(us))
}};

// len will slow down the computation, but we need a return value
def main(n:i64) : i64 := len(rec_div2(create_n(n))); 
