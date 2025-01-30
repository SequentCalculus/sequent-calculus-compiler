data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
data Bool { True, False }

def list_n(n:i64) : ListI64 := ifz(n,Nil,Cons(n,list_n(n-1)));

def shorterp(x:ListI64,y:ListI64) : Bool := y.case {
  Nil => False,
  Cons(y:i64,ys:ListI64) => x.case {
    Nil => True,
    Cons(x:i64,xs:ListI64) => shorterp(xs,ys)
  }
};

def tail(l:ListI64) : ListI64 := l.case{Nil=>Nil,Cons(x:i64,xs:ListI64) => xs};

def mas(x:ListI64,y:ListI64,z:ListI64) : ListI64 := shorterp(y,x).case{
  False => z,
  True => mas(
    mas(tail(x),y,z),
    mas(tail(y),z,x),
    mas(tail(z),x,y))
};

def len(l : ListI64) : i64 :=
    l.case { Nil => 0,
             Cons(x:i64,xs:ListI64) => 1 + len(xs) };

def main(x:i64,y:i64,z:i64) : i64 := len(mas(list_n(x),list_n(y),list_n(z)));
