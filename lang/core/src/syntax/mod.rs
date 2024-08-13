pub mod case;
pub mod clause;
pub mod cocase;
pub mod constructor;
pub mod consumer;
pub mod covariable;
pub mod cut;
pub mod destructor;
pub mod fun;
pub mod ifz;
pub mod literal;
pub mod mu;
pub mod mutilde;
pub mod names;
pub mod op;
pub mod producer;
pub mod statement;
pub mod variable;

pub use case::Case;
pub use clause::Clause;
pub use cocase::Cocase;
pub use constructor::Constructor;
pub use consumer::Consumer;
pub use covariable::Covariable;
pub use cut::Cut;
pub use destructor::Destructor;
pub use fun::Fun;
pub use ifz::IfZ;
pub use literal::Literal;
pub use mu::Mu;
pub use mutilde::MuTilde;
pub use names::{BinOp, Covar, Ctor, Dtor, Name, Var};
pub use op::Op;
pub use producer::Producer;
pub use statement::Statement;
pub use variable::Variable;

use std::fmt;

// Def
//
//

#[derive(Debug, Clone)]
pub struct Def<T> {
    pub name: Name,
    pub pargs: Vec<(Var, T)>,
    pub cargs: Vec<(Covar, T)>,
    pub body: Statement,
}

impl<T> std::fmt::Display for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pargs_joined: String = self
            .pargs
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let cargs_joined: String = self
            .cargs
            .iter()
            .map(|(x, _)| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(
            f,
            "def {}({};{}) := {};",
            self.name, pargs_joined, cargs_joined, self.body
        )
    }
}

// Prog
//
//

#[derive(Debug, Clone)]
pub struct Prog<T> {
    pub prog_defs: Vec<Def<T>>,
}

impl<T> fmt::Display for Prog<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let defs_joined: String = self
            .prog_defs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", defs_joined)
    }
}
