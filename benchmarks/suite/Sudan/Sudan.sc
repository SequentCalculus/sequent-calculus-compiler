def sudan(n:i64,x:i64,y:i64) : i64 {
  if n==0 {
    x+y
  }else{
    if y==0{
      x
    }else{
      let inner : i64 = sudan(n,x,y-1);
      sudan(n-1,inner,inner + y)
    }
  }
}

def main(n:i64,x:i64,y:i64) : i64 {
  sudan(n,x,y)
}
