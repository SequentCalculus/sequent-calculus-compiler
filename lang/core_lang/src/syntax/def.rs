//! This module defines top-level functions in Core.

use printer::tokens::DEF;
use printer::*;

use crate::bail;
use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::errors::Error;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::errors::LocatedTypeError;
use crate::typing::errors::TypeError;

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

impl ConstraintCollector for Def {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, Error> {
        self.body
            .collect_constraints(data_declarations, codata_declarations)
    }
}

impl Checked for Def {
    fn check(
        &self,
        type_params: &[Identifier],
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
        defs: &[Def],
    ) -> Result<(), LocatedTypeError> {
        // check existence of the function name in the program
        if !defs.iter().any(|def| def.name == self.name) {
            bail!(TypeError::UndefinedFunction(self.name.name.clone()));
        }

        // check well-formedness of the context
        self.context
            .check(type_params, data_declarations, codata_declarations, defs)?;

        // check the body of the function
        self.body
            .check(type_params, data_declarations, codata_declarations, defs)
    }
}

#[cfg(test)]
mod def_tests {
    use crate::typing::check::Checked;
    extern crate self as core_lang;
    use core_macros::{def, exit, id, lit, ty};

    #[test]
    fn check_def_present() {
        // def that refers to itself in defs -> should be ok
        let def = def!(id!("f"), [], exit!(lit!(0), ty!("int")));
        let defs = vec![def.clone()];
        assert!(def.check(&[], &[], &[], &defs).is_ok());
    }

    #[test]
    fn check_def_missing() {
        // missing def in defs -> error
        let missing = def!(id!("g"), [], exit!(lit!(0), ty!("int")));
        assert!(missing.check(&[], &[], &[], &[]).is_err());
    }
}
