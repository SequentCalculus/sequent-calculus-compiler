use crate::eliminate::{Eliminate, EliminateState};
use axcut::{
    syntax::statements::{Invoke, Statement},
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Eliminate for Invoke {
    type Target = Statement;
    fn eliminate(self, state: &mut EliminateState) -> Self::Target {
        let Some(clauses) = state.create_bindings.get(&self.var) else {
            return self.into();
        };

        let clause = clauses.iter().find(|c| c.xtor == self.tag).unwrap_or_else(|| panic!("No create clause for invoke"));
        let subst = clause
            .context
            .iter_vars_cloned().map(|v| v.id)
            .zip(self.args.into_iter_vars())
            .collect::<Vec<_>>();
        Rc::unwrap_or_clone(clause.body.clone().subst_sim(&subst))
    }
}
