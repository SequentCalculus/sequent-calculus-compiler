use axcut::syntax::{Name, Prog};
use std::rc::Rc;

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod free_bindings;
mod statement;

pub const MAX_RUNS: u64 = 10;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    let names = prog
        .defs
        .iter()
        .map(|def| &def.name)
        .cloned()
        .collect::<Vec<_>>();
    let mut ctx = RewriteContext::new(prog.defs);
    for name in names {
        ctx.current_def_runs = 1;
        rewrite_def(name, &mut ctx)?;
    }
    let prog = Prog {
        types: prog.types,
        defs: ctx.definitions,
    };
    Ok(prog)
}

fn rewrite_def(name: Name, ctx: &mut RewriteContext) -> Result<(), Error> {
    let mut current_def = ctx
        .get_def(&name)
        .ok_or(Error::DefinitionNotFound { name: name.clone() })?;
    ctx.set_current_def(&name, &current_def.used_vars);
    let new_body = current_def.body.rewrite(ctx)?;
    current_def.body = new_body;

    ctx.add_def(current_def);
    if ctx.new_changes && ctx.current_def_runs < MAX_RUNS {
        ctx.current_def_runs += 1;
        rewrite_def(name, ctx)?;
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
