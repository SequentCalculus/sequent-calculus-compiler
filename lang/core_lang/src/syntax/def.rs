//! This module defines top-level functions in Core.

use printer::tokens::DEF;
use printer::*;

use crate::syntax::*;
use crate::traits::*;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, and the body statement. The
/// type parameter `S` determines whether this is the unfocused variant (if `S` is instantiated
/// with [`Statement`], which is the default) or the focused variant (if `S` is instantiated with
/// [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def<S = Statement> {
    /// The name of the definition
    pub name: Identifier,
    /// The parameter context
    pub context: TypingContext,
    /// The body statement
    pub body: S,
}

pub type FsDef = Def<FsStatement>;

impl Def {
    /// This function applies the [`Focusing`] transformation to the body of the top-level function.
    pub fn focus(self, max_id: &mut ID) -> FsDef {
        FsDef {
            name: self.name,
            context: self.context,
            body: self.body.focus(max_id),
        }
    }

    /// This function applies the [`Uniquify`] transformation to the top-level function.
    pub fn uniquify(mut self, max_id: &mut ID) -> Self {
        let mut new_context = TypingContext::default();
        let mut var_subst: Vec<(Identifier, Term<Prd>)> = Vec::new();
        let mut covar_subst: Vec<(Identifier, Term<Cns>)> = Vec::new();

        for binding in self.context.bindings {
            if binding.var.id == 0 {
                let new_var = fresh_identifier(max_id, &binding.var.name);
                new_context.bindings.push(ContextBinding {
                    var: new_var.clone(),
                    chi: binding.chi.clone(),
                    ty: binding.ty.clone(),
                });

                if binding.chi == Chirality::Prd {
                    var_subst.push((
                        binding.var,
                        XVar {
                            prdcns: Prd,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                    ));
                } else {
                    covar_subst.push((
                        binding.var,
                        XVar {
                            prdcns: Cns,
                            var: new_var,
                            ty: binding.ty,
                        }
                        .into(),
                    ));
                }
            } else {
                new_context.bindings.push(binding);
            }
        }

        self.context = new_context;

        self.body = if var_subst.is_empty() && covar_subst.is_empty() {
            self.body.uniquify(max_id)
        } else {
            self.body
                .subst_sim(&var_subst, &covar_subst)
                .uniquify(max_id)
        };

        self
    }
}

impl<S: Print> Print for Def<S> {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let head = alloc
            .keyword(DEF)
            .append(alloc.space())
            .append(self.name.print(cfg, alloc))
            .append(self.context.print(cfg, alloc).parens())
            .append(alloc.space());

        let body = alloc
            .hardline()
            .append(self.body.print(cfg, alloc).group())
            .nest(cfg.indent)
            .append(alloc.hardline())
            .braces_anno();

        head.group().append(body)
    }
}
