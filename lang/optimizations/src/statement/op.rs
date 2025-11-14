use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::{
    syntax::{Var, statements::Op},
    traits::free_vars::FreeVars,
};
use std::collections::HashSet;

impl Rewrite for Op {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        let mut free_next = HashSet::new();
        let new_next = self.next.rewrite(ctx)?.free_vars(&mut free_next);
        Ok(Op {
            fst: self.fst,
            op: self.op,
            snd: self.snd,
            var: self.var,
            next: new_next,
            free_vars_next: Some(free_next),
        })
    }
}

impl GetUsedVars for Op {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.fst.clone(), self.snd.clone(), self.var.clone()]);
        used.extend(self.next.get_used_vars());
        used
    }
}
