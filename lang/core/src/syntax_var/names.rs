use super::TypeDeclaration;
use crate::traits::free_vars::FreeVars;
use crate::traits::shrink::Shrinking;
use crate::traits::substitution::SubstVar;

use std::collections::HashSet;
use std::fmt;

pub type Var = String;
pub type Name = String;

impl FreeVars for Var {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        vars.insert(self.clone());
    }
}

impl SubstVar for Var {
    type Target = Var;

    fn subst_sim(self, subst: &[(Var, Var)]) -> Var {
        match subst.iter().find(|(old, _)| *old == self) {
            None => self,
            Some((_, new)) => new.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOp {
    Prod,
    Sum,
    Sub,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Prod => write!(f, "*"),
            BinOp::Sum => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
        }
    }
}

impl Shrinking for BinOp {
    type Target = axcut::syntax::names::BinOp;

    fn shrink(
        self,
        _used_vars: &mut HashSet<Var>,
        _types: &[TypeDeclaration],
    ) -> axcut::syntax::names::BinOp {
        match self {
            BinOp::Prod => axcut::syntax::BinOp::Prod,
            BinOp::Sum => axcut::syntax::BinOp::Sub,
            BinOp::Sub => axcut::syntax::BinOp::Sum,
        }
    }
}
