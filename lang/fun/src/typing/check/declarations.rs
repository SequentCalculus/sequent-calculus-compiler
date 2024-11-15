#[cfg(test)]
mod decl_tests {
    use crate::{
        parser::util::ToMiette,
        syntax::{
            context::ContextBinding,
            declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig},
            substitution::SubstitutionBinding,
            terms::{Constructor, Lit},
            types::Ty,
        },
        typing::symbol_table::{BuildSymbolTable, Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::collections::HashSet;
    fn example_data() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: vec![],
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: vec![
                        ContextBinding::TypedVar {
                            var: "x".to_owned(),
                            ty: Ty::mk_int(),
                        },
                        ContextBinding::TypedVar {
                            var: "xs".to_owned(),
                            ty: Ty::mk_decl("ListInt"),
                        },
                    ],
                },
            ],
        }
    }
    fn example_codata() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: vec![],
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: vec![],
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }
    fn example_def() -> Definition {
        Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: vec![],
            ret_ty: Ty::mk_decl("ListInt"),
            body: Constructor {
                span: Span::default(),
                id: "Cons".to_owned(),
                args: vec![
                    SubstitutionBinding::TermBinding(
                        Lit {
                            span: Span::default(),
                            val: 1,
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
            .into(),
        }
    }

    #[test]
    fn data_check() {
        let mut symbol_table = SymbolTable::default();
        example_data().build(&mut symbol_table).unwrap();
        let result = example_data().check(&symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn codata_check() {
        let mut symbol_table = SymbolTable::default();
        example_codata().build(&mut symbol_table).unwrap();
        let result = example_codata().check(&symbol_table);
        assert!(result.is_ok())
    }
    #[test]
    fn def_check() {
        let mut symbol_table = SymbolTable::default();
        example_def().build(&mut symbol_table).unwrap();
        example_data().build(&mut symbol_table).unwrap();
        let result = example_def().check(&symbol_table).unwrap();
        let expected = Definition {
            span: Span::default(),
            name: "main".to_owned(),
            context: vec![],
            ret_ty: Ty::mk_decl("ListInt"),
            body: Constructor {
                span: Span::default(),
                id: "Cons".to_owned(),
                args: vec![
                    SubstitutionBinding::TermBinding(
                        Lit {
                            span: Span::default(),
                            val: 1,
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
            }
            .into(),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn dtor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        let result = symbol_table
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Fst".to_owned())
            .unwrap();
        let expected = Ty::mk_decl("LPairIntInt");
        assert_eq!(result, expected)
    }
    #[test]
    fn dtor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_dtor(&Span::default().to_miette(), &"Snd".to_owned());
        assert!(result.is_err())
    }
    #[test]
    fn ctor_lookup() {
        let mut symbol_table = SymbolTable::default();
        symbol_table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        let result = symbol_table
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil".to_owned())
            .unwrap();
        let expected = (
            Ty::mk_decl("ListInt"),
            HashSet::from(["Nil".to_owned(), "Cons".to_owned()]),
        );
        assert_eq!(result, expected)
    }
    #[test]
    fn ctor_lookup_fail() {
        let result = SymbolTable::default()
            .lookup_ty_for_ctor(&Span::default().to_miette(), &"Nil".to_owned());
        assert!(result.is_err())
    }
}
