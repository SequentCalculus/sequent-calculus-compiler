//! This module defines constructor terms of data types.

use derivative::Derivative;
use miette::SourceSpan;
use printer::*;

use crate::syntax::*;
use crate::traits::*;
use crate::typing::inference::Inference;
use crate::typing::inference::args_constraint_equations;
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

impl Check for Constructor {
    fn check(
        mut self,
        symbol_table: &mut SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let type_args = match expected {
            Ty::Decl { type_args, .. } => type_args,
            Ty::I64 { .. } => {
                return Err(Error::ExpectedI64ForConstructor {
                    span: self.span,
                    name: self.id,
                });
            }
        };

        // the name of the constructor in the symbol table for the instantiated data type, the
        // instance must exists already
        let name = self.id.clone() + &type_args.print_to_string(None);
        match symbol_table.ctors.get(&name) {
            Some(types) => {
                let (ty, _) = symbol_table.lookup_ty_for_ctor(&self.span, &name)?;

                self.args =
                    check_args(&self.span, symbol_table, context, self.args, &types.clone())?;

                check_equality(&self.span, symbol_table, expected, &ty)?;

                self.ty = Some(expected.clone());
                Ok(self)
            }
            None => Err(Error::Undefined {
                span: Some(self.span),
                name: self.id.clone(),
            }),
        }
    }
}

impl Inference for Constructor {
    fn constraint_equations(
            &mut self,
            symbol_table: &mut SymbolTable,
            context: &TypingContext,
            var_name_generator: &mut inference::VarNameGenerator,
            ty_var: Ty
        ) -> Result<Vec<(Ty,Ty)>, Error> {

        let data_type_name = match symbol_table.find_xdata_type_name(&self.id) {
            Some(type_name) => type_name,
            None => {
                return Err(Error::Undefined { span: Some(self.span.clone()), name: self.id.clone() });
            }
        };

        let (chirality, general_type_vars, _) = symbol_table.type_templates.get(&data_type_name).unwrap();

        if chirality == &Polarity::Codata {
            return Err(Error::ExpectedDataForNew { span: self.span, data: data_type_name });
        }
        
        // this instance of the Data Type is instanciated, whith replacing the general type vars
        // with instance type variables eg. (A -> a1)

        let mut type_var_mapping: HashMap<Name, Name> = HashMap::new();
        for type_var in &general_type_vars.bindings {
            type_var_mapping.insert(type_var.clone(), var_name_generator.get_new_name());
        }


        let instanciated_template = match symbol_table.ctor_templates.get(&self.id) {
            Some(ctor_template) => {
                let mut template = ctor_template.clone();

                // type var mapping logs the used type variables and maps them to the instantiated type var name
                for binding in &mut template.bindings {
                    match &mut binding.ty {
                        Ty::I64 { .. } => { continue;},
                        Ty::Decl {span , name, type_args } => {
                            // if we find a general typ variable as type name it is replaced with a new type variable
                            if let Some(used_type_name) = type_var_mapping.get(name) {
                                *name = used_type_name.clone();
                            }
                        }
                    }
                }

                template          
            },
            None => {
                return Err(Error::Undefined {
                span: Some(self.span),
                name: self.id.clone(),
            })}

        };

        if instanciated_template.bindings.len() != self.args.entries.len() {
            return Err(Error::WrongNumberOfArguments {
                span: self.span,
                expected: instanciated_template.bindings.len(),
                got: self.args.entries.len()});
        }

        let mut constraints = Vec::new();

        constraints.append(&mut args_constraint_equations(&mut self.args, &instanciated_template, symbol_table, context, var_name_generator, self.span)?);
        let expected_type = Ty::Decl { span: Some(self.span), name: data_type_name, type_args: TypeArgs {
            span: Some(self.span),
            args: instanciated_template.bindings.iter().map(|bind| bind.ty.clone()).collect()
        }};

        let new_type_var = var_name_generator.get_new_ty_var();

        constraints.push((new_type_var, ty_var.clone()));
        constraints.push((ty_var, expected_type));

        Ok(constraints)

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
    use crate::typing::*;

    #[test]
    fn check_nil() {
        let result = Constructor {
            span: dummy_span(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: None,
        }
        .check(
            &mut symbol_table_list(),
            &TypingContext::default(),
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Constructor {
            span: dummy_span(),
            id: "Nil".to_owned(),
            args: vec![].into(),
            ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_cons() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_i64());
        let result = Constructor {
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
        }
        .check(
            &mut symbol_table_list(),
            &ctx,
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        )
        .unwrap();
        let expected = Constructor {
            span: dummy_span(),
            id: "Cons".to_owned(),
            args: vec![
                XVar {
                    span: dummy_span(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_i64()),
                    chi: Some(Prd),
                }
                .into(),
                Constructor {
                    span: dummy_span(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                }
                .into(),
            ]
            .into(),
            ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_ctor_fail() {
        let result = Constructor {
            span: dummy_span(),
            id: "Cons".to_owned(),
            args: vec![
                Constructor {
                    span: dummy_span(),
                    id: "Nil".to_owned(),
                    args: vec![].into(),
                    ty: None,
                }
                .into(),
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
        }
        .check(
            &mut symbol_table_list(),
            &TypingContext {
                span: None,
                bindings: vec![],
            },
            &Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        assert!(result.is_err());
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
