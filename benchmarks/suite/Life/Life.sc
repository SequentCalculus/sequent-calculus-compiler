data Unit { Unit } 
data PairI64 { Tup(fst:i64,snd:i64) }
data ListPair { Nil, Cons(p:PairI64, ps:ListPair) }
data ListList { NilL, ConsL(l:ListPair,ls:ListList) }
data Gen { Gen(coordslist: ListPair) }
data Bool { True, False }
codata PredicatePair { ApBP(p:PairI64) : Bool }
codata FunPairI64 { ApP(p:PairI64) : i64 }
codata PredicateI64 { ApBI(i:i64) : Bool }
codata FunPairList { ApL(p:PairI64) : ListPair }
codata FunPairPair { ApPP(p:PairI64) : PairI64 }

def pair_eq(p1:PairI64,p2:PairI64) : Bool := p1.case {
  Tup(fst1:i64,snd1:i64) => p2.case{
    Tup(fst2:i64,snd2:i64) => ife(fst1,fst2,ife(snd1,snd2,True,False),False)
  }
};

def or(b1:Bool,b2:Bool):Bool := b1.case{
  True => True,
  False => b2
};

def not(b:Bool) : Bool := b.case {
  True => False,
  False => True
};

// instead of collect in ML
def flat_map(l:ListPair,f:FunPairList) : ListPair := l.case {
  Nil => Nil,
  Cons(p:PairI64,ps:ListPair) => append(f.ApL(p),flat_map(ps,f))
};

def map(l:ListPair,f:FunPairPair) : ListPair := l.case{
  Nil => Nil,
  Cons(p:PairI64,ps:ListPair) => Cons(f.ApPP(p),map(ps,f))
};

def member(l:ListPair,p:PairI64) : Bool := l.case{
  Nil => False, 
  Cons(p1:PairI64,ps:ListPair) => or(pair_eq(p,p1),member(ps,p))
};

def len(l:ListPair) : i64 := l.case{
  Nil => 0,
  Cons(x:PairI64,xs:ListPair) => 1+(len(xs))
};

def filter(l:ListPair,p:PredicatePair) : ListPair := l.case{
  Nil => Nil,
  Cons(pr:PairI64,ps:ListPair) => p.ApBP(pr).case{
    True => Cons(pr,filter(ps,p)),
    False => filter(ps,p)
  }
};

def append(l1:ListPair,l2:ListPair) : ListPair := l1.case{
  Nil => l2,
  Cons(p1:PairI64,ps:ListPair) => Cons(p1,append(ps,l2))
};

def lexless(a:PairI64) : PredicatePair := cocase { ApBP(b:PairI64) => 
  a.case {
    Tup(fst1:i64,snd1:i64) => b.case{
      Tup(fst2:i64,snd2:i64) => ifl(fst2,fst1,True,ife(fst2,fst1, ifl(snd2,snd1,True,False),False))
    }
}};

def lexgreater(a:PairI64) : PredicatePair := cocase { ApBP(b:PairI64) => 
  lexless(b).ApBP(a)
};

def lexordset(xs:ListPair) : ListPair := xs.case {
  Nil => Nil ,
  Cons(a:PairI64,x:ListPair) => append(append(
    lexordset(filter(x,lexless(a))),
    Cons(a,Nil)),
    lexordset(filter(x,lexgreater(a))))
};

// needs to be toplevel as there is no term-level recursion
def collect_neighbors(xover:ListPair,x3:ListPair,x2:ListPair,x1:ListPair,xs:ListPair) : ListPair :=   xs.case { 
  Nil => filter(x3, cocase { ApBP(p:PairI64) => not(member(xover,p)) }), 
  Cons(a:PairI64,x:ListPair) => member(xover,a).case{
    True => collect_neighbors(xover,x3,x2,x1,x),
    False => member(x3,a).case {
      True => collect_neighbors(Cons(a,xover),x3,x2,x1,x),
      False => member(x2,a).case {
        True => collect_neighbors(xover,Cons(a,x3),x2,x1,x),
        False => member(x1,a).case{
          True => collect_neighbors(xover,x3,Cons(a,x2),x1,x),
          False => collect_neighbors(xover,x3,x2,Cons(a,x1),x)
        }
      }
    }
  }
};

def occurs3(l:ListPair) : ListPair := collect_neighbors(Nil,Nil,Nil,Nil,l);


def neighbours(p:PairI64) : ListPair := p.case{
  Tup(fst:i64,snd:i64) => 
    Cons(Tup(fst-1,snd-1),
      Cons(Tup(fst-1,snd),
        Cons(Tup(fst-1,snd+1),
          Cons(Tup(fst,snd-1),
            Cons(Tup(fst,snd+1),
              Cons(Tup(fst+1,snd-1),
                Cons(Tup(fst+1,snd),
                  Cons(Tup(fst+1,snd+1),
                    Nil))))))))
};

def alive(g : Gen) : ListPair := g.case { Gen(livecoords:ListPair) => livecoords };

def mkgen(coordlist : ListPair) : Gen := Gen(lexordset(coordlist));

def mk_nextgen_fn(gen:Gen) : Gen :=
  let living : ListPair = alive(gen) in 
  let isalive : PredicatePair = cocase { ApBP(p:PairI64) => member(living,p) } in 
  let liveneighbours : FunPairI64 = cocase {ApP(p:PairI64) => len(filter(neighbours(p),isalive)) } in
  let twoorthree : PredicateI64 = cocase { ApBI(n:i64) => ife(n,2,True,ife(n,3,True,False)) } in 
  let survivors : ListPair = filter(living,cocase{ ApBP(p:PairI64) => twoorthree.ApBI(liveneighbours.ApP(p)) }) in
  let newnbrlist : ListPair = flat_map(living, 
    cocase { ApL(p:PairI64) => filter(neighbours(p),
      cocase { ApBP(n:PairI64) => not(isalive.ApBP(n))} )}) in
  let newborn : ListPair = occurs3(newnbrlist) in 
  mkgen(append(survivors,newborn));

def nthgen(g:Gen, i:i64) : Gen := ifz(i,g,nthgen(mk_nextgen_fn(g), i-1));

def gun() : Gen := mkgen(
  Cons(Tup(2,20),Cons(Tup(3,19),Cons(Tup(3,21),Cons(Tup(4,18),
    Cons(Tup(4,22),Cons(Tup(4,23),Cons(Tup(4,32),Cons(Tup(5,7),
      Cons(Tup(5,8),Cons(Tup(5,18),Cons(Tup(5,22),Cons(Tup(5,23),
        Cons(Tup(5,29),Cons(Tup(5,30),Cons(Tup(5,31),Cons(Tup(5,32),
          Cons(Tup(5,36),Cons(Tup(6,7),Cons(Tup(6,8),Cons(Tup(6,18),
            Cons(Tup(6,22),Cons(Tup(6,23),Cons(Tup(6,28),Cons(Tup(6,29),
              Cons(Tup(6,30),Cons(Tup(6,31),Cons(Tup(6,36),Cons(Tup(7,19),
                Cons(Tup(7,21),Cons(Tup(7,28),Cons(Tup(7,31),Cons(Tup(7,40),
                  Cons(Tup(7,41),Cons(Tup(8,20),Cons(Tup(8,28),Cons(Tup(8,29),
                    Cons(Tup(8,30),Cons(Tup(8,31),Cons(Tup(8,40),Cons(Tup(8,41),
                      Cons(Tup(9,29),Cons(Tup(9,30),Cons(Tup(9,31),Cons(Tup(9,32),Nil)
                        ))))))))))))))))))))))))))))))))))))))))))));

def go_gun(steps:i64) : Unit := let gen : Gen = nthgen(gun(), steps) in Unit;

def centerLine() : i64 := 5;
def bail() : ListPair := Cons(Tup(0,0),Cons(Tup(0,1),Cons(Tup(1,0),Cons(Tup(1,1),Nil)))); 

def shuttle() : ListPair := Cons(Tup(0, 3),Cons(Tup(1, 2),Cons(Tup(1, 4),Cons(Tup(2, 1), Cons(Tup(2, 5),
  Cons(Tup(3, 2), Cons(Tup(3, 3), Cons(Tup(3, 4),
    Cons(Tup(4, 1), Cons(Tup(4, 0), Cons(Tup(4, 5), Cons(Tup(4, 6),Nil
      ))))))))))));

def at_pos(coordlist:ListPair, p:PairI64) : ListPair := 
  let move : FunPairPair = cocase { ApPP(a:PairI64) => 
    a.case { Tup(fst1:i64,snd1:i64) => 
      p.case { Tup(fst2:i64,snd2:i64) => Tup(fst1+fst2,snd1+snd2) } 
    } 
  } in 
  map(coordlist,move);

def non_steady() : Gen := mkgen(append(append(
    at_pos(bail(), Tup(1, centerLine())),
    at_pos(bail(), Tup(21, centerLine()))),
    at_pos(shuttle(), Tup(6, centerLine()-2))));

def go_shuttle(steps:i64) : Unit := let gen : Gen = nthgen(non_steady(), steps) in Unit;

def main(steps:i64) : i64 := 
  let gun_res : Unit = go_gun(steps) in 
  let shuttle_res : Unit = go_shuttle(steps) in 
0;
