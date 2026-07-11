//! This module defines the call of a top-level function in Core.

use printer::*;

use crate::bail;
use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::constraints::collect_type_flow;
use crate::mono::errors::MonoError;
use crate::mono::specialize::Specialize;
use crate::mono::specialize::SpecializeContext;
use crate::syntax::types::TypeArgs;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::check::check_arity;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;
use crate::typing::errors::TypeError;

use std::collections::BTreeSet;

/// This struct defines the call of a top-level function in Core. It consists of the name of the
/// top-level function to call,  the type arguments, the arguments, and the type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The name of the top-level function being called
    pub name: Identifier,
    /// The type arguments
    pub type_args: TypeArgs,
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
            .append(self.type_args.print(cfg, alloc))
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
                    type_args: self.type_args,
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
    /// The type arguments
    pub type_args: TypeArgs,
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
    fn collect_constraints(&self, env: &GlobalEnv) -> Result<FlowConstraintSet, MonoError> {
        let mut constraints = self.ty.collect_constraints(env)?;

        constraints.extend(self.args.collect_constraints(env)?);
        constraints.extend(self.type_args.collect_constraints(env)?);

        let Some(def) = env.lookup_def(&self.name) else {
            return Err(MonoError::UndefinedFunction(self.name.name.clone()));
        };

        constraints.extend(collect_type_flow(&self.type_args.args, &def.type_params)?);

        Ok(constraints)
    }
}

impl Specialize for Call {
    fn specialize(&self, context: SpecializeContext) -> Self {
        let ground_type_args: Vec<Ty> = self
            .type_args
            .args
            .iter()
            .map(|ty| ty.substitute(context.subst))
            .collect();

        let specialized_name = if ground_type_args.is_empty() {
            // monomorphic call site: use the original name
            self.name.clone()
        } else {
            // polymorphic call site: look up the specialized name in the naming table
            context.table.lookup(&self.name, &ground_type_args).clone()
        };
        Call {
            name: specialized_name,
            type_args: TypeArgs::default(),
            args: self.args.specialize(context),
            ty: self.ty.specialize(context),
        }
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
            bail!(TypeError::UndefinedFunction(self.name.name.clone()));
        };

        // Check that the arity of the type arguments and the arity of the arguments match the definition
        check_arity(def.type_params.len(), self.type_args.args.len())?;
        check_arity(def.context.bindings.len(), self.args.entries.len())?;

        for ty_arg in &self.type_args.args {
            ty_arg.check(type_params, context, env)?;
        }

        // build the substitution mapping for the type parameters and type arguments
        let subst = (def.type_params.as_slice(), self.type_args.args.as_slice());

        // check that the types of the arguments match the types of the parameters after substitution
        for (binding, arg) in def.context.bindings.iter().zip(&self.args.entries) {
            let expected_substituted_ty = binding.ty.substitute(subst);

            if expected_substituted_ty != arg.get_type() {
                bail!(TypeError::TypeMismatch {
                    expected: expected_substituted_ty.print_to_string(None),
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
    use core_macros::{bind, call, def, exit, id, lit, prd, tvar, ty, var};

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
    fn call_check_poly_ok() {
        let poly_def = def!(
            id!("identity"),
            [id!("A")],
            [bind!(id!("x"), prd!(), tvar!(id!("A")))],
            exit!(var!(id!("x")), tvar!(id!("A")))
        );
        let defs = vec![poly_def];

        let call_stmt: Statement = call!(id!("identity"), [ty!("int")], [lit!(42)]).into();

        assert!(
            call_stmt
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &defs),
                )
                .is_ok()
        );
    }

    #[test]
    fn call_check_poly_type_mismatch() {
        let poly_def = def!(
            id!("identity"),
            [id!("A")],
            [bind!(id!("x"), prd!(), tvar!(id!("A")))],
            exit!(var!(id!("x")), tvar!(id!("A")))
        );
        let defs = vec![poly_def];

        let call_mismatch_stmt: Statement =
            call!(id!("identity"), [ty!(id!("Bool"))], [lit!(42)]).into();

        assert!(
            call_mismatch_stmt
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &defs)
                )
                .is_err()
        );
    }

    #[test]
    fn call_check_poly_arity_mismatch() {
        let poly_def = def!(
            id!("identity"),
            [id!("A")],
            [bind!(id!("x"), prd!(), tvar!(id!("A")))],
            exit!(var!(id!("x")), tvar!(id!("A")))
        );
        let defs = vec![poly_def];

        let call_arity_stmt: Statement =
            call!(id!("identity"), [ty!("int"), ty!("int")], [lit!(42)]).into();

        assert!(
            call_arity_stmt
                .check(
                    &[],
                    &TypingContext::default(),
                    &GlobalEnv::new(&vec![], &vec![], &defs)
                )
                .is_err()
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

#[cfg(test)]
mod collect_tests {
    use crate::{
        mono::{
            constraints::{ConstraintCollector, FlowConstraint, FlowConstraintSet},
            errors::MonoError,
        },
        typing::env::GlobalEnv,
    };
    extern crate self as core_lang;
    use core_macros::{bind, call, def, exit, id, lit, prd, tvar, ty, var};

    #[test]
    fn collect_constraints_mono_ok() {
        let def = def!(
            id!("f"),
            [bind!(id!("x"), prd!(), ty!("int"))],
            exit!(var!(id!("x")), ty!("int"))
        );

        let call = call!(id!("f"), [lit!(1)]);

        let res: Result<FlowConstraintSet, MonoError> =
            call.collect_constraints(&GlobalEnv::new(&[], &[], &[def]));
        assert!(res.is_ok());
        assert!(res.unwrap().constraints.is_empty());
    }

    #[test]
    fn collect_constraints_poly_instantiation() {
        let poly_def = def!(
            id!("identity"),
            [id!("A")],
            [bind!(id!("x"), prd!(), tvar!(id!("A")))],
            exit!(var!(id!("x")), tvar!(id!("A")))
        );

        let call = call!(id!("identity"), [ty!("int")], [lit!(42)]);

        let constraints = call
            .collect_constraints(&GlobalEnv::new(&vec![], &vec![], &vec![poly_def]))
            .unwrap();

        let expected_constraint = FlowConstraint {
            from: vec![ty!("int")],
            to: vec![(id!("A"))],
        };

        assert!(constraints.constraints.contains(&expected_constraint));
    }

    #[test]
    fn collect_constraints_undefined_function() {
        let call_expr = call!(id!("ghost"), []);

        let res = call_expr.collect_constraints(&GlobalEnv::default());

        assert!(matches!(
            res,
            Err(MonoError::UndefinedFunction(name)) if name == "ghost"
        ));
    }
}
