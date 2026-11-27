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

use std::collections::HashMap;
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

        let data_type_name = match state.symbol_table.find_xdata_type_name(&self.id) {
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
            .get(&data_type_name)
            .unwrap();

        if polarity == &Polarity::Codata {
            return Err(Error::ExpectedDataForNew {
                span: self.span,
                data: data_type_name,
            });
        }

        // this instance of the Data Type is instanciated, whith replacing the general type vars
        // with instance type variables eg. (A -> a1)

        let mut instantiation: HashMap<Name, Ty> = HashMap::new();
        for type_var in &type_params.bindings {
            instantiation.insert(type_var.clone(), state.fresh_names.fresh_ty_var());
        }

        let expected_type = Ty::Decl {
            span: Some(self.span),
            name: data_type_name,
            type_args: TypeArgs {
                span: Some(self.span),
                args: type_params
                    .bindings
                    .iter()
                    .map(|name| instantiation.get(name).unwrap().clone())
                    .collect(),
            },
        };

        let instanciated_template = match state.symbol_table.ctor_templates.get(&self.id) {
            Some(ctor_template) => ctor_template.clone().subst_ty(&instantiation),
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        if instanciated_template.bindings.len() != self.args.entries.len() {
            return Err(Error::WrongNumberOfArguments {
                span: self.span,
                expected: instanciated_template.bindings.len(),
                got: self.args.entries.len(),
            });
        }

        self.args
            .collect_constraints(&instanciated_template, context, state, self.span)?;

        state
            .constraints
            .push(VariationalConstraint::equal(expected_ty, expected_type));

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
