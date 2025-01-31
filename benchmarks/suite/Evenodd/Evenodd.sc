data Bool { True, False }

def even_gz(n:i64) : Bool := ifz(n,True, odd_gz(n-1));

def odd_gz(n:i64) : Bool := ifz(n,False,even_gz(n-1));

def abs(n:i64) : i64 := ifl(n,0,-1*n,n);

def even(n:i64) : Bool := even_gz(abs(n));
def odd(n:i64) : Bool := odd_gz(abs(n));

def and_not(b1:Bool,b2:Bool) : i64 := b1.case{
  False => -1,
  True => b2.case { 
    True => -1,
    False => 0
  }
};

def main(n:i64) : i64 := and_not(even(n),odd(n));
