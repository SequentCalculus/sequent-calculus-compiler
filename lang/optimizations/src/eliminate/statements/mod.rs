use crate::eliminate::{Eliminate, EliminateState};
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

impl Eliminate for Statement {
    type Target = Self;
    fn eliminate(self, state: &mut EliminateState) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.eliminate(state).into(),
            Statement::Call(call) => call.eliminate(state),
            Statement::Let(lt) => lt.eliminate(state),
            Statement::Switch(switch) => switch.eliminate(state),
            Statement::Create(cr) => cr.eliminate(state),
            Statement::Invoke(inv) => inv.eliminate(state),
            Statement::Literal(lit) => lit.eliminate(state).into(),
            Statement::Op(op) => op.eliminate(state).into(),
            Statement::PrintI64(prnt) => prnt.eliminate(state).into(),
            Statement::IfC(ifc) => ifc.eliminate(state).into(),
            Statement::Exit(ex) => ex.eliminate(state).into(),
        }
    }
}
