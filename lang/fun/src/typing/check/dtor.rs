use super::{check_args, check_equality, declarations::lookup_ty_for_dtor, terms::Check};
use crate::{
    parser::util::ToMiette,
    syntax::{context::TypingContext, terms::Destructor, types::Ty},
    typing::{errors::Error, symbol_table::SymbolTable},
};

impl Check for Destructor {
    fn check(
        self,
        symbol_table: &SymbolTable,
        context: &TypingContext,
        expected: &Ty,
    ) -> Result<Destructor, Error> {
        let ty = lookup_ty_for_dtor(&self.span.to_miette(), &self.id, symbol_table)?;
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
                    span: self.span,
                    id: self.id,
                    destructee: destructee_checked,
                    args: new_args,
                    ty: Some(expected.clone()),
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
        syntax::{
            context::ContextBinding,
            substitution::SubstitutionBinding,
            terms::{Destructor, Lit, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::rc::Rc;
    #[test]
    fn check_fst() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        symbol_table
            .dtors
            .insert("Fst".to_owned(), (vec![], Ty::mk_int()));
        symbol_table
            .dtors
            .insert("Snd".to_owned(), (vec![], Ty::mk_int()));
        let result = Destructor {
            span: Span::default(),
            id: "Fst".to_owned(),
            args: vec![],
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: None,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &symbol_table,
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("LPairIntInt"),
            }],
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
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "FunIntInt".to_owned(),
            (Polarity::Codata, vec!["Ap".to_owned()]),
        );
        symbol_table.dtors.insert(
            "Ap".to_owned(),
            (
                vec![
                    ContextBinding::TypedVar {
                        var: "x".to_owned(),
                        ty: Ty::mk_int(),
                    },
                    ContextBinding::TypedCovar {
                        covar: "a".to_owned(),
                        ty: Ty::mk_int(),
                    },
                ],
                Ty::mk_int(),
            ),
        );
        let result = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::CovarBinding("a".to_owned()),
            ],
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: None,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &symbol_table,
            &vec![
                ContextBinding::TypedVar {
                    var: "x".to_owned(),
                    ty: Ty::mk_decl("FunIntInt"),
                },
                ContextBinding::TypedCovar {
                    covar: "a".to_owned(),
                    ty: Ty::mk_int(),
                },
            ],
            &Ty::mk_int(),
        )
        .unwrap();
        let expected = Destructor {
            span: Span::default(),
            id: "Ap".to_owned(),
            args: vec![
                SubstitutionBinding::TermBinding(
                    Lit {
                        span: Span::default(),
                        val: 1,
                    }
                    .into(),
                ),
                SubstitutionBinding::CovarBinding("a".to_owned()),
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
            destructee: Rc::new(
                Var {
                    span: Span::default(),
                    var: "x".to_owned(),
                    ty: None,
                }
                .into(),
            ),
            ty: None,
        }
        .check(
            &SymbolTable::default(),
            &vec![ContextBinding::TypedVar {
                var: "x".to_owned(),
                ty: Ty::mk_decl("StreamInt"),
            }],
            &Ty::mk_int(),
        );
        assert!(result.is_err())
    }
}
