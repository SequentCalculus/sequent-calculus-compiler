use axcut::{
    syntax::{Def, Prog},
    traits::free_vars::FreeVars,
};
use std::{collections::HashSet, rc::Rc};

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod statement;

pub const MAX_RUNS: u64 = 10;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    Ok(Prog {
        types: prog.types,
        defs: prog
            .defs
            .into_iter()
            .map(|def| rewrite_def(def, 0))
            .collect::<Result<Vec<Def>, Error>>()?,
    })
}

fn rewrite_def(def: Def, num_run: u64) -> Result<Def, Error> {
    println!("rewriting {}, run {}", def.name, num_run);
    let mut ctx = RewriteContext::new();
    let mut free = HashSet::new();
    let new_body = def.body.rewrite(&mut ctx)?.free_vars(&mut free);
    let new_def = Def {
        name: def.name,
        context: def.context,
        body: new_body,
        used_vars: free,
    };
    if ctx.new_changes && num_run < MAX_RUNS {
        rewrite_def(new_def, num_run + 1)
    } else {
        Ok(new_def)
    }
}

trait Rewrite {
    type Target;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error>;
}

impl<T> Rewrite for Rc<T>
where
    T: Rewrite + Clone,
{
    type Target = Rc<T::Target>;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error> {
        Ok(Rc::new(Rc::unwrap_or_clone(self).rewrite(ctx)?))
    }
}
