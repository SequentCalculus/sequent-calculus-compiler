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
codata FunListPairPairListPair { ApLPP(l:ListPair,p:PairI64) : ListPair }
codata FunIntUnit { ApIU(i:i64) : Unit }


// Bool Functions 

def pair_eq(p1:PairI64,p2:PairI64) : Bool { 
  p1.case {
    Tup(fst1:i64,snd1:i64) => p2.case{
      Tup(fst2:i64,snd2:i64) => 
        if fst1==fst2 { 
          if snd1==snd2 {
            True
          } else {
            False
          }
        } else {
          False
        }
    }
  }
}

def or(b1:Bool,b2:Bool):Bool {
  b1.case{
    True => True,
    False => b2
  }
}

def not(b:Bool) : Bool {
  b.case {
    True => False,
    False => True
  }
}


// List Functions 

def revonto(x:ListPair, y:ListPair) : ListPair {
  accumulate( x, y, cocase { ApLPP(x:ListPair,a:PairI64) => Cons(a,x) } )
}

def accumulate(a:ListPair,xs:ListPair,f:FunListPairPairListPair) : ListPair {
  fold(a,xs,f)
}

def fold(a:ListPair,xs:ListPair,f:FunListPairPairListPair) : ListPair {
  xs.case{
    Nil => a,
    Cons(b:PairI64,x:ListPair) => fold(f.ApLPP(a,b),x,f)
  }
}

def collect_accum(sofar:ListPair, xs:ListPair, f:FunPairList) : ListPair {
  xs.case{
    Nil => sofar,
    Cons(p:PairI64,xs:ListPair) => collect_accum(revonto(sofar,f.ApL(p)),xs,f)
  }
}

def collect(l:ListPair,f:FunPairList) : ListPair {
  collect_accum(Nil,l,f)
}

def exists(l:ListPair,f:PredicatePair) : Bool{
  l.case{
    Nil => False,
    Cons(p:PairI64,ps:ListPair) => or(f.ApBP(p),exists(ps,f))
  }
}

def rev_loop(l:ListPair,acc:ListPair) : ListPair {
  l.case {
    Nil => acc,
    Cons(p:PairI64,ps:ListPair) => rev_loop(ps,Cons(p,acc))
  }
}

def rev(l:ListPair) : ListPair {
  rev_loop(l,Nil)
}

def map_loop(l:ListPair,f:FunPairPair,acc:ListPair) : ListPair {
  l.case {
    Nil => rev(acc),
    Cons(p:PairI64,ps:ListPair) => map_loop(ps,f,Cons(f.ApPP(p),acc))
  }
}

def map(l:ListPair,f:FunPairPair) : ListPair {
  map_loop(l,f,Nil)
}

def member(l:ListPair,p:PairI64) : Bool { 
  exists(l,cocase { ApBP(p1:PairI64) => pair_eq(p,p1) })
}

def len_loop(l:ListPair,acc:i64) : i64 {
  l.case {
    Nil => acc,
    Cons(p:PairI64,ps:ListPair) => len_loop(ps,acc+1)
  }
}

def len(l:ListPair) : i64 {
  len_loop(l,0)
}

def filter_loop(l:ListPair,f:PredicatePair,acc:ListPair) : ListPair {
  l.case{
    Nil => rev(acc),
    Cons(p:PairI64,ps:ListPair) => filter_loop(ps,f,
      f.ApBP(p).case{
        True => Cons(p,acc),
        False => acc
      })
  }
}

def filter(l:ListPair,p:PredicatePair) : ListPair {
  filter_loop(l,p,Nil)
}


def append(l1:ListPair,l2:ListPair) : ListPair {
  l1.case{
    Nil => l2,
    Cons(p1:PairI64,ps:ListPair) => Cons(p1,append(ps,l2))
  }
}

def lexordset(xs:ListPair) : ListPair {
  xs.case {
    Nil => Nil ,
    Cons(a:PairI64,x:ListPair) => append(append(
      lexordset(filter(x,lexless(a))),
      Cons(a,Nil)),
    lexordset(filter(x,lexgreater(a))))
  }
}

def lexless(a:PairI64) : PredicatePair {
  cocase { ApBP(b:PairI64) => 
    a.case {
      Tup(fst1:i64,snd1:i64) => b.case{
        Tup(fst2:i64,snd2:i64) => 
          if fst2<fst1 {
            True
          }else {
            if fst2==fst1{
              if snd2<snd1 {
                True
              } else {
                False
              }
            } else { 
                False
            }
          }
      }
  }}
}

def lexgreater(a:PairI64) : PredicatePair {
  cocase { ApBP(b:PairI64) => 
    lexless(b).ApBP(a)
  }
}

def diff(x:ListPair,y:ListPair) : ListPair{
  filter(x, cocase { ApBP(p:PairI64) => not(member(y,p)) })
}

def collect_neighbors(xover:ListPair,x3:ListPair,x2:ListPair,x1:ListPair,xs:ListPair) : ListPair {
  xs.case { 
    Nil => diff(x3,xover), 
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
  }
}

def occurs3(l:ListPair) : ListPair {
  collect_neighbors(Nil,Nil,Nil,Nil,l)
}


def neighbours(p:PairI64) : ListPair {
  p.case{
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
  }
}

// Gen Functions
def alive(g : Gen) : ListPair {
  g.case { Gen(livecoords:ListPair) => livecoords }
}

def mkgen(coordlist : ListPair) : Gen {
  Gen(lexordset(coordlist))
}

def mk_nextgen_fn(gen:Gen) : Gen {
  let living : ListPair = alive(gen);
  let isalive : PredicatePair = cocase { ApBP(p:PairI64) => member(living,p) };
  let liveneighbours : FunPairI64 = cocase {ApP(p:PairI64) => len(filter(neighbours(p),isalive)) };
  let twoorthree : PredicateI64 = cocase { ApBI(n:i64) => 
    if n==2{ 
      True
    } else {
      if n==3{
        True
      }else {
        False
      }
    } 
  };
  let survivors : ListPair = filter(living,cocase{ ApBP(p:PairI64) => twoorthree.ApBI(liveneighbours.ApP(p)) });
  let newnbrlist : ListPair = collect(living, 
    cocase { ApL(p:PairI64) => filter(neighbours(p),
    cocase { ApBP(n:PairI64) => not(isalive.ApBP(n))} )});
  let newborn : ListPair = occurs3(newnbrlist);
  mkgen(append(survivors,newborn))
}

def nthgen(g:Gen, i:i64) : Gen { 
  if i==0 {
    g
  } else { 
    nthgen(mk_nextgen_fn(g), i-1)
  }
}

def gun() : Gen { 
  mkgen(
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
                          ))))))))))))))))))))))))))))))))))))))))))))
}

def go_gun() : FunIntUnit {
  cocase { ApIU(steps:i64) => 
    let gen : Gen = nthgen(gun(), steps);
    Unit
  }
}

def centerLine() : i64 {
  5
}

def bail() : ListPair {
  Cons(Tup(0,0),Cons(Tup(0,1),Cons(Tup(1,0),Cons(Tup(1,1),Nil))))
}

def shuttle() : ListPair {
  Cons(Tup(0, 3),Cons(Tup(1, 2),Cons(Tup(1, 4),Cons(Tup(2, 1), Cons(Tup(2, 5),
    Cons(Tup(3, 2), Cons(Tup(3, 3), Cons(Tup(3, 4),
      Cons(Tup(4, 1), Cons(Tup(4, 0), Cons(Tup(4, 5), Cons(Tup(4, 6),Nil
        ))))))))))))
}

def at_pos(coordlist:ListPair, p:PairI64) : ListPair {
  let move : FunPairPair = cocase { ApPP(a:PairI64) => 
    a.case { Tup(fst1:i64,snd1:i64) => 
      p.case { Tup(fst2:i64,snd2:i64) => Tup(fst1+fst2,snd1+snd2) } 
    } 
  };
  map(coordlist,move)
}

def non_steady() : Gen {
  mkgen(append(append(
    at_pos(bail(), Tup(1, centerLine())),
    at_pos(bail(), Tup(21, centerLine()))),
    at_pos(shuttle(), Tup(6, centerLine()-2))))
}

def go_shuttle() : FunIntUnit {
  cocase { ApIU(steps:i64) => 
    let gen : Gen = nthgen(non_steady(), steps);
    Unit
  }
}

def go_loop(iters:i64,steps:i64,go:FunIntUnit) : i64 {
  if iters==0{
    0
  }else{
    let res : Unit = go.ApIU(steps);
    go_loop(iters-1,steps,go)
  }
}

def main(iters:i64, steps:i64) : i64 { 
  let gun_res : i64 = go_loop(iters,steps,go_gun());
  let shuttle_res : i64 = go_loop(iters,steps,go_shuttle());
  0
}
