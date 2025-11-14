use crate::{Error, GetUsedVars, Rewrite, RewriteContext};
use axcut::syntax::{Var, statements::IfC};
use std::collections::HashSet;

impl Rewrite for IfC {
    type Target = Self;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(IfC {
            sort: self.sort,
            fst: self.fst,
            snd: self.snd,
            thenc: self.thenc.rewrite(ctx)?,
            elsec: self.elsec.rewrite(ctx)?,
        })
    }
}

impl GetUsedVars for IfC {
    fn get_used_vars(&self) -> HashSet<Var> {
        let mut used = HashSet::from([self.fst.clone()]);
        if let Some(ref v) = self.snd {
            used.insert(v.clone());
        }
        used.extend(self.thenc.get_used_vars());
        used.extend(self.elsec.get_used_vars());
        used
    }
}
