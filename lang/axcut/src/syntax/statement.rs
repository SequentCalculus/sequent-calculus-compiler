use super::{Call, IfZ, Invoke, Leta, Literal, New, Op, Return, Substitute, Switch, Var};
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::{Linearizing, UsedBinders};
use crate::traits::substitution::Subst;

use printer::{theme::ThemeExt, tokens::DONE, Print};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Substitute(Substitute),
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

impl FreeVars for Statement {
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        match self {
            Statement::Substitute(s) => s.free_vars(vars),
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
            Statement::Substitute(s) => s.subst_sim(subst).into(),
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
            Statement::Substitute(s) => s.used_binders(used),
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
            Statement::Substitute(_) => {
                panic!("Linearization should only be done on terms without explicit substitutions")
            }
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

impl Print for Statement {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        match self {
            Statement::Substitute(substitute) => substitute.print(cfg, alloc),
            Statement::Call(call) => call.print(cfg, alloc),
            Statement::Leta(leta) => leta.print(cfg, alloc),
            Statement::Switch(switch) => switch.print(cfg, alloc),
            Statement::New(n) => n.print(cfg, alloc),
            Statement::Invoke(invoke) => invoke.print(cfg, alloc),
            Statement::Literal(literal) => literal.print(cfg, alloc),
            Statement::Op(op) => op.print(cfg, alloc),
            Statement::IfZ(if_z) => if_z.print(cfg, alloc),
            Statement::Return(r) => r.print(cfg, alloc),
            Statement::Done => alloc.keyword(DONE),
        }
    }
}
