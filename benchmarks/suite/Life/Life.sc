data Unit { Unit } 
data Pair[A,B] { Tup(fst:A,snd:B) }
data List[A] { Nil, Cons(a:A, as:List[A]) }
data Gen { Gen(coordslist: List[Pair[i64,i64]]) }
data Bool { True, False }
codata Fun[A,B] { Ap(a:A) : B }

// Bool Functions 

def pair_eq(p1:Pair[i64,i64],p2:Pair[i64,i64]) : Bool { 
  p1.case[i64,i64] {
    Tup(fst1,snd1) => p2.case[i64,i64]{
      Tup(fst2,snd2) => 
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

def revonto(x:List[Pair[i64,i64]], y:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  accumulate( x, y, cocase { Ap(x) => cocase { Ap(a) => Cons(a,x) } } )
}

def accumulate(a:List[Pair[i64,i64]],xs:List[Pair[i64,i64]],
  f:Fun[List[Pair[i64, i64]], Fun[Pair[i64, i64], List[Pair[i64, i64]]]]) : List[Pair[i64,i64]] {
  fold(a,xs,f)
}

def fold(a:List[Pair[i64,i64]],xs:List[Pair[i64,i64]],
  f:Fun[List[Pair[i64, i64]], Fun[Pair[i64, i64], List[Pair[i64, i64]]]]) : List[Pair[i64,i64]] {
  xs.case[Pair[i64,i64]]{
    Nil => a,
    Cons(b,x) => fold(
      f.Ap[List[Pair[i64,i64]],Fun[Pair[i64,i64],List[Pair[i64,i64]]]](a)
        .Ap[Pair[i64,i64],List[Pair[i64,i64]]](b),x,f)
  }
}

def collect_accum(sofar:List[Pair[i64,i64]], xs:List[Pair[i64,i64]], f:Fun[Pair[i64,i64],List[Pair[i64,i64]]]) : List[Pair[i64,i64]] {
  xs.case[Pair[i64,i64]]{
    Nil => sofar,
    Cons(p,xs) => collect_accum(revonto(sofar,f.Ap[Pair[i64,i64],List[Pair[i64,i64]]](p)),xs,f)
  }
}

def collect(l:List[Pair[i64,i64]],f:Fun[Pair[i64,i64],List[Pair[i64,i64]]]) : List[Pair[i64,i64]] {
  collect_accum(Nil,l,f)
}

def exists(l:List[Pair[i64,i64]],f:Fun[Pair[i64,i64],Bool]) : Bool{
  l.case[Pair[i64,i64]]{
    Nil => False,
    Cons(p,ps) => or(f.Ap[Pair[i64,i64],Bool](p),exists(ps,f))
  }
}

def rev_loop(l:List[Pair[i64,i64]],acc:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  l.case[Pair[i64,i64]]{
    Nil => acc,
    Cons(p,ps) => rev_loop(ps,Cons(p,acc))
  }
}

def rev(l:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  rev_loop(l,Nil)
}

def map_loop(l:List[Pair[i64,i64]],f:Fun[Pair[i64,i64],Pair[i64,i64]],acc:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  l.case[Pair[i64,i64]] {
    Nil => rev(acc),
    Cons(p,ps) => map_loop(ps,f,Cons(f.Ap[Pair[i64,i64],Pair[i64,i64]](p),acc))
  }
}

def map(l:List[Pair[i64,i64]],f:Fun[Pair[i64,i64],Pair[i64,i64]]) : List[Pair[i64,i64]] {
  map_loop(l,f,Nil)
}

def member(l:List[Pair[i64,i64]],p:Pair[i64,i64]) : Bool { 
  exists(l,cocase { Ap(p1) => pair_eq(p,p1) })
}

def len_loop(l:List[Pair[i64,i64]],acc:i64) : i64 {
  l.case[Pair[i64,i64]] {
    Nil => acc,
    Cons(p,ps) => len_loop(ps,acc+1)
  }
}

def len(l:List[Pair[i64,i64]]) : i64 {
  len_loop(l,0)
}

def filter_loop(l:List[Pair[i64,i64]],f:Fun[Pair[i64,i64],Bool],acc:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  l.case[Pair[i64,i64]]{
    Nil => rev(acc),
    Cons(p,ps) => filter_loop(ps,f,
      f.Ap[Pair[i64,i64],Bool](p).case{
        True => Cons(p,acc),
        False => acc
      })
  }
}

def filter(l:List[Pair[i64,i64]],p:Fun[Pair[i64,i64],Bool]) : List[Pair[i64,i64]] {
  filter_loop(l,p,Nil)
}


def append(l1:List[Pair[i64,i64]],l2:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  l1.case[Pair[i64,i64]]{
    Nil => l2,
    Cons(p1,ps) => Cons(p1,append(ps,l2))
  }
}

def lexordset(xs:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  xs.case[Pair[i64,i64]] {
    Nil => Nil ,
    Cons(a,x) => append(append(
      lexordset(filter(x,lexless(a))),
      Cons(a,Nil)),
    lexordset(filter(x,lexgreater(a))))
  }
}

def lexless(a:Pair[i64,i64]) : Fun[Pair[i64,i64],Bool] {
  cocase { Ap(b) => 
    a.case[i64,i64] {
      Tup(fst1,snd1) => b.case[i64,i64]{
        Tup(fst2,snd2) => 
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

def lexgreater(a:Pair[i64,i64]) : Fun[Pair[i64,i64],Bool] {
  cocase { Ap(b) => 
    lexless(b).Ap[Pair[i64,i64],Bool](a)
  }
}

def diff(x:List[Pair[i64,i64]],y:List[Pair[i64,i64]]) : List[Pair[i64,i64]]{
  filter(x, cocase { Ap(p) => not(member(y,p)) })
}

def collect_neighbors(xover:List[Pair[i64,i64]],x3:List[Pair[i64,i64]],x2:List[Pair[i64,i64]],x1:List[Pair[i64,i64]],xs:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  xs.case[Pair[i64,i64]] { 
    Nil => diff(x3,xover), 
    Cons(a,x) => member(xover,a).case{
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

def occurs3(l:List[Pair[i64,i64]]) : List[Pair[i64,i64]] {
  collect_neighbors(Nil,Nil,Nil,Nil,l)
}


def neighbours(p:Pair[i64,i64]) : List[Pair[i64,i64]] {
  p.case[i64,i64]{
    Tup(fst,snd) => 
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
def alive(g : Gen) : List[Pair[i64,i64]] {
  g.case { Gen(livecoords) => livecoords }
}

def mkgen(coordlist : List[Pair[i64,i64]]) : Gen {
  Gen(lexordset(coordlist))
}

def mk_nextgen_fn(gen:Gen) : Gen {
  let living : List[Pair[i64,i64]] = alive(gen);
  let isalive : Fun[Pair[i64,i64],Bool] = cocase { Ap(p) => member(living,p) };
  let liveneighbours : Fun[Pair[i64,i64],i64] = cocase {Ap(p) => len(filter(neighbours(p),isalive)) };
  let twoorthree : Fun[i64,Bool] = cocase { Ap(n) => 
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
  let survivors : List[Pair[i64,i64]] = filter(living,cocase{ Ap(p) => twoorthree.Ap[i64,Bool](liveneighbours.Ap[Pair[i64,i64],i64](p)) });
  let newnbrlist : List[Pair[i64,i64]] = collect(living, 
    cocase { Ap(p) => filter(neighbours(p),
    cocase { Ap(n) => not(isalive.Ap[Pair[i64,i64],Bool](n))} )});
  let newborn : List[Pair[i64,i64]] = occurs3(newnbrlist);
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

def go_gun() : Fun[i64,Unit] {
  cocase { Ap(steps) => 
    let gen : Gen = nthgen(gun(), steps);
    Unit
  }
}

def centerLine() : i64 {
  5
}

def bail() : List[Pair[i64,i64]] {
  Cons(Tup(0,0),Cons(Tup(0,1),Cons(Tup(1,0),Cons(Tup(1,1),Nil))))
}

def shuttle() : List[Pair[i64,i64]] {
  Cons(Tup(0, 3),Cons(Tup(1, 2),Cons(Tup(1, 4),Cons(Tup(2, 1), Cons(Tup(2, 5),
    Cons(Tup(3, 2), Cons(Tup(3, 3), Cons(Tup(3, 4),
      Cons(Tup(4, 1), Cons(Tup(4, 0), Cons(Tup(4, 5), Cons(Tup(4, 6),Nil
        ))))))))))))
}

def at_pos(coordlist:List[Pair[i64,i64]], p:Pair[i64,i64]) : List[Pair[i64,i64]] {
  let move : Fun[Pair[i64,i64],Pair[i64,i64]] = cocase { Ap(a) => 
    a.case[i64,i64] { Tup(fst1,snd1) => 
      p.case[i64,i64] { Tup(fst2,snd2) => Tup(fst1+fst2,snd1+snd2) } 
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

def go_shuttle() : Fun[i64,Unit] {
  cocase { Ap(steps) => 
    let gen : Gen = nthgen(non_steady(), steps);
    Unit
  }
}

def go_loop(iters:i64,steps:i64,go:Fun[i64,Unit]) : i64 {
  if iters==0{
    0
  }else{
    let res : Unit = go.Ap[i64,Unit](steps);
    go_loop(iters-1,steps,go)
  }
}

def main(iters:i64, steps:i64) : i64 { 
  let gun_res : i64 = go_loop(iters,steps,go_gun());
  let shuttle_res : i64 = go_loop(iters,steps,go_shuttle());
  0
}
