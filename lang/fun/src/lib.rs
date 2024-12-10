pub mod parser;
pub mod syntax;
pub mod typing;

#[cfg(test)]
pub mod test_common {
    use super::syntax::{
        context::{ContextBinding, TypingContext},
        declarations::{CodataDeclaration, CtorSig, DataDeclaration, Definition, DtorSig},
        substitution::SubstitutionBinding,
        terms::{BinOp, Case, Clause, Fun, Lit, Op, Var},
        types::Ty,
    };
    use codespan::Span;
    use std::rc::Rc;

    pub fn data_list() -> DataDeclaration {
        DataDeclaration {
            span: Span::default(),
            name: "ListInt".to_owned(),
            ctors: vec![
                CtorSig {
                    span: Span::default(),
                    name: "Nil".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                },
                CtorSig {
                    span: Span::default(),
                    name: "Cons".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![
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
                },
            ],
        }
    }

    pub fn codata_stream() -> CodataDeclaration {
        CodataDeclaration {
            span: Span::default(),
            name: "StreamInt".to_owned(),
            dtors: vec![
                DtorSig {
                    span: Span::default(),
                    name: "Hd".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_int(),
                },
                DtorSig {
                    span: Span::default(),
                    name: "Tl".to_owned(),
                    args: TypingContext {
                        span: Span::default(),
                        bindings: vec![],
                    },
                    cont_ty: Ty::mk_decl("StreamInt"),
                },
            ],
        }
    }

    pub fn def_mult() -> Definition {
        Definition {
            span: Span::default(),
            name: "mult".to_owned(),
            context: TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedVar {
                    var: "l".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                }],
            },
            body: Case {
                span: Span::default(),
                destructee: Rc::new(Var::mk("l").into()),
                cases: vec![
                    Clause {
                        span: Span::default(),
                        xtor: "Nil".to_owned(),
                        context: TypingContext {
                            span: Span::default(),
                            bindings: vec![],
                        },
                        rhs: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        xtor: "Cons".to_owned(),
                        context: TypingContext {
                            span: Span::default(),
                            bindings: vec![
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
            context: TypingContext {
                span: Span::default(),
                bindings: vec![ContextBinding::TypedVar {
                    var: "l".to_owned(),
                    ty: Ty::mk_decl("ListInt"),
                }],
            },
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
                        context: TypingContext {
                            span: Span::default(),
                            bindings: vec![],
                        },
                        rhs: Lit::mk(1).into(),
                    },
                    Clause {
                        span: Span::default(),
                        xtor: "Cons".to_owned(),
                        context: TypingContext {
                            span: Span::default(),
                            bindings: vec![
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
