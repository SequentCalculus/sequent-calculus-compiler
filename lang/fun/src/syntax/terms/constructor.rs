use codespan::Span;
use derivative::Derivative;
use printer::{theme::ThemeExt, Print};

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
pub struct Constructor {
    #[derivative(PartialEq = "ignore")]
    pub span: Span,
    pub id: Name,
    pub args: Substitution,
    pub ty: Option<Ty>,
}

impl OptTyped for Constructor {
    fn get_type(&self) -> Option<Ty> {
        self.ty.clone()
    }
}

impl Print for Constructor {
    fn print<'a>(
        &'a self,
        cfg: &printer::PrintCfg,
        alloc: &'a printer::Alloc<'a>,
    ) -> printer::Builder<'a> {
        if self.args.is_empty() {
            alloc.ctor(&self.id)
        } else {
            alloc
                .ctor(&self.id)
                .append(self.args.print(cfg, alloc).parens())
        }
    }
}

impl From<Constructor> for Term {
    fn from(value: Constructor) -> Self {
        Term::Constructor(value)
    }
}

impl Check for Constructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Self, Error> {
        match symbol_table.ctors.get(&self.id) {
            Some(types) => {
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                let (ty, _) = symbol_table.lookup_ty_for_ctor(&self.span.to_miette(), &self.id)?;
                check_equality(&self.span.to_miette(), expected, &ty)?;
                Ok(Constructor {
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
mod test {
    use super::{Check, Term};
    use crate::{
        parser::fun,
        syntax::context::TypingContext,
        syntax::terms::Lit,
        syntax::{
            substitution::SubstitutionBinding,
            terms::{Constructor, Var},
            types::Ty,
        },
        test_common::symbol_table_list,
    };
    use codespan::Span;
    use printer::Print;

    #[test]
    fn check_nil() {
        let result = Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![],
            ty: None,
        }
        .check(
            &mut symbol_table_list(),
            &TypingContext::default(),
            &Ty::mk_decl("ListInt"),
        )
        .unwrap();
        let expected = Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![],
            ty: Some(Ty::mk_decl("ListInt")),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_cons() {
        let mut ctx = TypingContext::default();
        ctx.add_var("x", Ty::mk_int());
        let result = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(Var::mk("x").into()),
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                ),
            ],
            ty: None,
        }
        .check(&mut symbol_table_list(), &ctx, &Ty::mk_decl("ListInt"))
        .unwrap();
        let expected = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Var {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_int()),
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: Some(Ty::mk_decl("ListInt")),
                    }
                    .into(),
                ),
            ],
            ty: Some(Ty::mk_decl("ListInt")),
        };
        assert_eq!(result, expected)
    }
    #[test]
    fn check_ctor_fail() {
        let result = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                ),
                SubstitutionBinding::TermBinding(
                    Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                ),
            ],
            ty: None,
        }
        .check(
            &symbol_table_list(),
            &TypingContext {
                span: Span::default(),
                bindings: vec![],
            },
            &Ty::mk_decl("ListInt"),
        );
        assert!(result.is_err());
    }

    fn example_nil() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![],
            ty: None,
        }
    }

    fn example_tup() -> Constructor {
        Constructor {
            span: Span::default(),
            id: "Tup".to_owned(),
            args: vec![Term::Lit(Lit::mk(2)).into(), Term::Lit(Lit::mk(4)).into()],
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
