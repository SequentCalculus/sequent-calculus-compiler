use crate::{Error, Rewrite, RewriteContext};
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
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        match self {
            Statement::Substitute(subst) => Ok(subst.rewrite(ctx)?.into()),
            Statement::Call(call) => Ok(call.rewrite(ctx)?.into()),
            Statement::Let(lt) => lt.rewrite(ctx),
            Statement::Switch(switch) => switch.rewrite(ctx),
            Statement::Create(cr) => Ok(cr.rewrite(ctx)?.into()),
            Statement::Invoke(inv) => Ok(inv.rewrite(ctx)?.into()),
            Statement::Literal(lit) => Ok(lit.rewrite(ctx)?.into()),
            Statement::Op(op) => Ok(op.rewrite(ctx)?.into()),
            Statement::PrintI64(prnt) => Ok(prnt.rewrite(ctx)?.into()),
            Statement::IfC(ifc) => Ok(ifc.rewrite(ctx)?.into()),
            Statement::Exit(ex) => Ok(ex.rewrite(ctx)?.into()),
        }
    }
}
