data List[A] { Nil, Cons(a: A, as: List[A]) }
data Bool { True, False }
data Either[A, B] { Left(a: A), Right(b: B) }
codata Fun2[A, B, C] { apply2(a: A,b: B): C }

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

def append(l1:List[Either[i64,Bool]],l2:List[Either[i64,Bool]]) : List[Either[i64,Bool]]{
  l1.case[Either[i64,Bool]] {
    Nil => l2,
    Cons(e,es) => Cons(e,append(es,l2))
  }
}


def apply_op_inner(
  ls: List[i64], a: i64,
  op: Fun2[i64, i64, Either[i64, Bool]],
): List[Either[i64, Bool]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(b, t2) =>
      Cons(op.apply2[i64, i64, Either[i64, Bool]](a, b), apply_op_inner(t2, a, op))
  }
}

def apply_op(
  ls: List[i64],
  astart: i64, astep: i64, alim: i64,
  op: Fun2[i64, i64, Either[i64, Bool]]
): List[Either[i64, Bool]] {
  ls.case[i64] {
    Nil => Nil,
    Cons(a, t1) => 
      append(
        apply_op_inner(enum_from_then_to(astart, astart + astep, alim), a, op),
        apply_op(t1,astart,astep,alim,op)
      )
  }
}

def integerbench(
  op: Fun2[i64, i64, Either[i64, Bool]],
  astart: i64, astep: i64, alim: i64,
): List[Either[i64, Bool]] {
  apply_op(enum_from_then_to(astart, astart + astep, alim), astart, astep, alim, op)
}


def runbench(
  jop: Fun2[i64, i64, Either[i64, Bool]],
  astart: i64, astep: i64, alim: i64,
): List[Either[i64, Bool]] {
  let res1: List[Either[i64, Bool]] = integerbench(jop, astart, astep, alim);
  integerbench(jop, astart, astep, alim)
}

def runalltests(astart: i64, astep: i64, alim: i64): List[Either[i64, Bool]] {
  let z_add: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Left(a + b) };
  let z_sub: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Left(a - b) };
  let z_mul: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Left(a * b) };
  let z_div: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Left(a / b) };
  let z_mod: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Left(a % b) };
  let z_equal: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Right(eq(a, b)) };
  let z_lt: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Right(lt(a, b)) };
  let z_leq: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Right(leq(a, b)) };
  let z_gt: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Right(gt(a, b)) };
  let z_geq: Fun2[i64, i64, Either[i64, Bool]] = new { apply2(a, b) => Right(geq(a, b)) };

  let add: List[Either[i64, Bool]] = runbench(z_add, astart, astep, alim);
  let sub: List[Either[i64, Bool]] = runbench(z_sub, astart, astep, alim);
  let mul: List[Either[i64, Bool]] = runbench(z_mul, astart, astep, alim);
  let div: List[Either[i64, Bool]] = runbench(z_div, astart, astep, alim);
  let mod: List[Either[i64, Bool]] = runbench(z_mod, astart, astep, alim);
  let equal: List[Either[i64, Bool]] = runbench(z_equal, astart, astep, alim);
  let lt: List[Either[i64, Bool]] = runbench(z_lt,  astart, astep, alim);
  let leq: List[Either[i64, Bool]] = runbench(z_leq,  astart, astep, alim);
  let gt: List[Either[i64, Bool]] = runbench(z_gt,  astart, astep, alim);
  runbench(z_geq, astart, astep, alim)
}

def test_integer_nofib(n: i64): List[Either[i64, Bool]] {
  runalltests(-2100000000, n, 2100000000)
}

def head(l: List[Either[i64, Bool]]): Either[i64,Bool] {
  l.case[Either[i64, Bool]] {
    Nil => Left(-1),
    Cons(e, es) => e
  }
}

def print_either(e:Either[i64,Bool]) : i64 {
  e.case[i64,Bool] {
    Left(i) => println_i64(i); 0,
    Right(b) => b.case{
      True => print_i64(1);
      println_i64(1);
      0,
      False => print_i64(0);
      println_i64(0);
      0
    }
  }
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: List[Either[i64, Bool]] = test_integer_nofib(n);
    print_either(head(res))
  } else {
    let res: List[Either[i64, Bool]] = test_integer_nofib(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
}
