//! This module defines top-level functions in Core.

use printer::tokens::DEF;
use printer::*;

use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::errors::MonoError;
use crate::mono::specialize::Specialize;
use crate::mono::specialize::SpecializeContext;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;

/// This struct defines top-level function definitions. A top-level function consists of a name
/// (unique in the program), a typing context defining the parameters, and the body statement. The
/// type parameter `S` determines whether this is the unfocused variant (if `S` is instantiated
/// with [`Statement`], which is the default) or the focused variant (if `S` is instantiated with
/// [`FsStatement`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Def<S = Statement> {
    /// The name of the definition
    pub name: Identifier,
    /// The type parameters
    pub type_params: Vec<Identifier>,
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
            type_params: self.type_params,
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
            .append(self.type_params.print(cfg, alloc))
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

impl ConstraintCollector for Def {
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        self.body.collect_constraints(env)
    }
}

impl Checked for Def {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // extend the type parameters of the clause with the type parameters of the definition
        let extended_type_params = [type_params, &self.type_params].concat();

        // check well-formedness of the context
        self.context.check(&extended_type_params, context, env)?;

        // extend the context of the clause with the bindings of the definition
        let mut extended_context = context.clone();
        for binding in &self.context.bindings {
            extended_context.bindings.push(binding.clone());
        }

        // check the body of the function under the context of the function
        self.body
            .check(&extended_type_params, &extended_context, env)
    }
}

impl Specialize for Def {
    fn specialize(&self, context: SpecializeContext) -> Self {
        Def {
            name: self.name.clone(),
            type_params: self.type_params.clone(),
            context: self.context.specialize(context),
            body: self.body.specialize(context),
        }
    }
}
