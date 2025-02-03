// example 2.1
def ex211() : i64 { 2 * 3 }
def ex212() : i64 { ifz(2, 5, 10) }

// example 2.2
def ex22() : i64 { let x : i64 = 2 * 2 in x * x }

// example 2.3
def fac(n:i64) : i64 { ifz(n, 1, n * fac(n - 1)) }
def ex23() : i64 { fac(1) }

// section 2.4
data ListI64 { Nil, Cons(x:i64, xs:ListI64) }
def sum(x:ListI64) : i64 { x.case { Nil => 0,
                                     Cons(y:i64, ys:ListI64) => y + sum(ys) }}

codata StreamI64 { Hd : i64, Tl : StreamI64 }
def repeat(x:i64) : StreamI64 { cocase { Hd => x, Tl => repeat(x) } }

// section 2.4.1, example 2.4
data TupI64I64 { Tup(x:i64, y:i64) } 
def swap(x:TupI64I64) : TupI64I64 { x.case { Tup(y:i64, z:i64) => Tup(z, y) } }

// section 2.4.2, example 2.5
codata LPairI64I64 { Fst : i64, Snd : i64 } 
def swaplazy(x:LPairI64I64) : LPairI64I64 { cocase { Fst => x.Snd, Snd => x.Fst } }

// example 2.6
def ex26() : i64 { cocase { Ap(x:i64) => x * x }.Ap(2) }

//example 2.7 def mult(l:ListI64) : i64 { label a { mult2(l, a) }}
def mult2(l:ListI64,a:cns i64) : i64 { l.case { Nil => 1,
                                                Cons(x:i64, xs:ListI64) => ifz(x, goto(0; a), x * mult2(xs, a))}}

// section 5.1
def sec51() : i64 { (2 * 3) * 4 }

//section 5.3
def letex() : i64 { let x : i64 = 2 in x * x }
def labelex() : i64 { label a { goto(0; a) } }

//section 5.4
def casecase() : ListI64 { Nil.case { Nil => Nil, Cons(x:i64, xs:ListI64) => xs}.case{
                   Nil => Nil,
                   Cons(y:i64, ys:ListI64) => ys }}

//section 5.5
def tltltl() : StreamI64 { repeat(1).Tl.Tl.Tl }

//section 5.6
codata FunI64I64 { Ap(x:i64) : i64 }
def criticalEta1(b:cns FunI64I64) : FunI64I64 { let x : FunI64I64 = cocase { Ap(y:i64) => goto(cocase { Ap(z:i64) => 1 }; b).Ap(y) } in cocase { Ap(z:i64) => 3 }}
def criticalEta2(b:cns FunI64I64) : FunI64I64 { let x : FunI64I64 = goto(cocase { Ap(z:i64) => 1 }; b) in cocase { Ap(z:i64) => 3 }}

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
//def main : StreamI64 := { repeat(1) }
//def main : TupI64I64 := { swap(Tup(1, 2)) }
//def main : i64 { println_i64(swaplazy(cocase { Fst => 1, Snd => 2 }).Snd);
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
//def main : ListI64 := { casecase() }
//def main : StreamI64 := { tltltl() }
//def main() : FunI64I64 := { label b { criticalEta2(b) } }
