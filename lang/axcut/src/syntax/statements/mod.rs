pub mod call;
pub mod clause;
pub mod ifc;
pub mod ifz;
pub mod invoke;
pub mod r#let;
pub mod literal;
pub mod new;
pub mod op;
pub mod print;
pub mod ret;
pub mod substitute;
pub mod switch;

pub use call::Call;
pub use clause::{Clause, print_clauses};
pub use ifc::IfC;
pub use ifz::IfZ;
pub use invoke::Invoke;
pub use r#let::Let;
pub use literal::Literal;
pub use new::New;
pub use op::Op;
pub use print::PrintI64;
pub use ret::Return;
pub use substitute::Substitute;
pub use switch::Switch;

use printer::{Print, theme::ThemeExt, tokens::DONE};

use super::Var;
use crate::traits::free_vars::FreeVars;
use crate::traits::linearize::Linearizing;
use crate::traits::substitution::Subst;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    Substitute(Substitute),
    Call(Call),
    Let(Let),
    Switch(Switch),
    New(New),
    Invoke(Invoke),
    Literal(Literal),
    Op(Op),
    PrintI64(PrintI64),
    IfC(IfC),
    IfZ(IfZ),
    Return(Return),
    Done,
}

impl FreeVars for Statement {
    fn free_vars(self, vars: &mut HashSet<Var>) -> Self {
        match self {
            Statement::Substitute(substitute) => substitute.free_vars(vars).into(),
            Statement::Call(call) => call.free_vars(vars).into(),
            Statement::Let(r#let) => r#let.free_vars(vars).into(),
            Statement::Switch(swich) => swich.free_vars(vars).into(),
            Statement::New(new) => new.free_vars(vars).into(),
            Statement::Invoke(invoke) => invoke.free_vars(vars).into(),
            Statement::Literal(lit) => lit.free_vars(vars).into(),
            Statement::Op(op) => op.free_vars(vars).into(),
            Statement::PrintI64(print) => print.free_vars(vars).into(),
            Statement::IfC(ifc) => ifc.free_vars(vars).into(),
            Statement::IfZ(ifz) => ifz.free_vars(vars).into(),
            Statement::Return(Return { ref var }) => {
                vars.insert(var.clone());
                self
            }
            Statement::Done => self,
        }
    }
}

impl Subst for Statement {
    fn subst_sim(self, subst: &[(Var, Var)]) -> Statement {
        match self {
            Statement::Substitute(substitute) => substitute.subst_sim(subst).into(),
            Statement::Call(call) => call.subst_sim(subst).into(),
            Statement::Let(r#let) => r#let.subst_sim(subst).into(),
            Statement::Switch(switch) => switch.subst_sim(subst).into(),
            Statement::New(new) => new.subst_sim(subst).into(),
            Statement::Invoke(invoke) => invoke.subst_sim(subst).into(),
            Statement::Literal(lit) => lit.subst_sim(subst).into(),
            Statement::Op(op) => op.subst_sim(subst).into(),
            Statement::PrintI64(print) => print.subst_sim(subst).into(),
            Statement::IfC(ifc) => ifc.subst_sim(subst).into(),
            Statement::IfZ(ifz) => ifz.subst_sim(subst).into(),
            Statement::Return(ret) => ret.subst_sim(subst).into(),
            Statement::Done => self,
        }
    }
}

impl Linearizing for Statement {
    type Target = Statement;
    fn linearize(self, context: Vec<Var>, used_vars: &mut HashSet<Var>) -> Statement {
        match self {
            Statement::Substitute(_) => {
                panic!("Linearization should only be done on terms without explicit substitutions")
            }
            Statement::Call(call) => call.linearize(context, used_vars),
            Statement::Let(r#let) => r#let.linearize(context, used_vars),
            Statement::Switch(switch) => switch.linearize(context, used_vars),
            Statement::New(new) => new.linearize(context, used_vars),
            Statement::Invoke(invoke) => invoke.linearize(context, used_vars),
            Statement::Literal(lit) => lit.linearize(context, used_vars),
            Statement::Op(op) => op.linearize(context, used_vars),
            Statement::PrintI64(print) => print.linearize(context, used_vars),
            Statement::IfC(ifc) => ifc.linearize(context, used_vars).into(),
            Statement::IfZ(ifz) => ifz.linearize(context, used_vars).into(),
            Statement::Return(ref _ret) => self,
            Statement::Done => self,
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
            Statement::Let(r#let) => r#let.print(cfg, alloc),
            Statement::Switch(switch) => switch.print(cfg, alloc),
            Statement::New(new) => new.print(cfg, alloc),
            Statement::Invoke(invoke) => invoke.print(cfg, alloc),
            Statement::Literal(lit) => lit.print(cfg, alloc),
            Statement::Op(op) => op.print(cfg, alloc),
            Statement::PrintI64(print) => print.print(cfg, alloc),
            Statement::IfC(ifc) => ifc.print(cfg, alloc),
            Statement::IfZ(ifz) => ifz.print(cfg, alloc),
            Statement::Return(ret) => ret.print(cfg, alloc),
            Statement::Done => alloc.keyword(DONE),
        }
    }
}
