use std::rc::Rc;

use crate::eliminate::{Eliminate, EliminateState};
use axcut::{
    syntax::{Def, Statement, TypingContext, statements::Clause},
    traits::substitution::Subst,
};

// Eliminate this matching `Clause` by substituting a given xtor context into its body
pub fn eliminate_clause_with(
    clause: Clause,
    xtor_context: TypingContext,
    state: &mut EliminateState,
    lifted: &mut Vec<Def>,
) -> Statement {
    let old = clause.context.iter_ids();
    let new = xtor_context.into_iter_vars();
    let subst = old.zip(new).collect::<Vec<_>>();
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
