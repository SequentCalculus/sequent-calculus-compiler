data List[A] { Nil, Cons(a:A,as:List[A])}
data Pair[A,B] { Tup(a:A,b:B) }
codata Fun[A,B] { Ap(a:A) : B}

def revonto(x:List[Pair[i64,i64]], y:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  accumulate( x, y, cocase { Ap(l) => cocase { Ap(a) => Cons(a,Nil) } } )
}

def accumulate(a:List[Pair[i64,i64]],xs:List[Pair[i64,i64]],
  f:Fun[List[Pair[i64, i64]], Fun[Pair[i64, i64], List[Pair[i64, i64]]]]) : List[Pair[i64,i64]] {
    fold(a,xs,f)
}
  def fold(a:List[Pair[i64,i64]],xs:List[Pair[i64,i64]],
    f:Fun[List[Pair[i64, i64]], Fun[Pair[i64, i64], List[Pair[i64, i64]]]]) : List[Pair[i64,i64]] {
      xs.case[Pair[i64,i64]]{
        Nil => a,
        Cons(b,x) => fold(
          f.Ap[List[Pair[i64,i64]],Fun[Pair[i64,i64],List[Pair[i64,i64]]]](a)
            .Ap[Pair[i64,i64],List[Pair[i64,i64]]](b),x,f)
      }
  }


def main():i64 { 0 }
