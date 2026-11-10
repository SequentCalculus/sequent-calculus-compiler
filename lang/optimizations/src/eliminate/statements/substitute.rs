use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::Substitute;

impl Eliminate for Substitute {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.next = self.next.eliminate(state);
        self
    }
}
