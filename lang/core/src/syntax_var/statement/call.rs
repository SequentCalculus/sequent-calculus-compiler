use crate::{
    syntax_var::{stringify_and_join, Name, TypeDeclaration, Var},
    traits::{free_vars::FreeVars, shrink::Shrinking, substitution::SubstVar},
};
use std::{collections::HashSet, fmt};

use super::Statement;

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

impl FreeVars for Call {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        self.args.free_vars(vars);
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

impl Shrinking for Call {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        axcut::syntax::Statement::Call(axcut::syntax::Call {
            label: self.name,
            args: self.args,
        })
    }
}
