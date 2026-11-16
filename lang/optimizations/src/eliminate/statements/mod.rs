use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{Def, Statement};

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
    fn eliminate(self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        match self {
            Statement::Substitute(subst) => subst.eliminate(state, lifted).into(),
            Statement::Call(call) => call.eliminate(state, lifted),
            Statement::Let(lt) => lt.eliminate(state, lifted),
            Statement::Switch(switch) => switch.eliminate(state, lifted),
            Statement::Create(cr) => cr.eliminate(state, lifted),
            Statement::Invoke(inv) => inv.eliminate(state, lifted),
            Statement::Literal(lit) => lit.eliminate(state, lifted).into(),
            Statement::Op(op) => op.eliminate(state, lifted).into(),
            Statement::PrintI64(prnt) => prnt.eliminate(state, lifted).into(),
            Statement::IfC(ifc) => ifc.eliminate(state, lifted).into(),
            Statement::Exit(ex) => ex.eliminate(state, lifted).into(),
        }
    }
}
