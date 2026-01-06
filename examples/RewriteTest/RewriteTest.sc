data List[A]{ Nil,Cons(x:A,xs:List[A]) }
codata Fun[A,B] { apply(a:A) : B }

def map(l:List[i64],f:Fun[i64,i64]): List[i64] {
  l.case[i64]{
    Nil => Nil,
    Cons(x,xs) => Cons(f.apply[i64,i64](x),map(xs,f))
  }
}

def main(): i64 {
  let f: Fun[i64,i64] = new { apply(x) => x + 1 };
  let l: List[i64] = Cons(1,Cons(2,Cons(3,Cons(4,Nil))));
  let res: List[i64] = map(l,f);
  println_i64(1);
  0
}
