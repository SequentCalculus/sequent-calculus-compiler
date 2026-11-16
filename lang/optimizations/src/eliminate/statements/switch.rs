use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::syntax::{Def, statements::{Statement, Switch}};

impl Eliminate for Switch {
    type Target = Statement;
    fn eliminate(mut self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        match state.let_bindings.remove(&self.var) {
            None => {
                self.clauses = self
                    .clauses
                    .into_iter()
                    .map(|clause| clause.eliminate(state, lifted))
                    .collect();
                self.into()
            }
            Some((xtor, context)) => {
                let matching_clause = self
                    .clauses
                    .into_iter()
                    .find(|clause| clause.xtor == xtor)
                    .unwrap();
                let eliminated = eliminate_clause_with(matching_clause, &context, state, lifted);
                // Readd the xtor binding for later use
                state.let_bindings.insert(self.var, (xtor, context));
                eliminated
            }
        }
    }
}
