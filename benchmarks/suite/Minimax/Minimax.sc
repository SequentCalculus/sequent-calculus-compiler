data Bool { True, False } 
data Unit { Unit }
data Option[A] { None, Some(a: A) }
data Pair[A, B] { Tup(a: A, b: B) }
data List[A] { Nil, Cons(a: A, as: List[A]) }
codata Fun[A, B] { Apply(a: A): B }
codata Fun2[A, B, C] { Apply2(a: A, b: B): C }
data RoseTree[A] { Rose(a: A, as: List[RoseTree[A]]) }
data Player { X, O }

// Tree functions

def mk_leaf(p: Pair[List[Option[Player]], i64]): RoseTree[Pair[List[Option[Player]], i64]] {
  Rose(p, Nil)
}

def top(t: RoseTree[Pair[List[Option[Player]], i64]]): Pair[List[Option[Player]], i64] {
  t.case[Pair[List[Option[Player]], i64]]{
    Rose(p, ps) => p
  }
}

// Tuple functions 

def snd(p: Pair[List[Option[Player]], i64]): i64  {
  p.case[List[Option[Player]], i64]{
    Tup(a, b) => b
  }
}

// Player functions

def player_eq(p1: Player, p2: Player): Bool {
  p1.case {
    X => p2.case {
      X => True,
      O => False
    },
    O => p2.case {
      X => False,
      O => True
    }
  }
}

def other(p: Player): Player { 
  p.case {
    X => O,
    O => X
  }
}

// Boolean functions

def and(b1: Bool, b2: Bool): Bool {
  b1.case {
    True => b2,
    False => False
  }
}

def or(b1: Bool, b2: Bool): Bool { 
  b1.case {
    True => True,
    False => b2
  }
}

def not(b: Bool): Bool { 
  b.case {
    True => False,
    False => True,
  }
}

//Option functions

def is_some(p: Option[Player]): Bool {
  p.case[Player] {
    None => False,
    Some(p) => True
  }
}

// List functions 

def head(l: List[Option[Player]]): Option[Player] {
  l.case[Option[Player]] {
    Nil => None, // should give a runtime error 
    Cons(p, ps) => p
  }
}

def tail(l: List[Option[Player]]): List[Option[Player]] {
  l.case[Option[Player]] {
    Nil => Nil, // should give a runtime error
    Cons(p, ps) => ps
  }
}

def rev_acc(l: List[i64], acc: List[i64]): List[i64] {
  l.case[i64] {
    Nil => acc,
    Cons(x, xs) => rev_acc(xs, Cons(x, acc))
  }
}

def rev(l: List[i64]): List[i64] {
  rev_acc(l, Nil)
}

def rev_i_board_acc(l: List[List[Option[Player]]], acc: List[List[Option[Player]]]): List[List[Option[Player]]] {
  l.case[List[Option[Player]]] {
    Nil => acc,
    Cons(x, xs) => rev_i_board_acc(xs, Cons(x, acc))
  }
}

def rev_i_board(l: List[List[Option[Player]]]): List[List[Option[Player]]] {
  rev_i_board_acc(l, Nil)
}

def rev_board_tree_acc(
  l: List[RoseTree[Pair[List[Option[Player]], i64]]],
  acc: List[RoseTree[Pair[List[Option[Player]], i64]]]
): List[RoseTree[Pair[List[Option[Player]], i64]]] {
  l.case[RoseTree[Pair[List[Option[Player]], i64]]] {
    Nil => acc,
    Cons(x, xs) => rev_board_tree_acc(xs, Cons(x, acc))
  }
}

def rev_board_tree(l: List[RoseTree[Pair[List[Option[Player]], i64]]]): List[RoseTree[Pair[List[Option[Player]], i64]]] {
  rev_board_tree_acc(l, Nil)
}

def map_i_board_acc(l: List[i64], f: Fun[i64, List[Option[Player]]], acc: List[List[Option[Player]]]): List[List[Option[Player]]] {
  l.case[i64] {
    Nil => rev_i_board(acc),
    Cons(x, xs) => map_i_board_acc(xs, f, Cons(f.Apply[i64, List[Option[Player]]](x), acc))
  }
}

def map_i_board(l: List[i64], f: Fun[i64, List[Option[Player]]]): List[List[Option[Player]]] {
  map_i_board_acc(l, f, Nil)
}

def map_board_tree_acc(
  l: List[List[Option[Player]]],
  f: Fun[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]],
  acc: List[RoseTree[Pair[List[Option[Player]], i64]]]
): List[RoseTree[Pair[List[Option[Player]], i64]]] {
  l.case[List[Option[Player]]] {
    Nil => rev_board_tree(acc),
    Cons(x, xs) => map_board_tree_acc(xs, f, Cons(f.Apply[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]](x), acc))
  }
}

def map_board_tree(
  l: List[List[Option[Player]]],
  f: Fun[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]]
): List[RoseTree[Pair[List[Option[Player]], i64]]] {
  map_board_tree_acc(l, f, Nil)
}

def map_tree_i_acc(
  l: List[RoseTree[Pair[List[Option[Player]], i64]]],
  f: Fun[RoseTree[Pair[List[Option[Player]], i64]], i64],
  acc: List[i64]
): List[i64] {
  l.case[RoseTree[Pair[List[Option[Player]], i64]]] {
    Nil => rev(acc),
    Cons(x, xs) => map_tree_i_acc(xs, f, Cons(f.Apply[RoseTree[Pair[List[Option[Player]], i64]], i64](x), acc))
  }
}

def map_tree_i(
  l: List[RoseTree[Pair[List[Option[Player]], i64]]],
  f: Fun[RoseTree[Pair[List[Option[Player]], i64]], i64]
): List[i64] {
  map_tree_i_acc(l, f, Nil)
}

def tabulate_loop(n: i64, len: i64, f: Fun[Unit, Option[Player]]): List[Option[Player]] {
  if n == len {
    Nil
  } else {
    Cons(f.Apply[Unit, Option[Player]](Unit), tabulate_loop(n + 1, len, f))
  }
}

def tabulate(len: i64, f: Fun[Unit, Option[Player]]): List[Option[Player]] {
  if len < 0 {
    Nil // should raise a runtime error 
  } else {
    tabulate_loop(0, len, f)
  }
}

def push(l: List[i64], i: i64): List[i64] {
  l.case[i64]{
    Nil => Cons(i, Nil),
    Cons(i1, is) => Cons(i1, push(is, i))
  }
}

def nth(l: List[Option[Player]], i: i64): Option[Player] { 
  l.case[Option[Player]] {
    Nil => None, // should give a runtime error 
    Cons(p, ps) => if i == 0 { p } else { nth(ps, i - 1) }
  }
}

def find(l: List[Option[Player]], i: i64): Option[Player] {
  l.case[Option[Player]] {
    Nil => None,
    Cons(p, ps) => if i == 0 { p } else { find(ps, i - 1) }
  }
}

def exists(f: Fun[List[i64], Bool], l: List[List[i64]]): Bool {
  l.case[List[i64]] {
    Nil => False,
    Cons(is, iss) => f.Apply[List[i64], Bool](is).case {
      True => True,
      False => exists(f, iss)
    }
  }
}

def all_i(f: Fun[i64, Bool], l: List[i64]): Bool { 
  l.case[i64] {
    Nil => True,
    Cons(i, is) => and(f.Apply[i64, Bool](i), all_i(f, is))
  }
}

// Actual functions

def empty(): List[Option[Player]] {
  tabulate(9, new { Apply(u) => None })
}

def all_board(l: List[Option[Player]], f: Fun[Option[Player], Bool]): Bool {
  l.case[Option[Player]] {
    Nil => True,
    Cons(p, ps) => and(f.Apply[Option[Player], Bool](p), all_board(ps, f))
  }
}

def is_full(board: List[Option[Player]]): Bool {
  all_board(board, new { Apply(p) => is_some(p) })
}

def is_cat(board: List[Option[Player]]): Bool {
  and(is_full(board), and(not(is_win_for(board, X)), not(is_win_for(board, O))))
}

def fold_i(f: Fun2[i64, i64, i64], start: i64, l: List[i64]): i64 {
  l.case[i64] {
    Nil => start,
    Cons(i, is) => fold_i(f, f.Apply2[i64, i64, i64](start, i), is)
  }
}

def list_extreme(f: Fun2[i64, i64, i64], l: List[i64]): i64 {
  l.case[i64] {
    Nil => 0, // should give a runtime error 
    Cons(i, is) => fold_i(f, i, is)
  }
}

def listmax(l: List[i64]): i64 {
  list_extreme(new { Apply2(a, b) => if b < a { a } else { b } }, l)
}

def listmin(l: List[i64]): i64 { 
  list_extreme(new { Apply2(a, b) => if a < b { a } else { b } }, l)
}

def rows(): List[List[i64]] {
  Cons(Cons(0, Cons(1, Cons(2, Nil))),
    Cons(Cons(3, Cons(4, Cons(5, Nil))), 
      Cons(Cons(6, Cons(7, Cons(8, Nil))), 
        Nil)))
}

def cols(): List[List[i64]] {
  Cons(Cons(0, Cons(3, Cons(6, Nil))),
    Cons(Cons(1, Cons(4, Cons(7, Nil))),
      Cons(Cons(2, Cons(5, Cons(8, Nil))), 
        Nil)))
}

def diags(): List[List[i64]] {
  Cons(Cons(0, Cons(4, Cons(8, Nil))), 
    Cons(Cons(2, Cons(4, Cons(6, Nil))),
      Nil))
}

def is_occupied(board: List[Option[Player]], i: i64): Bool { is_some(nth(board, i)) }

def player_occupies(p: Player, board: List[Option[Player]], i: i64): Bool { 
  find(board, i).case[Player] {
    Some(p0) => player_eq(p, p0),
    None => False 
  }
}

def has_trip(board: List[Option[Player]], p: Player, l: List[i64]): Bool {
  all_i(new { Apply(i) => player_occupies(p, board, i) }, l)
}

def has_row(board: List[Option[Player]], p: Player): Bool {
  exists(new { Apply(l) => has_trip(board, p, l) }, rows())
}

def has_col(board: List[Option[Player]], p: Player): Bool {
  exists(new { Apply(l) => has_trip(board, p, l) }, cols())
}

def has_diag(board: List[Option[Player]], p: Player): Bool {
  exists(new { Apply(l) => has_trip(board, p, l) }, diags())
}

def is_win_for(board: List[Option[Player]], p: Player): Bool {
  or(has_row(board, p), or(has_col(board, p), has_diag(board, p)))
}

def is_win(board: List[Option[Player]]): Bool {
  or(is_win_for(board, X), is_win_for(board, O))
}

def game_over(board: List[Option[Player]]): Bool {
  or(is_win(board), is_cat(board))
}

def score(board: List[Option[Player]]): i64 {
  is_win_for(board, X).case {
    True => 1,
    False => is_win_for(board, O).case {
      True => -1,
      False => 0
    }
  }
}

def put_at(x: Option[Player], xs: List[Option[Player]], i: i64): List[Option[Player]] {
  if i == 0 {
    Cons(x, tail(xs))
  } else {
    if 0 < i {
      Cons(head(xs), put_at(x, tail(xs), i - 1))
    } else {
      Nil // should give a runtime error 
    }
  }
}

def move_to(board: List[Option[Player]], p: Player, i: i64): List[Option[Player]] {
  is_occupied(board, i).case{
    True => Nil, // should give a runtime error 
    False => put_at(Some(p), board, i)
  }
}

def all_moves_rec(n: i64, board: List[Option[Player]], acc: List[i64]): List[i64] { 
  board.case[Option[Player]] { 
    Nil => rev(acc),
    Cons(p, more) => p.case[Player] {
      Some(p) => all_moves_rec(n + 1, more, acc),
      None => all_moves_rec(n + 1, more, Cons(n, acc))
    }
  }
}

def all_moves(board: List[Option[Player]]): List[i64] { all_moves_rec(0, board, Nil) } 

def successors(board: List[Option[Player]], p: Player): List[List[Option[Player]]] { 
  map_i_board(all_moves(board), new { Apply(i) => move_to(board, p, i) })
}

def minimax(p: Player, board: List[Option[Player]]): RoseTree[Pair[List[Option[Player]], i64]] {
  game_over(board).case {
    True => mk_leaf(Tup(board, score(board))),
    False => 
      let trees: List[RoseTree[Pair[List[Option[Player]], i64]]] = map_board_tree(successors(board, p), new { Apply(b) => minimax(other(p), b) });
      let scores: List[i64] = map_tree_i(trees, new{ Apply(t) => snd(top(t)) });
      p.case { 
        X => Rose(Tup(board, listmax(scores)), trees),
        O => Rose(Tup(board, listmin(scores)), trees)
      }
  }
}

def main_loop(iters: i64): i64{
  if iters == 0 {
    0
  } else {
    let res: RoseTree[Pair[List[Option[Player]], i64]] = minimax(X, empty());
    main_loop(iters - 1)
  }
}

def main(iters: i64): i64 {
  main_loop(iters)
}
