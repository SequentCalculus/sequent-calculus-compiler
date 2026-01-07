use crate::cleanup_inline::{CleanupInline, CleanupInlineGather, CleanupInlineState};
use crate::rewrite::{Rewrite, RewriteState};
use axcut::{
    syntax::statements::{Call, Invoke, Statement},
    traits::substitution::Subst,
};

use std::rc::Rc;

impl Rewrite for Invoke {
    type Target = Statement;
    fn rewrite(self, state: &mut RewriteState) -> Self::Target {
        let Some((clause, position)) = state.get_create_clause(&self.var, &self.tag) else {
            return self.into();
        };
        state.new_changes = true;

        if matches!(
            *clause.body,
            Statement::Exit(_) | Statement::Call(_) | Statement::Invoke(_)
        ) {
            // for leaf statements, we do not lift, but just substitute into them
            let subst = clause
                .context
                .into_iter_vars()
                .zip(self.args.into_iter_vars())
                .collect::<Vec<_>>();
            Rc::unwrap_or_clone(clause.body.subst_sim(&subst))
        } else {
            let (label, arg_positions, mut free_args) =
                state.lift_create_clause(clause, position, &self.var);
            for position in arg_positions {
                free_args.push(self.args.bindings[position].clone());
            }

            Call {
                label,
                args: free_args.into(),
            }
            .into()
        }
    }
}

impl CleanupInlineGather for Invoke {
    type Target = Self;
    fn cleanup_inline_gather(self, _: &mut CleanupInlineState) -> Self::Target {
        self
    }
}

impl CleanupInline for Invoke {
    type Target = Self;
    fn cleanup_inline(self, _: &mut CleanupInlineState) -> Self::Target {
        self
    }
}
