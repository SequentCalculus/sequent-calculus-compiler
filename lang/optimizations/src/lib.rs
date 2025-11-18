use axcut::syntax::{Def, Prog};
use std::rc::Rc;

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod statement;

pub const MAX_RUNS: u64 = 10;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    let mut new_defs = vec![];
    for def in prog.defs {
        new_defs.extend(rewrite_def(def, 1)?);
    }
    Ok(Prog {
        types: prog.types,
        defs: new_defs,
    })
}

fn rewrite_def(def: Def, num_run: u64) -> Result<Vec<Def>, Error> {
    let mut ctx = RewriteContext::new(&def.name, &def.used_vars);
    let new_body = def.body.rewrite(&mut ctx)?;
    let new_def = Def {
        name: def.name,
        context: def.context,
        used_vars: ctx.current_used_vars,
        body: new_body,
    };
    let mut defs: Vec<Def> = ctx.lifted_defs.into_values().collect();
    if ctx.new_changes && num_run < MAX_RUNS {
        let more_defs = rewrite_def(new_def, num_run + 1)?;
        defs.extend(more_defs);
    } else {
        defs.push(new_def);
    }
    Ok(defs)
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
