use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::Statement;

mod call;
mod clause;
mod create;
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
            Statement::Exit(_) => self,
            Statement::Default() => unreachable!("default statement should never occur"),
        }
    }
}

impl CleanupInlineGather for Statement {
    type Target = Self;
    fn cleanup_inline_gather(self, state: &mut CleanupInlineState) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.cleanup_inline_gather(state).into(),
            Statement::Call(call) => call.cleanup_inline_gather(state),
            Statement::Let(lt) => lt.cleanup_inline_gather(state).into(),
            Statement::Switch(switch) => switch.cleanup_inline_gather(state).into(),
            Statement::Create(cr) => cr.cleanup_inline_gather(state).into(),
            Statement::Invoke(inv) => inv.cleanup_inline_gather(state).into(),
            Statement::Literal(lit) => lit.cleanup_inline_gather(state).into(),
            Statement::Op(op) => op.cleanup_inline_gather(state).into(),
            Statement::PrintI64(prnt) => prnt.cleanup_inline_gather(state).into(),
            Statement::IfC(ifc) => ifc.cleanup_inline_gather(state).into(),
            Statement::Exit(_) => self,
            Statement::Default() => unreachable!("default statement should never occur"),
        }
    }
}

impl CleanupInline for Statement {
    type Target = Self;
    fn cleanup_inline(self, state: &mut CleanupInlineState) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.cleanup_inline(state).into(),
            Statement::Call(call) => call.cleanup_inline(state),
            Statement::Let(lt) => lt.cleanup_inline(state).into(),
            Statement::Switch(switch) => switch.cleanup_inline(state).into(),
            Statement::Create(cr) => cr.cleanup_inline(state).into(),
            Statement::Invoke(inv) => inv.cleanup_inline(state).into(),
            Statement::Literal(lit) => lit.cleanup_inline(state).into(),
            Statement::Op(op) => op.cleanup_inline(state).into(),
            Statement::PrintI64(prnt) => prnt.cleanup_inline(state).into(),
            Statement::IfC(ifc) => ifc.cleanup_inline(state).into(),
            Statement::Exit(_) => self,
            Statement::Default() => unreachable!("default statement should never occur"),
        }
    }
}
