//! This module defines constructor terms of data types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::TypeInference;
use crate::typing::inference::TypeInferenceState;
use crate::typing::inference::VariationalConstraint;
use crate::typing::inference::{TypeVarMapping, World};
use crate::typing::*;

use std::collections::HashSet;

/// This struct defines a constructor term of a data type. It consists of a name for the
/// constructor, the arguments of the constructor, and after typechecking also of the inferred
/// type.
///
/// Example:
/// `Cons(2, Nil)` is the constructor `Cons` with arguments `2` and constructor `Nil`.
#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Constructor {
    /// The source location
    #[derivative(PartialEq = "ignore")]
    pub span: SourceSpan,
    /// The constructor name
    pub id: Name,
    /// The arguments of the constructor
    pub args: Arguments,
    /// The (inferred) type of the constructor
    pub ty: Option<Ty>,
}

impl OptTyped for Constructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Constructor {
    fn print<'a>(&'a self, cfg: &PrintCfg, alloc: &'a Alloc<'a>) -> Builder<'a> {
        let args = if self.args.entries.is_empty() {
            alloc.nil()
        } else {
            self.args.print(cfg, alloc).parens()
        };

        alloc.ctor(&self.id).append(args.group())
    }
}

impl From<Constructor> for Term {
    fn from(value: Constructor) -> Self {
        Term::Constructor(value)
    }
}

impl TypeInference for Constructor {
    fn collect_constraints(
        &mut self,
        state: &mut TypeInferenceState,
        context: &TypingContext,
        expected_ty: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(expected_ty.clone());

        let (data_name, type_params, _) =
            state.symbol_table.data_type_of_ctor(&self.id, self.span)?;

        // every use of a constructor instantiates the type parameters of its data type with fresh
        // type variables
        let instantiation = type_params.instantiate(&[], self.span, &mut state.fresh_names)?;
        let data_type = Ty::mk_instance(&data_name, &type_params, &instantiation, Some(self.span));

        let instanciated_template = match state.symbol_table.ctor_templates.get(&self.id) {
            Some(ctor_template) => ctor_template.clone().subst_ty(&instantiation),
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        self.args
            .collect_constraints(&instanciated_template, context, state, self.span)?;

        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, data_type));

        Ok(())
    }

    fn insert_inferred_types(
        &mut self,
        type_vars: &TypeVarMapping,
        symbol_table: &mut SymbolTable,
        world: &World,
    ) -> Result<(), Error> {
        self.args
            .insert_inferred_types(type_vars, symbol_table, world)?;

        Ty::insert_inferred_annotation(&mut self.ty, self.span, type_vars, symbol_table)
    }
}

impl UsedBinders for Constructor {
    fn used_binders(&self, used: &mut HashSet<Var>) {
        self.args.entries.used_binders(used);
    }
}

#[cfg(test)]
mod test {
    use printer::Print;

    use crate::parser::fun;
    use crate::syntax::util::dummy_span;
    use crate::syntax::*;
    use crate::test_common::*;
    use crate::typing::inference::TypeInference;
    use crate::typing::inference::TypeInferenceState;
    use crate::typing::inference::VariationalConstraint;

    #[test]
    fn inference_nil() {
        let mut term = Constructor {
            span: dummy_span(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_list(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &TypingContext::default(), Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![VariationalConstraint::equal(
            Ty::mk_ty_var("x"),
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
        )];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    #[test]
    fn inference_cons() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let mut term = Constructor {
            span: dummy_span(),
            id: "Cons".to_owned(),
            args: vec![
                XVar::mk("x").into(),
                Constructor {
                    span: dummy_span(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
            ]
            .into(),
            ty: None,
        };

        let mut state = TypeInferenceState {
            symbol_table: symbol_table_list(),
            fresh_names: Default::default(),
            constraints: Default::default(),
            choices: Default::default(),
        };

        term.collect_constraints(&mut state, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            // cons
            VariationalConstraint::equal(Ty::mk_ty_var("0"), Ty::mk_i64()),
            // nil
            VariationalConstraint::equal(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")])),
            ),
            VariationalConstraint::equal(
                Ty::mk_ty_var("x"),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
            ),
        ];

        let TypeInferenceState {
            constraints: result,
            ..
        } = state;

        assert_eq!(result, expected);
        assert_eq!(term.ty, Some(Ty::mk_ty_var("x")));
    }

    fn example_nil() -> Constructor {
        Constructor {
            span: dummy_span(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            span: dummy_span(),
            id: "Tup".to_owned(),
            args: vec![Term::Lit(Lit::mk(2)).into(), Term::Lit(Lit::mk(4)).into()].into(),
            ty: None,
        }
    }

    #[test]
    fn display_nil() {
        assert_eq!(example_nil().print_to_string(Default::default()), "Nil")
    }

    #[test]
    fn parse_nil() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Nil"), Ok(example_nil().into()));
    }

    #[test]
    fn display_tup() {
        assert_eq!(
            example_tup().print_to_string(Default::default()),
            "Tup(2, 4)"
        )
    }

    #[test]
    fn parse_tup() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("Tup(2,4)"), Ok(example_tup().into()));
    }
}
