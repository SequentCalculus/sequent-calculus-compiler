use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::{syntax::statements::{Call, Statement}, traits::substitution::Subst};

impl Eliminate for Call {
    type Target = Statement;
    fn eliminate(self, state: &mut EliminateState) -> Self::Target {
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
            Some((xtor, xtor_context)) => {
                let matching_clause = info.clauses.iter().find(|c| c.xtor == xtor).unwrap();
                // Readd the let binding for potential later use
                state.let_bindings.insert(scrutinee_ident, (xtor, xtor_context.clone())); // TODO

                let param_ids = info.param_ids.iter().map(|p| *p);
                let arg_vars = self.args.iter_vars_cloned();
                let subst = param_ids.zip(arg_vars).collect::<Vec<_>>();
                // We can do this substitution now, as identifiers are globally unique 
                let substituted = matching_clause.clone().subst_sim(&subst);

                eliminate_clause_with(substituted, &xtor_context, state)
            }
        }
    }
}
