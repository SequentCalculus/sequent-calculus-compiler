use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{Def, statements::Op};

impl Eliminate for Op {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        self.next = self.next.eliminate(state, lifted);
        self
    }
}
