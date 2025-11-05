use axcut::syntax::{Prog, TypeDeclaration, Var};
use std::{collections::HashSet, rc::Rc};

pub mod errors;
use errors::Error;

mod arguments;
mod clause;
mod context;
mod declaration;
mod definition;
mod statement;
mod xtor;

pub fn inline_prog(prog: Prog) -> Result<Prog, Error> {
    let mut ctx = InlineContext {
        decls: prog.types.clone(),
    };
    Ok(Prog {
        defs: prog
            .defs
            .into_iter()
            .map(|def| def.inline(&mut ctx))
            .collect::<Result<Vec<_>, Error>>()?,
        types: prog
            .types
            .into_iter()
            .map(|def| def.inline(&mut ctx))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

pub struct InlineContext {
    decls: Vec<TypeDeclaration>,
}

impl InlineContext {
    pub fn lookup_ty(&self, name: &str) -> Result<TypeDeclaration, Error> {
        self.decls
            .iter()
            .find(|decl| decl.name == name)
            .ok_or(Error::unknown(name))
            .cloned()
    }
}

pub trait Inline {
    type Target;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error>;
}

impl<T> Inline for Rc<T>
where
    T: Inline + Clone,
{
    type Target = Rc<T::Target>;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        Ok(Rc::new(Rc::unwrap_or_clone(self).inline(ctx)?))
    }
}

pub fn fresh_var(used_vars: &HashSet<Var>) -> Var {
    let mut num = 0;
    let prefix = "x";
    let mut var = format!("{prefix}{num}");
    while used_vars.contains(&var) {
        num += 1;
        var = format!("{prefix}{num}");
    }
    var
}
