data Bool { True, False }

def not(b: Bool): Bool {
  b.case {
    True => False,
    False => True
  }
}

def abs_i(i: i64): i64 {
  if i < 0 { - 1 * i } else { i }
}

def even_abs(i: i64): Bool {
  if i == 0 {
    True
  } else {
    label k { odd_abs(i - 1, k) }
  }
}

def odd_abs(i: i64, k:cns Bool): Bool {
  if i == 0 {
    goto k (False)
  } else {
    goto k (even_abs(i - 1))
  }
}

def even(i: i64): Bool {
  even_abs(abs_i(i))
}

def odd(i: i64): Bool {
  label k { odd_abs(abs_i(i), k) }
}

def main_loop(iters: i64, n: i64): i64 {
  let res: Bool = even(n).case {
    True => not(odd(n)),
    False => False
  };
  if iters == 1 {
    res.case {
      True => println_i64(1);
              0,
      False => println_i64(0);
               0
    }
  } else {
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
