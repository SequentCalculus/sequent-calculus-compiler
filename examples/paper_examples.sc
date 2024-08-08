// example 2.1
def ex211(;) := 2 * 3;
def ex212(;) := ifz(2, 5, 10);

// example 2.2
def ex22(;) := let x = 2 * 2 in x * x;

// example 2.3
def fac(n;) := ifz(n, 1, n * (fac(n - 1;)));
def ex23(;) := fac(1;);

// section 2.4
def sum(x;) := case x of { Nil => 0,
                          Cons(y, ys) => y + (sum(ys;)) };
def repeat(x;) := cocase { hd => x, tl => repeat(x;) };

// section 2.4.1, example 2.4
def swap(x;) := case x of { Tup(y, z) => Tup(z, y) };

// section 2.4.2, example 2.5
def swaplazy(x;) := cocase { fst => x.snd, snd => x.fst };

// example 2.6
def ex26(;) := (\x => x * x) @ 2;

//example 2.7
def mult(l;) := label a { mult2(l; a) };
def mult2(l; a) := case l of { Nil => 1,
                               Cons(x, xs) => ifz(x, goto(0; a), x * (mult2(xs; a)))};

// section 5.1
def sec51(;) := (2 * 3) * 4;

//section 5.3
def letex(;) := let x = 2 in x * x;
def labelex(;) := label a { goto(0; a) };

//section 5.4
def casecase(;) := case (case Nil of { Nil => Nil, Cons(x, xs) => xs}) of {
                   Nil => Nil,
                   Cons(y, ys) => ys };

//section 5.5
def tltltl(;) := (repeat(1;)).tl.tl.tl;

//section 5.6
def criticalEta1(; b) := let x = \y => goto(\z => 1; b) @ y in \z => 3;
def criticalEta2(; b) := let x = goto(\z => 1; b) in \z => 3;

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
//def main := label b { criticalEta1(; b) };
def main(;) := label b { criticalEta2(; b) };
