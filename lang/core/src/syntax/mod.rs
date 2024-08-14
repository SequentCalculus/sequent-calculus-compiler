pub mod case;
pub mod clause;
pub mod cocase;
pub mod constructor;
pub mod consumer;
pub mod covariable;
pub mod cut;
pub mod def;
pub mod destructor;
pub mod fun;
pub mod ifz;
pub mod literal;
pub mod mu;
pub mod mutilde;
pub mod names;
pub mod op;
pub mod producer;
pub mod program;
pub mod statement;
pub mod variable;

pub use case::Case;
pub use clause::Clause;
pub use cocase::Cocase;
pub use constructor::Constructor;
pub use consumer::Consumer;
pub use covariable::Covariable;
pub use cut::Cut;
pub use def::Def;
pub use destructor::Destructor;
pub use fun::Fun;
pub use ifz::IfZ;
pub use literal::Literal;
pub use mu::Mu;
pub use mutilde::MuTilde;
pub use names::{BinOp, Covar, Ctor, Dtor, Name, Var};
pub use op::Op;
pub use producer::Producer;
pub use program::Prog;
pub use statement::Statement;
pub use variable::Variable;

use std::fmt;

fn stringify_and_join<T: fmt::Display>(vec: &[T]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}
