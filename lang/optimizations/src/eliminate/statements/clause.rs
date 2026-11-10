use std::rc::Rc;

use crate::eliminate::{Eliminate, EliminateState};
use axcut::{syntax::{Statement, TypingContext, statements::Clause}, traits::substitution::Subst};

pub fn eliminate_clause_with(clause: Clause, context: &TypingContext, state: &mut EliminateState) -> Statement {
    let subst = clause
        .context
        .into_iter_vars().map(|v| v.id)
        .zip(context.iter_vars_cloned())
        .collect::<Vec<_>>();

    Rc::unwrap_or_clone(clause.body.subst_sim(&subst).eliminate(state))
}

impl Eliminate for Clause {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.body = self.body.eliminate(state);
        self
    }
}
