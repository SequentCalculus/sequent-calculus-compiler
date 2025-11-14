use crate::{Error, Rewrite, RewriteContext};
use axcut::{
    syntax::statements::{Statement, Switch},
    traits::substitution::Subst,
};
use std::rc::Rc;

impl Rewrite for Switch {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let let_binding = match ctx.get_binding(&self.var) {
            Some(bnd) => bnd,
            None => return Ok(self.into()),
        };
        let clause_err = Error::clause(&self, &let_binding.tag);
        let rhs_clause = self
            .clauses
            .into_iter()
            .find(|clause| clause.xtor == let_binding.tag)
            .ok_or(clause_err)?;
        if rhs_clause.context.bindings.len() != let_binding.args.entries.len() {
            return Err(Error::arity(
                rhs_clause.context.bindings.len(),
                let_binding.args.entries.len(),
            ));
        }
        let subst = rhs_clause
            .context
            .bindings
            .into_iter()
            .map(|bnd| bnd.var)
            .zip(let_binding.args.entries)
            .collect::<Vec<_>>();
        Ok(Rc::unwrap_or_clone(rhs_clause.body.subst_sim(&subst)))
    }
}
