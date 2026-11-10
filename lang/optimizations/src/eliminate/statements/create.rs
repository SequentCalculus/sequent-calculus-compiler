use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::{Create, Statement};

// TODO: lift only when used?
impl Eliminate for Create {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        let mut lifted = Vec::with_capacity(self.clauses.len());
        self.clauses = state.lift_clauses(self.clauses, &mut lifted);
        state.defs.extend(lifted);

        state.create_bindings.insert(self.var.clone(), self.clauses);
        self.next = self.next.eliminate(state);
        self.clauses = state.create_bindings.remove(&self.var).unwrap();
        self.into()
    }
}
