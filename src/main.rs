use crate::core::syntax::Term;
pub mod core;

fn main() {
    let x = Term::Fun("test", vec![std::rc::Rc::new(Term::Var("x"))], vec!["y"]);
    println!("{}", x);
}
