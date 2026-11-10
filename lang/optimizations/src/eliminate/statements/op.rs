use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::Op;

impl Eliminate for Op {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.next = self.next.eliminate(state);
        self
    }
}
