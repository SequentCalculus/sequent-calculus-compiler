use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
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
    fn rewrite(self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        match self {
            Statement::Substitute(subst) => Ok(subst.rewrite(state)?.into()),
            Statement::Call(call) => Ok(call.rewrite(state)?.into()),
            Statement::Let(lt) => lt.rewrite(state),
            Statement::Switch(switch) => switch.rewrite(state),
            Statement::Create(cr) => cr.rewrite(state),
            Statement::Invoke(inv) => inv.rewrite(state),
            Statement::Literal(lit) => Ok(lit.rewrite(state)?.into()),
            Statement::Op(op) => Ok(op.rewrite(state)?.into()),
            Statement::PrintI64(prnt) => Ok(prnt.rewrite(state)?.into()),
            Statement::IfC(ifc) => Ok(ifc.rewrite(state)?.into()),
            Statement::Exit(ex) => Ok(ex.rewrite(state)?.into()),
        }
    }
}
