//! This module defines constructor terms of data types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Constraint;
use crate::typing::inference::ConstraintBank;
use crate::typing::inference::Inference;
use crate::typing::inference::args_constraint_equations;
use crate::typing::inference::args_insert_inferred_type;
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

impl Inference for Constructor {
    fn gather_constraints(
        &mut self,
        constraint_bank: &mut ConstraintBank,
        context: &TypingContext,
        ty_var: Ty,
    ) -> Result<(), Error> {
        self.ty = Some(ty_var.clone());

        let data_type_name = match constraint_bank.symbol_table.find_xdata_type_name(&self.id) {
            Some(type_name) => type_name,
            None => {
                return Err(Error::Undefined {
                    span: Some(self.span),
                    name: self.id.clone(),
                });
            }
        };

        let (chirality, general_type_vars, _) = constraint_bank
            .symbol_table
            .type_templates
            .get(&data_type_name)
            .unwrap();

        if chirality == &Polarity::Codata {
            return Err(Error::ExpectedDataForNew {
                span: self.span,
                data: data_type_name,
            });
        }

        // this instance of the Data Type is instanciated, whith replacing the general type vars
        // with instance type variables eg. (A -> a1)

        let mut type_var_mapping: HashMap<Name, Ty> = HashMap::new();
        for type_var in &general_type_vars.bindings {
            type_var_mapping.insert(
                type_var.clone(),
                constraint_bank.var_name_generator.get_new_ty_var(),
            );
        }

        let expected_type = Ty::Decl {
            span: Some(self.span),
            name: data_type_name,
            type_args: TypeArgs {
                span: Some(self.span),
                args: general_type_vars
                    .bindings
                    .iter()
                    .map(|name| type_var_mapping.get(name).unwrap().clone())
                    .collect(),
            },
        };

        let instanciated_template = match constraint_bank.symbol_table.ctor_templates.get(&self.id)
        {
            Some(ctor_template) => ctor_template.clone().subst_ty(&type_var_mapping),
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

        args_constraint_equations(
            &mut self.args,
            &instanciated_template,
            context,
            constraint_bank,
            self.span,
        )?;

        constraint_bank
            .constraints
            .push(Constraint::mk_only_ty(ty_var, expected_type));

        Ok(())
    }

    fn insert_inferred_type(
        &mut self,
        mappings: &HashMap<Name, Ty>,
        symbol_table: &mut SymbolTable,
        choices: &HashMap<u32, usize>,
    ) -> Result<(), Error> {
        args_insert_inferred_type(&mut self.args, mappings, symbol_table, choices)?;

        match &mut self.ty {
            Some(ty_var) => {
                ty_var.mut_subst_ty(mappings);
                ty_var.check(&Some(self.span), symbol_table)
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
    use crate::typing::inference::Constraint;
    use crate::typing::inference::ConstraintBank;
    use crate::typing::inference::Inference;

    #[test]
    fn inference_nil() {
        let mut term = Constructor {
            span: dummy_span(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        };

        let mut constraint_bank = ConstraintBank {
            symbol_table: symbol_table_list(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(
            &mut constraint_bank,
            &TypingContext::default(),
            Ty::mk_ty_var("x"),
        )
        .unwrap();

        let expected = vec![
            Constraint::mk_only_ty(
                Ty::mk_ty_var("x"),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
            ),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

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

        let mut constraint_bank = ConstraintBank {
            symbol_table: symbol_table_list(),
            var_name_generator: Default::default(),
            constraints: Default::default(),
            possible_choices: Default::default(),
        };

        term.gather_constraints(&mut constraint_bank, &ctx, Ty::mk_ty_var("x"))
            .unwrap();

        let expected = vec![
            // cons
            Constraint::mk_only_ty(Ty::mk_ty_var("0"), Ty::mk_i64()),
            // nil
            Constraint::mk_only_ty(
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("1")])),
            ),
            Constraint::mk_only_ty(
                Ty::mk_ty_var("x"),
                Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_ty_var("0")])),
            ),
        ];

        let ConstraintBank {
            constraints: result,
            ..
        } = constraint_bank;

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
