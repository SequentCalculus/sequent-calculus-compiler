def tak(x: i64, y: i64, z: i64): i64 {
  if y < x {
    tak(
      tak(x - 1, y, z),
      tak(y - 1, z, x),
      tak(z - 1, x, y))
  } else {
    z
  }
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
