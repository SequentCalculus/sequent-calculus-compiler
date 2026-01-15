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
fn test_list() {
    let ty_list = TypeDeclaration {
        name: "List".to_string(),
        xtors: vec![
            XtorSig {
                name: "Nil".to_string(),
                args: vec![].into(),
            },
            XtorSig {
                name: "Cons".to_string(),
                args: vec![
                    ContextBinding {
                        var: Var {
                            name: "xs".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: Var {
                            name: "x".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                ]
                .into(),
            },
        ],
    };

    let main_body = Statement::Let(Let {
        var: Var {
            name: "ws".to_string(),
            id: 0,
        },
        ty: Ty::Decl("List".to_string()),
        tag: "Nil".to_string(),
        args: vec![].into(),
        next: Rc::new(Statement::Literal(Literal {
            lit: 5,
            var: Var {
                name: "z".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Let(Let {
                var: Var {
                    name: "zs".to_string(),
                    id: 0,
                },
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec![
                    ContextBinding {
                        var: Var {
                            name: "z".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                    ContextBinding {
                        var: Var {
                            name: "ws".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                ]
                .into(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 7,
                    var: Var {
                        name: "y".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Let(Let {
                        var: Var {
                            name: "ys".to_string(),
                            id: 0,
                        },
                        ty: Ty::Decl("List".to_string()),
                        tag: "Cons".to_string(),
                        args: vec![
                            ContextBinding {
                                var: Var {
                                    name: "y".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
                            ContextBinding {
                                var: Var {
                                    name: "zs".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Prd,
                                ty: Ty::Decl("List".to_string()),
                            },
                        ]
                        .into(),
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 9,
                            var: Var {
                                name: "x".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Let(Let {
                                var: Var {
                                    name: "xs".to_string(),
                                    id: 0,
                                },
                                ty: Ty::Decl("List".to_string()),
                                tag: "Cons".to_string(),
                                args: vec![
                                    ContextBinding {
                                        var: Var {
                                            name: "x".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Ext,
                                        ty: Ty::I64,
                                    },
                                    ContextBinding {
                                        var: Var {
                                            name: "ys".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Prd,
                                        ty: Ty::Decl("List".to_string()),
                                    },
                                ]
                                .into(),
                                next: Rc::new(Statement::Switch(Switch {
                                    var: Var {
                                        name: "xs".to_string(),
                                        id: 0,
                                    },
                                    ty: Ty::Decl("List".to_string()),
                                    clauses: vec![
                                        Clause {
                                            xtor: "Nil".to_string(),
                                            context: vec![].into(),
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
                                            xtor: "Cons".to_string(),
                                            context: vec![
                                                ContextBinding {
                                                    var: Var {
                                                        name: "as".to_string(),
                                                        id: 0,
                                                    },
                                                    chi: Chirality::Prd,
                                                    ty: Ty::Decl("List".to_string()),
                                                },
                                                ContextBinding {
                                                    var: Var {
                                                        name: "a".to_string(),
                                                        id: 0,
                                                    },
                                                    chi: Chirality::Ext,
                                                    ty: Ty::I64,
                                                },
                                            ]
                                            .into(),
                                            body: Rc::new(Statement::Exit(Exit {
                                                var: Var {
                                                    name: "a".to_string(),
                                                    id: 0,
                                                },
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
                    })),
                    free_vars_next: None,
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
        types: vec![ty_list],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("list.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
