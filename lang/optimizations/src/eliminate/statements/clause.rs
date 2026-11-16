use std::rc::Rc;

use crate::eliminate::{Eliminate, EliminateState};
use axcut::{syntax::{Def, Statement, TypingContext, statements::Clause}, traits::substitution::Subst};

pub fn eliminate_clause_with(clause: Clause, xtor_context: &TypingContext, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Statement {
    let subst = clause
        .context
        .into_iter_vars().map(|v| v.id)
        .zip(xtor_context.iter_vars_cloned())
        .collect::<Vec<_>>();
    let substituted = clause.body.subst_sim(&subst);
    Rc::unwrap_or_clone(substituted.eliminate(state, lifted))
}

impl Eliminate for Clause {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        self.body = self.body.eliminate(state, lifted);
        self
    }
}
