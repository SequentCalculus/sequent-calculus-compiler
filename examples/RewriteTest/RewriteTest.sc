data List[A]{ Nil,Cons(x:A,xs:List[A]) }

def len(l:List[i64]) : i64{
  l.case[i64]{
    Nil => 0,
    Cons(x,xs) => 1 + len(xs)
  }
}

def main() : i64{
  let l: List[i64] = Cons(1,Cons(2,Nil));
  len(l)
}
