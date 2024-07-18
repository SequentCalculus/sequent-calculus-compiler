use fun::syntax::Term;
pub mod fun;

fn main() {
    let x = Term::Var("x");
    println!("{}", x);
}
