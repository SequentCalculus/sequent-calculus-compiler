data Bool { True, False }

def and(b1: Bool, b2: Bool): Bool {
  b1.case {
    True => b2,
    False => False
  }
}

def not(b: Bool): Bool {
  b.case {
    True => False,
    False => True
  }
}

def abs_i(n: i64): i64 {
  if n < 0 { - 1 * n } else { n }
}

def even_abs(n: i64): Bool {
  if n == 0 { True } else { odd_abs(n - 1) }
}

def odd_abs(n: i64): Bool {
  if n == 0 { False } else { even_abs(n - 1) }
}

def even(n: i64): Bool {
  even_abs(abs_i(n))
}

def odd(n: i64): Bool {
  odd_abs(abs_i(n))
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: Bool = and(even(n), not(odd(n)));
    res.case {
      True => println_i64(1);
              0,
      False => println_i64(0);
               0
    }
  } else {
    let res: Bool = and(even(n), not(odd(n)));
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
