use super::{Call, IfZ, Invoke, Leta, Literal, New, Op, Switch};
use crate::syntax::{Return, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Call(Call),
    Leta(Leta),
    Switch(Switch),
    New(New),
    Invoke(Invoke),
    Literal(Literal),
    Op(Op),
    IfZ(IfZ),
    Return(Return),
    Done,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Call(j) => j.fmt(f),
            Statement::Leta(l) => l.fmt(f),
            Statement::Switch(s) => s.fmt(f),
            Statement::New(n) => n.fmt(f),
            Statement::Invoke(i) => i.fmt(f),
            Statement::Literal(n) => n.fmt(f),
            Statement::Op(o) => o.fmt(f),
            Statement::IfZ(i) => i.fmt(f),
            Statement::Return(r) => r.fmt(f),
            Statement::Done => write!(f, "Done"),
        }
    }
}

impl FreeVars for Statement {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Statement::Call(c) => c.free_vars(vars),
            Statement::Leta(l) => l.free_vars(vars),
            Statement::Switch(s) => s.free_vars(vars),
            Statement::New(n) => n.free_vars(vars),
            Statement::Invoke(i) => i.free_vars(vars),
            Statement::Literal(n) => n.free_vars(vars),
            Statement::Op(o) => o.free_vars(vars),
            Statement::IfZ(i) => i.free_vars(vars),
            Statement::Return(Return { var }) => {
                vars.insert(var.clone());
            }
            Statement::Done => {}
        }
    }
}

impl Subst for Statement {
    type Target = Statement;
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Call(c) => c.subst_sim(subst).into(),
            Statement::Leta(l) => l.subst_sim(subst).into(),
            Statement::Switch(s) => s.subst_sim(subst).into(),
            Statement::New(n) => n.subst_sim(subst).into(),
            Statement::Invoke(i) => i.subst_sim(subst).into(),
            Statement::Literal(n) => n.subst_sim(subst).into(),
            Statement::Op(o) => o.subst_sim(subst).into(),
            Statement::IfZ(i) => i.subst_sim(subst).into(),
            Statement::Return(Return { var }) => Statement::Return(Return {
                var: var.subst_sim(subst),
            }),
            Statement::Done => Statement::Done,
        }
    }
}

impl UsedBinders for Statement {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        match self {
            Statement::Leta(l) => l.used_binders(used),
            Statement::Switch(s) => s.used_binders(used),
            Statement::New(n) => n.used_binders(used),
            Statement::Literal(n) => n.used_binders(used),
            Statement::Op(o) => o.used_binders(used),
            Statement::IfZ(i) => i.used_binders(used),
            _ => {}
        }
    }
}

impl Linearizing for Statement {
    type Target = crate::syntax::Statement;
    fn linearize(
        self,
        context: Vec<Var>,
        used_vars: &mut HashSet<Var>,
    ) -> crate::syntax::Statement {
        match self {
            Statement::Call(j) => j.linearize(context, used_vars).into(),
            Statement::Leta(l) => l.linearize(context, used_vars).into(),
            Statement::Switch(s) => s.linearize(context, used_vars).into(),
            Statement::New(n) => n.linearize(context, used_vars).into(),
            Statement::Invoke(i) => i.linearize(context, used_vars).into(),
            Statement::Literal(n) => n.linearize(context, used_vars).into(),
            Statement::Op(o) => o.linearize(context, used_vars).into(),
            Statement::IfZ(i) => i.linearize(context, used_vars).into(),
            Statement::Return(Return { var }) => Return { var }.into(),
            Statement::Done => crate::syntax::Statement::Done,
        }
    }
}
