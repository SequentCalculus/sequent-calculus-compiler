use core_lang::syntax::{
    Arguments, Cns, Prd,
    context::{Chirality, ContextBinding, TypingContext},
    declaration::{Codata, Data, TypeDeclaration, XtorSig},
    def::Def,
    program::Prog,
    statements::{
        Call, Cut, Exit, Statement,
        ifc::{IfC, IfSort},
    },
    terms::{Clause, Literal, Mu, Op, Term, XCase, XVar, Xtor, op::BinOp},
    types::Ty,
};
use macros::{
    bind, call, case, clause, cocase, codata, covar, ctor, ctor_sig, cut, data, def, dtor,
    dtor_sig, exit, ife, mu, mutilde, prog, sum, ty, var,
};
use std::{collections::HashSet, rc::Rc};

#[test]
fn cut_macro() {
    let cut1 = cut!(var!("x"), covar!("a"));
    let cut2 = Cut::new(XVar::var("x", Ty::I64), XVar::covar("a", Ty::I64), Ty::I64);
    assert_eq!(cut1, cut2)
}

#[test]
fn list_int() {
    let list1 = ctor!(
        "Cons",
        [var!("x"), ctor!("Nil", [], ty!("ListInt"))],
        ty!("ListInt")
    );

    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    arguments.add_prod(Xtor::ctor(
        "Nil",
        Arguments::default(),
        Ty::Decl("ListInt".to_string()),
    ));
    let list2 = Xtor::ctor("Cons", arguments, Ty::Decl("ListInt".to_string()));
    assert_eq!(list1, list2)
}

#[test]
fn fun_int() {
    let fun1 = dtor!("apply", [var!("x")], ty!("FunI64I64"));
    let mut arguments = Arguments::default();
    arguments.add_prod(XVar::var("x", Ty::I64));
    let fun2 = Xtor::dtor("apply", arguments, Ty::Decl("FunI64I64".to_string()));
    assert_eq!(fun1, fun2)
}

#[test]
fn if_zero() {
    let if1 = ife!(
        var!("x"),
        var!("y"),
        cut!(var!("a"), covar!("b")),
        cut!(var!("c"), covar!("d"))
    );
    let if2 = IfC {
        sort: IfSort::Equal,
        fst: Rc::new(Term::from(XVar::var("x", ty!("int")))),
        snd: Some(Rc::new(Term::from(XVar::var("y", ty!("int"))))),
        thenc: Rc::new(
            Cut::new(
                XVar::var("a", ty!("int")),
                XVar::covar("b", ty!("int")),
                ty!("int"),
            )
            .into(),
        ),
        elsec: Rc::new(
            Cut::new(
                XVar::var("c", ty!("int")),
                XVar::covar("d", ty!("int")),
                ty!("int"),
            )
            .into(),
        ),
    };
    assert_eq!(if1, if2)
}

#[test]
fn call_print() {
    let call1 = call!("print", [var!("x")],);
    let call2 = Call {
        name: "print".to_string(),
        args: Arguments {
            entries: vec![Term::from(XVar::var("x", Ty::I64)).into()],
        },
        ty: Ty::I64,
    };
    assert_eq!(call1, call2);
}

#[test]
fn example_prog() {
    let prog1 = prog!(
        [
            def!(
                "map",
                [
                    bind!("l", Chirality::Prd, ty!("ListInt")),
                    bind!("f", Chirality::Prd, ty!("FunIntInt")),
                    bind!("a", Chirality::Cns),
                ],
                cut!(
                    var!("l", ty!("ListInt")),
                    case!(
                        [
                            clause!(
                                Cns,
                                "Nil",
                                [],
                                cut!(
                                    ctor!("Nil", [], ty!("ListInt")),
                                    covar!("a"),
                                    ty!("ListInt")
                                ),
                            ),
                            clause!(
                                Cns,
                                "Cons",
                                [bind!("x", Chirality::Prd), bind!("xs", Chirality::Prd),],
                                cut!(
                                    ctor!(
                                        "Cons",
                                        [
                                            mu!(
                                                "a1",
                                                cut!(
                                                    var!("f", ty!("FunIntInt")),
                                                    dtor!(
                                                        "apply",
                                                        [var!("x"), covar!("a1"),],
                                                        ty!("FunIntInt")
                                                    ),
                                                    ty!("FunIntInt")
                                                ),
                                            ),
                                            mu!(
                                                "a2",
                                                call!(
                                                    "map",
                                                    [
                                                        var!("f", ty!("FunIntInt")),
                                                        var!("xs", ty!("ListInt")),
                                                        covar!("a2"),
                                                    ],
                                                    ty!("ListInt"),
                                                ),
                                                ty!("ListInt"),
                                            ),
                                        ],
                                        ty!("ListInt"),
                                    ),
                                    covar!("a"),
                                    ty!("ListInt")
                                ),
                            ),
                        ],
                        ty!("ListInt"),
                    ),
                    ty!("ListInt")
                ),
                ["l", "f"],
            ),
            def!(
                "main",
                [],
                cut!(
                    ctor!(
                        "Cons",
                        [Literal::new(1), ctor!("Nil", [], ty!("ListInt")),],
                        ty!("ListInt")
                    ),
                    mutilde!(
                        "x",
                        call!(
                            "map",
                            [
                                var!("x", ty!("ListInt")),
                                cocase!(
                                    [clause!(
                                        Prd,
                                        "apply",
                                        [bind!("x", Chirality::Prd), bind!("a", Chirality::Cns),],
                                        cut!(sum!(var!("x"), Literal::new(1)), covar!("a"),),
                                    )],
                                    ty!("FunIntInt"),
                                ),
                                mutilde!("y", exit!(Literal::new(0))),
                            ],
                            ty!("ListInt")
                        ),
                        ty!("ListInt")
                    ),
                    ty!("ListInt"),
                ),
                []
            ),
        ],
        [data!(
            "ListInt",
            [
                ctor_sig!("Nil", []),
                ctor_sig!(
                    "Cons",
                    [
                        bind!("x", Chirality::Prd),
                        bind!("xs", Chirality::Prd, ty!("ListInt")),
                    ],
                )
            ]
        ),],
        [codata!(
            "FunIntInt",
            [dtor_sig!(
                "apply",
                [bind!("x", Chirality::Prd), bind!("a", Chirality::Cns)]
            )],
        )],
    );
    let prog2 = Prog {
        defs: vec![
            Def {
                name: "map".to_string(),
                context: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: "l".to_string(),
                            chi: Chirality::Prd,
                            ty: Ty::Decl("ListInt".to_string()),
                        },
                        ContextBinding {
                            var: "f".to_string(),
                            chi: Chirality::Prd,
                            ty: Ty::Decl("FunIntInt".to_string()),
                        },
                        ContextBinding {
                            var: "a".to_string(),
                            chi: Chirality::Cns,
                            ty: Ty::I64,
                        },
                    ],
                },
                body: Statement::from(Cut {
                    producer: Rc::new(XVar::var("l", Ty::Decl("ListInt".to_string())).into()),
                    consumer: Rc::new(Term::from(XCase {
                        prdcns: Cns,
                        clauses: vec![
                            Clause {
                                prdcns: Cns,
                                xtor: "Nil".to_string(),
                                context: TypingContext { bindings: vec![] },
                                body: Rc::new(Statement::from(Cut {
                                    producer: Rc::new(Term::from(Xtor {
                                        prdcns: Prd,
                                        id: "Nil".to_string(),
                                        args: Arguments { entries: vec![] },
                                        ty: Ty::Decl("ListInt".to_string()),
                                    })),
                                    consumer: Rc::new(Term::from(XVar::covar("a", Ty::I64))),
                                    ty: Ty::Decl("ListInt".to_string()),
                                })),
                            },
                            Clause {
                                prdcns: Cns,
                                xtor: "Cons".to_string(),
                                context: TypingContext {
                                    bindings: vec![
                                        ContextBinding {
                                            var: "x".to_string(),
                                            chi: Chirality::Prd,
                                            ty: Ty::I64,
                                        },
                                        ContextBinding {
                                            var: "xs".to_string(),
                                            chi: Chirality::Prd,
                                            ty: Ty::I64,
                                        },
                                    ],
                                },
                                body: Rc::new(Statement::from(Cut {
                                    producer: Rc::new(Term::from(Xtor {
                                        prdcns: Prd,
                                        id: "Cons".to_string(),
                                        args: Arguments {
                                            entries: vec![
                                                Term::from(Mu {
                                                    prdcns: Prd,
                                                    variable: "a1".to_string(),
                                                    statement: Rc::new(Statement::from(Cut {
                                                        producer: Rc::new(Term::from(XVar::var(
                                                            "f",
                                                            Ty::Decl("FunIntInt".to_string()),
                                                        ))),
                                                        consumer: Rc::new(Term::from(Xtor {
                                                            prdcns: Cns,
                                                            id: "apply".to_string(),
                                                            args: Arguments {
                                                                entries: vec![
                                                                    Term::from(XVar::var(
                                                                        "x",
                                                                        Ty::I64,
                                                                    ))
                                                                    .into(),
                                                                    Term::from(XVar::covar(
                                                                        "a1",
                                                                        Ty::I64,
                                                                    ))
                                                                    .into(),
                                                                ],
                                                            },
                                                            ty: Ty::Decl("FunIntInt".to_string()),
                                                        })),
                                                        ty: Ty::Decl("FunIntInt".to_string()),
                                                    })),
                                                    ty: Ty::I64,
                                                })
                                                .into(),
                                                Term::from(Mu {
                                                    prdcns: Prd,
                                                    variable: "a2".to_string(),
                                                    statement: Rc::new(Statement::from(Call {
                                                        name: "map".to_string(),
                                                        args: Arguments {
                                                            entries: vec![
                                                                Term::from(XVar::var(
                                                                    "f",
                                                                    Ty::Decl(
                                                                        "FunIntInt".to_string(),
                                                                    ),
                                                                ))
                                                                .into(),
                                                                Term::from(XVar::var(
                                                                    "xs",
                                                                    Ty::Decl("ListInt".to_string()),
                                                                ))
                                                                .into(),
                                                                Term::from(XVar::covar(
                                                                    "a2",
                                                                    Ty::I64,
                                                                ))
                                                                .into(),
                                                            ],
                                                        },
                                                        ty: Ty::Decl("ListInt".to_string()),
                                                    })),
                                                    ty: Ty::Decl("ListInt".to_string()),
                                                })
                                                .into(),
                                            ],
                                        },
                                        ty: Ty::Decl("ListInt".to_string()),
                                    })),
                                    consumer: Rc::new(Term::from(XVar::covar("a", Ty::I64))),
                                    ty: Ty::Decl("ListInt".to_string()),
                                })),
                            },
                        ],
                        ty: Ty::Decl("ListInt".to_string()),
                    })),
                    ty: Ty::Decl("ListInt".to_string()),
                }),
                used_vars: HashSet::from(["l".to_string(), "f".to_string()]),
            },
            Def {
                name: "main".to_string(),
                context: TypingContext { bindings: vec![] },
                body: Statement::from(Cut {
                    producer: Rc::new(Term::from(Xtor {
                        prdcns: Prd,
                        id: "Cons".to_string(),
                        args: Arguments {
                            entries: vec![
                                Term::from(Literal::new(1)).into(),
                                Term::from(Xtor {
                                    prdcns: Prd,
                                    id: "Nil".to_string(),
                                    args: Arguments { entries: vec![] },
                                    ty: Ty::Decl("ListInt".to_string()),
                                })
                                .into(),
                            ],
                        },
                        ty: Ty::Decl("ListInt".to_string()),
                    })),
                    consumer: Rc::new(Term::from(Mu {
                        prdcns: Cns,
                        variable: "x".to_string(),
                        statement: Rc::new(Statement::from(Call {
                            name: "map".to_string(),
                            args: Arguments {
                                entries: vec![
                                    Term::from(XVar::var("x", Ty::Decl("ListInt".to_string())))
                                        .into(),
                                    Term::from(XCase {
                                        prdcns: Prd,
                                        clauses: vec![Clause {
                                            prdcns: Prd,
                                            xtor: "apply".to_string(),
                                            context: TypingContext {
                                                bindings: vec![
                                                    ContextBinding {
                                                        var: "x".to_string(),
                                                        chi: Chirality::Prd,
                                                        ty: Ty::I64,
                                                    },
                                                    ContextBinding {
                                                        var: "a".to_string(),
                                                        chi: Chirality::Cns,
                                                        ty: Ty::I64,
                                                    },
                                                ],
                                            },
                                            body: Rc::new(Statement::from(Cut {
                                                producer: Rc::new(Term::from(Op {
                                                    fst: Rc::new(Term::from(XVar::var(
                                                        "x",
                                                        Ty::I64,
                                                    ))),
                                                    op: BinOp::Sum,
                                                    snd: Rc::new(Term::from(Literal::new(1))),
                                                })),
                                                consumer: Rc::new(Term::from(XVar::covar(
                                                    "a",
                                                    Ty::I64,
                                                ))),
                                                ty: Ty::I64,
                                            })),
                                        }],
                                        ty: Ty::Decl("FunIntInt".to_string()),
                                    })
                                    .into(),
                                    Term::from(Mu {
                                        prdcns: Cns,
                                        variable: "y".to_string(),
                                        statement: Rc::new(Statement::from(Exit::exit(
                                            Literal::new(0),
                                            Ty::I64,
                                        ))),
                                        ty: Ty::I64,
                                    })
                                    .into(),
                                ],
                            },
                            ty: Ty::Decl("ListInt".to_string()),
                        })),
                        ty: Ty::Decl("ListInt".to_string()),
                    })),
                    ty: Ty::Decl("ListInt".to_string()),
                }),
                used_vars: HashSet::new(),
            },
        ],
        data_types: vec![TypeDeclaration {
            dat: Data,
            name: "ListInt".to_string(),
            xtors: vec![
                XtorSig {
                    xtor: Data,
                    name: "Nil".to_string(),
                    args: TypingContext { bindings: vec![] },
                },
                XtorSig {
                    xtor: Data,
                    name: "Cons".to_string(),
                    args: TypingContext {
                        bindings: vec![
                            ContextBinding {
                                var: "x".to_string(),
                                chi: Chirality::Prd,
                                ty: Ty::I64,
                            },
                            ContextBinding {
                                var: "xs".to_string(),
                                chi: Chirality::Prd,
                                ty: Ty::Decl("ListInt".to_string()),
                            },
                        ],
                    },
                },
            ],
        }],
        codata_types: vec![TypeDeclaration {
            dat: Codata,
            name: "FunIntInt".to_string(),
            xtors: vec![XtorSig {
                xtor: Codata,
                name: "apply".to_string(),
                args: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: "x".to_string(),
                            chi: Chirality::Prd,
                            ty: Ty::I64,
                        },
                        ContextBinding {
                            var: "a".to_string(),
                            chi: Chirality::Cns,
                            ty: Ty::I64,
                        },
                    ],
                },
            }],
        }],
    };
    assert_eq!(prog1, prog2);
}
