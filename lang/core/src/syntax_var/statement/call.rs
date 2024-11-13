use crate::{
    syntax_var::{stringify_and_join, Name, Statement, Var},
    traits::substitution::SubstVar,
};

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub name: Name,
    pub args: Vec<Var>,
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined = stringify_and_join(&self.args, ", ");
        write!(f, "{}({})", self.name, args_joined)
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl SubstVar for Call {
    type Target = Call;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Call {
        Call {
            name: self.name,
            args: self.args.subst_sim(subst),
        }
    }
}
