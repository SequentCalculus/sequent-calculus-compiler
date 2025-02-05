def tak(x:i64,y:i64,z:i64) : i64 {
  if y<x{
    tak(tak(x-1,y,z),
      tak(y-1,z,x),
      tak(z-1,x,y))
  }else{
    z
  }
}

def main(x:i64,y:i64,z:i64):i64 {
  tak(x,y,z)
}
