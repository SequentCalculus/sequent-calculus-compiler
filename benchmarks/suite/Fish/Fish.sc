data Vec { Vec(x: i64, y: i64) }
data Vec4 { Vec4(x: i64, y: i64, z: i64, w: i64) }
data List[A] { Nil, Cons(a: A, as: List[A]) }
codata Fun[A, B] { apply(a: A): B }
codata Fun3[A, B, C, D] { apply3(a: A, b: B, c: C): D }

def vec_add(v1: Vec, v2: Vec): Vec {
  v1.case {
    Vec(x1, y1) => v2.case {
      Vec(x2, y2) => Vec(x1 + x2, y1 + y2)
    }
  }
}

def vec_sub(v1: Vec, v2: Vec): Vec {
  v1.case {
    Vec(x1, y1) => v2.case {
      Vec(x2, y2) => Vec(x1 - x2, y1 - y2)
    }
  }
}

def scale_vec2(v: Vec, a: i64, b: i64): Vec {
  v.case {
    Vec(x, y) => Vec((x * a) / b, (y * a) / b)
  }
}

def p_tile(): List[Vec4] {
  let p5: List[Vec4] = Cons(Vec4(10, 4, 13, 5), Cons(Vec4(13, 5, 16, 4), Cons(Vec4(11, 0, 14, 2), Cons(Vec4(14, 2, 16, 2), Nil))));
  let p4: List[Vec4] = Cons(Vec4(8, 12, 16, 10), Cons(Vec4(8, 8, 12, 9), Cons(Vec4(12, 9, 16, 8), Cons(Vec4(9, 6, 12, 7), Cons(Vec4(12, 7, 16, 6), p5)))));
  let p3: List[Vec4] = Cons(Vec4(10, 16, 12, 14), Cons(Vec4(12, 14, 16, 13), Cons(Vec4(12, 16, 13, 15), Cons(Vec4(13, 15, 16, 14), Cons(Vec4(14, 16, 16, 15), p4)))));
  let p2: List[Vec4] = Cons(Vec4(4, 13, 0, 16), Cons(Vec4(0, 16, 6, 15), Cons(Vec4(6, 15, 8, 16), Cons(Vec4(8, 16, 12, 12), Cons(Vec4(12, 12, 16, 12), p3)))));
  let p1: List[Vec4] = Cons(Vec4(4, 10, 7, 6), Cons(Vec4(7, 6, 4, 5), Cons(Vec4(11, 0, 10, 4), Cons(Vec4(10, 4, 9, 6), Cons(Vec4(9, 6, 8, 8), Cons(Vec4(8, 8, 4, 13), p2))))));
  let p: List[Vec4] = Cons(Vec4(0, 3, 3, 4), Cons(Vec4(3, 4, 0, 8), Cons(Vec4(0, 8, 0, 3), Cons(Vec4(6, 0, 4, 4), Cons(Vec4(4, 5, 4, 10), p1)))));
  p
}


def q_tile(): List[Vec4] {
  let q7: List[Vec4] = Cons(Vec4(0, 0, 0, 8), Cons(Vec4(0, 12, 0, 16), Nil));
  let q6: List[Vec4] = Cons(Vec4(13, 0, 16, 6), Cons(Vec4(14, 0, 16, 4), Cons(Vec4(15, 0, 16, 2), Cons(Vec4(0, 0, 8, 0), Cons(Vec4(12, 0, 16, 0), q7)))));
  let q5: List[Vec4] = Cons(Vec4(10, 0, 14, 11), Cons(Vec4(12, 0, 13, 4), Cons(Vec4(13, 4, 16, 8), Cons(Vec4(16, 8, 15, 10), Cons(Vec4(15, 10, 16, 16), q6)))));
  let q4: List[Vec4] = Cons(Vec4(4, 5, 4, 7), Cons(Vec4(4, 0, 6, 5), Cons(Vec4(6, 5, 6, 7), Cons(Vec4(6, 0, 8, 5), Cons(Vec4(8, 5, 8, 8), q5)))));
  let q3: List[Vec4] = Cons(Vec4(11, 15, 9, 13), Cons(Vec4(10, 10, 8, 12), Cons(Vec4(8, 12, 12, 12), Cons(Vec4(12, 12, 10, 10), Cons(Vec4(2, 0, 4, 5), q4)))));
  let q2: List[Vec4] = Cons(Vec4(4, 16, 5, 14), Cons(Vec4(6, 16, 7, 15), Cons(Vec4(0, 10, 7, 11), Cons(Vec4(9, 13, 8, 15), Cons(Vec4(8, 15, 11, 15), q3)))));
  let q1: List[Vec4] = Cons(Vec4(0, 12, 3, 13), Cons(Vec4(3, 13, 5, 14), Cons(Vec4(5, 14, 7, 15), Cons(Vec4(7, 15, 8, 16), Cons(Vec4(2, 16, 3, 13), q2)))));
  let q: List[Vec4] = Cons(Vec4(0, 8, 4, 7), Cons(Vec4(4, 7, 6, 7), Cons(Vec4(6, 7, 8, 8), Cons(Vec4(8, 8, 12, 10), Cons(Vec4(12, 10, 16, 16), q1)))));
  q
}

def r_tile(): List[Vec4] {
  let r4: List[Vec4] = Cons(Vec4(11, 16, 12, 12), Cons(Vec4(12, 12, 16, 8), Cons(Vec4(13, 13, 16, 10), Cons(Vec4(14, 14, 16, 12), Cons(Vec4(15, 15, 16, 14), Nil)))));
  let r3: List[Vec4] = Cons(Vec4(2, 2, 8, 0), Cons(Vec4(3, 3, 8, 2), Cons(Vec4(8, 2, 12, 0), Cons(Vec4(5, 5, 12, 3), Cons(Vec4(12, 3, 16, 0), r4)))));
  let r2: List[Vec4] = Cons(Vec4(5, 10, 2, 12), Cons(Vec4(2, 12, 0, 16), Cons(Vec4(16, 8, 12, 12), Cons(Vec4(12, 12, 11, 16), Cons(Vec4(1, 1, 4, 0), r3)))));
  let r1: List[Vec4] = Cons(Vec4(16, 6, 11, 10), Cons(Vec4(11, 10, 6, 16), Cons(Vec4(16, 4, 14, 6), Cons(Vec4(14, 6, 8, 8), Cons(Vec4(8, 8, 5, 10), r2)))));
  let r: List[Vec4] =  Cons(Vec4(0, 0, 8, 8), Cons(Vec4(12, 12, 16, 16), Cons(Vec4(0, 4, 5, 10), Cons(Vec4(0, 8, 2, 12), Cons(Vec4(0, 12, 1, 14), r1)))));
  r
}

def s_tile(): List[Vec4] {
  let s5: List[Vec4] = Cons(Vec4(15, 5, 13, 7), Cons(Vec4(13, 7, 15, 8), Cons(Vec4(15, 8, 15, 5), Nil)));
  let s4: List[Vec4] = Cons(Vec4(15, 9, 16, 8), Cons(Vec4(10, 16, 11, 10), Cons(Vec4(12, 4, 10, 6), Cons(Vec4(10, 6, 12, 7), Cons(Vec4(12, 7, 12, 4), s5)))));
  let s3: List[Vec4] = Cons(Vec4(7, 8, 7, 13), Cons(Vec4(7, 13, 8, 16), Cons(Vec4(12, 16, 13, 13), Cons(Vec4(13, 13, 14, 11), Cons(Vec4(14, 11, 15, 9), s4)))));
  let s2: List[Vec4] = Cons(Vec4(14, 11, 16, 12), Cons(Vec4(15, 9, 16, 10), Cons(Vec4(16, 0, 10, 4), Cons(Vec4(10, 4, 8, 6), Cons(Vec4(8, 6, 7, 8), s3)))));
  let s1: List[Vec4] = Cons(Vec4(0, 8, 8, 6), Cons(Vec4(0, 10, 7, 8), Cons(Vec4(0, 12, 7, 10), Cons(Vec4(0, 14, 7, 13), Cons(Vec4(13, 13, 16, 14), s2)))));
  let s: List[Vec4] = Cons(Vec4(0, 0, 4, 2), Cons(Vec4(4, 2, 8, 2), Cons(Vec4(8, 2, 16, 0), Cons(Vec4(0, 4, 2, 1), Cons(Vec4(0, 6, 7, 4), s1)))));
  s
}

def nil(a: Vec, b: Vec, c: Vec): List[Vec4] { Nil }

def tup2(a_b: Vec, c_d: Vec): Vec4 {
  a_b.case {
    Vec(a, b) => c_d.case {
      Vec(c, d) => Vec4(a, b, c, d)
    }
  }
}


def grid(m: i64, n: i64, segments: List[Vec4], a: Vec, b: Vec, c: Vec): List[Vec4] {
  segments.case[Vec4] {
    Nil => Nil,
    Cons(v, t) => v.case {
      Vec4(x0, y0, x1, y1) =>
        Cons(
          tup2(
            vec_add(vec_add(a, scale_vec2(b, x0, m)), scale_vec2(c, y0, n)),
            vec_add(vec_add(a, scale_vec2(b, x1, m)), scale_vec2(c, y1, n))),
          grid(m, n, t, a, b, c))
    }
  }
}

def rot(p: Fun3[Vec, Vec, Vec, List[Vec4]], a: Vec, b: Vec, c: Vec): List[Vec4] {
  p.apply3[Vec, Vec, Vec, List[Vec4]](vec_add(a, b), c, vec_sub(Vec(0, 0), b))
}

def appendRev(l1: List[Vec4], l2: List[Vec4]): List[Vec4] {
  l1.case[Vec4] {
    Nil => l2,
    Cons(v, vs) => appendRev(vs, Cons(v, l2))
  }
}

def rev(l: List[Vec4]): List[Vec4] {
  appendRev(l, Nil)
}

def append(l1: List[Vec4], l2: List[Vec4]): List[Vec4] {
  l2.case[Vec4] {
    Nil => l1,
    Cons(v, vs) => appendRev(rev(l1), Cons(v, vs))
  }
}

def beside(m: i64, n: i64, p: Fun3[Vec, Vec, Vec, List[Vec4]], q: Fun3[Vec, Vec, Vec, List[Vec4]], a: Vec, b: Vec, c: Vec): List[Vec4] {
  append(
    p.apply3[Vec, Vec, Vec, List[Vec4]](a, scale_vec2(b, m, m + n), c),
    q.apply3[Vec, Vec, Vec, List[Vec4]](vec_add(a, scale_vec2(b, m, m + n)), scale_vec2(b, n, n + m), c))
}


def above(m: i64, n: i64, p: Fun3[Vec, Vec, Vec, List[Vec4]], q: Fun3[Vec, Vec, Vec, List[Vec4]], a: Vec, b: Vec, c: Vec): List[Vec4] {
  append(
    p.apply3[Vec, Vec, Vec, List[Vec4]](vec_add(a, scale_vec2(c, n, m + n)), b, scale_vec2(c, m, n + m)),
    q.apply3[Vec, Vec, Vec, List[Vec4]](a, b, scale_vec2(c, n, m + n)))
}

def tile_to_grid(arg: List[Vec4], arg2: Vec, arg3: Vec, arg4: Vec): List[Vec4] {
  grid(16, 16, arg, arg2, arg3, arg4)
}

def p(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  tile_to_grid(p_tile(), arg, q6, q7)
}

def q(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  tile_to_grid(q_tile(), arg, q6, q7)
}

def r(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  tile_to_grid(r_tile(), arg, q6, q7)
}

def s(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  tile_to_grid(s_tile(), arg, q6, q7)
}

def quartet(
  a: Fun3[Vec, Vec, Vec, List[Vec4]],
  b: Fun3[Vec, Vec, Vec, List[Vec4]],
  c: Fun3[Vec, Vec, Vec, List[Vec4]],
  d: Fun3[Vec, Vec, Vec, List[Vec4]],
  arg: Vec, a6: Vec, a7: Vec): List[Vec4] {
    above(1, 1,
      new { apply3(p5, p6, p7) => beside(1, 1, a, b, p5, p6, p7) },
      new { apply3(p5, p6, p7) => beside(1, 1, c, d, p5, p6, p7) },
      arg, a6, a7 )
}

def t(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => p(a, b, c) },
    new { apply3(a, b, c) => q(a, b, c) },
    new { apply3(a, b, c) => r(a, b, c) },
    new { apply3(a, b, c) => s(a, b, c) },
    arg, q6, q7)
}


def cycle_(p1: Fun3[Vec, Vec, Vec, List[Vec4]], arg: Vec, p3: Vec, p4: Vec): List[Vec4] {
  quartet(
    p1,
    new { apply3(a, b, c) =>
      rot(
        new { apply3(a, b, c) =>
          rot( new { apply3(a, b, c) => rot(p1, a, b, c) }, a, b, c)
        },
        a, b, c)
    },
    new { apply3(a, b, c) => rot(p1, a, b, c) },
    new { apply3(a, b, c) =>
      rot(new { apply3(a, b, c) => rot(p1, a, b, c) }, a, b, c)
    },
    arg,
    p3,
    p4)
}

def u(arg: Vec, p2: Vec, p3: Vec): List[Vec4] {
  cycle_(
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => q(a, b, c) }, a, b, c) },
    arg, p2, p3)
}

def side1(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => nil(a, b, c) },
    new { apply3(a, b, c) => nil(a, b, c) },
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => t(a, b, c) }, a, b, c) },
    new { apply3(a, b, c) => t(a, b, c) },
    arg, q6, q7)
}


def side2(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => side1(a, b, c) },
    new { apply3(a, b, c) => side1(a, b, c) },
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => t(a, b, c) }, a, b, c) },
    new { apply3(a, b, c) => t(a, b, c) },
    arg, q6, q7)
}

def corner1(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => nil(a, b, c) },
    new { apply3(a, b, c) => nil(a, b, c) },
    new { apply3(a, b, c) => nil(a, b, c) },
    new { apply3(a, b, c) => u(a, b, c) },
    arg, q6, q7)
}

def corner2(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => corner1(a, b, c) },
    new { apply3(a, b, c) => side1(a, b, c) },
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => side1(a, b, c) }, a, b, c) },
    new { apply3(a, b, c) => u(a, b, c) },
    arg, q6, q7)
}

def pseudocorner(arg: Vec, q6: Vec, q7: Vec): List[Vec4] {
  quartet(
    new { apply3(a, b, c) => corner2(a, b, c) },
    new { apply3(a, b, c) => side2(a, b, c) },
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => side2(a, b, c) }, a, b, c) },
    new { apply3(a, b, c) => rot(new { apply3(a, b, c) => t(a, b, c) }, a, b, c) },
    arg, q6, q7)
}

def pseudolimit(arg: Vec, p2: Vec, p3: Vec): List[Vec4] {
  cycle_(new { apply3(a, b, c) => pseudocorner(a, b, c) }, arg, p2, p3)
}

def enum_from_to(from: i64, t: i64): List[i64] {
  if from <= t {
    Cons(from, enum_from_to(from + 1, t))
  } else {
    Nil
  }
}

def min(i1: i64, i2: i64): i64 {
  if i1 < i2 {
    i1
  } else {
    i2
  }
}

def map(f: Fun[i64, List[Vec4]], l: List[i64]): List[List[Vec4]] {
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => Cons(f.apply[i64,List[Vec4]](i),map(f,is))
  }
}

def test_fish_nofib(n: i64): List[List[Vec4]] {
  map(
    new { apply(i) =>
      let n: i64 = min(0, i);
      pseudolimit(Vec(0, 0), Vec(640 + n, 0), Vec(0, 640 + n))
    },
    enum_from_to(1, n))
}

def length(l: List[Vec4]): i64 {
  l.case[Vec4] {
    Nil => 0,
    Cons(x, xs) => 1 + length(xs)
  }
}

def head(l: List[List[Vec4]]): List[Vec4] {
  l.case[List[Vec4]] {
    Nil => Nil,
    Cons(l, ls) => l
  }
}

def main(n: i64): i64 {
  let res: List[List[Vec4]] = test_fish_nofib(n);
  println_i64(length(head(res)));
  0
}
