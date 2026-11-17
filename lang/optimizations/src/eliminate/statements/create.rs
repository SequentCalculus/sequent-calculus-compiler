use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{
    Def,
    statements::{Create, Statement},
};

impl Eliminate for Create {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        let eliminated_clauses = self
            .clauses
            .into_iter()
            .map(|c| c.eliminate(state, lifted))
            .collect();
        let leaf_clauses = state.lift_clauses(eliminated_clauses, lifted, false);

        state.create_bindings.insert(self.var.clone(), leaf_clauses);
        self.next = self.next.eliminate(state, lifted);
        self.clauses = state.create_bindings.remove(&self.var).unwrap();
        self.into()
    }
}
