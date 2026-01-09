data List[A] { Nil, Cons(x:A,xs:List[A]) }
data Pair[A,B] { Tup(a:A,b:B) }
codata Fun[A,B] { apply(a:A): B }

def map[A,B](ls:List[A],f:Fun[A,B]): List[B]{
  ls.case[A]{
    Nil => Nil,
    Cons(a,b) => Cons(f.apply[A,B](a),map[A,B](b,f))
  }
}

def main(): i64 {
  let ls1: List[i64] = Cons(1,Cons(2,Cons(3,Nil)));
  let ls2: List[Pair[i64,i64]] = Cons(Tup(1,2),Cons(Tup(3,4),Cons(Tup(5,6),Nil)));
  let f1: Fun[i64,i64] = new { apply(a) => a + 1 };
  let f2: Fun[Pair[i64,i64],i64] = new { 
    apply(a) => a.case[i64,i64] {
      Tup(a,b) => a + b
    }
  };
  let ls1_mapped: List[i64] = map[i64,i64](ls1,f1);
  let ls2_mapped: List[i64] = map[Pair[i64,i64],i64](ls2,f2);
  0
}
