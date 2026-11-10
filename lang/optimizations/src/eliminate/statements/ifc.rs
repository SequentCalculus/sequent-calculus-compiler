use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::IfC;

impl Eliminate for IfC {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        self.thenc = self.thenc.eliminate(state);
        self.elsec = self.elsec.eliminate(state);
        self
    }
}
