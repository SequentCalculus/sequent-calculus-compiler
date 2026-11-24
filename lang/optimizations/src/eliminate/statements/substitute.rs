use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{Def, statements::Substitute};

impl Eliminate for Substitute {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        self.next = self.next.eliminate(state, lifted);
        self
    }
}
