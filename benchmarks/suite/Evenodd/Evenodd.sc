data Bool { True, False }

def even_gz(n:i64) : Bool { 
  if n==0 {
    True
  } else {
    odd_gz(n-1)
  }
}

def odd_gz(n:i64) : Bool { 
  if n==0 {
    False
  } else {
    even_gz(n-1)
  }
}

def abs_int(n:i64) : i64 {
  if n<0 {
    -1*n
  } else {
    n
  }
}

def even(n:i64) : Bool { 
  even_gz(abs_int(n))
}

def odd(n:i64) : Bool {
  odd_gz(abs_int(n))
}

def and_not(b1:Bool,b2:Bool) : Bool {
  b1.case{
    False => False,
    True => b2.case { 
      True => False,
      False => True
    }
  }
}

def main_loop(iters:i64,n:i64) : i64 {
  if iters==0{
    0
  }else{
    let res : Bool = and_not(even(n),odd(n));
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64 {
  main_loop(iters,n)
}
