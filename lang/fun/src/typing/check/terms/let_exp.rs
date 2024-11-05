use super::Check;
use crate::{
    syntax::{
        context::{ContextBinding, TypingContext},
        terms::Let,
        types::Ty,
    },
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Let {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Let, Error> {
        let bound_term = self.bound_term.check(symbol_table, context, &self.var_ty)?;
        let mut new_context = context.clone();
        new_context.push(ContextBinding::TypedVar {
            var: self.variable.clone(),
            ty: self.var_ty.clone(),
        });
        let new_in = self.in_term.check(symbol_table, &new_context, expected)?;
        Ok(Let {
            span: self.span,
            variable: self.variable,
            var_ty: self.var_ty,
            in_term: new_in,
            bound_term,
        })
    }
}

#[cfg(test)]
mod let_test {
    use super::Check;
    use crate::{
        syntax::{
            context::ContextBinding,
            substitution::SubstitutionBinding,
            terms::{Constructor, Let, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::rc::Rc;

    #[test]
    fn check_let1() {
        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            in_term: Rc::new(
                Var {
                    span: Span::default(),
                    ty: None,
                    var: "x".to_owned(),
                }
                .into(),
            ),
        }
        .check(&SymbolTable::default(), &vec![], &Ty::mk_int())
        .unwrap();
        let expected = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            in_term: Rc::new(
                Var {
                    span: Span::default(),
                    ty: Some(Ty::mk_int()),
                    var: "x".to_owned(),
                }
                .into(),
            ),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn check_let_fail() {
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

        let result = Let {
            span: Span::default(),
            variable: "x".to_owned(),
            var_ty: Ty::mk_int(),
            bound_term: Rc::new(
                Lit {
                    span: Span::default(),
                    val: 2,
                }
                .into(),
            ),
            in_term: Rc::new(
                Constructor {
                    span: Span::default(),
                    id: "Nil".to_owned(),
                    args: vec![SubstitutionBinding::TermBinding {
                        term: Var {
                            span: Span::default(),
                            var: "x".to_owned(),
                            ty: None,
                        }
                        .into(),
                        ty: None,
                    }],
                }
                .into(),
            ),
        }
        .check(&symbol_table, &vec![], &Ty::mk_decl("ListInt"));
        assert!(result.is_err())
    }
}
