data Bool { True, False }

data Unit { Unit }

data Option[A] { None, Some(a: A) }

data Pair[A, B] { Tup(a: A, b: B) }

data List[A] { Nil, Cons(a: A, as: List[A]) }

codata Fun[A, B] { apply(a: A): B }

codata Fun2[A, B, C] { apply2(a: A, b: B): C }

data RoseTree[A] { Rose(a: A, as: List[RoseTree[A]]) }

data Player { X, O }

def mk_leaf(p: Pair[List[Option[Player]], i64]): RoseTree[Pair[List[Option[Player]], i64]] {
    Rose(p, Nil)
}

def top(t: RoseTree[Pair[List[Option[Player]], i64]]): Pair[List[Option[Player]], i64] {
    t.case[Pair[List[Option[Player]], i64]] { Rose(p, ps) => p }
}

def snd(p: Pair[List[Option[Player]], i64]): i64 {
    p.case[List[Option[Player]], i64] { Tup(a, b) => b }
}

def player_eq(p1: Player, p2: Player): Bool {
    p1.case {
        X =>
            p2.case {
                X => True,
                O => False
            },
        O =>
            p2.case {
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

def not(b: Bool): Bool {
    b.case {
        True => False,
        False => True
    }
}

def is_some(p: Option[Player]): Bool {
    p.case[Player] {
        None => False,
        Some(p) => True
    }
}

def head(l: List[Option[Player]]): Option[Player] {
    l.case[Option[Player]] {
        Nil => None,
        Cons(p, ps) => p
    }
}

def tail(l: List[Option[Player]]): List[Option[Player]] {
    l.case[Option[Player]] {
        Nil => Nil,
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

def map_i_board(l: List[i64], f: Fun[i64, List[Option[Player]]]): List[List[Option[Player]]] {
    l.case[i64] {
        Nil => Nil,
        Cons(x, xs) => Cons(f.apply[i64, List[Option[Player]]](x), map_i_board(xs, f))
    }
}

def map_board_tree(
    l: List[List[Option[Player]]],
    f: Fun[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]]
): List[RoseTree[Pair[List[Option[Player]], i64]]] {
    l.case[List[Option[Player]]] {
        Nil => Nil,
        Cons(x, xs) =>
            Cons(
                f.apply[List[Option[Player]], RoseTree[Pair[List[Option[Player]], i64]]](x),
                map_board_tree(xs, f)
            )
    }
}

def map_tree_i(
    l: List[RoseTree[Pair[List[Option[Player]], i64]]],
    f: Fun[RoseTree[Pair[List[Option[Player]], i64]], i64]
): List[i64] {
    l.case[RoseTree[Pair[List[Option[Player]], i64]]] {
        Nil => Nil,
        Cons(x, xs) =>
            Cons(f.apply[RoseTree[Pair[List[Option[Player]], i64]], i64](x), map_tree_i(xs, f))
    }
}

def tabulate_loop(n: i64, len: i64, f: Fun[Unit, Option[Player]]): List[Option[Player]] {
    if n == len {
        Nil
    } else {
        Cons(f.apply[Unit, Option[Player]](Unit), tabulate_loop(n + 1, len, f))
    }
}

def tabulate(len: i64, f: Fun[Unit, Option[Player]]): List[Option[Player]] {
    if len < 0 { Nil } else { tabulate_loop(0, len, f) }
}

def push(l: List[i64], i: i64): List[i64] {
    l.case[i64] {
        Nil => Cons(i, Nil),
        Cons(i1, is) => Cons(i1, push(is, i))
    }
}

def nth(l: List[Option[Player]], i: i64): Option[Player] {
    l.case[Option[Player]] {
        Nil => None,
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
        Cons(is, iss) =>
            f.apply[List[i64], Bool](is)
                .case {
                    True => True,
                    False => exists(f, iss)
                }
    }
}

def all_i(f: Fun[i64, Bool], l: List[i64]): Bool {
    l.case[i64] {
        Nil => True,
        Cons(i, is) =>
            f.apply[i64, Bool](i)
                .case {
                    True => all_i(f, is),
                    False => False
                }
    }
}

def emptyBoard(): List[Option[Player]] {
    tabulate(9, new { apply(u) => None })
}

def all_board(l: List[Option[Player]], f: Fun[Option[Player], Bool]): Bool {
    l.case[Option[Player]] {
        Nil => True,
        Cons(p, ps) =>
            f.apply[Option[Player], Bool](p)
                .case {
                    True => all_board(ps, f),
                    False => False
                }
    }
}

def is_full(board: List[Option[Player]]): Bool {
    all_board(board, new { apply(p) => is_some(p) })
}

def is_cat(board: List[Option[Player]]): Bool {
    is_full(board).case {
        True =>
            not(is_win_for(board, X)).case {
                True => not(is_win_for(board, O)),
                False => False
            },
        False => False
    }
}

def fold_i(f: Fun2[i64, i64, i64], start: i64, l: List[i64]): i64 {
    l.case[i64] {
        Nil => start,
        Cons(i, is) => fold_i(f, f.apply2[i64, i64, i64](start, i), is)
    }
}

def list_extreme(f: Fun2[i64, i64, i64], l: List[i64]): i64 {
    l.case[i64] {
        Nil => 0,
        Cons(i, is) => fold_i(f, i, is)
    }
}

def listmax(l: List[i64]): i64 {
    list_extreme(new { apply2(a, b) => if b < a { a } else { b } }, l)
}

def listmin(l: List[i64]): i64 {
    list_extreme(new { apply2(a, b) => if a < b { a } else { b } }, l)
}

def rows(): List[List[i64]] {
    Cons(
        Cons(0, Cons(1, Cons(2, Nil))),
        Cons(Cons(3, Cons(4, Cons(5, Nil))), Cons(Cons(6, Cons(7, Cons(8, Nil))), Nil))
    )
}

def cols(): List[List[i64]] {
    Cons(
        Cons(0, Cons(3, Cons(6, Nil))),
        Cons(Cons(1, Cons(4, Cons(7, Nil))), Cons(Cons(2, Cons(5, Cons(8, Nil))), Nil))
    )
}

def diags(): List[List[i64]] {
    Cons(Cons(0, Cons(4, Cons(8, Nil))), Cons(Cons(2, Cons(4, Cons(6, Nil))), Nil))
}

def is_occupied(board: List[Option[Player]], i: i64): Bool {
    is_some(nth(board, i))
}

def player_occupies(p: Player, board: List[Option[Player]], i: i64): Bool {
    find(board, i).case[Player] {
        Some(p0) => player_eq(p, p0),
        None => False
    }
}

def has_trip(board: List[Option[Player]], p: Player, l: List[i64]): Bool {
    all_i(new { apply(i) => player_occupies(p, board, i) }, l)
}

def has_row(board: List[Option[Player]], p: Player): Bool {
    exists(new { apply(l) => has_trip(board, p, l) }, rows())
}

def has_col(board: List[Option[Player]], p: Player): Bool {
    exists(new { apply(l) => has_trip(board, p, l) }, cols())
}

def has_diag(board: List[Option[Player]], p: Player): Bool {
    exists(new { apply(l) => has_trip(board, p, l) }, diags())
}

def is_win_for(board: List[Option[Player]], p: Player): Bool {
    has_row(board, p).case {
        True => True,
        False =>
            has_col(board, p).case {
                True => True,
                False => has_diag(board, p)
            }
    }
}

def is_win(board: List[Option[Player]]): Bool {
    is_win_for(board, X).case {
        True => True,
        False => is_win_for(board, O)
    }
}

def game_over(board: List[Option[Player]]): Bool {
    is_win(board).case {
        True => True,
        False => is_cat(board)
    }
}

def score(board: List[Option[Player]]): i64 {
    is_win_for(board, X).case {
        True => 1,
        False =>
            is_win_for(board, O).case {
                True => -1,
                False => 0
            }
    }
}

def put_at(x: Option[Player], xs: List[Option[Player]], i: i64): List[Option[Player]] {
    if i == 0 {
        Cons(x, tail(xs))
    } else {
        if i > 0 { Cons(head(xs), put_at(x, tail(xs), i - 1)) } else { Nil }
    }
}

def move_to(board: List[Option[Player]], p: Player, i: i64): List[Option[Player]] {
    is_occupied(board, i).case {
        True => Nil,
        False => put_at(Some(p), board, i)
    }
}

def all_moves_rec(n: i64, board: List[Option[Player]], acc: List[i64]): List[i64] {
    board.case[Option[Player]] {
        Nil => rev(acc),
        Cons(p, more) =>
            p.case[Player] {
                Some(p) => all_moves_rec(n + 1, more, acc),
                None => all_moves_rec(n + 1, more, Cons(n, acc))
            }
    }
}

def all_moves(board: List[Option[Player]]): List[i64] {
    all_moves_rec(0, board, Nil)
}

def successors(board: List[Option[Player]], p: Player): List[List[Option[Player]]] {
    map_i_board(all_moves(board), new { apply(i) => move_to(board, p, i) })
}

def minimax(p: Player, board: List[Option[Player]]): RoseTree[Pair[List[Option[Player]], i64]] {
    game_over(board).case {
        True => mk_leaf(Tup(board, score(board))),
        False =>
            let trees: List[RoseTree[Pair[List[Option[Player]], i64]]] = map_board_tree(
                successors(board, p),
                new { apply(b) => minimax(other(p), b) }
            );
            let scores: List[i64] = map_tree_i(trees, new { apply(t) => snd(top(t)) });
            p.case {
                X => Rose(Tup(board, listmax(scores)), trees),
                O => Rose(Tup(board, listmin(scores)), trees)
            }
    }
}

def main_loop(iters: i64): i64 {
    let res: RoseTree[Pair[List[Option[Player]], i64]] = minimax(X, emptyBoard());
    if iters == 1 {
        println_i64(snd(top(res)));
        0
    } else {
        main_loop(iters - 1)
    }
}

def main(iters: i64): i64 {
    main_loop(iters)
}