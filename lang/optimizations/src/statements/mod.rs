use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::Statement;

mod call;
mod clause;
mod create;
mod exit;
mod ifc;
mod invoke;
mod r#let;
mod lit;
mod op;
mod print;
mod substitute;
mod switch;

impl Rewrite for Statement {
    type Target = Self;
    fn rewrite(self, state: &mut RewriteState) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.rewrite(state).into(),
            Statement::Call(call) => call.rewrite(state),
            Statement::Let(lt) => lt.rewrite(state),
            Statement::Switch(switch) => switch.rewrite(state),
            Statement::Create(cr) => cr.rewrite(state),
            Statement::Invoke(inv) => inv.rewrite(state),
            Statement::Literal(lit) => lit.rewrite(state).into(),
            Statement::Op(op) => op.rewrite(state).into(),
            Statement::PrintI64(prnt) => prnt.rewrite(state).into(),
            Statement::IfC(ifc) => ifc.rewrite(state).into(),
            Statement::Exit(ex) => ex.rewrite(state).into(),
            Statement::Default() => unreachable!("default statement should never occur"),
        }
    }
}
