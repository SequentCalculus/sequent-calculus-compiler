codata FunI64I64 { Ap(x:i64) : i64 }

def cps_tak(x:i64,y:i64,z:i64, k:FunI64I64) : i64 { 
  if x < y { 
    k.Ap(z)
  } else { 
    cps_tak(x-1,y,z, cocase { Ap(v1:i64) => 
      cps_tak(y-1,z,x, cocase { Ap(v2:i64) => 
        cps_tak(z-1,x,y, cocase { Ap(v3:i64) => 
          cps_tak(v1,v2,v3,k)
        })
      })
    })
  }
}

def tak(x:i64,y:i64,z:i64) : i64 { 
  cps_tak(x,y,z,cocase { Ap(a:i64) => a })
}

def main(x:i64,y:i64,z:i64) : i64 { 
  tak(x,y,z)
}
