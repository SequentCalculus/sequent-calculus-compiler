data ListI64 { Nil, Cons(x:i64,xs:ListI64) }
data PairListI64 { MkPair(i:i64,l:ListI64) }
data OptionI64 { None, Some(i:i64) }

def len(l : ListI64) : i64 :=
    l.case { Nil => 0,
             Cons(x:i64,xs:ListI64) => 1 + len(xs) };

// this could probably be done faster if we had a direct way to index lists
def at(l:ListI64,ind:i64) : OptionI64 := l.case {
  Nil => None,
  Cons(x:i64,xs:ListI64) => ifz(ind,x,at(xs,ind-1))
};


def partition(l:ListI64, lo:i64,hi:i64) : PairListI64 := 
  let p = at(l,hi);

def quicksort(l:ListI64,lo:i64,hi:i64) : ListI64 := ifl(len(l),2,l,
  let piv = partition(l)
