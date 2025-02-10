data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
data Bool { True, False }

def not(b:Bool) : Bool{
  b.case{
    True => False,
    False => True
  }
}

def and(b1:Bool,b2:Bool):Bool{
  b1.case{
    True => b2,
    False => False
  }
}

def or(b1:Bool,b2:Bool):Bool{
  b1.case{
    True => True,
    False => b2
  }
}

def list_n(n:i64) : ListI64 {
  if n==0{
    Nil
  } else { 
    Cons(n,list_n(n-1))
  }
}

def null(x:ListI64) : Bool{ 
  x.case{
    Nil => True,
    Cons(x:i64,xs:ListI64) => False
  }
}

def tail(x:ListI64) : ListI64{
  x.case{
    Nil => Nil, // should give a runtime error
    Cons(x:i64,xs:ListI64) => xs
  }
}

def shorterp(x:ListI64,y:ListI64) : Bool {
  and(not(null(y)),or(null(x),shorterp(tail(x),tail(y))))
}

def mas(x:ListI64,y:ListI64,z:ListI64) : ListI64 {
  not(shorterp(y,x)).case{
    True => z,
    False => mas(
      mas(tail(x),y,z),
      mas(tail(y),z,x),
      mas(tail(z),x,y))
  }
}

def len(l : ListI64) : i64 {
  l.case { Nil => 0,
  Cons(x:i64,xs:ListI64) => 1 + len(xs) }
}

def main_loop(iters:i64,x:i64,y:i64,z:i64) : i64{
  if iters==0{
    0
  }else{
    let res : i64 = len(mas(list_n(x),list_n(y),list_n(z)));
    main_loop(iters-1,x,y,z)
  }
}

def main(iters:i64,x:i64,y:i64,z:i64) : i64 {
  main_loop(iters,x,y,z)
}
