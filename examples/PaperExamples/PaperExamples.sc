// example 2.1
def ex211() : i64 { 2 * 3 }
def ex212() : i64 { if 2 == 0 {5} else {10} }

// example 2.2
def ex22() : i64 { let x : i64 = 2 * 2; x * x }

// example 2.3
def fac(n:i64) : i64 { if n == 0 {1} else {n * fac(n - 1)} }
def ex23() : i64 { fac(1) }

// section 2.4
data List[A] { Nil, Cons(x: A, xs: List[A]) }
def sum(x:List[i64]) : i64 { x.case[i64] { Nil => 0,
                                           Cons(y, ys) => y + sum(ys) }}

codata Stream[A] { Hd : A, Tl : Stream[A] }
def repeat(x:i64) : Stream[i64] { new { Hd => x, Tl => repeat(x) } }

// section 2.4.1, example 2.4
data Pair[A, B] { Tup(x:A, y:B) }
def swap(x:Pair[i64, i64]) : Pair[i64, i64] { x.case[i64, i64] { Tup(y, z) => Tup(z, y) } }

// section 2.4.2, example 2.5
codata LazyPair[A, B] { Fst : A, Snd : B }
def swaplazy(x:LazyPair[i64, i64]) : LazyPair[i64, i64] { new { Fst => x.Snd[i64, i64], Snd => x.Fst[i64, i64] } }

// example 2.6
def ex26() : i64 { new { Apply(x) => x * x }.Apply[i64, i64](2) }

//example 2.7 def mult(l:List[i64]) : i64 { label a { mult2(l, a) }}
def mult2(l:List[i64],a:cns i64) : i64 { l.case[i64] { Nil => 1,
                                                       Cons(x, xs) => if x == 0 {return 0 to a} else {x * mult2(xs, a)}}}

// section 5.1
def sec51() : i64 { (2 * 3) * 4 }

//section 5.3
def letex() : i64 { let x : i64 = 2; x * x }
def labelex() : i64 { label a { return 0 to a } }

//section 5.4
def casecase() : List[i64] { Nil.case[i64] { Nil => Nil, Cons(x, xs) => xs}.case[i64] {
                   Nil => Nil,
                   Cons(y, ys) => ys }}

//section 5.5
def tltltl() : Stream[i64] { repeat(1).Tl[i64].Tl[i64].Tl[i64] }

//section 5.6
codata Fun[A, B] { Apply(x: A) : B }
def criticalEta1(b:cns Fun[i64, i64]) : Fun[i64, i64] { let x : Fun[i64, i64] = new { Apply(y) => (return new { Apply(z) => 1 } to b).Apply[i64, i64](y) }; new { Apply(z) => 3 }}
def criticalEta2(b:cns Fun[i64, i64]) : Fun[i64, i64] { let x : Fun[i64, i64] = return new { Apply(z) => 1 } to b; new { Apply(z) => 3 }}

//def main : i64 { println_i64(ex211());
//                 0 }
//def main : i64 { println_i64(ex212());
//                 0 }
//def main : i64 { println_i64(ex22());
//                 0 }
//def main : i64 { println_i64(ex23());
//                 0 }
//def main : i64 { println_i64(sum(Cons(1, Cons(1, Cons(1, Nil)))));
//                 0 }
//def main : Stream[i64] := { repeat(1) }
//def main : Pair[i64, i64] := { swap(Tup(1, 2)) }
//def main : i64 { println_i64(swaplazy(new { Fst => 1, Snd => 2 }).Snd[i64, i64]);
//                 0 }
//def main : i64 { println_i64(ex26());
//                 0 }
//def main : i64 { println_i64(mult(Cons(2, Cons(2, Cons(0, Cons(3, Nil))))));
//                 0 }
//def main : i64 { println_i64(sec51());
//                 0 }
//def main : i64 { println_i64(letex());
//                 0 }
def main : i64 { println_i64(labelex());
                 0 }
//def main : List[i64] := { casecase() }
//def main : Stream[i64] := { tltltl() }
//def main() : Fun[i64, i64] := { label b { criticalEta2(b) } }
