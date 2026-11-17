use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::{
    Def,
    statements::{Let, Statement},
};

impl Eliminate for Let {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        state
            .let_bindings
            .insert(self.var.clone(), (self.tag, self.args));
        self.next = self.next.eliminate(state, lifted);
        (self.tag, self.args) = state.let_bindings.remove(&self.var).unwrap();
        self.into()
    }
}
