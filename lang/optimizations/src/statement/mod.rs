use super::{Inline, InlineContext};
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
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.inline(ctx).into(),
            Statement::Call(call) => call.inline(ctx).into(),
            Statement::Let(lt) => lt.inline(ctx).into(),
            Statement::Switch(sw) => sw.inline(ctx).into(),
            Statement::Create(cr) => cr.inline(ctx).into(),
            Statement::Invoke(inv) => inv.inline(ctx).into(),
            Statement::Literal(lit) => lit.inline(ctx).into(),
            Statement::Op(op) => op.inline(ctx).into(),
            Statement::PrintI64(print) => print.inline(ctx).into(),
            Statement::IfC(ifc) => ifc.inline(ctx).into(),
            Statement::Exit(ex) => ex.inline(ctx).into(),
        }
    }
}
