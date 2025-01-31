data ListI64 { Nil, Cons(x: i64, xs: ListI64) }

def isEmpty(xs: ListI64): i64 { xs.case { Nil => 0, Cons(x: i64,xs: ListI64) => 1 } }

def safeHead(xs: ListI64): i64 { xs.case { Nil => 0, Cons(y: i64, ys: ListI64) => y}}
