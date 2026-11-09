use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::PrintI64;

impl Eliminate for PrintI64 {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.next = self.next.eliminate(state);
        self
    }
}
