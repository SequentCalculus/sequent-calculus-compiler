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
    };

    let mut subst = Substitution::default();
    subst.add_prod(terms::XVar::var("l", Ty::Decl("ListInt".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    let fmult = Def {
        name: "fmult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "l".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_string()),
                },
                ContextBinding {
                    var: "a0".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statements::Cut::new(
            Term::Mu(terms::Mu::mu(
                "a",
                Statement::Call(statements::Call {
                    name: "mult".to_string(),
                    args: subst,
                    ty: Ty::Decl("Int".to_string()),
                }),
                Ty::I64,
            )),
            Term::XVar(terms::XVar::covar("a0", Ty::I64)),
            Ty::I64,
        )),
        used_vars: HashSet::from(["l".to_string(), "a".to_string(), "a0".to_string()]),
    };

    let mut subst = Substitution::default();
    subst.add_prod(terms::XVar::var("xs", Ty::Decl("ListInt".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(terms::XVar::covar("a1", Ty::Decl("Int".to_string())));

    let mult = Def {
        name: "mult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "l".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("ListInt".to_string()),
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: "a0".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statements::Cut::new(
            Term::XVar(terms::XVar::var("l", Ty::Decl("ListInt".to_string()))),
            Term::XCase(terms::XCase {
                prdcns: terms::Cns,
                clauses: vec![
                    Clause {
                        prdcns: terms::Cns,
                        xtor: "Nil".to_string(),
                        context: TypingContext { bindings: vec![] },
                        rhs: Rc::new(Statement::Cut(statements::Cut::new(
                            Term::Literal(terms::Literal { lit: 1 }),
                            Term::XVar(terms::XVar::covar("a0", Ty::I64)),
                            Ty::I64,
                        ))),
                    },
                    Clause {
                        prdcns: terms::Cns,
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
                                    ty: Ty::Decl("ListInt".to_string()),
                                },
                            ],
                        },
                        rhs: Rc::new(Statement::IfZ(statements::IfZ {
                            sort: statements::IfZSort::Equal,
                            ifc: Rc::new(Term::XVar(terms::XVar::var("x", Ty::I64))),
                            thenc: Rc::new(Statement::Cut(statements::Cut::new(
                                Term::Literal(terms::Literal { lit: 0 }),
                                Term::XVar(terms::XVar::covar("a", Ty::I64)),
                                Ty::I64,
                            ))),
                            elsec: Rc::new(Statement::Op(statements::Op {
                                fst: Rc::new(Term::XVar(terms::XVar::var("x", Ty::I64))),
                                op: BinOp::Prod,
                                snd: Rc::new(Term::Mu(terms::Mu::mu(
                                    "a1",
                                    Statement::Call(statements::Call {
                                        name: "mult".to_string(),
                                        args: subst,
                                        ty: Ty::I64,
                                    }),
                                    Ty::I64,
                                ))),
                                continuation: Rc::new(Term::XVar(terms::XVar::covar(
                                    "a0",
                                    Ty::I64,
                                ))),
                            })),
                        })),
                    },
                ],
                ty: Ty::Decl("ListInt".to_string()),
            }),
            Ty::Decl("ListInt".to_string()),
        )),
        used_vars: HashSet::from([
            "l".to_string(),
            "a".to_string(),
            "a0".to_string(),
            "a1".to_string(),
            "x".to_string(),
            "xs".to_string(),
        ]),
    };

    let nil = terms::Xtor::ctor(
        "Nil",
        Substitution::default(),
        Ty::Decl("ListInt".to_string()),
    );

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_prod(nil);
    let cons1 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_prod(cons1);
    let cons2 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(0));
    subst.add_prod(cons2);
    let cons3 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(2));
    subst.add_prod(cons3);
    let cons4 = terms::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(cons4);
    subst.add_cons(terms::XVar::covar("a0", Ty::I64));

    let main = Def {
        name: "main".to_string(),
        context: TypingContext {
            bindings: vec![ContextBinding {
                var: "a0".to_string(),
                chi: Chirality::Cns,
                ty: Ty::I64,
            }],
        },
        body: Statement::Call(statements::Call {
            name: "fmult".to_string(),
            args: subst,
            ty: Ty::Decl("Int".to_string()),
        }),
        used_vars: HashSet::from(["a0".to_string()]),
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
