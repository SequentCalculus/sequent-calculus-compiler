use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::syntax::statements::{Statement, Switch};

impl Eliminate for Switch {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState) -> Self::Target {
        match state.let_bindings.remove(&self.var) {
            None => {
                self.clauses = self
                    .clauses
                    .into_iter()
                    .map(|clause| clause.eliminate(state))
                    .collect();
                self.into()
            }
            Some((xtor, context)) => {
                let clause = self
                    .clauses
                    .into_iter()
                    .find(|clause| clause.xtor == xtor)
                    .unwrap_or_else(|| panic!("Could not find switch clause binding for {}", xtor.name));
                state.let_bindings.insert(self.var.clone(), (xtor, context.clone()));
                eliminate_clause_with(clause, &context, state)
            }
        }
    }
}
