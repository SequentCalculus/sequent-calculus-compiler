use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::{syntax::statements::{Call, Statement}, traits::substitution::Subst};

impl Eliminate for Call {
    type Target = Statement;
    fn eliminate(self, state: &mut EliminateState) -> Self::Target {
        // here we temporarily remove the Switch from its definition
        let Some(info) = state.remove_global_switch(&self.label) else {
            return self.into();
        };

        let scrutinee_ident = self.args.bindings[info.scrutinee_pos].var.clone();
        let scrutinee = state.let_bindings.remove(&scrutinee_ident);

        match scrutinee {
            None => {
                state.readd_global_switch(self.label.clone(), info);
                self.into()
            }
            Some((xtor, context)) => {
                let clause = info.clauses.iter().find(|c| c.xtor == xtor).unwrap_or_else(|| panic!("No switch clause for xtor"));
                state.let_bindings.insert(scrutinee_ident, (xtor, context.clone()));
                let subst = info.param_ids.iter()
                    .zip(self.args.iter_vars_cloned())
                    .map(|(p, a)| (*p, a))
                    .collect::<Vec<_>>();
                eliminate_clause_with(clause.clone(), &context, state).subst_sim(&subst)
            }
        }
    }
}
