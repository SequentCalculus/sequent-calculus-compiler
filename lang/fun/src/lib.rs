pub mod parser;
pub mod syntax;
pub mod typing;

#[cfg(feature = "test-common")]
pub mod test_common {
    use super::{
        syntax::{
            context::TypingContext,
            declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig},
            substitution::SubstitutionBinding,
            terms::{BinOp, Case, Clause, Fun, Lit, Op, Var},
            types::Ty,
        },
        typing::symbol_table::{Polarity, SymbolTable},
    };
    use codespan::Span;
    use std::rc::Rc;

    fn context_cons() -> TypingContext {
        let mut ctx_cons = TypingContext::default();
        ctx_cons.add_var("x", Ty::mk_int());
        ctx_cons.add_var("xs", Ty::mk_decl("ListInt"));
        ctx_cons
    }

    pub fn data_list() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext::default(),
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: context_cons(),
                },
            ],
        }
    }

    pub fn symbol_table_list() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.ty_ctors.insert(
            "ListInt".to_owned(),
            (Polarity::Data, vec!["Nil".to_owned(), "Cons".to_owned()]),
        );
        table
            .ctors
            .insert("Nil".to_owned(), TypingContext::default());
        table.ctors.insert("Cons".to_owned(), context_cons());
        table
    }

    fn context_tup() -> TypingContext {
        let mut ctx_tup = TypingContext::default();
        ctx_tup.add_var("x", Ty::mk_int());
        ctx_tup.add_var("y", Ty::mk_int());
        ctx_tup
    }
    pub fn data_tup() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "TupIntInt".to_owned(),
            ctors: vec![CtorSig {
                span: Span::default(),
                name: "Tup".to_owned(),
                args: context_tup(),
            }],
        }
    }

    pub fn symbol_table_tup() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.ty_ctors.insert(
            "TupIntInt".to_owned(),
            (Polarity::Data, vec!["Tup".to_owned()]),
        );
        table.ctors.insert("Tup".to_owned(), context_tup());
        table
    }

    pub fn codata_stream() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }

    pub fn symbol_table_stream() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.ty_ctors.insert(
            "StreamInt".to_owned(),
            (Polarity::Codata, vec!["Hd".to_owned(), "Tl".to_owned()]),
        );
        table
            .dtors
            .insert("Hd".to_owned(), (TypingContext::default(), Ty::mk_int()));
        table.dtors.insert(
            "Tl".to_owned(),
            (TypingContext::default(), Ty::mk_decl("StreamInt")),
        );
        table
    }

    fn context_ap() -> TypingContext {
        let mut ctx_ap = TypingContext::default();
        ctx_ap.add_var("x", Ty::mk_int());
        ctx_ap.add_covar("a", Ty::mk_int());
        ctx_ap
    }

    pub fn codata_fun() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "FunIntInt".to_owned(),
            dtors: vec![DtorSig {
                span: Span::default(),
                name: "Ap".to_owned(),
                args: context_ap(),
                cont_ty: Ty::mk_int(),
            }],
        }
    }

    pub fn symbol_table_fun() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.ty_ctors.insert(
            "FunIntInt".to_owned(),
            (Polarity::Codata, vec!["Ap".to_owned()]),
        );
        table
            .dtors
            .insert("Ap".to_owned(), (context_ap(), Ty::mk_int()));
        table
    }

    pub fn codta_lpair() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "LPairIntInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Fst".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Snd".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_int(),
                },
            ],
        }
    }

    pub fn symbol_table_lpair() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.ty_ctors.insert(
            "LPairIntInt".to_owned(),
            (Polarity::Codata, vec!["Fst".to_owned(), "Snd".to_owned()]),
        );
        table
            .dtors
            .insert("Fst".to_owned(), (TypingContext::default(), Ty::mk_int()));
        table
            .dtors
            .insert("Snd".to_owned(), (TypingContext::default(), Ty::mk_int()));
        table
    }

    fn context_mult() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("ListInt"));
        ctx
    }

    pub fn def_mult() -> Definition {
        Definition {
            span: Span::default(),
            name: "mult".to_owned(),
            context: context_mult(),
            body: Case {
                span: Span::default(),
                destructee: Rc::new(Var::mk("l").into()),
                cases: vec![
                    Clause {
                        span: Span::default(),
                        xtor: "Nil".to_owned(),
                        context: TypingContext::default(),
                        rhs: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        xtor: "Cons".to_owned(),
                        context: context_cons(),
                        rhs: Op {
                            span: Span::default(),
                            fst: Rc::new(Var::mk("x").into()),
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Fun {
                                    span: Span::default(),
                                    name: "mult".to_owned(),
                                    args: vec![SubstitutionBinding::TermBinding(
                                        Var::mk("xs").into(),
                                    )],
                                    ret_ty: None,
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    },
                ],
                ty: None,
            }
            .into(),
            ret_ty: Ty::mk_int(),
        }
    }

    pub fn def_mult_typed() -> Definition {
        Definition {
            span: Span::default(),
            name: "mult".to_owned(),
            context: context_mult(),
            ret_ty: Ty::mk_int(),
            body: Case {
                span: Span::default(),
                destructee: Rc::new(
                    Var {
                        span: Span::default(),
                        var: "l".to_owned(),
                        ty: Some(Ty::mk_decl("ListInt")),
                    }
                    .into(),
                ),
                cases: vec![
                    Clause {
                        span: Span::default(),
                        xtor: "Nil".to_owned(),
                        context: TypingContext::default(),
                        rhs: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        xtor: "Cons".to_owned(),
                        context: context_cons(),
                        rhs: Op {
                            span: Span::default(),
                            fst: Rc::new(
                                Var {
                                    span: Span::default(),
                                    var: "x".to_owned(),
                                    ty: Some(Ty::mk_int()),
                                }
                                .into(),
                            ),
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Fun {
                                    span: Span::default(),
                                    name: "mult".to_owned(),
                                    args: vec![SubstitutionBinding::TermBinding(
                                        Var {
                                            span: Span::default(),
                                            var: "xs".to_owned(),
                                            ty: Some(Ty::mk_decl("ListInt")),
                                        }
                                        .into(),
                                    )],
                                    ret_ty: Some(Ty::mk_int()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    },
                ],
                ty: Some(Ty::mk_int()),
            }
            .into(),
        }
    }
}
