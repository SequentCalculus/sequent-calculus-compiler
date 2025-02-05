data Bool { True, False } 
data Player { X,O }
data OptionPlayer { None, Some(p:Player) }
data PairBoardScore { TupBS(board:Board,score:i64) }
data Board { NilB,ConsB(p:OptionPlayer,ps:Board) }
data ListBoard { NilLB, ConsLB(ps:Board,pss:ListBoard) }
data ListI64 { NilI,ConsI(i:i64,is:ListI64) }
data ListListI64 { NilL, ConsL(is:ListI64,iss:ListListI64) }
data ListPair { NilP, ConsP(p:PairBoardScore,ps:ListPair) }
data ListTree { NilT,ConsT(t:RoseTreePair,ts:ListTree)}
codata FunPlayerBool {ApPB(p:OptionPlayer) : Bool}
codata FunListBool { ApLLB(l:ListI64) : Bool }
codata FunIBool { ApIB(i:i64) : Bool }
codata FunIBoard { ApIBd(i:i64) : Board }
codata FunBoardTree { ApBT(b:Board) : RoseTreePair }
codata FunTreeI { ApTI(t:RoseTreePair) : i64 }
codata FunIII { ApIII(i1:i64,i2:i64) : i64 }
data RoseTreePair { Rose(p:PairBoardScore, ps:ListTree) } 

def mk_leaf(p:PairBoardScore) : RoseTreePair {
  Rose(p,NilT)
}

def top(t:RoseTreePair) : PairBoardScore {
  t.case{
    Rose(p:PairBoardScore,ps:ListTree) => p
  }
}

def snd(p:PairBoardScore) : i64  {
  p.case{
    TupBS(b:Board,score:i64) => score
  }
}

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

def is_some(p:OptionPlayer) : Bool {
  p.case{
    None => False,
    Some(p:Player) => True
  }
}

def head(l:Board) : OptionPlayer {
  l.case{
    NilB => None, //should give  a runtime error 
    ConsB(p:OptionPlayer,ps:Board) => p
  }
}

def tail(l:Board) : Board {
  l.case{
    NilB => NilB,//should give a runtime error
    ConsB(p:OptionPlayer,ps:Board) => ps
  }
}

def all_board(l:Board,f:FunPlayerBool) : Bool {
  l.case{
    NilB => True,
    ConsB(p:OptionPlayer,ps:Board) => and(f.ApPB(p),all_board(ps,f))
  }
}

def is_full(board:Board) : Bool {
  all_board(board,cocase { ApPB(p:OptionPlayer) => is_some(p)})
}

def is_cat(board:Board) : Bool {
  and(is_full(board), and(not(is_win_for(board,X)),not(is_win_for(board,O))))
}

def fold_i(f:FunIII,start:i64,l:ListI64) : i64 {
  l.case {
    NilI => start,
    ConsI(i:i64,is:ListI64) => fold_i(f,f.ApIII(start,i),is)
  }
}

def list_extreme(f:FunIII,l:ListI64) : i64 {
  l.case {
    NilI => 0,// should give a runtime error 
    ConsI(i:i64,is:ListI64) => fold_i(f,i,is)
  }
}

def listmax(l:ListI64) : i64 {
  list_extreme(cocase { ApIII(a:i64, b:i64) => if b<a {a } else {b } },l)
}
def listmin(l:ListI64) : i64 { 
  list_extreme(cocase { ApIII(a:i64, b:i64) => if a<b {a } else {b } },l)
}

// Not sure if using rev + push is the best way to reverse the list
def push(l:ListI64,i:i64) : ListI64 {
  l.case{
    NilI => ConsI(i,NilI),
    ConsI(i1:i64,is:ListI64) => ConsI(i1,push(is,i))
  }
}

def rev(l:ListI64) : ListI64 {
  l.case{
    NilI => NilI,
    ConsI(i:i64,is:ListI64) => push(rev(is),i)
  }
}

def map_i_board(l:ListI64,f:FunIBoard) : ListBoard {
  l.case{
    NilI => NilLB,
    ConsI(i:i64,is:ListI64) => ConsLB(f.ApIBd(i),map_i_board(is,f))
  }
}

def map_board_tree(l:ListBoard,f:FunBoardTree) : ListTree {
  l.case{
    NilLB => NilT,
    ConsLB(b:Board,bs:ListBoard) => ConsT(f.ApBT(b),map_board_tree(bs,f))
  }
}

def map_tree_i(l:ListTree,f:FunTreeI) : ListI64 { 
  l.case{
    NilT => NilI,
    ConsT(t:RoseTreePair,ts:ListTree) => ConsI(f.ApTI(t),map_tree_i(ts,f)),
  }
}

def nth(l:Board,i:i64) : OptionPlayer { 
  l.case{
    NilB => None, //should give a runtime error 
    ConsB(p:OptionPlayer,ps:Board) => if i==0 {p} else {nth(ps,i-1)}
  }
}

def find(l:Board,i:i64) : OptionPlayer {
  l.case {
    NilB => None,
    ConsB(p:OptionPlayer,ps:Board) => if i==0 {p} else {find(ps,i-1)}
  }
}

def exists(f:FunListBool,l:ListListI64) : Bool {
  l.case {
    NilL => False,
    ConsL(is:ListI64,iss:ListListI64) => f.ApLLB(is).case{
      True => True,
      False => exists(f,iss)
    }
  }
}

def all_i(f:FunIBool,l:ListI64) : Bool { 
  l.case{
    NilI => True,
    ConsI(i:i64,is:ListI64) => and(f.ApIB(i),all_i(f,is))
  }
}

def rows() : ListListI64 {
  ConsL(ConsI(0,ConsI(1,ConsI(2,NilI))),
    ConsL(ConsI(3,ConsI(4,ConsI(5,NilI))), 
      ConsL(ConsI(6,ConsI(7,ConsI(8,NilI))), 
        NilL)))
}

def cols() : ListListI64  {
  ConsL(ConsI(0,ConsI(3,ConsI(6,NilI))),
    ConsL(ConsI(1,ConsI(4,ConsI(7,NilI))),
      ConsL( ConsI(2,ConsI(5,ConsI(8,NilI))), 
        NilL)))
}

def diags() : ListListI64 {
  ConsL(ConsI(0,ConsI(4,ConsI(8,NilI))), 
    ConsL(ConsI(2,ConsI(4,ConsI(6,NilI))),
      NilL))
}

def is_occupied(board:Board,i:i64) : Bool { is_some(nth(board,i)) }

def player_occupies(p:Player,board:Board) : FunIBool { 
  cocase { ApIB(i:i64) => 
    find(board,i).case {
      Some(p_prime:Player) => player_eq(p, p_prime),
      None => False 
    }
  }
}

def has_trip(board:Board, p:Player) : FunListBool{ cocase {
  ApLLB(l:ListI64) =>  all_i(player_occupies(p,board),l)
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

def put_at(x:OptionPlayer, xs:Board, i:i64) : Board{
  if i==0 {
  ConsB(x,tail(xs)) }
else {
  if 0<i {
    ConsB(head(xs),put_at(x,tail(xs),i-1))
  } else {
    NilB // should give a runtime error 
  }
}
}

def move_to(board:Board, p:Player) : FunIBoard {
  cocase { ApIBd(i:i64) =>
    is_occupied(board,i).case{
      True => NilB, // should give a runtime error 
      False => put_at (Some(p), board, i)
    }
  }
}

def all_moves_rec(n:i64,board:Board,acc:ListI64) : ListI64 { 
  board.case { 
    NilB => rev(acc),
    ConsB(p:OptionPlayer,more:Board) => p.case {
      Some(p:Player) => all_moves_rec(n+1, more, acc),
      None => all_moves_rec(n+1, more, ConsI(n,acc))
    }
  }
}

def all_moves(board:Board) : ListI64 { all_moves_rec(0,board,NilI)} 

def successors(board:Board, p:Player) : ListBoard { 
  map_i_board(all_moves(board),move_to(board, p))
}

def minimax(p:Player) : FunBoardTree {
  cocase { ApBT(board:Board) => 
    game_over(board).case {
      True => mk_leaf(TupBS(board, score(board))),
      False => 
        let trees : ListTree = map_board_tree(successors(board, p),minimax(other(p)));
        let scores : ListI64 = map_tree_i(trees,cocase{ ApTI(t:RoseTreePair) => snd(top(t))});
        p.case { 
          X => Rose(TupBS(board, listmax(scores)), trees),
          O => Rose(TupBS(board, listmin(scores)), trees)
        }
    }
  }
}

def loop(iters:i64) : i64 {
  if iters==0 {
    0
  } else {
    let res:RoseTreePair=minimax(X).ApBT(NilB);
    loop(iters-1)
  }
}

def main(iters:i64) : i64 {
  loop(iters)
}
