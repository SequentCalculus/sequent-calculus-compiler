def sudan(n:i64,x:i64,y:i64, k:cns i64) : i64 := ifz(n,goto(x+y;k),
  ifz(y,goto(x;k),
    let inner : i64 = label a { sudan(n,x,y-1,a) } 
    in sudan(n-1,inner,inner+y,k)));

def main(n:i64,x:i64,y:i64) : i64 := label a { sudan(n,x,y,a) };
