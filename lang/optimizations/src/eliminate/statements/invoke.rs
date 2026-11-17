use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::syntax::{
    Def,
    statements::{Invoke, Statement},
};

impl Eliminate for Invoke {
    type Target = Statement;
    fn eliminate(self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        let Some(clauses) = state.create_bindings.get(&self.var) else {
            return self.into();
        };

        let matching_clause = clauses.iter().find(|c| c.xtor == self.tag).unwrap();
        eliminate_clause_with(matching_clause.clone(), self.args.clone(), state, lifted)
    }
}
