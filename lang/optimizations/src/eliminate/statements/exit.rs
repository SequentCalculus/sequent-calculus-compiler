use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{Def, statements::Exit};

impl Eliminate for Exit {
    type Target = Self;
    fn eliminate(self, _state: &mut EliminateState, _lifted: &mut Vec<Def>) -> Self::Target {
        self
    }
}
