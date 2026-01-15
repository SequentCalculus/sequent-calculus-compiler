use core_lang::syntax::terms::*;
use core_lang::syntax::*;

use context::TypingContext;
use printer::Print;

use std::{collections::HashSet, rc::Rc};

fn main() {
    let ty_list = DataDeclaration {
        dat: Data,
        name: "ListInt".to_string(),
        xtors: vec![
            CtorSig {
                xtor: Data,
                name: "Nil".to_string(),
                args: TypingContext::default(),
            },
            CtorSig {
                xtor: Data,
                name: "Cons".to_string(),
                args: TypingContext {
                    bindings: vec![
                        ContextBinding {
                            var: Var {
                                name: "x".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Prd,
                            ty: Ty::I64,
                        },
                        ContextBinding {
                            var: Var {
                                name: "xs".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Prd,
                            ty: Ty::Decl("ListInt".to_string()),
                        },
                    ],
                },
            },
        ],
    };

    let mut subst = Arguments::default();
    subst.add_prod(terms::XVar::var(
        Var {
            name: "l".to_string(),
            id: 0,
        },
        Ty::Decl("ListInt".to_string()),
    ));
    subst.add_cons(terms::XVar::covar(
        Var {
            name: "a".to_string(),
            id: 0,
        },
        Ty::Decl("Int".to_string()),
    ));
    subst.add_cons(terms::XVar::covar(
        Var {
            name: "a".to_string(),
            id: 0,
        },
        Ty::Decl("Int".to_string()),
    ));
    let fmult = Def {
        name: "fmult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: Var {
                        name: "l".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_string()),
                },
                ContextBinding {
                    var: Var {
                        name: "a0".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statements::Cut::new(
            Term::Mu(terms::Mu::mu(
                Var {
                    name: "a".to_string(),
                    id: 0,
                },
                Statement::Call(statements::Call {
                    name: "mult".to_string(),
                    args: subst,
                    ty: Ty::Decl("Int".to_string()),
                }),
                Ty::I64,
            )),
            Term::XVar(terms::XVar::covar(
                Var {
                    name: "a0".to_string(),
                    id: 0,
                },
                Ty::I64,
            )),
            Ty::I64,
        )),
        used_vars: HashSet::from([
            Var {
                name: "l".to_string(),
                id: 0,
            },
            Var {
                name: "a".to_string(),
                id: 0,
            },
            Var {
                name: "a0".to_string(),
                id: 0,
            },
        ]),
    };

    let mut subst = Arguments::default();
    subst.add_prod(terms::XVar::var(
        Var {
            name: "xs".to_string(),
            id: 0,
        },
        Ty::Decl("ListInt".to_string()),
    ));
    subst.add_cons(terms::XVar::covar(
        Var {
            name: "a".to_string(),
            id: 0,
        },
        Ty::Decl("Int".to_string()),
    ));
    subst.add_cons(terms::XVar::covar(
        Var {
            name: "a1".to_string(),
            id: 0,
        },
        Ty::Decl("Int".to_string()),
    ));

    let mult = Def {
        name: "mult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: Var {
                        name: "l".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_string()),
                },
                ContextBinding {
                    var: Var {
                        name: "a".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: Var {
                        name: "a0".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statements::Cut::new(
            Term::XVar(terms::XVar::var(
                Var {
                    name: "l".to_string(),
                    id: 0,
                },
                Ty::Decl("ListInt".to_string()),
            )),
            Term::XCase(terms::XCase {
                prdcns: terms::Cns,
                clauses: vec![
                    Clause {
                        prdcns: terms::Cns,
                        xtor: "Nil".to_string(),
                        context: TypingContext { bindings: vec![] },
                        body: Rc::new(Statement::Cut(statements::Cut::new(
                            Term::Literal(terms::Literal { lit: 1 }),
                            Term::XVar(terms::XVar::covar(
                                Var {
                                    name: "a0".to_string(),
                                    id: 0,
                                },
                                Ty::I64,
                            )),
                            Ty::I64,
                        ))),
                    },
                    Clause {
                        prdcns: terms::Cns,
                        xtor: "Cons".to_string(),
                        context: TypingContext {
                            bindings: vec![
                                ContextBinding {
                                    var: Var {
                                        name: "x".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Prd,
                                    ty: Ty::I64,
                                },
                                ContextBinding {
                                    var: Var {
                                        name: "xs".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Prd,
                                    ty: Ty::Decl("ListInt".to_string()),
                                },
                            ],
                        },
                        body: Rc::new(Statement::IfC(statements::IfC {
                            sort: statements::IfSort::Equal,
                            fst: Rc::new(Term::XVar(terms::XVar::var(
                                Var {
                                    name: "x".to_string(),
                                    id: 0,
                                },
                                Ty::I64,
                            ))),
                            snd: None,
                            thenc: Rc::new(Statement::Cut(statements::Cut::new(
                                Term::Literal(terms::Literal { lit: 0 }),
                                Term::XVar(terms::XVar::covar(
                                    Var {
                                        name: "a".to_string(),
                                        id: 0,
                                    },
                                    Ty::I64,
                                )),
                                Ty::I64,
                            ))),
                            elsec: Rc::new(Statement::Cut(statements::Cut::new(
                                Term::Op(terms::Op {
                                    fst: Rc::new(Term::XVar(terms::XVar::var(
                                        Var {
                                            name: "x".to_string(),
                                            id: 0,
                                        },
                                        Ty::I64,
                                    ))),
                                    op: BinOp::Prod,
                                    snd: Rc::new(Term::Mu(terms::Mu::mu(
                                        Var {
                                            name: "a1".to_string(),
                                            id: 0,
                                        },
                                        Statement::Call(statements::Call {
                                            name: "mult".to_string(),
                                            args: subst,
                                            ty: Ty::I64,
                                        }),
                                        Ty::I64,
                                    ))),
                                }),
                                Term::XVar(terms::XVar::covar(
                                    Var {
                                        name: "a0".to_string(),
                                        id: 0,
                                    },
                                    Ty::I64,
                                )),
                                Ty::I64,
                            ))),
                        })),
                    },
                ],
                ty: Ty::Decl("ListInt".to_string()),
            }),
            Ty::Decl("ListInt".to_string()),
        )),
        used_vars: HashSet::from([
            Var {
                name: "l".to_string(),
                id: 0,
            },
            Var {
                name: "a".to_string(),
                id: 0,
            },
            Var {
                name: "a0".to_string(),
                id: 0,
            },
            Var {
                name: "a1".to_string(),
                id: 0,
            },
            Var {
                name: "x".to_string(),
                id: 0,
            },
            Var {
                name: "xs".to_string(),
                id: 0,
            },
        ]),
    };

    let nil = terms::Xtor::ctor("Nil", Arguments::default(), Ty::Decl("ListInt".to_string()));

    let mut subst = Arguments::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_prod(nil);
    let cons1 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Arguments::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_prod(cons1);
    let cons2 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Arguments::default();
    subst.add_prod(terms::Literal::new(0));
    subst.add_prod(cons2);
    let cons3 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Arguments::default();
    subst.add_prod(terms::Literal::new(2));
    subst.add_prod(cons3);
    let cons4 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Arguments::default();
    subst.add_prod(cons4);
    subst.add_cons(terms::XVar::covar(
        Var {
            name: "a0".to_string(),
            id: 0,
        },
        Ty::I64,
    ));

    let main = Def {
        name: "main".to_string(),
        context: TypingContext {
            bindings: vec![ContextBinding {
                var: Var {
                    name: "a0".to_string(),
                    id: 0,
                },
                chi: Chirality::Cns,
                ty: Ty::I64,
            }],
        },
        body: Statement::Call(statements::Call {
            name: "fmult".to_string(),
            args: subst,
            ty: Ty::Decl("Int".to_string()),
        }),
        used_vars: HashSet::from([Var {
            name: "a0".to_string(),
            id: 0,
        }]),
    };

    let program = Prog {
        defs: vec![main, mult, fmult],
        data_types: vec![ty_list],
        codata_types: vec![],
    };

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::shrink_prog(program);
    println!("{}", program.print_to_string(None))
}
