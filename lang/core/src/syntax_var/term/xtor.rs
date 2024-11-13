use super::Term;
use crate::{
    syntax_var::{stringify_and_join, Name, Var},
    traits::substitution::SubstVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Xtor {
    pub id: Name,
    pub args: Vec<Var>,
}

impl std::fmt::Display for Xtor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args_joined: String = stringify_and_join(&self.args, ", ");
        write!(f, "{}({})", self.id, args_joined)
    }
}

impl From<Xtor> for Term {
    fn from(value: Xtor) -> Self {
        Term::Xtor(value)
    }
}

impl SubstVar for Xtor {
    type Target = Xtor;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Self::Target {
        Xtor {
            id: self.id,
            args: self.args.subst_sim(subst),
        }
    }
}
