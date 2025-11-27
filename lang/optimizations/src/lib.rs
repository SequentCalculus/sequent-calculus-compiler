use axcut::syntax::{Name, Prog};
use std::rc::Rc;

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod statement;

pub const MAX_RUNS_DEF: u64 = 10;
pub const MAX_RUNS: u64 = 10;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    let names = prog
        .defs
        .iter()
        .map(|def| &def.name)
        .cloned()
        .collect::<Vec<_>>();
    let mut ctx = RewriteContext::new(prog.defs);
    let mut num_run = 1;
    let mut new_changes = true;
    while new_changes && num_run < MAX_RUNS {
        new_changes = false;
        for name in names.iter() {
            ctx.current_def_runs = 1;
            new_changes = new_changes || rewrite_def(name, &mut ctx)?;
        }
        num_run += 1;
    }
    let prog = Prog {
        types: prog.types,
        defs: ctx.definitions,
    };
    Ok(prog)
}

fn rewrite_def(name: &Name, ctx: &mut RewriteContext) -> Result<bool, Error> {
    let mut current_def = ctx
        .get_def(&name)
        .ok_or(Error::DefinitionNotFound { name: name.clone() })?;
    ctx.set_current_def(&name, &current_def.used_vars);
    let new_body = current_def.body.rewrite(ctx)?;
    current_def.body = new_body;

    ctx.add_def(current_def);
    if ctx.new_changes && ctx.current_def_runs < MAX_RUNS_DEF {
        ctx.current_def_runs += 1;
        rewrite_def(name, ctx)?;
    }
    Ok(ctx.new_changes)
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
