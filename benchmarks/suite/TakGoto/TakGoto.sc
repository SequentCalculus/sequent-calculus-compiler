def tak(x:i64,y:i64,z:i64,k:cns i64) : i64 {
  if y<x{
    tak(label a { tak(x-1,y,z,a) },
      label b { tak(y-1,z,x,b) },
      label c {tak(z-1,x,y,c)},k)
  } else {
    goto(z;k)
  }
}

def main(x:i64,y:i64,z:i64):i64 {
  label a { tak(x,y,z,a) }
}
