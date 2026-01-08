use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Statement, Switch},
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Rewrite for Switch {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        match state.get_let(&self.var) {
            None => {
                self.clauses = self
                    .clauses
                    .into_iter()
                    .map(|clause| clause.rewrite(state))
                    .collect();
                self.into()
            }
            Some((xtor, args)) => {
                state.new_changes = true;

                let clause = self
                    .clauses
                    .into_iter()
                    .find(|clause| clause.xtor == xtor)
                    .unwrap_or_else(|| panic!("Could not find switch clause binding for {xtor}"));
                let subst = clause
                    .context
                    .into_iter_vars()
                    .zip(args.into_iter_vars())
                    .collect::<Vec<_>>();

                Rc::unwrap_or_clone(clause.body.subst_sim(&subst).rewrite(state))
            }
        }
    }
}
