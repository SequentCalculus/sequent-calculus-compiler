pub mod clause;
pub mod consumer;
pub mod context;
pub mod declaration;
pub mod def;
pub mod names;
pub mod producer;
pub mod program;
pub mod statement;
pub mod substitution;
pub mod types;

pub use clause::Clause;
pub use consumer::case::Case;
pub use consumer::covariable::Covariable;
pub use consumer::destructor::Destructor;
pub use consumer::mutilde::MuTilde;
pub use consumer::Consumer;
pub use def::Def;
pub use names::{BinOp, Covar, Name, Var};
pub use producer::cocase::Cocase;
pub use producer::constructor::Constructor;
pub use producer::literal::Literal;
pub use producer::mu::Mu;
pub use producer::variable::Variable;
pub use producer::Producer;
pub use program::Prog;
pub use statement::Statement;

use std::fmt;

fn stringify_and_join<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
