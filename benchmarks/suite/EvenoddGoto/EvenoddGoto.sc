data Bool{True,False}

def even_prime(i:i64) : Bool := ife(i,0,True, label k { odd_prime(i-1,k) });

def odd_prime(i:i64,k:cns Bool) : Bool := ife(i,0,goto(False;k),goto(even_prime(i-1);k));

def abs_int(i:i64) : i64 := ifl(i,0,-1*i,i);

def even(i:i64) : Bool := even_prime(abs_int(i));
def odd(i:i64) : Bool  := label a { odd_prime(abs_int(i),a) };

def and_not(b1:Bool,b2:Bool) : i64 := b1.case{
  False => -1,
  True => b2.case { 
    True => -1,
    False => 0
  }
};

def main(n:i64) : i64 := and_not(even(n),odd(n));
