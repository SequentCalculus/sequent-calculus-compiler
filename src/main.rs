use crate::core::syntax::Term;
pub mod core;

fn main() {
    let x = Term::Var("test");
    println!("{}", x);
}
