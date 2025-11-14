use axcut::syntax::{Def, Prog};

mod context;
pub mod errors;
use context::RewriteContext;
use errors::Error;

mod statement;

pub fn rewrite(prog: Prog) -> Result<Prog, Error> {
    Ok(Prog {
        types: prog.types,
        defs: prog
            .defs
            .into_iter()
            .map(rewrite_def)
            .collect::<Result<Vec<Def>, Error>>()?,
    })
}

fn rewrite_def(def: Def) -> Result<Def, Error> {
    let mut ctx = RewriteContext::new();
    let new_body = def.body.rewrite(&mut ctx)?;
    Ok(Def {
        name: def.name,
        context: def.context,
        body: new_body,
        used_vars: def.used_vars,
    })
}

trait Rewrite {
    type Target;
    fn rewrite(self, ctx: &mut RewriteContext) -> Result<Self::Target, Error>;
}
