codata Fun { apply(x: i64): i64 }

def cps_tak(x: i64, y: i64, z: i64, k: Fun): i64 {
  if x <= y {
    k.apply(z)
  } else {
    cps_tak(x - 1, y, z, new { apply(v1) =>
      cps_tak(y - 1, z, x, new { apply(v2) =>
        cps_tak(z - 1, x, y, new { apply(v3) =>
          cps_tak(v1, v2, v3, k)
        })
      })
    })
  }
}

def tak(x: i64, y: i64, z: i64): i64 {
  cps_tak(x, y, z, new { apply(a) => a })
}

def main_loop(iters: i64, x: i64, y: i64, z: i64): i64 {
  let res: i64 = tak(x, y, z);
  if iters == 1 {
    println_i64(res);
    0
  } else {
    main_loop(iters - 1, x, y, z)
  }
}

def main(iters: i64, x: i64, y: i64, z: i64): i64 {
  main_loop(iters, x, y, z)
}
