//! This crate contains the [syntax], [parser] and [type checker](typing) for the surface
//! language Fun, as well as some infrastructure [traits].
pub mod parser;
pub mod syntax;
pub mod traits;
pub mod typing;

/// Some infrastructure for unit tests.
#[cfg(feature = "test-common")]
pub mod test_common {
    use super::{
        syntax::{
            context::{Chirality::Prd, NameContext, TypeContext, TypingContext},
            declarations::{Codata, CtorSig, Data, Def, DtorSig, Polarity},
            terms::{BinOp, Call, Case, Clause, Lit, Op, XVar},
            types::{Ty, TypeArgs},
        },
        typing::symbol_table::SymbolTable,
    };
    use codespan::Span;
    use std::rc::Rc;

    fn context_cons(type_param: &str) -> TypingContext {
        let mut ctx_cons = TypingContext::default();
        ctx_cons.add_var("x", Ty::mk_decl(type_param, TypeArgs::default()));
        ctx_cons.add_var(
            "xs",
            Ty::mk_decl(
                "List",
                TypeArgs::mk(vec![Ty::mk_decl(type_param, TypeArgs::default())]),
            ),
        );
        ctx_cons
    }

    fn context_cons_i64_names() -> NameContext {
        let mut ctx_cons_names = NameContext::default();
        ctx_cons_names.bindings.push("x".to_string());
        ctx_cons_names.bindings.push("xs".to_string());
        ctx_cons_names
    }

    fn context_cons_i64() -> TypingContext {
        let mut ctx_cons = TypingContext::default();
        ctx_cons.add_var("x", Ty::mk_i64());
        ctx_cons.add_var("xs", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        ctx_cons
    }

    pub fn data_list() -> Data {
        Data {
            span: Span::default(),
            name: "List".to_owned(),
            type_params: TypeContext::mk(&vec!["A"]),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext::default(),
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: context_cons("A"),
                },
            ],
        }
    }

    pub fn data_list_i64() -> Data {
        Data {
            span: Span::default(),
            name: "List[i64]".to_owned(),
            type_params: TypeContext::default(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext::default(),
                },
                CtorSig {
                    span: Span::default(),
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
            span: Span::default(),
            name: "Stream".to_owned(),
            type_params: TypeContext::mk(&vec!["A"]),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("A", TypeArgs::default()),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
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
                vec!["Hd".to_owned(), "Tl".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "Hd".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "Tl".to_owned(),
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
                vec!["Hd".to_owned(), "Tl".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "Hd".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "Tl".to_owned(),
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
                vec!["Hd".to_owned(), "Tl".to_owned()],
            ),
        );
        table.dtors.insert(
            "Hd[i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table.dtors.insert(
            "Tl[i64]".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("Stream", TypeArgs::mk(vec![Ty::mk_i64()])),
            ),
        );
        table
    }

    fn context_ap(type_param_in: &str, type_param_out: &str) -> TypingContext {
        let mut ctx_ap = TypingContext::default();
        ctx_ap.add_var("x", Ty::mk_decl(type_param_in, TypeArgs::default()));
        ctx_ap.add_covar("a", Ty::mk_decl(type_param_out, TypeArgs::default()));
        ctx_ap
    }

    fn context_ap_i64() -> TypingContext {
        let mut ctx_ap = TypingContext::default();
        ctx_ap.add_var("x", Ty::mk_i64());
        ctx_ap.add_covar("a", Ty::mk_i64());
        ctx_ap
    }

    pub fn codata_fun() -> Codata {
        Codata {
            span: Span::default(),
            name: "Fun".to_owned(),
            type_params: TypeContext::mk(&vec!["A", "B"]),
            dtors: vec![DtorSig {
                span: Span::default(),
                name: "Apply".to_owned(),
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
                vec!["Apply".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "Apply".to_owned(),
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
                vec!["Apply".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "Apply".to_owned(),
            (context_ap("A", "B"), Ty::mk_decl("B", TypeArgs::default())),
        );
        table.types.insert(
            "Fun[i64, i64]".to_owned(),
            (
                Polarity::Codata,
                TypeArgs::mk(vec![Ty::mk_i64(), Ty::mk_i64()]),
                vec!["Apply".to_owned()],
            ),
        );
        table.dtors.insert(
            "Apply[i64, i64]".to_owned(),
            (context_ap_i64(), Ty::mk_i64()),
        );
        table
    }

    pub fn codta_lpair() -> Codata {
        Codata {
            span: Span::default(),
            name: "LPair".to_owned(),
            type_params: TypeContext::mk(&vec!["A", "B"]),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Fst".to_owned(),
                    args: TypingContext::default(),
                    cont_ty: Ty::mk_decl("A", TypeArgs::default()),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Snd".to_owned(),
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
                vec!["Fst".to_owned(), "Snd".to_owned()],
            ),
        );
        table.dtor_templates.insert(
            "Fst".to_owned(),
            (
                TypingContext::default(),
                Ty::mk_decl("A", TypeArgs::default()),
            ),
        );
        table.dtor_templates.insert(
            "Snd".to_owned(),
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
                vec!["Fst".to_owned(), "Snd".to_owned()],
            ),
        );
        table.dtors.insert(
            "Fst[i64, i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table.dtors.insert(
            "Snd[i64, i64]".to_owned(),
            (TypingContext::default(), Ty::mk_i64()),
        );
        table
    }

    fn context_mult() -> TypingContext {
        let mut ctx = TypingContext::default();
        ctx.add_var("l", Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()])));
        ctx
    }

    pub fn def_mult() -> Def {
        Def {
            span: Span::default(),
            name: "mult".to_owned(),
            context: context_mult(),
            body: Case {
                span: Span::default(),
                destructee: Rc::new(XVar::mk("l").into()),
                type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
                clauses: vec![
                    Clause {
                        span: Span::default(),
                        pol: Polarity::Data,
                        xtor: "Nil".to_owned(),
                        context_names: NameContext::default(),
                        context: TypingContext::default(),
                        body: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        pol: Polarity::Data,
                        xtor: "Cons".to_owned(),
                        context_names: context_cons_i64_names(),
                        context: TypingContext::default(),
                        body: Op {
                            span: Span::default(),
                            fst: Rc::new(XVar::mk("x").into()),
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Call {
                                    span: Span::default(),
                                    name: "mult".to_owned(),
                                    args: vec![XVar::mk("xs").into()],
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
            span: Span::default(),
            name: "mult".to_owned(),
            context: context_mult(),
            ret_ty: Ty::mk_i64(),
            body: Case {
                span: Span::default(),
                destructee: Rc::new(
                    XVar {
                        span: Span::default(),
                        var: "l".to_owned(),
                        ty: Some(Ty::mk_decl("List", TypeArgs::mk(vec![Ty::mk_i64()]))),
                        chi: Some(Prd),
                    }
                    .into(),
                ),
                type_args: TypeArgs::mk(vec![Ty::mk_i64()]),
                clauses: vec![
                    Clause {
                        span: Span::default(),
                        pol: Polarity::Data,
                        xtor: "Nil".to_owned(),
                        context_names: NameContext::default(),
                        context: TypingContext::default(),
                        body: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        pol: Polarity::Data,
                        xtor: "Cons".to_owned(),
                        context_names: context_cons_i64_names(),
                        context: context_cons_i64(),
                        body: Op {
                            span: Span::default(),
                            fst: Rc::new(
                                XVar {
                                    span: Span::default(),
                                    var: "x".to_owned(),
                                    ty: Some(Ty::mk_i64()),
                                    chi: Some(Prd),
                                }
                                .into(),
                            ),
                            op: BinOp::Prod,
                            snd: Rc::new(
                                Call {
                                    span: Span::default(),
                                    name: "mult".to_owned(),
                                    args: vec![
                                        XVar {
                                            span: Span::default(),
                                            var: "xs".to_owned(),
                                            ty: Some(Ty::mk_decl(
                                                "List",
                                                TypeArgs::mk(vec![Ty::mk_i64()]),
                                            )),
                                            chi: Some(Prd),
                                        }
                                        .into(),
                                    ],
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
