use crate::eliminate::{Eliminate, EliminateState};
use axcut::syntax::statements::{Let, Statement};

impl Eliminate for Let {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        state.let_bindings.insert(self.var.clone(), (self.tag.clone(), self.args.clone()));
        self.next = self.next.eliminate(state);
        self.into()
    }
}
