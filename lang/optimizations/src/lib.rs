use axcut::syntax::{Def, Prog};
use std::rc::Rc;

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod free_bindings;
mod statement;

pub const MAX_RUNS: u64 = 10;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    let mut ctx = RewriteContext::new(&prog.defs);
    for def in prog.defs {
        ctx.current_def_runs = 1;
        rewrite_def(def, &mut ctx)?;
    }
    let prog = Prog {
        types: prog.types,
        defs: ctx.definitions,
    };
    Ok(prog)
}

fn rewrite_def(def: Def, ctx: &mut RewriteContext) -> Result<(), Error> {
    ctx.set_def(&def.name, &def.used_vars);
    let new_body = def.body.rewrite(ctx)?;
    let new_def = Def {
        name: def.name,
        context: def.context,
        used_vars: ctx.current_used_vars.clone(),
        body: new_body,
    };
    if ctx.new_changes && ctx.current_def_runs < MAX_RUNS {
        ctx.current_def_runs += 1;

        rewrite_def(new_def, ctx)?;
    } else {
        ctx.add_def(new_def);
    }
    Ok(())
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
