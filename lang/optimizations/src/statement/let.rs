use crate::{Error, Rewrite, RewriteContext};
use axcut::{
    syntax::statements::{Let, Statement},
    traits::free_vars::FreeVars,
};
use std::{collections::HashSet, rc::Rc};

impl Rewrite for Let {
    type Target = Statement;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        ctx.add_let(&self);
        let mut free_vars_next = HashSet::new();
        let new_next = self.next.rewrite(ctx)?.free_vars(&mut free_vars_next);
        if !free_vars_next.contains(&self.var) {
            ctx.new_changes = true;
            Ok(Rc::unwrap_or_clone(new_next))
        } else {
            Ok(Let {
                var: self.var,
                ty: self.ty,
                tag: self.tag,
                args: self.args,
                next: new_next,
                free_vars_next: Some(free_vars_next),
            }
            .into())
        }
    }
}
