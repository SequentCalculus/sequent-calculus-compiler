def sudan(n:i64,x:i64,y:i64, k:cns i64) : i64 {
  if n==0{
    goto(x+y;k)
  }else{

    if y==0{
      goto(x;k) 
    } else {
      let inner : i64 = label a { sudan(n,x,y-1,a) };
      sudan(n-1,inner,inner+y,k)
    }
  }
}

def main(n:i64,x:i64,y:i64) : i64 {
  label a { sudan(n,x,y,a) }
}
