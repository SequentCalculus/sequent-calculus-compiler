use super::{check_args, Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Constructor, types::Ty},
    typing::{
        check::{check_equality, lookup_ty_for_ctor},
        errors::Error,
        symbol_table::SymbolTable,
    },
};

impl Check for Constructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Constructor, Error> {
        match symbol_table.ctors.get(&self.id) {
            Some(types) => {
                let new_args = check_args(
                    &self.span.to_miette(),
                    symbol_table,
                    context,
                    self.args,
                    types,
                )?;
                let (ty, _) = lookup_ty_for_ctor(&self.span.to_miette(), &self.id, symbol_table)?;
                check_equality(&self.span.to_miette(), expected, &ty)?;
                Ok(Constructor {
                    span: self.span,
                    id: self.id,
                    args: new_args,
                    ty: Some(ty.clone()),
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
mod constructor_tests {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            substitution::SubstitutionBinding,
            terms::{Constructor, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;

    fn example_symbols() -> SymbolTable {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        symbol_table.ctors.insert("Nil".to_owned(), vec![]);
        symbol_table.ctors.insert(
            "Cons".to_owned(),
            vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_int(),
                },
                ContextBinding::TypedVar {
                    var: "xs".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                },
            ],
        );
        symbol_table
    }
    #[test]
    fn check_nil() {
        let result = Constructor {
            span: Span::default(),
            id: "Nil".to_owned(),
            args: vec![],
            ty: None,
        }
        .check(&mut example_symbols(), &vec![], &Ty::mk_decl("ListInt"))
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
        let result = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding {
                    term: Var {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: None,
                    }
                    .into(),
                    ty: None,
                },
                SubstitutionBinding::TermBinding {
                    term: Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                    ty: None,
                },
            ],
            ty: None,
        }
        .check(
            &mut example_symbols(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_int(),
            }],
            &Ty::mk_decl("ListInt"),
        )
        .unwrap();
        let expected = Constructor {
            span: Span::default(),
            id: "Cons".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding {
                    term: Var {
                        span: Span::default(),
                        var: "x".to_owned(),
                        ty: Some(Ty::mk_int()),
                    }
                    .into(),
                    ty: Some(Ty::mk_int()),
                },
                SubstitutionBinding::TermBinding {
                    term: Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: Some(Ty::mk_decl("ListInt")),
                    }
                    .into(),
                    ty: Some(Ty::mk_decl("ListInt")),
                },
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
                SubstitutionBinding::TermBinding {
                    term: Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                    ty: None,
                },
                SubstitutionBinding::TermBinding {
                    term: Constructor {
                        span: Span::default(),
                        id: "Nil".to_owned(),
                        args: vec![],
                        ty: None,
                    }
                    .into(),
                    ty: None,
                },
            ],
            ty: None,
        }
        .check(&example_symbols(), &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err());
    }
}
