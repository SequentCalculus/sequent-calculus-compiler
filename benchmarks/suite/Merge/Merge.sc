data ListI64 { Nil, Cons(x:i64,xs:ListI64) }

def merge(l1:ListI64,l2:ListI64) : ListI64 := l1.case {
  Nil => l2,
  Cons(x1:i64,xs1:ListI64) => l2.case{
    Nil => l1,
      Cons(x2:i64,xs2:ListI64) => ifl(x1,x2,Cons(x1,merge(xs1,l2)),Cons(x2,merge(l1,xs2)))
  },
};

def create_n(n:i64) : ListI64 := ifz(n,Nil,Cons(0,create_n(n-1)));

def len(l:ListI64):i64 := l.case {
  Nil => 0,
  Cons(x:i64,xs:ListI64) => 1+len(xs)};

// we again need len to have a return value
// optionally, this should allow two arguments for different lengths
def main(n:i64) : i64 := len(merge(create_n(n),create_n(n)));
