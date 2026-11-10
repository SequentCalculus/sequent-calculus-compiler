use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::Exit;

impl Eliminate for Exit {
    type Target = Self;
    fn eliminate(self, _: &mut EliminateState) -> Self::Target {
        self
    }
}
