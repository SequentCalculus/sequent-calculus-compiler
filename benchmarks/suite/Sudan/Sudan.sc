def sudan(n:i64,x:i64,y:i64) : i64 := ifz(n,x+y,
  ifz(y,x,
    let inner : i64 = sudan(n,x,y-1) in 
    sudan(n-1,inner,inner + y)));

def main(n:i64,x:i64,y:i64) : i64 := sudan(n,x,y);
