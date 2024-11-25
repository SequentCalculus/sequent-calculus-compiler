// example 2.1
def ex211() : Int := 2 * 3;
def ex212() : Int := ifz(2, 5, 10);

// example 2.2
def ex22() : Int := let x : Int = 2 * 2 in x * x;

// example 2.3
def fac(n:Int) : Int := ifz(n, 1, n * fac(n - 1));
def ex23() : Int := fac(1);

// section 2.4
data ListInt { Nil, Cons(x:Int, xs:ListInt) }
def sum(x:ListInt) : Int := x.case { Nil => 0,
                                     Cons(y:Int, ys:ListInt) => y + sum(ys) };

codata StreamInt { Hd : Int, Tl : StreamInt }
def repeat(x:Int) : StreamInt := cocase { Hd => x, Tl => repeat(x) };

// section 2.4.1, example 2.4
data TupIntInt { Tup(x:Int, y:Int) } 
def swap(x:TupIntInt) : TupIntInt := x.case { Tup(y:Int, z:Int) => Tup(z, y) };

// section 2.4.2, example 2.5
codata LPairIntInt { Fst : Int, Snd : Int } 
def swaplazy(x:LPairIntInt) : LPairIntInt := cocase { Fst => x.Snd, Snd => x.Fst };

// example 2.6
def ex26() : Int := cocase { Ap(x:Int) => x * x }.Ap(2);

//example 2.7 def mult(l:ListInt) : Int := label 'a { mult2(l, 'a) };
def mult2(l:ListInt,'a:cnt Int) : Int := l.case { Nil => 1,
                                                  Cons(x:Int, xs:ListInt) => ifz(x, goto(0; 'a), x * mult2(xs, 'a))};

// section 5.1
def sec51() : Int := (2 * 3) * 4;

//section 5.3
def letex() : Int := let x : Int = 2 in x * x;
def labelex() : Int := label 'a { goto(0; 'a) };

//section 5.4
def casecase() : ListInt := Nil.case { Nil => Nil, Cons(x:Int, xs:ListInt) => xs}.case{
                   Nil => Nil,
                   Cons(y:Int, ys:ListInt) => ys };

//section 5.5
def tltltl() : StreamInt := repeat(1).Tl.Tl.Tl;

//section 5.6
codata FunIntInt { Ap(x:Int) : Int }
def criticalEta1('b:cnt FunIntInt) : FunIntInt := let x : FunIntInt = cocase { Ap(y:Int) => goto(cocase { Ap(z:Int) => 1 }; 'b).Ap(y) } in cocase { Ap(z:Int) => 3 };
def criticalEta2('b:cnt FunIntInt) : FunIntInt := let x : FunIntInt = goto(cocase { Ap(z:Int) => 1 }; 'b) in cocase { Ap(z:Int) => 3 };

//def main : Int  := ex211();
//def main : Int := ex212();
//def main : Int := ex22();
//def main : Int := ex23();
//def main : Int := sum(Cons(1, Cons(1, Cons(1, Nil))));
//def main : StreamInt := repeat(1);
//def main : TupIntInt := swap(Tup(1, 2));
//def main : Int := swaplazy(cocase { Fst => 1, Snd => 2 }).Snd;
//def main : Int := ex26();
//def main : Int := mult(Cons(2, Cons(2, Cons(0, Cons(3, Nil)))));
//def main : Int := sec51();
//def main : Int := letex();
//def main : Int := labelex();
//def main : ListInt := casecase();
//def main : StreamInt := tltltl();
def main() : FunIntInt := label 'b { criticalEta2('b) };
