use crate::syntax_var::{TypeDeclaration, Var};
use crate::traits::{
    shrink::{Shrinking, UsedBinders},
    substitution::SubstVar,
};
use std::{collections::HashSet, fmt};

pub mod call;
pub mod cut;
pub mod ifz;
pub mod op;

pub use call::*;
pub use cut::*;
pub use ifz::*;
pub use op::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Cut(Cut),
    Op(Op),
    IfZ(IfZ),
    Call(Call),
    Done(),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Cut(c) => c.fmt(f),
            Statement::Op(op) => op.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Call(fun) => fun.fmt(f),
            Statement::Done() => write!(f, "Done"),
        }
    }
}

impl SubstVar for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Cut(c) => c.subst_sim(subst).into(),
            Statement::Op(o) => o.subst_sim(subst).into(),
            Statement::IfZ(i) => i.subst_sim(subst).into(),
            Statement::Call(f) => f.subst_sim(subst).into(),
            Statement::Done() => Statement::Done(),
        }
    }
}

impl UsedBinders for Statement {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Statement::Cut(c) => c.used_binders(used),
            Statement::Op(op) => op.used_binders(used),
            Statement::IfZ(i) => i.used_binders(used),
            _ => {}
        }
    }
}

impl Shrinking for Statement {
    type Target = axcut::syntax::Statement;

    fn shrink(
        self,
        used_vars: &mut HashSet<Var>,
        types: &[TypeDeclaration],
    ) -> axcut::syntax::Statement {
        match self {
            Statement::Cut(cut) => cut.shrink(used_vars, types),
            Statement::Op(op) => op.shrink(used_vars, types),
            Statement::IfZ(ifz) => ifz.shrink(used_vars, types),
            Statement::Call(fun) => fun.shrink(used_vars, types),
            Statement::Done() => axcut::syntax::Statement::Done,
        }
    }
}
