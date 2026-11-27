//! This module defines invoking destructors of codata types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::tokens::DOT;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::VariationalConstraint;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

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

        let (codata_name, type_params, _) = state
            .symbol_table
            .codata_type_of_dtor(&self.id, self.span)?;

        // the type parameters of the codata type are instantiated with the annotated type
        // arguments or with fresh type variables
        let instantiation =
            type_params.instantiate(&self.type_args.args, self.span, &mut state.fresh_names)?;

        // collecting the expected signature of the dtor
        let (arg_types, out_type) = match state.symbol_table.dtor_templates.get(&self.id) {
            Some((in_tys, out_ty)) => (
                in_tys.clone().subst_ty(&instantiation),
                out_ty.clone().subst_ty(&instantiation),
            ),
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        let scrutinee_type = Ty::mk_instance(&codata_name, &type_params, &instantiation, None);

        self.scrutinee
            .collect_constraints(state, context, scrutinee_type)?;

        self.args
            .collect_constraints(&arg_types, context, state, self.span)?;

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
            ty.insert_inferred(&Some(self.span), type_vars, symbol_table)?;
        }

        self.args
            .insert_inferred_types(type_vars, symbol_table, world)?;

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
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
