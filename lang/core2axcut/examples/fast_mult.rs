use core_lang::syntax::term::*;
use core_lang::syntax::*;

use context::Context;
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
                args: TypingContext::empty(),
            },
            CtorSig {
                xtor: Data,
                name: "Cons".to_string(),
                args: Context {
                    bindings: vec![
                        VarBinding {
                            var: "x".to_string(),
                            ty: Ty::I64,
                        },
                        VarBinding {
                            var: "xs".to_string(),
                            ty: Ty::Decl("ListInt".to_string()),
                        },
                    ],
                },
            },
        ],
    };

    let mut subst = Substitution::default();
    subst.add_prod(term::XVar::var("l", Ty::Decl("ListInt".to_string())));
    subst.add_cons(term::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(term::XVar::covar("a", Ty::Decl("Int".to_string())));
    let fmult = Def {
        name: "fmult".to_string(),
        context: Context {
            bindings: vec![
                VarBinding {
                    var: "l".to_string(),
                    ty: Ty::Decl("ListInt".to_string()),
                },
                CovarBinding {
                    covar: "a0".to_string(),
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::Mu(term::Mu::mu(
                "a",
                Statement::Call(statement::Call {
                    name: "mult".to_string(),
                    args: subst,
                    ty: Ty::Decl("Int".to_string()),
                }),
                Ty::I64,
            )),
            Term::XVar(term::XVar::covar("a0", Ty::I64)),
            Ty::I64,
        )),
        used_vars: HashSet::from(["l".to_string(), "a".to_string(), "a0".to_string()]),
    };

    let mut subst = Substitution::default();
    subst.add_prod(term::XVar::var("xs", Ty::Decl("ListInt".to_string())));
    subst.add_cons(term::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(term::XVar::covar("a1", Ty::Decl("Int".to_string())));

    let mult = Def {
        name: "mult".to_string(),
        context: Context {
            bindings: vec![
                VarBinding {
                    var: "l".to_string(),
                    ty: Ty::Decl("ListInt".to_string()),
                },
                CovarBinding {
                    covar: "a".to_string(),
                    ty: Ty::I64,
                },
                CovarBinding {
                    covar: "a0".to_string(),
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::XVar(term::XVar::var("l", Ty::Decl("ListInt".to_string()))),
            Term::XCase(term::XCase {
                prdcns: term::Cns,
                clauses: vec![
                    Clause {
                        prdcns: term::Cns,
                        xtor: "Nil".to_string(),
                        context: Context { bindings: vec![] },
                        rhs: Rc::new(Statement::Cut(statement::Cut::new(
                            Term::Literal(term::Literal { lit: 1 }),
                            Term::XVar(term::XVar::covar("a0", Ty::I64)),
                            Ty::I64,
                        ))),
                    },
                    Clause {
                        prdcns: term::Cns,
                        xtor: "Cons".to_string(),
                        context: Context {
                            bindings: vec![
                                VarBinding {
                                    var: "x".to_string(),
                                    ty: Ty::I64,
                                },
                                VarBinding {
                                    var: "xs".to_string(),
                                    ty: Ty::Decl("ListInt".to_string()),
                                },
                            ],
                        },
                        rhs: Rc::new(Statement::IfZ(statement::IfZ {
                            sort: statement::IfZSort::Equal,
                            ifc: Rc::new(Term::XVar(term::XVar::var("x", Ty::I64))),
                            thenc: Rc::new(Statement::Cut(statement::Cut::new(
                                Term::Literal(term::Literal { lit: 0 }),
                                Term::XVar(term::XVar::covar("a", Ty::I64)),
                                Ty::I64,
                            ))),
                            elsec: Rc::new(Statement::Op(statement::Op {
                                fst: Rc::new(Term::XVar(term::XVar::var("x", Ty::I64))),
                                op: BinOp::Prod,
                                snd: Rc::new(Term::Mu(term::Mu::mu(
                                    "a1",
                                    Statement::Call(statement::Call {
                                        name: "mult".to_string(),
                                        args: subst,
                                        ty: Ty::I64,
                                    }),
                                    Ty::I64,
                                ))),
                                continuation: Rc::new(Term::XVar(term::XVar::covar("a0", Ty::I64))),
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

    let nil = term::Xtor::ctor(
        "Nil",
        Substitution::default(),
        Ty::Decl("ListInt".to_string()),
    );

    let mut subst = Substitution::default();
    subst.add_prod(term::Literal::new(3));
    subst.add_prod(nil);
    let cons1 = term::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(term::Literal::new(3));
    subst.add_prod(cons1);
    let cons2 = term::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(term::Literal::new(0));
    subst.add_prod(cons2);
    let cons3 = term::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(term::Literal::new(2));
    subst.add_prod(cons3);
    let cons4 = term::Xtor::ctor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(cons4);
    subst.add_cons(term::XVar::covar("a0", Ty::I64));

    let main = Def {
        name: "main".to_string(),
        context: Context {
            bindings: vec![CovarBinding {
                covar: "a0".to_string(),
                ty: Ty::I64,
            }],
        },
        body: Statement::Call(statement::Call {
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
