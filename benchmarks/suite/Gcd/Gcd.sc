<<<<<<< HEAD
data Triple[A, B, C] { Trip(a: A, b: B, c: C) }
data Pair[A, B] { Pair(a: A, b: B) }
data List[A] { Nil, Cons(a: A, as: List[A]) }
codata Fun[A, B] { Apply(a: A): B }

def quot_rem(a: i64, b: i64): Pair[i64, i64] {
  Pair(a / b, a % b)
}

def g(u1u2u3: Triple[i64, i64, i64], v1v2v3: Triple[i64, i64, i64]): Triple[i64, i64, i64] {
  u1u2u3.case[i64, i64, i64] {
    Trip(u1, u2, u3) => v1v2v3.case[i64, i64, i64] {
      Trip(v1, v2, v3) =>
        if v3 == 0 {
          Trip(u3, u1, u2)
        } else {
          quot_rem(u3, v3).case[i64, i64] {
            Pair(q, r) => g(Trip(v1, v2, v3), Trip(u1 - (q * v1), u2 - (q * v2), r))
=======
data Triple[A,B,C] { Trip(a:A,b:B,c:C) }
data Pair[A,B] { Pair(a:A,b:B) }
data List[A] { Nil,Cons(a:A,as:List[A]) }
codata Fun[A,B] { Ap(a:A):B }

def quot_rem(a:i64, b:i64) : Pair[i64,i64] {
  Pair(a/b, a%b)
}

def g(u1u2u3:Triple[i64,i64,i64], v1v2v3:Triple[i64,i64,i64]) : Triple[i64,i64,i64]{
  u1u2u3.case[i64,i64,i64]{
    Trip(u1,u2,u3) => v1v2v3.case[i64,i64,i64]{
      Trip(v1,v2,v3) => 
        if v3 == 0 {
          Trip(u3, u1, u2)
        } else {
          quot_rem(u3, v3).case[i64,i64]{
            Pair(q,r) => g(Trip(v1, v2, v3), Trip(u1 - (q * v1), u2 - (q * v2), r)) 
>>>>>>> 0669e1c (added gcd)
          }
        }
    }
  }
}

<<<<<<< HEAD
def gcd_e(x: i64, y: i64): Triple[i64, i64, i64] {
  if x == 0 {
    Trip(y, 0, 1)
  } else {
    g(Trip(1, 0, x), Trip(0, 1, y))
  }
}

def max_(ls: List[i64]): i64 {
  ls.case[i64] {
    Nil => -1, //runtime error
    Cons(x, xs) => xs.case[i64] {
      Nil => x,
      Cons(y, ys) => if x < y { max_(Cons(y, ys)) } else { max_(Cons(x, ys)) }
=======
def gcd_e(x:i64, y:i64): Triple[i64,i64,i64]{
  if x == 0 {
    Trip(y, 0, 1)
  }else {
    g(Trip(1, 0, x), Trip(0, 1, y)) 
  }
}

def max_(ls:List[i64]) : i64 {
  ls.case[i64]{
    Nil => -1, //runtime error
    Cons(x,xs) => xs.case[i64]{
      Nil => x,
      Cons(y,ys) => if x<y { max_(Cons(y,ys)) } else { max_(Cons(x,ys)) }
>>>>>>> 0669e1c (added gcd)
    }
  }
}

<<<<<<< HEAD
def enum_from_to(from: i64, t: i64): List[i64] {
  if from <= t {
    Cons(from, enum_from_to(from + 1, t))
  } else {
    Nil
  }
}

def test_lscomp2(p2: List[i64], t1: List[i64], ms: List[i64], h1: i64): List[Pair[i64, i64]] {
  p2.case[i64] {
    Nil => test_lscomp1(t1, ms),
    Cons(h2, t2) => Cons(Pair(h1, h2), test_lscomp2(t2, t1, ms, h1))
  }
}

def test_lscomp1(p1: List[i64], ms: List[i64]): List[Pair[i64, i64]] {
  p1.case[i64] {
    Nil => Nil,
    Cons(h1, t1) => test_lscomp2(ms, t1, ms, h1)
  }
}

def rev_pairs_loop(
  l1: List[Triple[i64, i64, Triple[i64, i64, i64]]],
  l2: List[Triple[i64, i64, Triple[i64, i64, i64]]]
): List[Triple[i64, i64, Triple[i64, i64, i64]]] {
  l1.case[Triple[i64, i64, Triple[i64, i64, i64]]] {
    Nil => l2,
    Cons(is, iss) => rev_pairs_loop(iss, Cons(is, l2))
  }
}

def rev_pairs(
  l: List[Triple[i64, i64, Triple[i64, i64, i64]]]
): List[Triple[i64, i64, Triple[i64, i64, i64]]] {
  rev_pairs_loop(l, Nil)
}

def map_pairs_loop(
  f: Fun[Pair[i64, i64], Triple[i64, i64, Triple[i64, i64, i64]]],
  l: List[Pair[i64, i64]],
  acc: List[Triple[i64, i64, Triple[i64, i64, i64]]]
): List[Triple[i64, i64, Triple[i64, i64, i64]]] {
  l.case[Pair[i64, i64]] {
    Nil => rev_pairs(acc),
    Cons(p, ps) => map_pairs_loop(f, ps, Cons(f.Apply[Pair[i64, i64], Triple[i64, i64, Triple[i64, i64, i64]]](p), acc))
  }
}

def map_pairs(
  f: Fun[Pair[i64, i64], Triple[i64, i64, Triple[i64, i64, i64]]],
  l: List[Pair[i64, i64]]
): List[Triple[i64, i64, Triple[i64, i64, i64]]] {
  map_pairs_loop(f, l, Nil)
}

def rev_triples_loop(
  l1: List[i64],
  l2: List[i64]
): List[i64] {
  l1.case[i64] {
    Nil => l2,
    Cons(is, iss) => rev_triples_loop(iss, Cons(is, l2))
  }
}

def rev_triples(
  l: List[i64]
): List[i64] {
  rev_triples_loop(l, Nil)
}

def map_triples_loop(
  f: Fun[Triple[i64, i64, Triple[i64, i64, i64]], i64],
  l: List[Triple[i64, i64, Triple[i64, i64, i64]]],
  acc: List[i64]
): List[i64] {
  l.case[Triple[i64, i64, Triple[i64, i64, i64]]] {
    Nil => rev_triples(acc),
    Cons(p, ps) => map_triples_loop(f, ps, Cons(f.Apply[Triple[i64, i64, Triple[i64, i64, i64]], i64](p), acc))
  }
}

def map_triples(
  f: Fun[Triple[i64, i64, Triple[i64, i64, i64]], i64],
  l: List[Triple[i64, i64, Triple[i64, i64, i64]]]
): List[i64] {
  map_triples_loop(f, l, Nil)
}

def abs_int(i: i64): i64 {
  if i < 0 {
    -1 * i
  } else {
=======
def enum_from_to(from:i64,t:i64) : List[i64]{
  if from==t{
    Nil
  }else{
    Cons(from,enum_from_to(from+1,t))
  }
}

def test_lscomp2(p2:List[i64],t1:List[i64],ms:List[i64],h1:i64): List[Pair[i64,i64]]{
  p2.case[i64]{
    Nil => test_lscomp1(t1,ms),
    Cons(h2,t2) => Cons(Pair(h1, h2),test_lscomp2(t2,t1,ms,h1))
  }
}

def test_lscomp1(p1:List[i64],ms:List[i64]) : List[Pair[i64,i64]]{
  p1.case[i64]{
    Nil => Nil,
    Cons(h1,t1) => test_lscomp2(ms,t1,ms,h1)
  }
}

def map_pairs(f:Fun[Pair[i64,i64],Triple[i64,i64,Triple[i64,i64,i64]]],ls:List[Pair[i64,i64]]) : List[Triple[i64,i64,Triple[i64,i64,i64]]]{
  ls.case[Pair[i64,i64]]{
    Nil => Nil,
    Cons(p,ps) => Cons(f.Ap[Pair[i64,i64],Triple[i64,i64,Triple[i64,i64,i64]]](p),map_pairs(f,ps))
  }
}

def map_triples(f:Fun[Triple[i64,i64,Triple[i64,i64,i64]],i64],ls:List[Triple[i64,i64,Triple[i64,i64,i64]]]) : List[i64]{
  ls.case[Triple[i64,i64,Triple[i64,i64,i64]]]{
    Nil => Nil,
    Cons(t,ts) => Cons(f.Ap[Triple[i64,i64,Triple[i64,i64,i64]],i64](t),map_triples(f,ts))
  }
}

def abs_int(i:i64) : i64 {
  if i<0{
    -1*i
  }else {
>>>>>>> 0669e1c (added gcd)
    i
  }
}

<<<<<<< HEAD
def test(d: i64): i64 {
  let ns: List[i64] = enum_from_to(5000, 5000 + d);
  let ms: List[i64] = enum_from_to(10000, 10000 + d);
  let tripls: List[Triple[i64, i64, Triple[i64, i64, i64]]] = map_pairs(
    new { Apply(p) => p.case[i64, i64] { Pair(x, y) => Trip(x, y, gcd_e(x, y)) } },
    test_lscomp1(ns, ms));
  let rs: List[i64] = map_triples(
    new { Apply(t) =>
      t.case[i64, i64, Triple[i64, i64, i64]] { Trip(d1, d2, t) =>
        t.case[i64, i64, i64] { Trip(gg, u, v) => abs_int((gg + u) + v) }
      }
    },
    tripls);
  max_(rs)
}

def test_gcd_nofib(x: i64): i64 {
  test(x)
}

def main_loop(iters: i64, n: i64): i64 {
  if iters == 1 {
    let res: i64 = test_gcd_nofib(n);
    println_i64(res);
    0
  } else {
    let res: i64 = test_gcd_nofib(n);
    main_loop(iters - 1, n)
  }
}

def main(iters: i64, n: i64): i64 {
  main_loop(iters, n)
=======
def test(d:i64) : i64 {
  let ns: List[i64] = enum_from_to(5000, 5000 + d);
  let ms: List[i64] = enum_from_to(10000, 10000 + d);
  let tripls: List[Triple[i64,i64,Triple[i64,i64,i64]]] = map_pairs(
    new { Ap(p) => p.case[i64,i64] { Pair(x, y) => Trip(x, y, gcd_e(x, y)) } }, 
    test_lscomp1(ns,ms));
  let rs: List[i64] = map_triples(
    new { Ap(t) => 
      t.case[i64,i64,Triple[i64,i64,i64]] { Trip(d1, d2,t) => 
        t.case[i64,i64,i64]{ Trip(gg, u, v) => abs_int((gg + u) + v) } 
      } 
    }, tripls);
  max_(rs) 
}

def test_gcd_nofib(x:i64) : i64{
  test(x) 
}

def main_loop(iters:i64,n:i64) : i64{
  if iters==1{
    let res: i64 = test_gcd_nofib(n);
    println_i64(res);
    0
  }else{
    let res: i64 = test_gcd_nofib(n);
    main_loop(iters-1,n)
  }
}

def main(iters:i64,n:i64) : i64{
  main_loop(iters,n)
>>>>>>> 0669e1c (added gcd)
}
