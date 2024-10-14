pub mod clause;
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod program;
pub mod statement;
pub mod substitution;
pub mod term;
pub mod types;

pub use clause::Clause;
pub use def::Def;
pub use names::{BinOp, Covar, Name, Var};
pub use program::Prog;
pub use statement::Statement;

use std::fmt;

fn stringify_and_join<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
