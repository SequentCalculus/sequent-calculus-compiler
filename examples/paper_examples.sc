// example 2.1
def ex211() : Int := 2 * 3;
def ex212() : Int := ifz(2, 5, 10);

// example 2.2
def ex22() : Int := let x = 2 * 2 in x * x;

// example 2.3
def fac(n:Int) : Int := ifz(n, 1, n * (fac(n - 1)));
def ex23() : Int := fac(1);

// section 2.4
def sum(x:ListInt) : Int := case x of { Nil => 0,
                           Cons(y:Int, ys:ListInt) => y + (sum(ys)) };
def repeat(x:Int) : StreamInt := cocase { hd => x, tl => repeat(x) };

// section 2.4.1, example 2.4
def swap(x:TupIntInt) : TupIntInt := case x of { Tup(y:Int, z:Int) => Tup(z, y) };

// section 2.4.2, example 2.5
def swaplazy(x:LPairInt) : LPairIntInt := cocase { fst => x.snd, snd => x.fst };

// example 2.6
def ex26() : Int := cocase { ap(x:Int) => x * x }.ap(2);

//example 2.7
def mult(l:ListInt) : Int := label 'a { mult2(l, 'a) };
def mult2(l:Lostint,'a:cnt Int) : Int := case l of { Nil => 1,
                               Cons(x:Int, xs:ListInt) => ifz(x, goto(0; 'a), x * (mult2(xs, 'a)))};

// section 5.1
def sec51() : Int := (2 * 3) * 4;

//section 5.3
def labelex() : Int := label 'a { goto(0; 'a) };

//section 5.4
def casecase() : ListInt := case (case Nil of { Nil => Nil, Cons(x:Int, xs:ListInt) => xs}) of {
                   Nil => Nil,
                   Cons(y:Int, ys:ListInt) => ys };

//section 5.5
def tltltl() : StreamInt := (repeat(1)).tl.tl.tl;

//section 5.6
def criticalEta1('b:cnt Int) : Int := let x = cocase { ap(y:Int) => goto(cocase { ap(z:Int) => 1 }; 'b).ap(y) } in cocase { ap(z:Int) => 3 };
def criticalEta2('b:cnt Int) : Int := let x = goto(cocase { ap(z:Int) => 1 }; 'b) in cocase { ap(z:Int) => 3 };

// section 2.4
def repeat(x:Int) : StreamInt := cocase { hd => x, tl => repeat(x) };

//def main := ex211();
//def main := ex212();
//def main := ex22();
//def main := ex23();
//def main := sum(Cons(1, Cons(1, Cons(1, Nil))));
//def main := repeat(1);
//def main := swap(Tup(1, 2));
//def main := swaplazy(cocase { fst => 1, snd => 2 }).snd;
//def main := ex26();
//def main := mult(Cons(2, Cons(2, Cons(0, Cons(3, Nil)))));
//def main := sec51();
//def main := letex();
//def main := labelex();
//def main := casecase();
//def main := tltltl();
def main() : Int := label 'b { criticalEta2('b) };
