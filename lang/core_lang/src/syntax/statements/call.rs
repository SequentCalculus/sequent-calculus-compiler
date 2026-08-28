//! This module defines the call of a top-level function in Core.

use printer::*;

use crate::bail;
use crate::mono::constraints::ConstraintCollector;
use crate::mono::constraints::FlowConstraintSet;
use crate::mono::constraints::collect_type_flow;
use crate::mono::erasure::erase_ty;
use crate::mono::errors::MonoError;
use crate::mono::specialize::Specialize;
use crate::mono::specialize::SpecializeContext;
use crate::splitting::labeling::{DeclSignatures, LabelAndUnify, SplitState};
use crate::splitting::rewrite::Rewrite;
use crate::splitting::split_table::SplitTable;
use crate::syntax::arguments::Argument;
use crate::syntax::types::TypeArgs;
use crate::syntax::*;
use crate::traits::*;
use crate::typing::check::Checked;
use crate::typing::check::check_arity;
use crate::typing::check::check_polarity;
use crate::typing::env::GlobalEnv;
use crate::typing::errors::LocatedTypeError;
use crate::typing::errors::TypeError;

use std::collections::BTreeSet;

/// This struct defines the call of a top-level function in Core. It consists of the name of the
/// top-level function to call, the type arguments, and the arguments. Unlike Fun, Core calls have
/// no independent return type: the continuation is passed as the last (consumer) argument, and
/// that continuation's type is the call's type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// The name of the top-level function being called
    pub name: Identifier,
    /// The type arguments
    pub type_args: TypeArgs,
    /// The arguments, whose last entry is the continuation
    pub args: Arguments,
}

impl Typed for Call {
    fn get_type(&self) -> Ty {
        self.args
            .entries
            .iter()
            .rev()
            .find_map(|arg| match arg {
                Argument::Consumer(term) => Some(term.get_type()),
                Argument::Producer(_) => None,
            })
            .expect("a well-formed Call must have a consumer argument (its continuation)")
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
        let mut constraints = self.args.collect_constraints(env)?;
        constraints.extend(self.type_args.collect_constraints(env)?);

        let Some(def) = env.lookup_def(&self.name) else {
            return Err(MonoError::UndefinedFunction(self.name.name.clone()));
        };

        let def_type_param_ids: Vec<Identifier> =
            def.type_params.iter().map(|p| p.id.clone()).collect();
        constraints.extend(collect_type_flow(
            &self.type_args.args,
            &def_type_param_ids,
        )?);

        Ok(constraints)
    }
}

impl Specialize for Call {
    fn specialize(&self, context: &SpecializeContext) -> Self {
        let ground_type_args: Vec<Ty> = self
            .type_args
            .args
            .iter()
            .map(|ty| {
                erase_ty(
                    &ty.substitute((&context.subst.0, &context.subst.1)),
                    &context.erased_decls.0,
                )
            })
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
        }
    }
}

impl Checked for Call {
    fn check(
        &self,
        type_params: &[TypeParam],
        context: &TypingContext,
        env: &GlobalEnv,
    ) -> Result<(), LocatedTypeError> {
        // Check that the called function is defined
        let Some(def) = env.lookup_def(&self.name) else {
            bail!(TypeError::UndefinedFunction(self.name.name.clone()));
        };

        // Check that the arity of the type arguments and the arity of the arguments match the definition
        check_arity(def.type_params.len(), self.type_args.args.len())?;
        check_arity(def.context.bindings.len(), self.args.entries.len())?;

        // check well-formedness and polarity of each type argument against the def's own
        // declared type parameters
        for (ty_arg, declared_param) in self.type_args.args.iter().zip(&def.type_params) {
            ty_arg.check(type_params, context, env)?;
            let got = if ty_arg.is_codata(env.codata_decls, type_params) {
                ParamPolarity::Codata
            } else {
                ParamPolarity::Data
            };
            check_polarity(declared_param.polarity, got)?;
        }

        // build the substitution mapping for the type parameters and type arguments
        let def_type_param_ids: Vec<Identifier> =
            def.type_params.iter().map(|p| p.id.clone()).collect();
        let subst = (
            def_type_param_ids.as_slice(),
            self.type_args.args.as_slice(),
        );

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

impl LabelAndUnify for Call {
    fn label_and_unify(
        &self,
        state: &mut SplitState,
        sigs: &DeclSignatures,
        scope: &TypingContext,
    ) -> Self {
        // Label this call site's own explicit type arguments (instantiating the callee's own
        // generic type parameters, e.g. `identity[int](42)`) before using them.
        let type_args = TypeArgs {
            args: self
                .type_args
                .args
                .iter()
                .map(|a| state.label_ty(a))
                .collect(),
        };
        let args = self.args.label_and_unify(state, sigs, scope);

        let sig = sigs
            .get(&self.name)
            .unwrap_or_else(|| panic!("missing signature for def: {}", self.name.name));
        // Substitute the callee's own type parameters with this call site's labeled type_args
        // before unifying, otherwise a parameter declared as `Ty::Var(A)` would never unify
        // with anything, and the concrete type flowing through a generic parameter would
        // silently escape type splitting.
        let subst = (sig.own_type_params.as_slice(), type_args.args.as_slice());
        for (arg, param_ty) in args.entries.iter().zip(&sig.tys) {
            let expected = param_ty.substitute(subst);
            state.unify_ty(&arg.get_type(), &expected);
        }

        Call {
            name: self.name.clone(),
            type_args,
            args,
        }
    }
}

impl Rewrite for Call {
    fn rewrite(&self, table: &SplitTable) -> Self {
        // `self.name` refers to a `Def`, which splitting never duplicates, only its argument
        // and result types can reference split declarations.
        Call {
            name: self.name.clone(),
            type_args: self.type_args.rewrite(table),
            args: self.args.rewrite(table),
        }
    }
}

#[cfg(test)]
mod check_tests {
    use crate::{
        syntax::{Statement, TypingContext},
        typing::{check::Checked, env::GlobalEnv},
    };
    extern crate self as core_lang;
    use core_macros::{bind, call, def, exit, id, lit, prd, tparam, tvar, ty, var};

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
            [tparam!(id!("A"), "+")],
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
            [tparam!(id!("A"), "+")],
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
            [tparam!(id!("A"), "+")],
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
    fn call_check_poly_polarity_mismatch() {
        // def identity[A-](x: A): A - A is declared negative/codata
        let poly_def = def!(
            id!("identity"),
            [tparam!(id!("A"), "-")],
            [bind!(id!("x"), prd!(), tvar!(id!("A")))],
            exit!(var!(id!("x")), tvar!(id!("A")))
        );
        let defs = vec![poly_def];

        // identity[i64](42) - i64 is always positive/data, mismatching the declared polarity
        let call_stmt: Statement = call!(id!("identity"), [ty!("int")], [lit!(42)]).into();

        assert!(matches!(
            call_stmt.check(
                &[],
                &TypingContext::default(),
                &GlobalEnv::new(&vec![], &vec![], &defs),
            ),
            Err(crate::typing::errors::LocatedTypeError {
                error: crate::typing::errors::TypeError::PolarityMismatch { .. },
                ..
            })
        ));
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
    use core_macros::{bind, call, def, exit, id, lit, prd, tparam, tvar, ty, var};

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
            [tparam!(id!("A"), "+")],
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

#[cfg(test)]
mod label_and_unify_tests {
    use crate::splitting::labeling::{DeclSignature, DeclSignatures, LabelAndUnify, SplitState};
    use crate::syntax::*;
    use crate::traits::*;
    extern crate self as core_lang;
    use core_macros::{call, ctor, id, ty};

    #[test]
    fn label_and_unify_merges_argument_with_declared_parameter() {
        let mut state = SplitState::default();
        let param_label = state.label_ty(&ty!(id!("Box")));

        let mut sigs = DeclSignatures::new();
        sigs.insert(
            id!("f"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![param_label.clone()],
                self_referential: vec![],
            },
        );
        sigs.insert(
            id!("Pack"),
            DeclSignature {
                decl_type_params: vec![],
                own_type_params: vec![],
                tys: vec![],
                self_referential: vec![],
            },
        );

        // the argument is itself a freshly constructed `Box`, independently labeled from the
        // declared parameter type in `sigs`
        let example = call!(id!("f"), [ctor!(id!("Pack"), [], [], ty!(id!("Box")))]);

        let result: Call = example.label_and_unify(&mut state, &sigs, &TypingContext::default());
        let arg_ty = result.args.entries[0].get_type();

        let (
            Ty::Decl {
                name: param_name, ..
            },
            Ty::Decl { name: arg_name, .. },
        ) = (&param_label, &arg_ty)
        else {
            panic!("expected Ty::Decl on both sides");
        };
        assert_eq!(state.uf.find(param_name), state.uf.find(arg_name));
    }
}
