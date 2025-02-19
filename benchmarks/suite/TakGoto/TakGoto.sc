def tak(x: i64, y: i64, z: i64, k: cns i64): i64 {
  if y < x {
    tak(
      label a { tak(x - 1, y, z, a) },
      label b { tak(y - 1, z, x, b) },
      label c { tak(z - 1, x, y, c) },
      k)
  } else {
    return z to k
  }
}

def main_loop(iters: i64, x: i64, y: i64, z: i64): i64 {
  if iters == 0 {
    0
  } else {
    let res: i64 = label a { tak(x, y, z, a) };
    main_loop(iters - 1, x, y, z)
  }
}

def main(iters: i64, x: i64, y: i64, z: i64): i64 {
  main_loop(iters, x, y, z)
}
