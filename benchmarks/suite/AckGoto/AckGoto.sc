def ack(m:i64,n:i64, k:cns i64) : i64 := 
  ifz(m,goto(n+1;k),
    ifz(n,ack(m-1,1,k),
      ack(m-1,label a { ack(m,n-1,a) },k)));

def main(n:i64,m:i64) : i64 := label a { ack(n,m,a) };
