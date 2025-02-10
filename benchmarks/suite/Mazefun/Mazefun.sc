data Pt { Pt(x:i64,y:i64), Empty }
data PairII { Tup(fst:i64,snd:i64) }
data ListPt { Nil, Cons(pt:Pt,pts:ListPt) }
data ListStr { NilS, ConsS(s:String,ss:ListStr) }
data ListListPt { NilL, ConsL(lpt:ListPt,lpts:ListListPt) }
data ListListStr { NilSL, ConsSL(ls :ListStr, lss:ListListStr) }
data Bool { True,False }
codata FunInInttPt { ApIIPt(x:i64) : FunIntPt }
codata FunIntPt { ApIPt(y:i64) : Pt }
codata FunPtPt { ApPP(pt:Pt) : Pt }
codata FunLLPtPt { ApLLPP(lpt:ListListPt,pt:Pt) : ListListPt }
codata FunIntIntBool { ApIIB(i:i64,j:i64) : Bool }
codata FunPtString { ApPtS(pt:Pt) : String }

def or(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => True,
    False => b2
  }
}

def tup1(tup:PairII) : i64 {
  tup.case { Tup(fst:i64,snd:i64) => fst }
}

def tup2(tup:PairII) : i64 {
  tup.case { Tup(fst:i64,snd:i64) => snd }
}

def fst(pt:Pt) : i64 {
  pt.case {
    Empty => 0, //should probably give a runtime error
    Pt(x:i64,y:i64) => x
  }
}

def snd(pt:Pt) : i64 {
  pt.case {
    Empty => 0, //should probably give a runtime error
    Pt(x:i64,y:i64) => y
  }
}

def pt_eq(pt1:Pt,pt2:Pt) : Bool { 
  pt1.case{
    Empty => pt2.case{
      Empty => True,
      Pt(x:i64,y:i64) => False
    },
    Pt(x1:i64,y1:i64) => pt2.case{
      Empty => False,
      Pt(x2:i64,y2:i64) => 
        if x1==x2 {
          if y1==y2 {
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

def fori(start:i64,end:i64,fun:FunIntPt) : ListPt {
  if start==end {
    Nil
  } else {
    Cons(fun.ApIPt(start),for(start+1,end,fun))
  }
}

def forii(start_outer:i64,end_outer:i64,start_inner:i64,end_inner:i64, fun:FunIntIntPt) : ListListPt {

  if start_outer==end_outer{
    Nil
  }else {
    ConsL(
      fori(start_inner,end_inner,fun.ApIIPt(start_outer)),
      forii(start_outer+1,end_outer,start_inner,end_inner,fun))
  }
}

def map(l:ListPt,f:FunPtPt) : ListPt {
  l.case{
    Nil => Nil,
    Cons(pt:Pt,pts:ListPt) => Cons(f.ApPP(pt),map(pts,f))
  }
}

def map_s(l:ListPt,f:FunPtString) : ListStr {
  l.case{
    Nil => NilS,
    Cons(pt:Pt,pts:ListPt) => ConsS(f.ApPtS(pt),map_s(pts,f))
  }
}

def filter_empty(l:ListPt) : ListPt {
  l.case {
    Nil => l,
    Cons(pt:Pt,pts:ListPt) => pt.case{
      Pt(x:i64,y:i64) => Cons(pt,filter_empty(pts)),
      Empty => filter_empty(pts)
    }
  }
}

def append(l1:ListPt,l2:ListPt) : ListPt {
  l1.case{
    Nil => l2,
    Cons(pt:Pt,pts:ListPt) => Cons(pt,append(pts,l2))
  }
}

def concat(l:ListListPt) : ListPt {
  l.case{
    NilL => Nil,
    ConsL(lpt:ListPt,lpts:ListListPts) => append(filter_empty(lpt),concat(lpts))
  }
}

def list_read(lst:ListPt,i:i64) : Pt {
  lst.case {
    Nil => Empty,
    Cons(pt:Pt,pts:ListPt) => if i == 0{ 
      pt
    }else{
      list_read(pts,(i-1))
    }
  }
}

def list_write(lst:ListPt,i:i64, new_pt:Pt) : ListPt { 
  lst.case{
    Nil=>Nil,
    Cons(pt:Pt,pts:ListPt) => 
      if i==0{
        Cons(new_pt,pts)
      }else {
        Cons(pt,list_write(pts,i-1,new_pt))
      }
  }
}

def list_remove_pos(lst:ListPt,i:i64) : ListPt {
  lst.case {
    Nil => Nil, // should probably be a runtime error 
    Cons(pt:Pt,pts:Pts) => 
      if i==0 {
        pts 
      } else {
        Cons(pt,list_remove_pos(pts,(i-1)))
      }
  }
}

def is_empty(lst:ListPt) : Bool {
  lst.case{
    Nil => True,
    Cons(pt:Pt,pts:ListPt) => False 
  }
}

def len(lst:ListPt) : i64 { lst.case {
  Nil => 0,
  Cons(pt:Pt,pts:ListPt) => 1+len(pts)
}
}

def member(lst:ListPt,pt:Pt) : Bool {
  lst.case{
    Nil => false,
    Cons(pt1:Pt,pts:ListPt) => or(pt_eq(pt1,pt),member(pts,pt))
  }
}

def has_duplicates(lst:ListPt) : Bool {
  lst.case {
    Nil => False,
    Cons(pt:Pt, pts:ListPt) => or(member(pts,pt),has_duplicates(xs))
  }
}

def len_l(lst:ListListPt) : i64 {
  lst.case {
    Nil => 0,
    Cons(lpt:ListPt,lpts:ListListPt) => 1+len_l(lpts)
  }
}

def next_random(cur:i64) : i64 {
  ((cur * 3581) + 12751) % 131072
}

// needs to be toplevel because of no term-level recursion
def shuf(lst:ListPt,rand:i64) : ListPt {
  is_empty(lst).case{
    True => Nil,
    False =>  
      let new_rand : i64 = next_random(rand);
      let i : i64 = new_rand % len(lst);
      Cons(list_read(lst,i),shuf(list_remove_pos(lst,i),new_rand))
  }
}

def shuffle(lst:ListPt) : ListPt {
  shuf(lst,0)
}

def neighboring_cavities(pos:Pt,cave:ListListPt) : ListPt {
  let size : PairII = matrix_size(cave);
  let n : i64 = tup1(size);
  let m : i64 = tup2(size);
  let i : i64 = fst(pos); 
  let j : i64 = snd(pos);
  let not_empty : FunIntIntBool = cocase {ApIIB(i:i64, j:i64) => 
    matrix_read(cave,i,j).case {
      Empty => False,
      Pt(x:i64,y:i64) => True
    }
  };
  concat( 
    ConsL(if 1<i   { not_empty.ApIIB(i,  j  ).case {True => Cons(Pt(i-1, j),  Nil), False => Nil} } else { Nil },
      ConsL(if i<n-1 { not_empty.ApIIB(i+1,j  ).case {True => Cons(Pt(i+1, j),  Nil), False => Nil} } else { Nil },
        ConsL(if 1<j   { not_empty.ApIIB(i,  j-1).case {True => Cons(Pt(i,   j-1),Nil), False => Nil} } else { Nil },
          ConsL(if j<m-1 { not_empty.ApIIB(i,  j+1).case {True => Cons(Pt(i,   j+1),Nil), False => Nil} } else { Nil },
            NilL)))))
}


// must be toplevel due to no term-level recusion
def change(cave:ListPtPt,pos:Pt,new_id:Pt,old_id:Pt) : ListPtPt {
  let i : i64 = fst(pos);
  let j : i64 = snd(pos);
  let cavityID : Pt = matrix_read(cave,i,j);
  pt_eq(cavityID, old_id).case{
    True => matrix_fold( 
      cocase {APLLPP(c:ListPtPt, nc:Pt) => change(c,nc,new_id,old_id) },
      matrix_write(cave,i,j,new_id),
      neighboring_cavities(pos,cave)),
    False => cave
  }
}

def change_cavity(cave:ListPtPt,pos:Pt, new_id:Pt) : ListPtPt { 
  change(cave,pos,new_id,matrix_read(cave,fst(pos),snd(pos)))
}

def pierce(pos:Pt,cave : ListPtPt) : ListListPt {
  matrix_write(cave,fst(pos),snd(pos),pos)
}

def try_to_pierce(pos:Pt,cave:ListListPt) : ListListPt {
  let ncs : ListPt = neighboring_cavities(pos,cave);
  has_duplicates(map(ncs,cocase{ApPP(pt:Pt) => matrix_read(cave,fst(pt),snd(pt))})).case{
    True => cave,
    False => pierce(pos,matrix_fold(cocase { ApLLPP(c:ListPtPt, nc:Pt) => changeCavity(c,nc,pos) },cave,ncs))
  }
}

def pierce_randomly(possible_holes:ListPt,cave:ListListPt) : ListLisPt {
  possibleHoles.case{
    Nil => cave,
    Cons(hole:Pt, rest:ListPt) => pierce_randomly(rest,(try_to_pierce(hole,cave)))
  }
}

def make_matrix(n:i64,m:i64,init:FunIntIntPt) : ListListPt {
  forii(0,n,0,m, cocase { ApIIPt(i:i64) => cocase { ApIPt(j:i64) =>init.ApIIPt(i,j) } })
}

def matrix_size(mat :ListListPt) : PairII {
  mat.case {
    NilL => Tup(0,0), // should probably give a runtime error
    Cons(lpt:ListPt,lpts:ListListPt) => Tup(len_l(mat), len(lpt))
  }
}

def matrix_read(mat:ListListPt,i:i64,j:i64) : Pt { 
  mat.case {
    NilL => Empty,
    Cons(lpt:ListPt, lpts:ListListPt) => 
      if j==0{
        list_read(lpt,i)
      } else { 
        matrix_read(lpts,j-1)
      }
  }
}

def matrix_write(mat:ListPtPt,i:i64,j:i64,new_pt:Pt) : ListPtPt {
  mat.case{
    NilL => NilL,
    ConsL(lpt:ListPt,lpts:ListPtPt) => 
      if i==0{
        ConsL(list_write(lpt,j,new_pt),lpts)
      } else {
        ConsL(lpt,matrix_write(mat,i-1,j,new_pt))
      }
  }
}

def matrix_fold(f:FunLLPtPt,start:ListPtPt,pts:ListPt) : ListListPt {
  pts.case{
    Nil => start,
    Cons(pt:Pt,pts:ListPt) => matrix_fold(f,f.ApLLPP(start,pt),pts)
  }
}

def maze_map(f:FunPtString,maze:ListPtPt) : ListListStr {
  maze.case{
    NilL => NilSL,
    ConsL(lpt:ListPt,lpts:ListListPt) => ConsSL(map_s(lpt,f),maze_map(f,lpts))
  }
}

def maze_elm2string() : FunPtString {
  cocase { ApPtS(pt:Pt) => 
    p.case {
      Pt(x:i64,y:i64) => " _",
      Empty => " *" 
    }
  }
}

def not_empty(cave,ListListPt,i:i64, j:i64) : Bool {
  matrix_read(cave,i,j).case{
    Empty => False,
    Pt(x:i64,y:i64) => True
  }
} 

def cave2maze(cave:ListPtPt) : ListListStr {
  maze_map(maze_elm2string,cave)
}

def make_maze(n:i64,m:i64) :ListListStr {
  if n%2==0{ 
    NilSL // should give an error 
  }else {
    if m%2==0 {
      NilLSL // should give an error 
    }else {
      let init : FunIntIntPt = cocase { ApIIPt(x:i64) => 
        cocase { ApIPt(y:i64) => ife(x%2,0,ife(y%2,0,Pt(x,y),Empty),Empty)} 
      };
      let cave : ListListPt = make_matrix(n,m,init);
      // this is technically not the same behaviour as the original
      // as the inner cocase should produce lists of points instead of points directly
      // this would be solved by polymorphic lists
      let possible_holes : ListPt = concat(forii(0,n,0,m,
        cocase { ApIIPt(i:i64) => 
          cocase { ApIPt(j:i64) => 
            if i%2 == 0{ 
              if j%2==0{
                Pt(i,j) 
              }else {
                Empty
              }
              }else {
                if j%2==1{
                  Pt(i,j)
                } else { 
                  Empty
                }
              }
          }
        }));
      cave_to_maze(pierce_randomly(shuffle(possible_holes),cave))
    }
  }
}

def main_loop(iters:i64,n:i64,m:i64) : i64{
  if iters==0{
    0
  } else{
    let res : ListListStr = make_maze(n,m);
    main_loop(iters-1,n,m)
  }
}

def main(iters:i64,n:i64,m:i64) : i64{
  main_loop(iters,n,m)
}
