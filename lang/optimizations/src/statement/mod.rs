use super::{Error, Inline, InlineContext};
use axcut::syntax::Statement;

mod call;
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

impl Inline for Statement {
    type Target = Statement;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        match self {
            Statement::Substitute(subst) => Ok(subst.inline(ctx)?.into()),
            Statement::Call(call) => Ok(call.inline(ctx)?.into()),
            Statement::Let(lt) => Ok(lt.inline(ctx)?.into()),
            Statement::Switch(sw) => Ok(sw.inline(ctx)?.into()),
            Statement::Create(cr) => Ok(cr.inline(ctx)?.into()),
            Statement::Invoke(inv) => Ok(inv.inline(ctx)?.into()),
            Statement::Literal(lit) => Ok(lit.inline(ctx)?.into()),
            Statement::Op(op) => Ok(op.inline(ctx)?.into()),
            Statement::PrintI64(print) => Ok(print.inline(ctx)?.into()),
            Statement::IfC(ifc) => Ok(ifc.inline(ctx)?.into()),
            Statement::Exit(ex) => Ok(ex.inline(ctx)?.into()),
        }
    }
}
