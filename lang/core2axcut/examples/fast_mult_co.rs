use core_lang::syntax::terms::*;
use core_lang::syntax::*;

use context::TypingContext;
use printer::Print;

use std::{collections::HashSet, rc::Rc};

fn main() {
    let mut ctx = TypingContext::default();
    ctx.add_var("x", Ty::I64);
    ctx.add_covar("xs", Ty::Decl("ListInt".to_owned()));
    let ty_list = CodataDeclaration {
        dat: Codata,
        name: "ListInt".to_string(),
        xtors: vec![
            DtorSig {
                xtor: Codata,
                name: "Nil".to_string(),
                args: TypingContext::default(),
            },
            DtorSig {
                xtor: Codata,
                name: "Cons".to_string(),
                args: ctx,
            },
        ],
    };
    let mut subst = Substitution::default();
    subst.add_cons(terms::XVar::covar("l", Ty::Decl("ListInt".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    let fmult = Def {
        name: "fmult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "l".to_string(),
                    chi: Chirality::Cns,
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
    subst.add_cons(terms::XVar::covar("xs", Ty::Decl("ListInt".to_string())));
    subst.add_cons(terms::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_cons(terms::XVar::covar("a1", Ty::Decl("Int".to_string())));

    let mult = Def {
        name: "mult".to_string(),
        context: TypingContext {
            bindings: vec![
                ContextBinding {
                    var: "l".to_string(),
                    chi: Chirality::Cns,
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
            Term::XCase(terms::XCase {
                prdcns: terms::Prd,
                clauses: vec![
                    Clause {
                        prdcns: terms::Prd,
                        xtor: "Nil".to_string(),
                        context: TypingContext { bindings: vec![] },
                        rhs: Rc::new(Statement::Cut(statements::Cut::new(
                            Term::Literal(terms::Literal { lit: 1 }),
                            Term::XVar(terms::XVar::covar("a0", Ty::I64)),
                            Ty::I64,
                        ))),
                    },
                    Clause {
                        prdcns: terms::Prd,
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
                                    chi: Chirality::Cns,
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
            Term::XVar(terms::XVar::covar("l", Ty::Decl("ListInt".to_string()))),
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

    let nil = terms::Xtor::dtor(
        "Nil",
        Substitution::default(),
        Ty::Decl("ListInt".to_string()),
    );
    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_cons(nil);

    let cons1 = terms::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(3));
    subst.add_cons(cons1);
    let cons2 = terms::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(0));
    subst.add_cons(cons2);
    let cons3 = terms::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_prod(terms::Literal::new(2));
    subst.add_cons(cons3);
    let cons4 = terms::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_cons(cons4);
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
        data_types: vec![],
        codata_types: vec![ty_list],
    };

    println!("{}\n", program.print_to_string(None));
    let program = program.focus();
    println!("{}\n", program.print_to_string(None));
    let program = core2axcut::program::translate_prog(program);
    println!("{}", program.print_to_string(None))
}
