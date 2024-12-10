use std::rc::Rc;

use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, tokens::DOT, Print};

use crate::{
    parser::util::ToMiette,
    syntax::{
        context::TypingContext,
        substitution::Substitution,
        types::{OptTyped, Ty},
        Name,
    },
    typing::{
        check::{check_args, check_equality, Check},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

use super::Term;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq)]
pub struct Destructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub destructee: Rc<Term>,
    pub args: Substitution,
    pub ty: Option<Ty>,
}

impl OptTyped for Destructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Destructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            self.destructee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
        } else {
            self.destructee
                .print(cfg, alloc)
                .append(DOT)
                .append(alloc.dtor(&self.id))
                .append(self.args.print(cfg, alloc).parens())
        }
    }
}

impl From<Destructor> for Term {
    fn from(value: Destructor) -> Self {
        Term::Destructor(value)
    }
}

impl Check for Destructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        let ty = symbol_table.lookup_ty_for_dtor(&self.span.to_miette(), &self.id)?;
        let destructee_checked = self.destructee.check(symbol_table, context, &ty)?;
        match symbol_table.dtors.get(&self.id) {
            Some((types, ret_ty)) => {
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                check_equality(&self.span.to_miette(), expected, ret_ty)?;
                Ok(Destructor {
                    destructee: destructee_checked,
                    args: new_args,
                    ty: Some(expected.clone()),
                    ..self
                })
            }
            None => Err(Error::Undefined {
                span: self.span.to_miette(),
                name: self.id.clone(),
            }),
        }
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::Check;
    use crate::{
        parser::fun,
        syntax::{
            context::{ContextBinding, TypingContext},
            substitution::SubstitutionBinding,
            terms::{Destructor, Lit, Var},
            types::Ty,
        },
        test_common::{symbol_table_fun, symbol_table_lpair},
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use printer::Print;
    use std::rc::Rc;

    #[test]
    fn check_fst() {
        let symbol_table = symbol_table_lpair();
        let result = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            args: vec![],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("LPairIntInt"),
                }],
            },
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            args: vec![],
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("LPairIntInt")),
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ap() {
        let symbol_table = symbol_table_fun();
        let result = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(Lit::mk(1).into()),
                SubstitutionBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: None,
                },
            ],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &symbol_table,
            &TypingContext {
                span: Span::default(),
                bindings: vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_decl("FunIntInt"),
                    },
                    ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: Ty::mk_int(),
                    },
                ],
            },
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(Lit::mk(1).into()),
                SubstitutionBinding::CovarBinding {
                    covar: "a".to_owned(),
                    ty: Some(Ty::mk_int()),
                },
            ],
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: Some(Ty::mk_decl("FunIntInt")),
                }
                .into(),
            ),
            ty: Some(Ty::mk_int()),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_dtor_fail() {
        let result = Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            args: vec![],
            destructee: Rc::new(Var::mk("x").into()),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("StreamInt"),
                }],
            },
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }

    /// "x.hd"
    fn example_1() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![],
            ty: None,
        }
    }

    /// "x.hd.hd"
    fn example_2() -> Destructor {
        Destructor {
            span: Span::default(),
            id: "Hd".to_owned(),
            destructee: Rc::new(example_1().into()),
            args: vec![],
            ty: None,
        }
    }

    #[test]
    fn display_1() {
        assert_eq!(example_1().print_to_string(Default::default()), "x.Hd")
    }

    #[test]
    fn display_2() {
        assert_eq!(example_2().print_to_string(Default::default()), "x.Hd.Hd")
    }

    #[test]
    fn display_3() {
        let dest = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            destructee: Rc::new(Var::mk("x").into()),
            args: vec![Var::mk("y").into(), Var::mk("z").into()],
            ty: None,
        };
        let result = dest.print_to_string(Default::default());
        let expected = "x.Fst(y, z)".to_owned();
        assert_eq!(result, expected)
    }

    #[test]
    fn parse_1() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd"), Ok(example_1().into()));
    }

    #[test]
    fn parse_2() {
        let parser = fun::TermParser::new();
        assert_eq!(parser.parse("x.Hd.Hd"), Ok(example_2().into()));
    }
}
