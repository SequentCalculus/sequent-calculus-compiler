use super::{Error, Inline, InlineContext, fresh_var};
use axcut::{
    syntax::{Arguments, ContextBinding, Def, TypingContext, statements::Let, types::Ty},
    traits::free_vars::FreeVars,
};
use std::{collections::HashSet, rc::Rc};

impl Inline for Def {
    type Target = Def;
    fn inline(self, ctx: &mut InlineContext) -> Result<Self::Target, Error> {
        let mut next = self.body.inline(ctx)?;
        let mut new_bindings = vec![];
        for ctx_bind in self.context.bindings {
            if let Ty::Decl(ref name) = ctx_bind.ty {
                let ty_decl = ctx.lookup_ty(&name)?;
                if ty_decl.xtors.len() == 1 {
                    let xtor_decl = ty_decl.xtors.first().unwrap();
                    let mut used_vars = self.used_vars.clone();

                    let mut let_vars = vec![];
                    for xtor_bind in xtor_decl.args.bindings.iter() {
                        used_vars.insert(xtor_bind.var.clone());
                        let next_var = fresh_var(&used_vars);
                        used_vars.insert(next_var.clone());
                        let next_binding = ContextBinding {
                            var: next_var.clone(),
                            chi: xtor_bind.chi.clone(),
                            ty: xtor_bind.ty.clone(),
                        };
                        let_vars.push(next_var);
                        new_bindings.push(next_binding);
                    }
                    let mut next_vars = HashSet::new();
                    next = next.free_vars(&mut next_vars);
                    let next_let = Let {
                        var: ctx_bind.var,
                        tag: name.clone(),
                        ty: ctx_bind.ty,
                        args: Arguments { entries: let_vars },
                        next: Rc::new(next),
                        free_vars_next: Some(next_vars),
                    };
                    next = next_let.into();
                }
            }
        }
        Ok(Def {
            name: self.name,
            context: TypingContext {
                bindings: new_bindings,
            },
            body: next,
            used_vars: self.used_vars,
        })
    }
}
