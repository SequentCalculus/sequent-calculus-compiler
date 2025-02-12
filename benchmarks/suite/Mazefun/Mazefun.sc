data Pt { Empty, Pt(x:i64,y:i64)}
data Pair[A,B] { Tup(fst:A,snd:B) }
data List[A] { Nil, Cons(a:A,as:List[A]) }
data Bool { True,False }
codata Fun[A,B] { Ap(a:A) : B }

def or(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => True,
    False => b2
  }
}

def tup1(tup:Pair[i64,i64]) : i64 {
  tup.case[i64,i64] { Tup(fst,snd) => fst }
}

def tup2(tup:Pair[i64,i64]) : i64 {
  tup.case { Tup(fst,snd) => snd }
}

def fst(pt:Pt) : i64 {
  pt.case {
    Empty => 0, //should probably give a runtime error
    Pt(x,y) => x
  }
}

def snd(pt:Pt) : i64 {
  pt.case {
    Empty => 0, //should probably give a runtime error
    Pt(x,y) => y
  }
}

def pt_eq(pt1:Pt,pt2:Pt) : Bool { 
  pt1.case{
    Empty => pt2.case{
      Empty => True,
      Pt(x,y) => False
    },
    Pt(x1,y1) => pt2.case{
      Empty => False,
      Pt(x2,y2) => 
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

def fori(start:i64,end:i64,fun:Fun[i64,Pt]) : List[Pt] {
  if start==end {
    Nil
  } else {
    Cons(fun.Ap[i64,Pt](start),for(start+1,end,fun))
  }
}

def forii(start_outer:i64,end_outer:i64,start_inner:i64,end_inner:i64, fun:Fun[i64,Fun[i64,Pt]]) : ListListPt {

  if start_outer==end_outer{
    Nil
  }else {
    Cons(
      fori(start_inner,end_inner,fun.Ap[i64,Fun[i64,Pt]](start_outer)),
      forii(start_outer+1,end_outer,start_inner,end_inner,fun))
  }
}

def map(l:List[Pt],f:Fun[Pt,Pt]) : List[Pt] {
  l.case[Pt]{
    Nil => Nil,
    Cons(pt,pts) => Cons(f.Ap[Pt,Pt](pt),map(pts,f))
  }
}

def map_s(l:List[Pt],f:Fun[Pt,String]) : List[String] {
  l.case[Pt]{
    Nil => Nil,
    Cons(pt,pts) => Cons(f.Ap[Pt,String](pt),map_s(pts,f))
  }
}

def filter_empty(l:List[Pt]) : List[Pt] {
  l.case[Pt] {
    Nil => l,
    Cons(pt,pts) => pt.case{
      Empty => filter_empty(pts)
      Pt(x,y) => Cons(pt,filter_empty(pts)),
    }
  }
}

def append(l1:List[Pt],l2:List[Pt]) : List[Pt] {
  l1.case[Pt]{
    Nil => l2,
    Cons(pt,pts) => Cons(pt,append(pts,l2))
  }
}

def concat(l:List[List[Pt]]) : List[Pt] {
  l.case[List[Pt]]{
    Nil => Nil,
    Cons(lpt,lpts) => append(filter_empty(lpt),concat(lpts))
  }
}

def list_read(lst:List[Pt],i:i64) : Pt {
  lst.case[Pt] {
    Nil => Empty,
    Cons(pt,pts) => if i == 0{ 
      pt
    }else{
      list_read(pts,(i-1))
    }
  }
}

def list_write(lst:List[Pt],i:i64, new_pt:Pt) : List[Pt] { 
  lst.case[Pt]{
    Nil=>Nil,
    Cons(pt,pts) => 
      if i==0{
        Cons(new_pt,pts)
      }else {
        Cons(pt,list_write(pts,i-1,new_pt))
      }
  }
}

def list_remove_pos(lst:List[Pt],i:i64) : List[Pt] {
  lst.case[Pt] {
    Nil => Nil, // should probably be a runtime error 
    Cons(pt,pts) => 
      if i==0 {
        pts 
      } else {
        Cons(pt,list_remove_pos(pts,(i-1)))
      }
  }
}

def is_empty(lst:List[Pt]) : Bool {
  lst.case[Pt]{
    Nil => True,
    Cons(pt,pts) => False 
  }
}

def len(lst:List[Pt]) : i64 { 
  lst.case[Pt] {
    Nil => 0,
    Cons(pt,pts) => 1+len(pts)
  }
}

def member(lst:List[Pt],pt:Pt) : Bool {
  lst.case[Pt]{
    Nil => False,
    Cons(pt1,pts) => or(pt_eq(pt1,pt),member(pts,pt))
  }
}

def has_duplicates(lst:List[Pt]) : Bool {
  lst.case[Pt] {
    Nil => False,
    Cons(pt, pts) => or(member(pts,pt),has_duplicates(xs))
  }
}

def len_l(lst:List[List[Pt]]) : i64 {
  lst.case[List[Pt]] {
    Nil => 0,
    Cons(lpt,lpts) => 1+len_l(lpts)
  }
}

def next_random(cur:i64) : i64 {
  ((cur * 3581) + 12751) % 131072
}

// needs to be toplevel because of no term-level recursion
def shuf(lst:List[Pt],rand:i64) : List[Pt] {
  is_empty(lst).case{
    True => Nil,
    False =>  
      let new_rand : i64 = next_random(rand);
      let i : i64 = new_rand % len(lst);
      Cons(list_read(lst,i),shuf(list_remove_pos(lst,i),new_rand))
  }
}

def shuffle(lst:List[Pt]) : List[Pt] {
  shuf(lst,0)
}

def neighboring_cavities(pos:Pt,cave:List[List[Pt]]) : List[Pt] {
  let size : Pair[i64,i64] = matrix_size(cave);
  let n : i64 = tup1(size);
  let m : i64 = tup2(size);
  let i : i64 = fst(pos); 
  let j : i64 = snd(pos);
  let not_empty : Fun[i64,Fun[i64,Bool]] = cocase {Ap(i) => cocase { Ap(j) => 
    matrix_read(cave,i,j).case {
      Empty => False,
      Pt(x,y) => True
    }
  }};
  concat( 
    Cons(if 1<i { not_empty.Ap[i64,Fun[i64,Bool]](i).Ap[i64,Bool](j).case {True => Cons(Pt(i-1, j),  Nil), False => Nil} } else { Nil },
      Cons(if i<n-1 { not_empty.Ap[i64,Fun[i64,Bool]](i+1).Ap[i64,Bool](j).case {True => Cons(Pt(i+1, j),  Nil), False => Nil} } else { Nil },
        Cons(if 1<j { not_empty.Ap[i64,Fun[i64,Bool]](i).Ap[i64,Bool](j-1).case {True => Cons(Pt(i,   j-1),Nil), False => Nil} } else { Nil },
          Cons(if j<m-1 { not_empty.Ap[i64,Fun[i64,Bool]](i).Ap[i64,Bool](j+1).case {True => Cons(Pt(i,   j+1),Nil), False => Nil} } else { Nil },
            Nil)))))
}


// must be toplevel due to no term-level recusion
def change(cave:List[Pair[Pt,Pt]],pos:Pt,new_id:Pt,old_id:Pt) : List[Pair[Pt,Pt]] {
  let i : i64 = fst(pos);
  let j : i64 = snd(pos);
  let cavityID : Pt = matrix_read(cave,i,j);
  pt_eq(cavityID, old_id).case{
    True => matrix_fold( 
      cocase {Ap(c:List[Pair[Pt,Pt]], nc:Pt) => change(c,nc,new_id,old_id) },
      matrix_write(cave,i,j,new_id),
      neighboring_cavities(pos,cave)),
    False => cave
  }
}

def change_cavity(cave:List[Pair[Pt,Pt]],pos:Pt, new_id:Pt) : List[Pair[Pt,Pt]]{ 
  change(cave,pos,new_id,matrix_read(cave,fst(pos),snd(pos)))
}

def pierce(pos:Pt,cave : List[Pair[Pt,Pt]]) : List[List[Pt]] {
  matrix_write(cave,fst(pos),snd(pos),pos)
}

def try_to_pierce(pos:Pt,cave:List[List[Pt]]) : List[List[Pt]] {
  let ncs : List[Pt] = neighboring_cavities(pos,cave);
  has_duplicates(map(ncs,cocase{Ap(pt:Pt) => matrix_read(cave,fst(pt),snd(pt))})).case{
    True => cave,
    False => pierce(pos,matrix_fold(cocase { Ap(c:List[Pair[Pt,Pt]], nc:Pt) => changeCavity(c,nc,pos) },cave,ncs))
  }
}

def pierce_randomly(possible_holes:List[Pt],cave:List[List[Pt]]) : List[List[Pt]] {
  possibleHoles.case[Pt]{
    Nil => cave,
    Cons(hole:Pt, rest:List[Pt]) => pierce_randomly(rest,(try_to_pierce(hole,cave)))
  }
}

def make_matrix(n:i64,m:i64,init:Fun[i64,Fun[i64,Pt]]) : List[List[Pt]] {
  forii(0,n,0,m, cocase { Ap(i) => cocase { Ap(j) =>init.Ap[i64,Fun[i64,Pt]]](i,j) } })
}

def matrix_size(mat :List[List[Pt]]) : Pair[i64,i64] {
  mat.case[List[Pt]] {
    Nil => Tup(0,0), // should probably give a runtime error
    Cons(lpt:List[Pt],lpts:List[List[Pt]]) => Tup(len_l(mat), len(lpt))
  }
}

def matrix_read(mat:List[List[Pt]],i:i64,j:i64) : Pt { 
  mat.case[List[Pt]] {
    Nil => Empty,
    Cons(lpt:List[Pt], lpts:List[List[Pt]]) => 
      if j==0{
        list_read(lpt,i)
      } else { 
        matrix_read(lpts,j-1)
      }
  }
}

def matrix_write(mat:List[List[Pt]],i:i64,j:i64,new_pt:Pt) : List[List[Pt]] {
  mat.case[List[Pt]]{
    Nil => Nil,
    Cons(lpt,lpts) => 
      if i==0{
        Cons(list_write(lpt,j,new_pt),lpts)
      } else {
        Cons(lpt,matrix_write(mat,i-1,j,new_pt))
      }
  }
}

def matrix_fold(f:Fun[List[List[Pt]],Fun[Pt,List[Pt]]],start:ListPtPt,pts:ListPt) : List[List[Pt]] {
  pts.case[List[Pt]]{
    Nil => start,
    Cons(pt,pts) => matrix_fold(f,f.Ap[List[Pt],Fun[Pt,List[Pt]]](start).Ap[Pt,List[Pt]](pt),pts)
  }
}

def maze_map(f:Fun[Pt,String],maze:List[List[Pt]]) : List[List[Str]] {
  maze.case[List[Pt]]{
    Nil => Nil,
    Cons(lpt,lpts) => Cons(map_s(lpt,f),maze_map(f,lpts))
  }
}

def maze_elm2string() : Fun[Pt,String] {
  cocase { Ap(pt) => 
    p.case {
      Empty => " *" 
      Pt(x,y) => " _",
    }
  }
}

def not_empty(cave,List[List[Pt]],i:i64, j:i64) : Bool {
  matrix_read(cave,i,j).case{
    Empty => False,
    Pt(x,y) => True
  }
} 

def cave2maze(cave:List[List[Pt]]) : List[List[String]] {
  maze_map(maze_elm2string,cave)
}

def make_maze(n:i64,m:i64) :List[List[String]] {
  if n%2==0{ 
    Nil // should give an error 
  }else {
    if m%2==0 {
      Nil // should give an error 
    }else {
      let init : Fun[i64,Fun[i64,Pt]]= cocase { Ap(x) => 
        cocase { Ap(y) => ife(x%2,0,ife(y%2,0,Pt(x,y),Empty),Empty)} 
      };
      let cave : List[List[Pt]] = make_matrix(n,m,init);
      // this is technically not the same behaviour as the original
      // as the inner cocase should produce lists of points instead of points directly
      // this would be solved by polymorphic lists
      let possible_holes : List[Pt] = concat(forii(0,n,0,m,
        cocase { Ap(i) => 
          cocase { Ap(j) => 
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
    let res : List[List[String]] = make_maze(n,m);
    main_loop(iters-1,n,m)
  }
}

def main(iters:i64,n:i64,m:i64) : i64{
  main_loop(iters,n,m)
}
