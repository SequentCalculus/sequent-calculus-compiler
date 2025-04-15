data List[A] { Nil, Cons(a: A, as: List[A]) }
data Bool { True, False }
data Either[A, B] { Left(a: A), Right(b: B) }
codata Fun2[A, B, C] { Apply2(a: A,b: B): C }

def eq(i1: i64, i2: i64): Bool {
  if i1 == i2 {
    True
  } else {
    False
  }
}

def lt(i1: i64, i2: i64): Bool {
  if i1 < i2 {
    True
  } else {
    False
  }
}

def leq(i1: i64, i2: i64): Bool {
  if i1 <= i2 {
    True
  } else {
    False
  }
}

def gt(i1: i64, i2: i64): Bool {
  lt(i2, i1)
}

def geq(i1: i64, i2: i64): Bool {
  leq(i2, i1)
}

def enum_from_then_to(from: i64, then: i64, t: i64): List[i64] {
  if from <= t {
    Cons(from, enum_from_then_to(then, (2 * then) - from, t))
  } else {
    Nil
  }
}


def bench_lscomp2(
  ls: List[i64], t1: List[i64], a: i64,
  op: Fun2[i64, i64, Either[i64, Bool]],
  bstart: i64, bstep: i64, blim: i64
): List[Either[i64, Bool]] {
  ls.case[i64] {
    Nil => bench_lscomp1(t1, bstart, bstep, blim, op),
    Cons(b, t2) =>
      Cons(op.Apply2[i64, i64, Either[i64, Bool]](a, b), bench_lscomp2(t2, t1, a, op, bstart, bstep, blim))
  }
}

def bench_lscomp1(
  ls: List[i64],
  bstart: i64, bstep: i64, blim: i64,
  op: Fun2[i64, i64, Either[i64, Bool]]
): List[Either[i64, Bool]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(a, t1) => bench_lscomp2(enum_from_then_to(bstart, bstart + bstep, blim), t1, a, op, bstart, bstep, blim)
  }
}

def integerbench(
  op: Fun2[i64, i64, Either[i64, Bool]],
  astart: i64, astep: i64, alim: i64,
  bstart: i64, bstep: i64, blim: i64
): List[Either[i64, Bool]] {
  bench_lscomp1(enum_from_then_to(astart, astart + astep, alim), bstart, bstep, blim, op)
}


def runbench(
  jop: Fun2[i64, i64, Either[i64, Bool]], iop: Fun2[i64, i64, Either[i64, Bool]],
  astart: i64, astep: i64, alim: i64,
  bstart: i64, bstep: i64, blim: i64
): List[Either[i64, Bool]] {
  let res1: List[Either[i64, Bool]] = integerbench(iop, astart, astep, alim, astart, astep, alim);
  integerbench(jop, astart, astep, alim, astart, astep, alim)
}

def runalltests(astart: i64, astep: i64, alim: i64, bstart: i64, bstep: i64, blim: i64): List[Either[i64, Bool]] {
  let z_add: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Left(a + b) };
  let z_sub: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Left(a - b) };
  let z_mul: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Left(a * b) };
  let z_div: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Left(a / b) };
  let z_mod: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Left(a % b) };
  let z_equal: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Right(eq(a, b)) };
  let z_lt: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Right(lt(a, b)) };
  let z_leq: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Right(leq(a, b)) };
  let z_gt: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Right(gt(a, b)) };
  let z_geq: Fun2[i64, i64, Either[i64, Bool]] = new { Apply2(a, b) => Right(geq(a, b)) };

  let add: List[Either[i64, Bool]] = runbench(z_add, new { Apply2(a, b) => Left(a + b) }, astart, astep, alim, astart, astep, alim);
  let sub: List[Either[i64, Bool]] = runbench(z_sub, new { Apply2(a, b) => Left(a - b) }, astart, astep, alim, astart, astep, alim);
  let mul: List[Either[i64, Bool]] = runbench(z_mul, new { Apply2(a, b) => Left(a * b) }, astart, astep, alim, astart, astep, alim);
  let div: List[Either[i64, Bool]] = runbench(z_div, new { Apply2(a, b) => Left(a / b) }, astart, astep, alim, astart, astep, alim);
  let mod: List[Either[i64, Bool]] = runbench(z_mod, new { Apply2(a, b) => Left(a % b) }, astart, astep, alim, astart, astep, alim);
  let equal: List[Either[i64, Bool]] = runbench(z_equal, new { Apply2(a,b) => Right(eq(a, b)) }, astart, astep, alim, astart, astep, alim);
  let lt: List[Either[i64, Bool]] = runbench(z_lt, new { Apply2(a,b) => Right(lt(a, b)) }, astart, astep, alim, astart, astep, alim);
  let leq: List[Either[i64, Bool]] = runbench(z_leq, new { Apply2(a,b) => Right(leq(a, b)) }, astart, astep, alim, astart, astep, alim);
  let gt: List[Either[i64, Bool]] = runbench(z_gt, new { Apply2(a,b) => Right(gt(a, b)) }, astart, astep, alim, astart, astep, alim);
  runbench(z_geq, new { Apply2(a,b) => Right(geq(a, b)) }, astart, astep, alim, astart, astep, alim)
}

def test_integer_nofib(n: i64): List[Either[i64, Bool]] {
  runalltests(-2100000000, n, 2100000000, -2100000000, n, 2100000000)
}

def head(l: List[Either[i64, Bool]]): i64 {
  l.case[Either[i64, Bool]] {
    Nil => -1,
    Cons(e, es) => e.case[i64, Bool] {
      Left(i) => i,
      Right(b) => b.case {
        True => -2,
        False => -3
      }
    }
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: List[Either[i64, Bool]] = test_integer_nofib(n);
    println_i64(head(res));
    0
  } else {
    let res: List[Either[i64, Bool]] = test_integer_nofib(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
