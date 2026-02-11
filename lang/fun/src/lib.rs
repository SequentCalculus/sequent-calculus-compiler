//! This crate contains the [syntax], [parser] and [type checker](typing) for the surface
//! language Fun, as well as some infrastructure [traits].
pub mod parser;
pub mod syntax;
pub mod traits;
pub mod typing;

/// Some infrastructure for unit tests.
#[cfg(feature = "test-common")]
pub mod test_common {
    use crate::syntax::util::dummy_span;

    use super::{
        syntax::{
            context::{Chirality::Prd, NameContext, TypeContext, TypingContext},
            declarations::{Codata, CtorSig, Data, Def, DtorSig, Polarity},
            names::Var,
            terms::{BinOp, Call, Case, Clause, Lit, Op, XVar},
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use std::rc::Rc;

    fn context_cons(type_param: &str) -> TypingContext {
        let mut ctx_cons = TypingContext::default();
        ctx_cons.add_var("x", 0, Ty::mk_decl(type_param, TypeArgs::default()));
        ctx_cons.add_var(
            "xs",
            0,
            Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl(type_param, TypeArgs::default())]),
            ),
        );
        ctx_cons
    }

    fn context_cons_i64_names() -> NameContext {
        let mut ctx_cons_names = NameContext::default();
        ctx_cons_names.bindings.push(Var {
            name: "x".to_string(),
            id: 0,
        });
        ctx_cons_names.bindings.push(Var {
            name: "xs".to_string(),
            id: 0,
        });
        ctx_cons_names
    }

    fn context_cons_i64() -> TypingContext {
        let mut ctx_cons = TypingContext::default();
        ctx_cons.add_var("x", 0, Ty::mk_i64());
        ctx_cons.add_var(
            "xs",
            0,
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        ctx_cons
    }

    pub fn data_list() -> Data {
        Data {
            span: None,
            name: "List".to_owned(),
            type_params: TypeContext::mk(&vec!["A"]),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_owned(),
                    args: TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_owned(),
                    args: context_cons("A"),
                },
            ],
        }
    }

    pub fn data_list_i64() -> Data {
        Data {
            span: None,
            name: "List[i64]".to_owned(),
            type_params: TypeContext::default(),
            ctors: vec![
                CtorSig {
                    span: None,
                    name: "Nil".to_owned(),
                    args: TypingContext::default(),
                },
                CtorSig {
                    span: None,
                    name: "Cons".to_owned(),
                    args: context_cons_i64(),
                },
            ],
        }
    }

    pub fn symbol_table_list_template() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "List".to_owned(),
            (
                Polarity::Data,
                TypeContext::mk(&vec!["A"]),
                vec!["Nil".to_owned(), "Cons".to_owned()],
            ),
        );
        table
            .ctor_templates
            .insert("Nil".to_owned(), TypingContext::default());
        table
            .ctor_templates
            .insert("Cons".to_owned(), context_cons("A"));
        table
    }

    pub fn symbol_table_list() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "List".to_owned(),
            (
                Polarity::Data,
                TypeContext::mk(&vec!["A"]),
                vec!["Nil".to_owned(), "Cons".to_owned()],
            ),
        );
        table
            .ctor_templates
            .insert("Nil".to_owned(), TypingContext::default());
        table
            .ctor_templates
            .insert("Cons".to_owned(), context_cons("A"));
        table.types.insert(
            "List[i64]".to_owned(),
            (
                Polarity::Data,
                TypeArgs::mk(vec![Ty::mk_i64()]),
                vec!["Nil".to_owned(), "Cons".to_owned()],
            ),
        );
        table
            .ctors
            .insert("Nil[i64]".to_owned(), TypingContext::default());
        table
            .ctors
            .insert("Cons[i64]".to_owned(), context_cons_i64());
        table
    }

    pub fn codata_stream() -> Codata {
        Codata {
            span: None,
            name: "Stream".to_owned(),
            type_params: TypeContext::mk(&vec!["A"]),
            dtors: vec![
                DtorSig {
                    span: None,
                    name: "head".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("A", TypeArgs::default()),
                },
                DtorSig {
                    span: None,
                    name: "tail".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl(
                        "Stream",
                        TypeArgs::mk(vec![Ty::mk_decl("A", TypeArgs::default())]),
                    ),
                },
            ],
        }
    }

    pub fn symbol_table_stream_template() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "Stream".to_owned(),
            (
                Polarity::Codata,
                TypeContext::mk(&vec!["A"]),
                vec!["head".to_owned(), "tail".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "head".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "tail".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl(
                    "Stream",
                    TypeArgs::mk(vec![Ty::mk_decl("A", TypeArgs::default())]),
                ),
            ),
        );
        table
    }

    pub fn symbol_table_stream() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "Stream".to_owned(),
            (
                Polarity::Codata,
                TypeContext::mk(&vec!["A"]),
                vec!["head".to_owned(), "tail".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "head".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "tail".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl(
                    "Stream",
                    TypeArgs::mk(vec![Ty::mk_decl("A", TypeArgs::default())]),
                ),
            ),
        );
        table.types.insert(
            "Stream[i64]".to_owned(),
            (
                Polarity::Codata,
                TypeArgs::mk(vec![Ty::mk_i64()]),
                vec!["head".to_owned(), "tail".to_owned()],
            ),
        );
        table.dtors.insert(
            "head[i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table.dtors.insert(
            "tail[i64]".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        );
        table
    }

    fn context_ap(type_param_in: &str, type_param_out: &str) -> TypingContext {
        let mut ctx_ap = TypingContext::default();
        ctx_ap.add_var("x", 0, Ty::mk_decl(type_param_in, TypeArgs::default()));
        ctx_ap.add_covar("a", 0, Ty::mk_decl(type_param_out, TypeArgs::default()));
        ctx_ap
    }

    fn context_ap_i64() -> TypingContext {
        let mut ctx_ap = TypingContext::default();
        ctx_ap.add_var("x", 0, Ty::mk_i64());
        ctx_ap.add_covar("a", 0, Ty::mk_i64());
        ctx_ap
    }

    pub fn codata_fun() -> Codata {
        Codata {
            span: None,
            name: "Fun".to_owned(),
            type_params: TypeContext::mk(&vec!["A", "B"]),
            dtors: vec![DtorSig {
                span: None,
                name: "apply".to_owned(),
                args: context_ap("A", "B"),
                cont_ty: Ty::mk_decl("B", TypeArgs::default()),
            }],
        }
    }

    pub fn symbol_table_fun_template() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "Fun".to_owned(),
            (
                Polarity::Codata,
                TypeContext::mk(&vec!["A", "B"]),
                vec!["apply".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "apply".to_owned(),
            (context_ap("A", "B"), Ty::mk_decl("B", TypeArgs::default())),
        );
        table
    }

    pub fn symbol_table_fun() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "Fun".to_owned(),
            (
                Polarity::Codata,
                TypeContext::mk(&vec!["A", "B"]),
                vec!["apply".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "apply".to_owned(),
            (context_ap("A", "B"), Ty::mk_decl("B", TypeArgs::default())),
        );
        table.types.insert(
            "Fun[i64, i64]".to_owned(),
            (
                Polarity::Codata,
                TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                vec!["apply".to_owned()],
            ),
        );
        table.dtors.insert(
            "apply[i64, i64]".to_owned(),
            (context_ap_i64(), Ty::mk_i64()),
        );
        table
    }

    pub fn codta_lpair() -> Codata {
        Codata {
            span: None,
            name: "LPair".to_owned(),
            type_params: TypeContext::mk(&vec!["A", "B"]),
            dtors: vec![
                DtorSig {
                    span: None,
                    name: "fst".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("A", TypeArgs::default()),
                },
                DtorSig {
                    span: None,
                    name: "snd".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("B", TypeArgs::default()),
                },
            ],
        }
    }

    pub fn symbol_table_lpair() -> SymbolTable {
        let mut table = SymbolTable::default();
        table.type_templates.insert(
            "LPair".to_owned(),
            (
                Polarity::Codata,
                TypeContext::mk(&vec!["A", "B"]),
                vec!["fst".to_owned(), "snd".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "fst".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "snd".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("B", TypeArgs::default()),
            ),
        );
        table.types.insert(
            "LPair[i64, i64]".to_owned(),
            (
                Polarity::Codata,
                TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                vec!["fst".to_owned(), "snd".to_owned()],
            ),
        );
        table.dtors.insert(
            "fst[i64, i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table.dtors.insert(
            "snd[i64, i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table
    }

    fn context_mult() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var(
            "l",
            0,
            Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])),
        );
        ctx
    }

    pub fn def_mult() -> Def {
        Def {
            span: dummy_span(),
            name: "mult".to_owned(),
            context: context_mult(),
            body: Case {
                span: dummy_span(),
                scrutinee: Rc::new(
                    XVar::mk(Var {
                        name: "l".to_string(),
                        id: 0,
                    })
                    .into(),
                ),
                type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
                clauses: vec![
                    Clause {
                        span: dummy_span(),
                        pol: Polarity::Data,
                        xtor: "Nil".to_owned(),
                        context_names: NameContext::default(),
                        context: TypingContext::default(),
                        body: Lit::mk(1).into(),
                    },
                    Clause {
                        span: dummy_span(),
                        pol: Polarity::Data,
                        xtor: "Cons".to_owned(),
                        context_names: context_cons_i64_names(),
                        context: TypingContext::default(),
                        body: Op {
<<<<<<< HEAD
                            span: dummy_span(),
                            fst: Rc::new(XVar::mk("x").into()),
=======
                            span: Span::default(),
                            fst: Rc::new(
                                XVar::mk(Var {
                                    name: "x".to_string(),
                                    id: 0,
                                })
                                .into(),
                            ),
>>>>>>> e4688eb (fixed fun test syntax)
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Call {
                                    span: dummy_span(),
                                    name: "mult".to_owned(),
                                    args: vec![
                                        XVar::mk(Var {
                                            name: "xs".to_string(),
                                            id: 0,
                                        })
                                        .into(),
                                    ]
                                    .into(),
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
            ret_ty: Ty::mk_i64(),
        }
    }

    pub fn def_mult_typed() -> Def {
        Def {
            span: dummy_span(),
            name: "mult".to_owned(),
            context: context_mult(),
            ret_ty: Ty::mk_i64(),
            body: Case {
                span: dummy_span(),
                scrutinee: Rc::new(
                    XVar {
                        span: dummy_span(),
                        var: "l".to_owned(),
                        var: Var {
                            name: "l".to_owned(),
                            id: 0,
                        },
                        ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                        chi: Some(Prd),
                    }
                    .into(),
                ),
                type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
                clauses: vec![
                    Clause {
                        span: dummy_span(),
                        pol: Polarity::Data,
                        xtor: "Nil".to_owned(),
                        context_names: NameContext::default(),
                        context: TypingContext::default(),
                        body: Lit::mk(1).into(),
                    },
                    Clause {
                        span: dummy_span(),
                        pol: Polarity::Data,
                        xtor: "Cons".to_owned(),
                        context_names: context_cons_i64_names(),
                        context: context_cons_i64(),
                        body: Op {
                            span: dummy_span(),
                            fst: Rc::new(
                                XVar {
<<<<<<< HEAD
                                    span: dummy_span(),
                                    var: "x".to_owned(),
=======
                                    span: Span::default(),
                                    var: Var {
                                        name: "x".to_owned(),
                                        id: 0,
                                    },
>>>>>>> e4688eb (fixed fun test syntax)
                                    ty: Some(Ty::mk_i64()),
                                    chi: Some(Prd),
                                }
                                .into(),
                            ),
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Call {
                                    span: dummy_span(),
                                    name: "mult".to_owned(),
                                    args: vec![
                                        XVar {
<<<<<<< HEAD
                                            span: dummy_span(),
                                            var: "xs".to_owned(),
=======
                                            span: Span::default(),
                                            var: Var {
                                                name: "xs".to_owned(),
                                                id: 0,
                                            },
>>>>>>> e4688eb (fixed fun test syntax)
                                            ty: Some(Ty::mk_decl(
                                                "List",
                                                TypeArgs::mk(vec![Ty::mk_i64()]),
                                            )),
                                            chi: Some(Prd),
                                        }
                                        .into(),
                                    ]
                                    .into(),
                                    ret_ty: Some(Ty::mk_i64()),
                                }
                                .into(),
                            ),
                        }
                        .into(),
                    },
                ],
                ty: Some(Ty::mk_i64()),
            }
            .into(),
        }
    }
}
