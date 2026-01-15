use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2rv64::Backend;
use axcut2rv64::into_routine::into_rv64_routine;
use goldenfile::Mint;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_either() {
    let ty_either = TypeDeclaration {
        name: "Either".to_string(),
        xtors: vec![
            XtorSig {
                name: "Left".to_string(),
                args: vec![ContextBinding {
                    var: Var {
                        name: "x".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            },
            XtorSig {
                name: "Right".to_string(),
                args: vec![ContextBinding {
                    var: Var {
                        name: "y".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            },
        ],
    };

    let main_body = Statement::Literal(Literal {
        lit: 1,
        var: Var {
            name: "z".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: Var {
                name: "x".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Let(Let {
                var: Var {
                    name: "p".to_string(),
                    id: 0,
                },
                ty: Ty::Decl("Either".to_string()),
                tag: "Right".to_string(),
                args: vec![ContextBinding {
                    var: Var {
                        name: "x".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
                next: Rc::new(Statement::Switch(Switch {
                    var: Var {
                        name: "p".to_string(),
                        id: 0,
                    },
                    ty: Ty::Decl("Either".to_string()),
                    clauses: vec![
                        Clause {
                            xtor: "Left".to_string(),
                            context: vec![ContextBinding {
                                var: Var {
                                    name: "a".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            body: Rc::new(Statement::Literal(Literal {
                                lit: -1,
                                var: Var {
                                    name: "err".to_string(),
                                    id: 0,
                                },
                                next: Rc::new(Statement::Exit(Exit {
                                    var: Var {
                                        name: "err".to_string(),
                                        id: 0,
                                    },
                                })),
                                free_vars_next: None,
                            })),
                        },
                        Clause {
                            xtor: "Right".to_string(),
                            context: vec![ContextBinding {
                                var: Var {
                                    name: "b".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            body: Rc::new(Statement::Op(Op {
                                fst: Var {
                                    name: "b".to_string(),
                                    id: 0,
                                },
                                op: BinOp::Sum,
                                snd: Var {
                                    name: "z".to_string(),
                                    id: 0,
                                },
                                var: Var {
                                    name: "c".to_string(),
                                    id: 0,
                                },
                                next: Rc::new(Statement::Exit(Exit {
                                    var: Var {
                                        name: "c".to_string(),
                                        id: 0,
                                    },
                                })),
                                free_vars_next: None,
                            })),
                        },
                    ],
                    free_vars_clauses: None,
                })),
                free_vars_next: None,
            })),
            free_vars_next: None,
        })),
        free_vars_next: None,
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_either],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
