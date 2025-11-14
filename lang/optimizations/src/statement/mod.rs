use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Statement, Var};
use std::collections::HashSet;

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
            Statement::Create(cr) => cr.rewrite(ctx),
            Statement::Invoke(inv) => inv.rewrite(ctx),
            Statement::Literal(lit) => Ok(lit.rewrite(ctx)?.into()),
            Statement::Op(op) => Ok(op.rewrite(ctx)?.into()),
            Statement::PrintI64(prnt) => Ok(prnt.rewrite(ctx)?.into()),
            Statement::IfC(ifc) => Ok(ifc.rewrite(ctx)?.into()),
            Statement::Exit(ex) => Ok(ex.rewrite(ctx)?.into()),
        }
    }
}

impl GetUsedVars for Statement {
    fn get_used_vars(&self) -> HashSet<Var> {
        match self {
            Statement::Substitute(subst) => subst.get_used_vars(),
            Statement::Call(call) => call.get_used_vars(),
            Statement::Let(lt) => lt.get_used_vars(),
            Statement::Switch(switch) => switch.get_used_vars(),
            Statement::Create(cr) => cr.get_used_vars(),
            Statement::Invoke(inv) => inv.get_used_vars(),
            Statement::Literal(lit) => lit.get_used_vars(),
            Statement::Op(op) => op.get_used_vars(),
            Statement::PrintI64(prnt) => prnt.get_used_vars(),
            Statement::IfC(ifc) => ifc.get_used_vars(),
            Statement::Exit(ex) => ex.get_used_vars(),
        }
    }
}
