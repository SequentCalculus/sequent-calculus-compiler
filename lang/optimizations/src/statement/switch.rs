use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::{
    syntax::{
        Var,
        statements::{Let, Statement, Switch},
    },
    traits::{free_vars::FreeVars, substitution::Subst},
};
use std::{collections::HashSet, rc::Rc};

impl Rewrite for Switch {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        match ctx.get_let(&self.var) {
            Some(let_binding) => {
                ctx.new_changes = true;
                rewrite_subst(let_binding, self)
            }
            None => rewrite_no_subst(self, ctx),
        }
    }
}

fn rewrite_subst(let_binding: Let, switch: Switch) -> Result<Statement, Error> {
    let clause_err = Error::switch_clause(&switch, &let_binding.tag);
    let rhs_clause = switch
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

fn rewrite_no_subst(switch: Switch, ctx: &mut RewriteContext) -> Result<Statement, Error> {
    let mut free_clauses = HashSet::new();
    let mut new_clauses = vec![];
    for clause in switch.clauses {
        let next_clause = clause.rewrite(ctx)?.free_vars(&mut free_clauses);
        new_clauses.push(next_clause);
    }
    Ok(Switch {
        var: switch.var,
        ty: switch.ty,
        clauses: new_clauses,
        free_vars_clauses: Some(free_clauses),
    }
    .into())
}

impl GetUsedVars for Switch {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.var.clone()]);
        for clause in self.clauses.iter() {
            used.extend(clause.get_used_vars());
        }
        used
    }
}
