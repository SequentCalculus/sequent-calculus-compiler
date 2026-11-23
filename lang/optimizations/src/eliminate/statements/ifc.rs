use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{Def, statements::IfC};

impl Eliminate for IfC {
    type Target = Self;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        self.thenc = self.thenc.eliminate(state, lifted);
        self.elsec = self.elsec.eliminate(state, lifted);
        self
    }
}
