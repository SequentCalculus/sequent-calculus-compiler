use core_lang::syntax::term::*;
use core_lang::syntax::*;

use context::Context;
use printer::Print;

use std::rc::Rc;

fn main() {
    let ty_list = CodataDeclaration {
        dat: Codata,
        name: "ListInt".to_string(),
        xtors: vec![
            DtorSig {
                xtor: Codata,
                name: "Nil".to_string(),
                args: Context { bindings: vec![] },
            },
            DtorSig {
                xtor: Codata,
                name: "Cons".to_string(),
                args: Context {
                    bindings: vec![
                        VarBinding {
                            var: "x".to_string(),
                            ty: Ty::Int,
                        },
                        CovarBinding {
                            covar: "xs".to_string(),
                            ty: Ty::Decl("ListInt".to_string()),
                        },
                    ],
                },
            },
        ],
    };

    let fmult = Def {
        name: "fmult".to_string(),
        context: Context {
            bindings: vec![
                CovarBinding {
                    covar: "l".to_string(),
                    ty: Ty::Decl("ListInt".to_string()),
                },
                CovarBinding {
                    covar: "a0".to_string(),
                    ty: Ty::Int,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::Mu(term::Mu::mu(
                "a",
                Statement::Fun(statement::Fun {
                    name: "mult".to_string(),
                    args: vec![
                        ConsumerBinding(Term::XVar(term::XVar::covar(
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
                Ty::Int,
            )),
            Term::XVar(term::XVar::covar("a0", Ty::Int)),
            Ty::Int,
        )),
    };

    let mult = Def {
        name: "mult".to_string(),
        context: Context {
            bindings: vec![
                CovarBinding {
                    covar: "l".to_string(),
                    ty: Ty::Decl("ListInt".to_string()),
                },
                CovarBinding {
                    covar: "a".to_string(),
                    ty: Ty::Int,
                },
                CovarBinding {
                    covar: "a0".to_string(),
                    ty: Ty::Int,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::XCase(term::XCase {
                prdcns: term::Prd,
                clauses: vec![
                    Clause {
                        xtor: "Nil".to_string(),
                        context: Context { bindings: vec![] },
                        rhs: Rc::new(Statement::Cut(statement::Cut::new(
                            Term::Literal(term::Literal { lit: 1 }),
                            Term::XVar(term::XVar::covar("a0", Ty::Int)),
                            Ty::Int,
                        ))),
                    },
                    Clause {
                        xtor: "Cons".to_string(),
                        context: Context {
                            bindings: vec![
                                VarBinding {
                                    var: "x".to_string(),
                                    ty: Ty::Int,
                                },
                                CovarBinding {
                                    covar: "xs".to_string(),
                                    ty: Ty::Decl("ListInt".to_string()),
                                },
                            ],
                        },
                        rhs: Rc::new(Statement::IfZ(statement::IfZ {
                            ifc: Rc::new(Term::XVar(term::XVar::var("x", Ty::Int))),
                            thenc: Rc::new(Statement::Cut(statement::Cut::new(
                                Term::Literal(term::Literal { lit: 0 }),
                                Term::XVar(term::XVar::covar("a", Ty::Int)),
                                Ty::Int,
                            ))),
                            elsec: Rc::new(Statement::Op(statement::Op {
                                fst: Rc::new(Term::XVar(term::XVar::var("x", Ty::Int))),
                                op: BinOp::Prod,
                                snd: Rc::new(Term::Mu(term::Mu::mu(
                                    "a1",
                                    Statement::Fun(statement::Fun {
                                        name: "mult".to_string(),
                                        args: vec![
                                            ConsumerBinding(Term::XVar(term::XVar::covar(
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
                                        ty: Ty::Int,
                                    }),
                                    Ty::Int,
                                ))),
                                continuation: Rc::new(Term::XVar(term::XVar::covar("a0", Ty::Int))),
                            })),
                        })),
                    },
                ],
                ty: Ty::Decl("ListInt".to_string()),
            }),
            Term::XVar(term::XVar::covar("l", Ty::Decl("ListInt".to_string()))),
            Ty::Decl("ListInt".to_string()),
        )),
    };

    let main = Def {
        name: "main".to_string(),
        context: Context {
            bindings: vec![CovarBinding {
                covar: "a0".to_string(),
                ty: Ty::Int,
            }],
        },
        body: Statement::Fun(statement::Fun {
            name: "fmult".to_string(),
            args: vec![
                ConsumerBinding(Term::Xtor(term::Xtor::dtor(
                    "Cons",
                    vec![
                        ProducerBinding(Term::Literal(term::Literal::new(2))),
                        ConsumerBinding(Term::Xtor(term::Xtor::dtor(
                            "Cons",
                            vec![
                                ProducerBinding(Term::Literal(term::Literal::new(0))),
                                ConsumerBinding(Term::Xtor(term::Xtor::dtor(
                                    "Cons",
                                    vec![
                                        ProducerBinding(Term::Literal(term::Literal::new(3))),
                                        ConsumerBinding(Term::Xtor(term::Xtor::dtor(
                                            "Cons",
                                            vec![
                                                ProducerBinding(Term::Literal(term::Literal::new(
                                                    3,
                                                ))),
                                                ConsumerBinding(Term::Xtor(term::Xtor::dtor(
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
                ConsumerBinding(Term::XVar(term::XVar::covar("a0", Ty::Int))),
            ],
            ty: Ty::Decl("Int".to_string()),
        }),
    };

    let program = Prog {
        defs: vec![main, mult, fmult],
        data_types: vec![],
        codata_types: vec![ty_list],
    };

    println!("{}\n", program.print_to_string(None));
    let program = program::transform_prog(program);
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::translate_prog(program);
    println!("{}", program.print_to_string(None))
}
