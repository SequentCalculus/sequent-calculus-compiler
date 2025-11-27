use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::{
        Name, TypingContext,
        statements::{Statement, Switch},
    },
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Rewrite for Switch {
    type Target = Statement;
    fn rewrite(mut self, state: &mut RewriteState) -> Self::Target {
        match state.get_let(&self.var) {
            None => {
                let mut new_clauses = Vec::with_capacity(self.clauses.len());
                for clause in self.clauses {
                    new_clauses.push(clause.rewrite(state));
                }
                self.clauses = new_clauses;
                self.into()
            }
            Some((name, args)) => {
                *state.new_changes = true;
                rewrite_subst(name, args, self, state)
            }
        }
    }
}

fn rewrite_subst(
    let_xtor: Name,
    let_args: TypingContext,
    switch: Switch,
    state: &mut RewriteState,
) -> Statement {
    let clause = switch
        .clauses
        .into_iter()
        .find(|clause| clause.xtor == let_xtor)
        .expect("Could not find Switch clause for xtor");
    if clause.context.bindings.len() != let_args.bindings.len() {
        panic!("Number of Switch Clause Arguments does not match Xtor Arguments")
    }

    let subst = clause
        .context
        .bindings
        .into_iter()
        .map(|binding| binding.var)
        .zip(let_args.vars())
        .collect::<Vec<_>>();
    Rc::unwrap_or_clone(clause.body.subst_sim(&subst).rewrite(state))
}
