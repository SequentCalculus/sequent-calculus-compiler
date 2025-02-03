data ListI64 { NilI,ConsI(i:i64,is:ListI64) }
data ListListI64 { NilLI,ConsLI(is:ListI64,iss:ListListI64)}
data Bool { True, False }

def and(b1:Bool,b2:Bool) : Bool := b1.case{
  True => b2,
  False => False 
};

def neq_i(i1:i64,i2:i64) : Bool := ife(i1,i2,False,True);

def len_listlist(l:ListListI64) : i64 := l.case{
  NilLI => 0,
  ConsLI(is:ListI64,iss:ListListI64) => 1+len_listlist(iss)
};

def append_listlist(l1:ListListI64,l2:ListListI64) : ListListI64 := l1.case{
  NilLI => l2,
  ConsLI(is:ListI64,iss:ListListI64) => ConsLI(is,append_listlist(iss,l2))
};

def safe(n:i64,m:i64,l:ListI64) : Bool := l.case{
  NilI => True,
  ConsI(i:i64,is:ListI64) => 
    and(neq_i(n,i),
      and(neq_i(n,i+m),
        and(neq_i(n,i-m),
        safe(n,m+1,is))))
};

def check(l:ListListI64,acc:ListListI64,n:i64) : ListListI64 := l.case{
  NilLI => acc,
  ConsLI(is:ListI64,iss:ListListI64) => safe(n,1,is).case{
    True => check(iss, ConsLI(ConsI(n,is),acc),n),
    False => check(iss,acc,n)
  }
};

def enumerate(n:i64,acc:ListListI64) : ListListI64 := ifz(n,acc,
  let bs : ListListI64 = gen(n-1) in 
  let res : ListListI64 = check(bs,NilLI,n) in
  enumerate(n-1,append_listlist(res,acc)));

def gen(n:i64) : ListListI64 := ifz(n,NilLI,enumerate(n,NilLI));

def nsoln(n:i64) : i64 := len_listlist(gen(n));

def loop(n:i64,iters:i64) : i64 := ifz(iters,0,let res : i64 = nsoln(n) in loop(n,iters-1));

def main(n:i64,iters:i64) : i64 := loop(n,iters);
