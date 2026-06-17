//! This module defines the call of a top-level function in Core.

use printer::*;

use crate::bail;
use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::errors::MonoError;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;
use crate::typing::errors::TypeError;

use std::collections::BTreeSet;

/// This struct defines the call of a top-level function in Core. It consists of the name of the
/// top-level function to call, the arguments, and the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The name of the top-level function being called
    pub name: Identifier,
    /// The arguments
    pub args: Arguments,
    /// The type (which is the return type of the definition)
    pub ty: Ty,
}

impl Typed for Call {
    fn get_type(&self) -> Ty {
        self.ty.clone()
    }
}

impl Print for Call {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<Call> for Statement {
    fn from(value: Call) -> Self {
        Statement::Call(value)
    }
}

impl Subst for Call {
    type Target = Call;
    fn subst_sim(
        mut self,
        prod_subst: &[(Identifier, Term<Prd>)],
        cons_subst: &[(Identifier, Term<Cns>)],
    ) -> Self::Target {
        self.args = self.args.subst_sim(prod_subst, cons_subst);
        self
    }
}

impl TypedFreeVars for Call {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        self.args.typed_free_vars(vars);
    }
}

impl Uniquify for Call {
    fn uniquify(mut self, max_id: &mut ID) -> Call {
        self.args = self.args.uniquify(max_id);
        self
    }
}

impl Focusing for Call {
    type Target = FsStatement;
    // focus(f(t_i)) = bind(t_i)[λas.f(as)]
    fn focus(self, max_id: &mut ID) -> FsStatement {
        bind_many(
            self.args.into(),
            Box::new(|bindings, _: &mut ID| {
                FsCall {
                    name: self.name,
                    args: bindings.into(),
                }
                .into()
            }),
            max_id,
        )
    }
}

/// This struct defines the focused version of [`Call`]s of top-level functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsCall {
    /// The name of the top-level function being called
    pub name: Identifier,
    /// The arguments (only (co)variables here)
    pub args: TypingContext,
}

impl Print for FsCall {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        self.name
            .print(cfg, alloc)
            .append(self.args.print(cfg, alloc).parens().group())
    }
}

impl From<FsCall> for FsStatement {
    fn from(value: FsCall) -> Self {
        FsStatement::Call(value)
    }
}

impl SubstVar for FsCall {
    type Target = FsCall;
    fn subst_sim(mut self, subst: &[(ID, Identifier)]) -> FsCall {
        self.args = self.args.subst_sim(subst);
        self
    }
}

impl TypedFreeVars for FsCall {
    fn typed_free_vars(&self, vars: &mut BTreeSet<ContextBinding>) {
        vars.extend(self.args.bindings.iter().cloned());
    }
}

impl ConstraintCollector for Call {
    fn collect_constraints(
        &self,
        data_declarations: &[DataDeclaration],
        codata_declarations: &[CodataDeclaration],
    ) -> Result<FlowConstraintSet, MonoError> {
        let mut constraints = self
            .ty
            .collect_constraints(data_declarations, codata_declarations)?;

        constraints.extend(
            self.args
                .collect_constraints(data_declarations, codata_declarations)?,
        );
        Ok(constraints)
    }
}

impl Checked for Call {
    fn check(
        &self,
        type_params: &[Identifier],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // check well-formedness of the type
        self.ty.check(type_params, context, env)?;

        // Check that the called function is defined
        let Some(def) = env.lookup_def(&self.name) else {
            bail!(TypeError::UndefinedFunction(
                self.name.clone().name.to_string()
            ));
        };

        // check arity
        if def.context.bindings.len() != self.args.entries.len() {
            bail!(TypeError::ArityMismatch {
                expected: def.context.bindings.len(),
                got: self.args.entries.len(),
            });
        }

        // check that the types of the arguments match the types of the parameters
        for (binding, arg) in def.context.bindings.iter().zip(&self.args.entries) {
            if binding.ty != arg.get_type() {
                bail!(TypeError::TypeMismatch {
                    expected: binding.ty.print_to_string(None),
                    got: arg.get_type().print_to_string(None),
                    msg: None,
                });
            }
        }
        self.args.check(type_params, context, env)?;

        Ok(())
    }
}

#[cfg(test)]
mod check_tests {
    use crate::{
        syntax::{Statement, TypingContext},
        typing::{check::Checked, env::GlobalEnv},
    };
    extern crate self as core_lang;
    use core_macros::{bind, call, def, exit, id, lit, prd, ty, var};

    #[test]
    fn call_check_ok() {
        let def = def!(
            id!("f"),
            [bind!(id!("x"), prd!(), ty!("int"))],
            exit!(var!(id!("x")), ty!("int"))
        );
        let defs = vec![def.clone()];

        let call_ok_stmt: Statement = call!(id!("f"), [lit!(1)]).into();
        assert!(
            call_ok_stmt
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &defs)
                )
                .is_ok()
        );
    }

    #[test]
    fn call_check_undefined() {
        let call_undef: Statement = call!(id!("g"), []).into();
        assert!(
            call_undef
                .check(&[], &TypingContext::default(), &GlobalEnv::default())
                .is_err()
        );
    }

    #[test]
    fn call_check_arity_mismatch() {
        let call_arity: Statement = call!(id!("f"), [lit!(1)]).into();
        let def_no_args = def!(id!("f"), [], exit!(lit!(0), ty!("int")));
        assert!(
            call_arity
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &vec![def_no_args])
                )
                .is_err()
        );
    }

    #[test]
    fn call_check_type_mismatch() {
        let def_param_other = def!(
            id!("h"),
            [bind!(id!("x"), prd!(), ty!(id!("List")))],
            exit!(lit!(0), ty!(id!("List")))
        );
        let call_type_mismatch: Statement = call!(id!("h"), [lit!(42)]).into();
        assert!(
            call_type_mismatch
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &vec![def_param_other])
                )
                .is_err()
        );
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::traits::*;
    extern crate self as core_lang;
    use core_macros::{bind, call, cns, covar, fs_call, id, prd, var};

    #[test]
    fn transform_call1() {
        let result = call!(id!("main"), []).focus(&mut Default::default());
        let expected = fs_call!(id!("main"), []).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn transform_call2() {
        let result =
            call!(id!("fun"), [var!(id!("x")), covar!(id!("a"))],).focus(&mut Default::default());
        let expected = fs_call!(
            id!("fun"),
            [bind!(id!("x"), prd!()), bind!(id!("a"), cns!())]
        )
        .into();
        assert_eq!(result, expected)
    }
}
