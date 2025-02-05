codata FunI64I64 { Ap(x:i64) : i64 }

// this needs to be a toplevel definition as we have no term-level recursion
def lp(i:i64,tot:i64,stop:i64,f:FunI64I64,k:cns i64) : i64 {
  if stop<i{
    goto(tot;k)
  } else {
    lp(i+1,(f.Ap(i))+tot,stop,f,k)
  }
}

def sum(f:FunI64I64,start:i64,stop:i64,k:cns i64) : i64 {
  lp(start,0,stop,f,k)
}

def motz(n:i64,k:cns i64) : i64 {
  if n<2{
    1
  }else{
    let limit : i64 = n-2;
    let product : FunI64I64 = cocase { Ap(i:i64) => label a { motz(i,a) } * label b { motz(limit-i,b) } };
    label a { motz(n-1,a) } + label b {sum(product,0,limit,b)}
  }
}

def main(n:i64) : i64 {
  label k { motz(n,k) }
}
