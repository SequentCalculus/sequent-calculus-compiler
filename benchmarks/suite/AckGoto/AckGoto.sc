def ack(m:i64,n:i64, k:cns i64) : i64 {
  if m==0 {
    goto(n+1;k)
  } else {
    if n == 0 {
      ack(m-1,1,k)
    } else {
      ack(m-1,label a { ack(m,n-1,a) },k)
    }
  }
}

def main(n:i64,m:i64) : i64 { label a { ack(n,m,a) } }
