//! This module defines invoking destructors of codata types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::DOT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::VariationalConstraint;
use crate::typing::inference::insert_arg_types;
use crate::typing::inference::{TypeInference, collect_arg_constraints};
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::collections::HashMap;
use std::{collections::HashSet, rc::Rc};

/// This struct defines an invocation of a destructor of codata type. It consists of the scrutinee
/// on which to invoke the destructor, the name of the destructor, a list of type arguments
/// instantiating the type parameters of the codata type, the arguments of the destructor, and
/// after typechecking also of the inferred type.
///
/// Example:
/// `stream.Head[i64]` invokes the destructor `Head` on a `stream` with type argument `i64`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The term the destructor is invoked on
    pub scrutinee: Rc<Term>,
    /// The destructor name
    pub id: Name,
    /// The type arguments instantiating the type parameters of the type
    pub type_args: TypeArgs,
    /// The arguments of the destructor
    pub args: Arguments,
    /// Type (inferred) of the term
    pub ty: Option<Ty>,
}

impl OptTyped for Destructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Destructor {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        if (matches!(*self.scrutinee, Term::XVar(_))
            || matches!(*self.scrutinee, Term::Call(ref call) if call.args.entries.is_empty()))
            && (self.scrutinee.print_to_string(Some(cfg)).len() <= cfg.indent.cast_unsigned())
        {
            self.scrutinee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
        } else {
            self.scrutinee
                .print(cfg, alloc)
                .append(alloc.line_())
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.type_args.print(cfg, alloc))
                .append(args.group())
                .nest(cfg.indent)
                .align()
        }
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

impl TypeInference for Destructor {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(expected_ty.clone());

        let codata_type_name = match state.symbol_table.find_xdata_type_name(&self.id) {
            Some(type_name) => type_name,
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        let (polarity, type_params, _) = state
            .symbol_table
            .type_templates
            .get(&codata_type_name)
            .unwrap();

        if polarity == &Polarity::Data {
            return Err(Error::ExpectedCovariableGotTerm { span: self.span });
        }

        // instanciating new type variables

        let mut instantiation: HashMap<Name, Ty> = HashMap::new();

        if type_params.bindings.len() == self.type_args.args.len() {
            // if the right amount of type arguments is given they are used

            for (type_var_name, given_ty) in
                type_params.bindings.iter().zip(self.type_args.args.iter())
            {
                instantiation.insert(type_var_name.clone(), given_ty.clone());
            }
        } else if self.type_args.args.is_empty() {
            // if no type Arguments are given, they are all replaced by variables,

            for type_var in &type_params.bindings {
                instantiation.insert(type_var.clone(), state.fresh_names.fresh_ty_var());
            }
        } else {
            // if the wrong amount of type arguments are given, an error is returned

            return Err(Error::WrongNumberOfTypeArguments {
                span: Some(self.span),
                expected: type_params.bindings.len(),
                got: self.type_args.args.len(),
            });
        }

        // collecting the expected signature of the dtor
        let (mut arg_types, mut out_type) = match state.symbol_table.dtor_templates.get(&self.id) {
            Some((in_tys, out_tys)) => (in_tys.clone(), out_tys.clone()),
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        // replacing the general type vars for instaciated ones

        let new_arg_types = arg_types
            .bindings
            .iter()
            .map(|arg_ty| arg_ty.clone().subst_ty(&instantiation))
            .collect();
        arg_types.bindings = new_arg_types;

        out_type = out_type.subst_ty(&instantiation);

        // putting together the instantiated and expected type for the scrutinee and creating the constraints for it
        let scrutinee_type_args: Vec<Ty> = type_params
            .bindings
            .iter()
            .map(|binding| instantiation.get(binding).unwrap())
            .cloned()
            .collect();
        let scrutinee_type = Ty::mk_decl(
            &codata_type_name,
            TypeArgs {
                span: None,
                args: scrutinee_type_args,
            },
        );

        self.scrutinee
            .collect_constraints(state, context, scrutinee_type)?;

        collect_arg_constraints(&mut self.args, &arg_types, context, state, self.span)?;

        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, out_type));

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.scrutinee
            .insert_inferred_types(type_vars, symbol_table, world)?;

        for ty in &mut self.type_args.args {
            ty.mut_subst_ty(type_vars);
            ty.check(&Some(self.span), symbol_table)?;
        }

        insert_arg_types(&mut self.args, type_vars, symbol_table, world)?;

        match &mut self.ty {
            Some(ty) => {
                ty.mut_subst_ty(type_vars);
                ty.check(&Some(self.span), symbol_table)
            }
            None => panic!(
                "The Type of the term {:?} is not set after type inference",
                self
            ),
        }
    }
}

impl UsedBinders for Destructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.scrutinee.used_binders(used);
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod destructor_tests {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;

    use std::rc::Rc;
    use std::vec;

    #[test]
    fn inference_lpair_fst() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );

        let mut term = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_lpair(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let scrutinee_type = Ty::mk_decl(
            "LPair",
            TypeArgs::mk(vec![Ty::mk_ty_var("0"), Ty::mk_ty_var("1")]),
        );

        let expected = vec![
            VariationalConstraint::equal(
                scrutinee_type.clone(),
                Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
            ),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_ty_var("0")),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_lpair_fst_with_type_annotation() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );

        let mut term = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_lpair(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let scrutinee_type = Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]));

        let expected = vec![
            VariationalConstraint::equal(
                scrutinee_type.clone(),
                Ty::mk_decl("LPair", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
            ),
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_i64()),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_ap() {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "x",
            Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
        );
        ctx.add_covar("a", Ty::mk_i64());

        let mut term = Destructor {
            span: dummy_span(),
            id: "apply".to_owned(),
            type_args: TypeArgs::mk(vec![]),
            args: vec![Lit::mk(1).into(), XVar::mk("a").into()].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_fun_template(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            // scrutinee,
            VariationalConstraint::equal(
                Ty::mk_decl(
                    "Fun",
                    TypeArgs::mk(vec![Ty::mk_ty_var("0"), Ty::mk_ty_var("1")]),
                ),
                Ty::mk_decl("Fun", TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()])),
            ),
            // argument 1
            VariationalConstraint::equal(Ty::mk_ty_var("0"), Ty::mk_i64()),
            // argument 2,
            VariationalConstraint::equal(Ty::mk_ty_var("2"), Ty::mk_i64()),
            VariationalConstraint::equal(Ty::mk_ty_var("1"), Ty::mk_i64()),
            //final type constraint
            VariationalConstraint::equal(Ty::mk_ty_var("x"), Ty::mk_ty_var("1")),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected)
    }

    #[test]
    fn inference_dtor_fail() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])));

        let mut state = TypeInferenceState {
            symbol_table: Default::default(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        let result = Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            args: vec![].into(),
            scrutinee: Rc::new(XVar::mk("x").into()),
            ty: None,
        }
        .collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"));

        assert!(result.is_err())
    }

    /// "x.head"
    fn example_1() -> Destructor {
        Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![].into(),
            ty: None,
        }
    }

    /// "x.head.head"
    fn example_2() -> Destructor {
        Destructor {
            span: dummy_span(),
            id: "head".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
            scrutinee: Rc::new(example_1().into()),
            args: vec![].into(),
            ty: None,
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(
            example_1().print_to_string(Default::default()),
            "x.head[i64]"
        )
    }

    #[test]
    fn display_2() {
        assert_eq!(
            example_2().print_to_string(Default::default()),
            "x.head[i64]\n    .head[i64]"
        )
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: dummy_span(),
            id: "fst".to_owned(),
            type_args: TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
            scrutinee: Rc::new(XVar::mk("x").into()),
            args: vec![XVar::mk("y").into(), XVar::mk("z").into()].into(),
            ty: None,
        };
        let result = dest.print_to_string(Default::default());
        let expected = "x.fst[i64, i64](y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.head[i64]"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(
            parser.parse("x.head[i64].head[i64]"),
            Ok(example_2().into())
        );
    }
}
