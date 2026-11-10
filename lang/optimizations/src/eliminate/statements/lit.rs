use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::Literal;

impl Eliminate for Literal {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.next = self.next.eliminate(state);
        self
    }
}
