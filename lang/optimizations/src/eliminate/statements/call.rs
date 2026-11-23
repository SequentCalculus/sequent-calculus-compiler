use crate::eliminate::{Eliminate, EliminateState, statements::clause::eliminate_clause_with};
use axcut::{
    syntax::{
        Def,
        statements::{Call, Statement},
    },
    traits::substitution::Subst,
};

impl Eliminate for Call {
    type Target = Statement;
    fn eliminate(self, state: &mut EliminateState, lifted: &mut Vec<Def>) -> Self::Target {
        let Some(info) = state.global_switches.get(&self.label) else {
            return self.into();
        };

        let scrutinee_ident = &self.args.bindings[info.scrutinee_pos].var;
        let Some((xtor, xtor_context)) = state.let_bindings.get(scrutinee_ident) else {
            return self.into();
        };
        let matching_clause = info.clauses.iter().find(|c| c.xtor == *xtor).unwrap();

        let param_ids = info.param_ids.clone().into_iter();
        let arg_vars = self.args.iter_vars_cloned();
        let subst = param_ids.zip(arg_vars).collect::<Vec<_>>();
        // Doing this substitution now causes no accidental captures, as identifiers are globally unique
        let substituted = matching_clause.clone().subst_sim(&subst);
        eliminate_clause_with(substituted, xtor_context.clone(), state, lifted)
    }
}
