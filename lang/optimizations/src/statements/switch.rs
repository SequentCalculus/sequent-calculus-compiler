use crate::{
    errors::Error,
    rewrite::{Rewrite, RewriteState},
};
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
    fn rewrite(mut self, state: &mut RewriteState) -> Result<Self::Target, Error> {
        match state.get_let(&self.var) {
            None => {
                let mut new_clauses = Vec::with_capacity(self.clauses.len());
                for clause in self.clauses {
                    new_clauses.push(clause.rewrite(state)?);
                }
                self.clauses = new_clauses;
                Ok(self.into())
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
) -> Result<Statement, Error> {
    let clause_err = Error::switch_clause(&switch, &let_xtor);
    let clause = switch
        .clauses
        .into_iter()
        .find(|clause| clause.xtor == let_xtor)
        .ok_or(clause_err)?;
    if clause.context.bindings.len() != let_args.bindings.len() {
        return Err(Error::arity(
            clause.context.bindings.len(),
            let_args.bindings.len(),
        ));
    }

    let subst = clause
        .context
        .bindings
        .into_iter()
        .map(|binding| binding.var)
        .zip(let_args.vars())
        .collect::<Vec<_>>();
    Ok(Rc::unwrap_or_clone(
        clause.body.subst_sim(&subst).rewrite(state)?,
    ))
}
