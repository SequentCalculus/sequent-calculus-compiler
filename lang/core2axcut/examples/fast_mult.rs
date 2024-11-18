use core::syntax::*;

use printer::Print;

use std::rc::Rc;

fn main() {
    let ty_list = DataDeclaration {
        dat: Data,
        name: "ListInt".to_string(),
        xtors: vec![
            CtorSig {
                xtor: Data,
                name: "Nil".to_string(),
                args: vec![],
            },
            CtorSig {
                xtor: Data,
                name: "Cons".to_string(),
                args: vec![
                    VarBinding {
                        var: "x".to_string(),
                        ty: Ty::Int(),
                    },
                    VarBinding {
                        var: "xs".to_string(),
                        ty: Ty::Decl("ListInt".to_string()),
                    },
                ],
            },
        ],
    };

    let fmult = Def {
        name: "fmult".to_string(),
        context: vec![
            VarBinding {
                var: "l".to_string(),
                ty: Ty::Decl("ListInt".to_string()),
            },
            CovarBinding {
                covar: "a0".to_string(),
                ty: Ty::Int(),
            },
        ],
        body: Statement::Cut(statement::Cut::new(
            Term::Mu(term::Mu::mu(
                "a",
                Statement::Fun(statement::Fun {
                    name: "mult".to_string(),
                    args: vec![
                        ProducerBinding(Term::XVar(term::XVar::var(
                            "l",
                            Ty::Decl("ListInt".to_string()),
                        ))),
                        ConsumerBinding(Term::XVar(term::XVar::covar(
                            "a",
                            Ty::Decl("Int".to_string()),
                        ))),
                        ConsumerBinding(Term::XVar(term::XVar::covar(
                            "a",
                            Ty::Decl("Int".to_string()),
                        ))),
                    ],
                    ty: Ty::Decl("Int".to_string()),
                }),
                Ty::Int(),
            )),
            Term::XVar(term::XVar::covar("a0", Ty::Int())),
            Ty::Int(),
        )),
    };

    let mult = Def {
        name: "mult".to_string(),
        context: vec![
            VarBinding {
                var: "l".to_string(),
                ty: Ty::Decl("ListInt".to_string()),
            },
            CovarBinding {
                covar: "a".to_string(),
                ty: Ty::Int(),
            },
            CovarBinding {
                covar: "a0".to_string(),
                ty: Ty::Int(),
            },
        ],
        body: Statement::Cut(statement::Cut::new(
            Term::XVar(term::XVar::var("l", Ty::Decl("ListInt".to_string()))),
            Term::XCase(term::XCase {
                prdcns: term::Cns,
                clauses: vec![
                    Clause {
                        xtor: "Nil".to_string(),
                        context: vec![],
                        rhs: Rc::new(Statement::Cut(statement::Cut::new(
                            Term::Literal(term::Literal { lit: 1 }),
                            Term::XVar(term::XVar::covar("a0", Ty::Int())),
                            Ty::Int(),
                        ))),
                    },
                    Clause {
                        xtor: "Cons".to_string(),
                        context: vec![
                            VarBinding {
                                var: "x".to_string(),
                                ty: Ty::Int(),
                            },
                            VarBinding {
                                var: "xs".to_string(),
                                ty: Ty::Decl("ListInt".to_string()),
                            },
                        ],
                        rhs: Rc::new(Statement::IfZ(statement::IfZ {
                            ifc: Rc::new(Term::XVar(term::XVar::var("x", Ty::Int()))),
                            thenc: Rc::new(Statement::Cut(statement::Cut::new(
                                Term::Literal(term::Literal { lit: 0 }),
                                Term::XVar(term::XVar::covar("a", Ty::Int())),
                                Ty::Int(),
                            ))),
                            elsec: Rc::new(Statement::Op(statement::Op {
                                fst: Rc::new(Term::XVar(term::XVar::var("x", Ty::Int()))),
                                op: BinOp::Prod,
                                snd: Rc::new(Term::Mu(term::Mu::mu(
                                    "a1",
                                    Statement::Fun(statement::Fun {
                                        name: "mult".to_string(),
                                        args: vec![
                                            ProducerBinding(Term::XVar(term::XVar::var(
                                                "xs",
                                                Ty::Decl("ListInt".to_string()),
                                            ))),
                                            ConsumerBinding(Term::XVar(term::XVar::covar(
                                                "a",
                                                Ty::Decl("Int".to_string()),
                                            ))),
                                            ConsumerBinding(Term::XVar(term::XVar::covar(
                                                "a1",
                                                Ty::Decl("Int".to_string()),
                                            ))),
                                        ],
                                        ty: Ty::Int(),
                                    }),
                                    Ty::Int(),
                                ))),
                                continuation: Rc::new(Term::XVar(term::XVar::covar(
                                    "a0",
                                    Ty::Int(),
                                ))),
                            })),
                        })),
                    },
                ],
                ty: Ty::Decl("ListInt".to_string()),
            }),
            Ty::Decl("ListInt".to_string()),
        )),
    };

    let main = Def {
        name: "main".to_string(),
        context: vec![CovarBinding {
            covar: "a0".to_string(),
            ty: Ty::Int(),
        }],
        body: Statement::Fun(statement::Fun {
            name: "fmult".to_string(),
            args: vec![
                ProducerBinding(Term::Xtor(term::Xtor::ctor(
                    "Cons",
                    vec![
                        ProducerBinding(Term::Literal(term::Literal::new(2))),
                        ProducerBinding(Term::Xtor(term::Xtor::ctor(
                            "Cons",
                            vec![
                                ProducerBinding(Term::Literal(term::Literal::new(0))),
                                ProducerBinding(Term::Xtor(term::Xtor::ctor(
                                    "Cons",
                                    vec![
                                        ProducerBinding(Term::Literal(term::Literal::new(3))),
                                        ProducerBinding(Term::Xtor(term::Xtor::ctor(
                                            "Cons",
                                            vec![
                                                ProducerBinding(Term::Literal(term::Literal::new(
                                                    3,
                                                ))),
                                                ProducerBinding(Term::Xtor(term::Xtor::ctor(
                                                    "Nil",
                                                    vec![],
                                                    Ty::Decl("ListInt".to_string()),
                                                ))),
                                            ],
                                            Ty::Decl("ListInt".to_string()),
                                        ))),
                                    ],
                                    Ty::Decl("ListInt".to_string()),
                                ))),
                            ],
                            Ty::Decl("ListInt".to_string()),
                        ))),
                    ],
                    Ty::Decl("ListInt".to_string()),
                ))),
                ConsumerBinding(Term::XVar(term::XVar::covar("a0", Ty::Int()))),
            ],
            ty: Ty::Decl("Int".to_string()),
        }),
    };

    let program = Prog {
        defs: vec![main, mult, fmult],
        data_types: vec![ty_list],
        codata_types: vec![],
    };

    println!("{}\n", program.print_to_string(None));
    let program = program::transform_prog(program);
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::translate_prog(program);
    println!("{}", program.print_to_string(None))
}
