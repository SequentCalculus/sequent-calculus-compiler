use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState, Rename};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::syntax::{Statement, names::Identifier};

use std::collections::HashSet;

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

impl Rename for Statement {
    fn rename(self, vars_to_rename: &HashSet<Identifier>, max_id: &mut usize) -> Self {
        match self {
            Statement::Substitute(subst) => subst.rename(vars_to_rename, max_id).into(),
            Statement::Call(_) => self,
            Statement::Let(lt) => lt.rename(vars_to_rename, max_id).into(),
            Statement::Switch(switch) => switch.rename(vars_to_rename, max_id).into(),
            Statement::Create(cr) => cr.rename(vars_to_rename, max_id).into(),
            Statement::Invoke(_) => self,
            Statement::Literal(lit) => lit.rename(vars_to_rename, max_id).into(),
            Statement::Op(op) => op.rename(vars_to_rename, max_id).into(),
            Statement::PrintI64(prnt) => prnt.rename(vars_to_rename, max_id).into(),
            Statement::IfC(ifc) => ifc.rename(vars_to_rename, max_id).into(),
            Statement::Exit(_) => self,
            Statement::Default() => {
                unreachable!("default to_rename, used_varsment should never occur")
            }
        }
    }
}
