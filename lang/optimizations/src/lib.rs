use axcut::syntax::{Prog, TypeDeclaration, Var};
use std::{collections::HashSet, rc::Rc};

mod arguments;
mod clause;
mod context;
mod declaration;
mod definition;
mod statement;
mod xtor;

pub fn inline_prog(prog: Prog) -> Prog {
    prog
}

pub struct InlineContext {
    decls: Vec<TypeDeclaration>,
}

pub trait Inline {
    type Target;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target;
}

impl<T> Inline for Rc<T>
where
    T: Inline + Clone,
{
    type Target = Rc<T::Target>;
    fn inline(self, ctx: &mut InlineContext) -> Self::Target {
        Rc::new(Rc::unwrap_or_clone(self).inline(ctx))
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
