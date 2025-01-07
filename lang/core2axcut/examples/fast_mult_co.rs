use core_lang::syntax::term::*;
use core_lang::syntax::*;

use context::Context;
use printer::Print;

use std::rc::Rc;

fn main() {
    let mut ctx = TypingContext::empty();
    ctx.add_var("x", Ty::I64);
    ctx.add_covar("xs", Ty::Decl("ListInt".to_owned()));
    let ty_list = CodataDeclaration {
        dat: Codata,
        name: "ListInt".to_string(),
        xtors: vec![
            DtorSig {
                xtor: Codata,
                name: "Nil".to_string(),
                args: TypingContext::empty(),
            },
            DtorSig {
                xtor: Codata,
                name: "Cons".to_string(),
                args: ctx,
            },
        ],
    };
    let mut subst = Substitution::default();
    subst.add_consumer(term::XVar::covar("l", Ty::Decl("ListInt".to_string())));
    subst.add_consumer(term::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_consumer(term::XVar::covar("a", Ty::Decl("Int".to_string())));
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
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::Mu(term::Mu::mu(
                "a",
                Statement::Fun(statement::Fun {
                    name: "mult".to_string(),
                    args: subst,
                    ty: Ty::Decl("Int".to_string()),
                }),
                Ty::I64,
            )),
            Term::XVar(term::XVar::covar("a0", Ty::I64)),
            Ty::I64,
        )),
    };

    let mut subst = Substitution::default();
    subst.add_consumer(term::XVar::covar("xs", Ty::Decl("ListInt".to_string())));
    subst.add_consumer(term::XVar::covar("a", Ty::Decl("Int".to_string())));
    subst.add_consumer(term::XVar::covar("a1", Ty::Decl("Int".to_string())));

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
                    ty: Ty::I64,
                },
                CovarBinding {
                    covar: "a0".to_string(),
                    ty: Ty::I64,
                },
            ],
        },
        body: Statement::Cut(statement::Cut::new(
            Term::XCase(term::XCase {
                prdcns: term::Prd,
                clauses: vec![
                    Clause {
                        prdcns: term::Prd,
                        xtor: "Nil".to_string(),
                        context: Context { bindings: vec![] },
                        rhs: Rc::new(Statement::Cut(statement::Cut::new(
                            Term::Literal(term::Literal { lit: 1 }),
                            Term::XVar(term::XVar::covar("a0", Ty::I64)),
                            Ty::I64,
                        ))),
                    },
                    Clause {
                        prdcns: term::Prd,
                        xtor: "Cons".to_string(),
                        context: Context {
                            bindings: vec![
                                VarBinding {
                                    var: "x".to_string(),
                                    ty: Ty::I64,
                                },
                                CovarBinding {
                                    covar: "xs".to_string(),
                                    ty: Ty::Decl("ListInt".to_string()),
                                },
                            ],
                        },
                        rhs: Rc::new(Statement::IfZ(statement::IfZ {
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
                                    Statement::Fun(statement::Fun {
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
            Term::XVar(term::XVar::covar("l", Ty::Decl("ListInt".to_string()))),
            Ty::Decl("ListInt".to_string()),
        )),
    };

    let nil = term::Xtor::dtor(
        "Nil",
        Substitution::default(),
        Ty::Decl("ListInt".to_string()),
    );
    let mut subst = Substitution::default();
    subst.add_producer(term::Literal::new(3));
    subst.add_consumer(nil);

    let cons1 = term::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_producer(term::Literal::new(3));
    subst.add_consumer(cons1);
    let cons2 = term::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_producer(term::Literal::new(0));
    subst.add_consumer(cons2);
    let cons3 = term::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_producer(term::Literal::new(2));
    subst.add_consumer(cons3);
    let cons4 = term::Xtor::dtor("Cons", subst, Ty::Decl("ListInt".to_string()));

    let mut subst = Substitution::default();
    subst.add_consumer(cons4);
    subst.add_consumer(term::XVar::covar("a0", Ty::I64));

    let main = Def {
        name: "main".to_string(),
        context: Context {
            bindings: vec![CovarBinding {
                covar: "a0".to_string(),
                ty: Ty::I64,
            }],
        },
        body: Statement::Fun(statement::Fun {
            name: "fmult".to_string(),
            args: subst,
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
