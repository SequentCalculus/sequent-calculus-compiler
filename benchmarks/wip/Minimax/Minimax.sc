data Bool { True, False } 
data Unit { Unit }
data Option[A] { None, Some(a:A) }
data Pair[A,B] { Tup(a:A,b:B) }
data List[A] { Nil,Cons(a:A,as:List[A]) }
codata Fun[A,B] { Ap(a:A) : B}
data RoseTree[A] { Rose(a:A,as:List[RoseTree[A]]) }
data Player { X,O }

type Board = List[Option[Player]];

// Tree Functions
def mk_leaf(p:Pair[Board,i64]) : RoseTree[Pair[Board,i64]] {
  Rose(p,NilT)
}

def top(t:RoseTree[Pair[Board,i64]]) : Pair[Board,i64] {
  t.case[Pair[Board,i64]]{
    Rose(p,ps) => p
  }
}

// Tuple Functions 
def snd(p:Pair[Board,i64]) : i64  {
  p.case[Board,i64]{
    Tup(b,score) => score
  }
}

// Player Functions
def player_eq(p1:Player,p2:Player) : Bool {
  p1.case {
    X => p2.case{
      X => True,
      O => False
    },
    O => p2.case{
      X => False,
      O => True
    }
  }
}

def other(p:Player) : Player { 
  p.case {
    X => O,
    O => X
  }
}

// Boolean Functions
def or(b1:Bool,b2:Bool) : Bool { 
  b1.case{
    True => True,
    False => b2
  }
}

def not(b:Bool) : Bool { 
  b.case{
    True => False,
    False => True,
  }
}

def and(b1:Bool,b2:Bool) : Bool {
  b1.case{
    True => b2,
    False => False
  }
}

//Option Functions
def is_some(p:Option[Player]) : Bool {
  p.case[Player]{
    None => False,
    Some(p) => True
  }
}

// List Functions 

def tabulate_loop(n:i64,len:i64,f:Fun[Unit,Option[Player]]) : Board{
  if n==len{
    Nil
  }else{
    Cons(f.Ap[Unit,Option[Player]](Unit),tabulate_loop(n+1,len,f))
  }
}

def tabulate(len:i64,f:Fun[Unit,Option[Player]]) : Board{
  if len<0{
    Nil // should rais a runtime error 
  } else {
    tabulate_loop(0,len,f)
  }
}

def empty() : Board {
  tabulate(9,new { Ap(u:Unit) => None })
}

def head(l:Board) : Option[Player] {
  l.case[Option[Player]]{
    Nil => None, //should give  a runtime error 
    Cons(p,ps) => p
  }
}

def tail(l:Board) : Board {
  l.case[Option[Player]]{
    Nil => Nil,//should give a runtime error
    Cons(p,ps) => ps
  }
}

def all_board(l:Board,f:Fun[Option[Player],Bool]) : Bool {
  l.case[Option[Player]]{
    Nil => True,
    Cons(p,ps) => and(f.Ap[Option[Player],Bool](p),all_board(ps,f))
  }
}

def is_full(board:Board) : Bool {
  all_board(board,new { ApPB(p:Option[Player]) => is_some(p)})
}

def is_cat(board:Board) : Bool {
  and(is_full(board), and(not(is_win_for(board,X)),not(is_win_for(board,O))))
}

def fold_i(f:Fun[i64,Fun[i64,i64]],start:i64,l:List[i64]) : i64 {
  l.case[i64] {
    Nil => start,
    Cons(i,is) => fold_i(f,f.Ap[i64,Fun[i64,i64]](start).Ap[i64,i64](i),is)
  }
}

def list_extreme(f:Fun[i64,Fun[i64,i64]],l:List[i64]) : i64 {
  l.case[i64] {
    Nil => 0,// should give a runtime error 
    Cons(i,is) => fold_i(f,i,is)
  }
}

def listmax(l:List[i64]) : i64 {
  list_extreme(new { Ap(a) => new { Ap(b) => if b<a { a } else { b } } },l)
}
def listmin(l:List[i64]) : i64 { 
  list_extreme(new { Ap(a) => new { Ap(b) => if a<b { a } else { b } } },l)
}

def push(l:List[i64],i:i64) : List[i64] {
  l.case[i64]{
    Nil => Cons(i,Nil),
    Cons(i1,is) => Cons(i1,push(is,i))
  }
}

def rev(l:List[i64]) : List[i64] {
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => push(rev(is),i)
  }
}

def map_i_board(l:List[i64],f:Fun[i64,Board]) : List[Board] {
  l.case[i64]{
    Nil => Nil,
    Cons(i,is) => Cons(f.Ap[i64,Board](i),map_i_board(is,f))
  }
}

def map_board_tree(l:List[Board],f:Fun[Board,RoseTree[Pair[Board,i64]]]) : List[RoseTree[Board,i64]] {
  l.case[Board]{
    Nil => Nil,
    Cons(b,bs) => Cons(f.Ap[Board,RoseTree[Pair[Board,i64]]](b),map_board_tree(bs,f))
  }
}

def map_tree_i(l:List[RoseTree[Pair[Board,i64]]],f:Fun[RoseTree[Pair[Board,i64]],i64]) : List[i64] { 
  l.case[RoseTree[Pair[Board,i64]]] {
    Nil => Nil,
    Cons(t,ts) => Cons(f.Ap[RoseTree[Pair[Board,i64]],i64](t),map_tree_i(ts,f)),
  }
}

def nth(l:Board,i:i64) : Option[Player] { 
  l.case[Option[Player]]{
    Nil => None, //should give a runtime error 
    Cons(p,ps) => if i==0 {p} else {nth(ps,i-1)}
  }
}

def find(l:Board,i:i64) : Option[Player] {
  l.case[Option[Player]] {
    Nil => None,
    Cons(p,ps) => if i==0 {p} else {find(ps,i-1)}
  }
}

def exists(f:Fun[List[List[i64]],Bool],l:List[List[i64]]) : Bool {
  l.case[List[i64]] {
    Nil => False,
    Cons(is:List[i64],iss:List[List[i64]]) => f.Ap[List[List[i64]],Bool](is).case{
      True => True,
      False => exists(f,iss)
    }
  }
}

def all_i(f:Fun[i64,Bool],l:List[i64]) : Bool { 
  l.case[i64] {
    Nil => True,
    Cons(i,is) => and(f.Ap[i64,Bool](i),all_i(f,is))
  }
}

def rows() : List[List[i64]] {
  Cons(Cons(0,Cons(1,Cons(2,Nil))),
    Cons(Cons(3,Cons(4,Cons(5,Nil))), 
      Cons(Cons(6,Cons(7,Cons(8,Nil))), 
        Nil)))
}

def cols() : List[List[i64]] {
  Cons(Cons(0,Cons(3,Cons(6,Nil))),
    Cons(Cons(1,Cons(4,Cons(7,Nil))),
      Cons( Cons(2,Cons(5,Cons(8,Nil))), 
        Nil)))
}

def lookup_trans(p:Player,board:Board) {
  TODO
}

def diags() : List[List[i64]] {
  Cons(Cons(0,Cons(4,Cons(8,Nil))), 
    Cons(Cons(2,Cons(4,Cons(6,Nil))),
      Nil))
}

def is_occupied(board:Board,i:i64) : Bool { is_some(nth(board,i)) }

def player_occupies(p:Player,board:Board) : Fun[i64,Bool] { 
  new { ApIB(i) => 
    find(board,i).case[Player] {
      Some(p_prime) => player_eq(p, p_prime),
      None => False 
    }
  }
}

def has_trip(board:Board, p:Player) : Fun[List[i64],Bool] { new {
  Ap(l) =>  all_i(player_occupies(p,board),l)
}
}

def has_row(board:Board, p:Player) : Bool {
  exists(has_trip(board, p),rows())
}

def has_col(board:Board, p:Player) : Bool {
  exists(has_trip(board, p),cols())
}

def has_diag(board:Board, p:Player) : Bool {
  exists(has_trip(board, p),diags())
}

def is_win_for(board:Board,p:Player) : Bool {
  or(has_row(board,p),or(has_col(board,p),has_diag(board,p)))
}

def is_win(board:Board) : Bool {
  or(is_win_for(board,X),is_win_for(board,O))
}

def game_over(board:Board) : Bool {
  or(is_win(board),is_cat(board))
}

def score(board:Board) : i64 {
  is_win_for(board,X).case{
    True => 1,
    False => is_win_for(board,O).case{
      True => -1,
      False => 0
    }
  }
}

def put_at(x:Option[Player], xs:Board, i:i64) : Board{
  if i==0 {
  Cons(x,tail(xs)) }
else {
  if 0<i {
    Cons(head(xs),put_at(x,tail(xs),i-1))
  } else {
    Nil // should give a runtime error 
  }
}
}

def move_to(board:Board, p:Player) : Fun[i64,Board] {
  new { Ap(i) =>
    is_occupied(board,i).case{
      True => Nil, // should give a runtime error 
      False => put_at(Some(p), board, i)
    }
  }
}

def all_moves_rec(n:i64,board:Board,acc:List[i64]) : List[i64] { 
  board.case[Option[Player]] { 
    Nil => rev(acc),
    Cons(p,more) => p.case[Player] {
      Some(p) => all_moves_rec(n+1, more, acc),
      None => all_moves_rec(n+1, more, Cons(n,acc))
    }
  }
}

def all_moves(board:Board) : List[i64] { all_moves_rec(0,board,Nil)} 

def successors(board:Board, p:Player) : List[Board] { 
  map_i_board(all_moves(board),move_to(board, p))
}

def minimax(p:Player) : Fun[Board,RoseTree[Pair[Board,i64]]] {
  new { Ap(board) => 
    game_over(board).case {
      True => mk_leaf(Tup(board, score(board))),
      False => 
        let trees : List[RoseTree[Pair[Board,i64]]] = map_board_tree(successors(board, p),minimax(other(p)));
        let scores : List[i64] = map_tree_i(trees,new{ Ap(t) => snd(top(t))});
        p.case { 
          X => Rose(Tup(board, listmax(scores)), trees),
          O => Rose(Tup(board, listmin(scores)), trees)
        }
    }
  }
}

def minimax_trans(p:Player) : Fun[Board,RoseTree[Pair[Board,i64]]]{
  new { Ap(board) => 
    game_over(board).case{
      True => mk_leaf(Tup(board,score(board))),
      False => TODO
    }
  }
}

def main_loop(iters:i64) : i64{
  if iters==0{
    0
  }else{
    let res : RoseTree[Pair[Board,i64]] = minimax(X).Ap(empty());
    let res : RoseTree[Pair[Board,i64]] = minimax_trans(X).Ap(emtpy());
    main_loop(iters-1)
  }
}

def main(iters:i64) : i64 {
  main_loop(iters)
}
