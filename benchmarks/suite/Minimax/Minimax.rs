use std::rc::Rc;

#[derive(Debug, Clone)]
enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}

impl<A> List<A> {
    fn tabulate(len: u64, f: Box<impl Fn() -> A>) -> List<A> {
        List::tabulate_loop(0, len, f)
    }

    fn tabulate_loop(n: u64, len: u64, f: Box<impl Fn() -> A>) -> List<A> {
        if n == len {
            List::Nil
        } else {
            List::Cons(f(), Rc::new(List::tabulate_loop(n + 1, len, f)))
        }
    }

    fn split_head(self) -> (A, List<A>)
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take head of empty list"),
            List::Cons(hd, tl) => (hd, Rc::unwrap_or_clone(tl)),
        }
    }

    fn tail(self) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => panic!("Cannot take tail of empty list"),
            List::Cons(_, as_) => Rc::unwrap_or_clone(as_),
        }
    }

    fn rev_acc(self, acc: List<A>) -> List<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => acc,
            List::Cons(a, as_) => Rc::unwrap_or_clone(as_).rev_acc(List::Cons(a, Rc::new(acc))),
        }
    }

    fn rev(self) -> List<A>
    where
        A: Clone,
    {
        self.rev_acc(List::Nil)
    }

    fn map<B>(self, f: Box<impl Fn(A) -> B>) -> List<B>
    where
        A: Clone,
    {
        match self {
            List::Nil => List::Nil,
            List::Cons(a, as_) => List::Cons(f(a), Rc::new(Rc::unwrap_or_clone(as_).map(f))),
        }
    }

    fn exists(&self, f: Box<dyn Fn(&A) -> bool>) -> bool {
        match self {
            List::Nil => false,
            List::Cons(a, as_) => f(a) || as_.exists(f),
        }
    }

    fn all(&self, f: Box<dyn Fn(&A) -> bool>) -> bool {
        match self {
            List::Nil => true,
            List::Cons(a, as_) => f(a) && as_.all(f),
        }
    }

    fn find(&self, i: i64) -> Option<A>
    where
        A: Clone,
    {
        match self {
            List::Nil => None,
            List::Cons(a, as_) => {
                if i == 0 {
                    Some(a.clone())
                } else {
                    as_.find(i - 1)
                }
            }
        }
    }

    fn max(self) -> A
    where
        A: PartialOrd + Clone + Default,
    {
        self.extreme(Box::new(|a, b| if b < a { a } else { b }))
    }

    fn min(self) -> A
    where
        A: PartialOrd + Clone + Default,
    {
        self.extreme(Box::new(|a, b| if a < b { a } else { b }))
    }

    fn extreme(self, f: Box<impl Fn(A, A) -> A>) -> A
    where
        A: Default + Clone,
    {
        match self {
            List::Nil => A::default(),
            List::Cons(t, ts) => Rc::unwrap_or_clone(ts).fold(f, t),
        }
    }

    fn fold<T>(self, f: Box<impl Fn(A, T) -> T>, start: T) -> T
    where
        A: Clone,
    {
        match self {
            List::Nil => start,
            List::Cons(a, as_) => {
                let new_start = f(a, start);
                Rc::unwrap_or_clone(as_).fold(f, new_start)
            }
        }
    }
}

impl<A, const N: usize> From<[A; N]> for List<A> {
    fn from(as_: [A; N]) -> List<A> {
        let mut list = List::Nil;
        for a in as_ {
            list = List::Cons(a, Rc::new(list));
        }
        list
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    X,
    O,
}

impl Player {
    fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone)]
struct RoseTree<A> {
    a: A,
    as_: Rc<List<RoseTree<A>>>,
}

impl<A> RoseTree<A> {
    fn leaf(a: A) -> RoseTree<A> {
        RoseTree {
            a,
            as_: Rc::new(List::Nil),
        }
    }

    fn top(self) -> A {
        self.a
    }
}

fn diags() -> List<List<i64>> {
    [[0, 4, 8].into(), [1, 4, 7].into(), [2, 5, 8].into()].into()
}

fn rows() -> List<List<i64>> {
    [[0, 1, 2].into(), [3, 4, 5].into(), [6, 7, 8].into()].into()
}

fn cols() -> List<List<i64>> {
    [[0, 3, 6].into(), [1, 4, 7].into(), [2, 5, 8].into()].into()
}

fn empty() -> List<Option<Player>> {
    List::<Option<Player>>::tabulate(9, Box::new(|| None))
}

fn is_full(board: &List<Option<Player>>) -> bool {
    board.all(Box::new(|p| p.is_some()))
}

fn is_win(board: &List<Option<Player>>) -> bool {
    is_win_for(board, Player::X) || is_win_for(board, Player::O)
}

fn is_cat(board: &List<Option<Player>>) -> bool {
    is_full(board) && !is_win_for(board, Player::X) && !is_win_for(board, Player::O)
}

fn game_over(board: &List<Option<Player>>) -> bool {
    is_win(board) || is_cat(board)
}

fn player_occupies(p: Player, board: &List<Option<Player>>, i: i64) -> bool {
    match board.find(i) {
        None | Some(None) => false,
        Some(Some(p0)) => p == p0,
    }
}

fn has_trip(board: &List<Option<Player>>, p: Player, l: &List<i64>) -> bool {
    let board_ = board.clone();
    l.all(Box::new(move |&i| player_occupies(p, &board_, i)))
}

fn has_row(board: &List<Option<Player>>, p: Player) -> bool {
    let board_ = board.clone();
    rows().exists(Box::new(move |l| has_trip(&board_, p, l)))
}

fn has_col(board: &List<Option<Player>>, p: Player) -> bool {
    let board_ = board.clone();
    cols().exists(Box::new(move |l| has_trip(&board_, p, l)))
}

fn has_diag(board: &List<Option<Player>>, p: Player) -> bool {
    let board_ = board.clone();
    diags().exists(Box::new(move |l| has_trip(&board_, p, l)))
}

fn is_win_for(board: &List<Option<Player>>, p: Player) -> bool {
    has_row(board, p) || has_col(board, p) || has_diag(board, p)
}

fn score(board: &List<Option<Player>>) -> i64 {
    if is_win_for(board, Player::X) {
        1
    } else if is_win_for(board, Player::O) {
        -1
    } else {
        0
    }
}

fn nth(board: &List<Option<Player>>, i: i64) -> Option<Player> {
    match board {
        List::Nil => None,
        List::Cons(p, _) if i == 0 => p.clone(),
        List::Cons(_, ps) => nth(&ps, i - 1),
    }
}

fn is_occupied(board: &List<Option<Player>>, i: i64) -> bool {
    nth(board, i).is_some()
}

fn put_at(x: Option<Player>, xs: List<Option<Player>>, i: i64) -> List<Option<Player>> {
    if i == 0 {
        List::Cons(x, Rc::new(xs.tail()))
    } else if 0 < i {
        let (hd, tl) = xs.split_head();
        List::Cons(hd, Rc::new(put_at(x, tl, i - 1)))
    } else {
        List::Nil
    }
}

fn move_to(board: &List<Option<Player>>, p: Player, i: i64) -> List<Option<Player>> {
    if is_occupied(board, i) {
        List::Nil
    } else {
        put_at(Some(p), board.clone(), i)
    }
}

fn all_moves_rec(n: i64, board: List<Option<Player>>, acc: List<i64>) -> List<i64> {
    match board {
        List::Nil => acc.rev(),
        List::Cons(p, more) => match p {
            Some(_) => all_moves_rec(n + 1, Rc::unwrap_or_clone(more), acc),
            None => all_moves_rec(
                n + 1,
                Rc::unwrap_or_clone(more),
                List::Cons(n, Rc::new(acc)),
            ),
        },
    }
}

fn all_moves(board: List<Option<Player>>) -> List<i64> {
    all_moves_rec(0, board, List::Nil)
}

fn successors(board: List<Option<Player>>, p: Player) -> List<List<Option<Player>>> {
    all_moves(board.clone()).map(Box::new(move |i| move_to(&board, p, i)))
}

fn minimax(p: Player, board: List<Option<Player>>) -> RoseTree<(List<Option<Player>>, i64)> {
    if game_over(&board) {
        let score = score(&board);
        RoseTree::leaf((board, score))
    } else {
        let trees = successors(board.clone(), p).map(Box::new(|b| minimax(p.other(), b)));
        let scores = trees
            .clone()
            .map(Box::new(|t: RoseTree<(List<Option<Player>>, i64)>| {
                t.top().1
            }));
        match p {
            Player::X => RoseTree {
                a: (board, scores.max()),
                as_: Rc::new(trees),
            },
            Player::O => RoseTree {
                a: (board, scores.min()),
                as_: Rc::new(trees),
            },
        }
    }
}

fn main_loop(iters: u64) -> i64 {
    let res = minimax(Player::X, empty());
    if iters == 1 {
        println!("{:?}", res.top().1);
        0
    } else {
        main_loop(iters - 1)
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let iters = args
        .next()
        .expect("Missing Argument iterations")
        .parse::<u64>()
        .expect("Iterations must be a number");
    std::process::exit(main_loop(iters) as i32)
}
