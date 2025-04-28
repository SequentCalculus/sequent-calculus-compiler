use std::rc::Rc;

#[derive(Debug, Clone)]
enum Tree<A> {
    Leaf(A),
    Node {
        left: Rc<Tree<A>>,
        right: Rc<Tree<A>>,
    },
}

impl<A> Tree<A> {
    fn create(i: u64, n: u64) -> Tree<u64> {
        if i < n {
            let t = Tree::<u64>::create(i + 1, n);
            Tree::Node {
                left: Rc::new(t.clone()),
                right: Rc::new(t),
            }
        } else {
            Tree::Leaf(n)
        }
    }

    fn lookup(&self) -> A
    where
        A: Clone,
    {
        match self {
            Tree::Leaf(a) => a.clone(),
            Tree::Node { left, right: _ } => left.lookup(),
        }
    }
}

fn main_loop(iters: u64, n: u64) -> i64 {
    let res = Tree::<u64>::create(0, n).lookup();
    if iters == 1 {
        println!("{:?}", res);
        0
    } else {
        main_loop(iters - 1, n)
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
    let n = args
        .next()
        .expect("Missing Argument n")
        .parse::<u64>()
        .expect("n must be a number");
    std::process::exit(main_loop(iters, n) as i32)
}
