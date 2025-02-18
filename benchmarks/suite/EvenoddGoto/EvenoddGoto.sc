data Bool{True,False}

def even_gz(i:i64) : Bool {
  if i==0 {
    True
  } else {
    label k { odd_gz(i-1,k) }
  }
}

def odd_gz(i:i64,k:cns Bool) : Bool {
  if i==0 { 
    return False to k
  } else {
    return even_gz(i-1) to k
  }
}

def abs_int(i:i64) : i64 { 
  if i<0 {
    -1*i
  } else {
    i
  }
}

def even(i:i64) : Bool {
  even_gz(abs_int(i))
}

def odd(i:i64) : Bool {
  label a { odd_gz(abs_int(i),a) }
}

def and(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => b2,
    False => False
  }
}

def not(b:Bool) : Bool{
  b.case{
    True=>False,
    False=>True
  }
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==0{
    0
  }else{
    let res : Bool = and(even(n),not(odd(n)));
    main_loop(iters-1,n)
  }
}

def main(iters:i64, n:i64) : i64 {
  main_loop(iters,n)
}
