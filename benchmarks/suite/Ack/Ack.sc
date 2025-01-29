def ack(m:i64,n:i64) : i64 := 
  ifz(m,n+1, 
    ifz(n, ack(m-1,1), 
      ack(m-1, ack(m,n-1))));

def main(m:i64,n:i64) : i64 := ack(m,n);
