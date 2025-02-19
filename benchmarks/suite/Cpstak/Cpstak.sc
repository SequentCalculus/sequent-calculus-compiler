codata Fun { Apply(x: i64): i64 }

def tak(x: i64, y: i64, z: i64, k: Fun): i64 {
  if x <= y {
    k.Apply(z)
  } else {
    tak(x - 1, y, z, new { Apply(v1) =>
      tak(y - 1, z, x, new { Apply(v2) =>
        tak(z - 1, x, y, new { Apply(v3) =>
          tak(v1, v2, v3, k)
        })
      })
    })
  }
}

def cps_tak(x: i64, y: i64, z: i64): i64 {
  tak(x, y, z, new { Apply(a) => a })
}

def main_loop(iters: i64, x: i64, y: i64, z: i64): i64 {
  if iters == 0 {
    0
  } else {
    let res: i64 = tak(x, y, z);
    main_loop(iters - 1, x, y, z)
  }
}

def main(iters: i64, x: i64, y: i64, z: i64): i64 {
  main_loop(iters, x, y, z)
}
