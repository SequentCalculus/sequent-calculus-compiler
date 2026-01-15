use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2x86_64::Backend;
use axcut2x86_64::into_routine::into_x86_64_routine;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_closure() {
    let ty_cont = TypeDeclaration {
        name: "Cont".to_string(),
        xtors: vec![XtorSig {
            name: "Ret".to_string(),
            args: vec![ContextBinding {
                var: Var {
                    name: "r".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
        }],
    };

    let ty_func = TypeDeclaration {
        name: "Fun".to_string(),
        xtors: vec![XtorSig {
            name: "apply".to_string(),
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
                        name: "k".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Cns,
                    ty: Ty::Decl("Cont".to_string()),
                },
            ]
            .into(),
        }],
    };

    let main_body = Statement::Literal(Literal {
        lit: 9,
        var: Var {
            name: "a".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Create(Create {
            var: Var {
                name: "f".to_string(),
                id: 0,
            },
            ty: Ty::Decl("Fun".to_string()),
            context: Some(
                vec![ContextBinding {
                    var: Var {
                        name: "a".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            ),
            clauses: vec![Clause {
                xtor: "apply".to_string(),
                context: vec![
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
                            name: "k".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Cns,
                        ty: Ty::Decl("Cont".to_string()),
                    },
                ]
                .into(),
                body: Rc::new(Statement::Op(Op {
                    fst: Var {
                        name: "a".to_string(),
                        id: 0,
                    },
                    op: BinOp::Sum,
                    snd: Var {
                        name: "x".to_string(),
                        id: 0,
                    },
                    var: Var {
                        name: "b".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: Var {
                                        name: "b".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                Var {
                                    name: "b".to_string(),
                                    id: 0,
                                },
                            ),
                            (
                                ContextBinding {
                                    var: Var {
                                        name: "k".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("Cont".to_string()),
                                },
                                Var {
                                    name: "k".to_string(),
                                    id: 0,
                                },
                            ),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: Var {
                                name: "k".to_string(),
                                id: 0,
                            },
                            tag: "Ret".to_string(),
                            ty: Ty::Decl("Cont".to_string()),
                            args: vec![].into(),
                        })),
                    })),
                    free_vars_next: None,
                })),
            }],
            free_vars_clauses: None,
            next: Rc::new(Statement::Create(Create {
                var: Var {
                    name: "k".to_string(),
                    id: 0,
                },
                ty: Ty::Decl("Cont".to_string()),
                context: Some(Vec::new().into()),
                clauses: vec![Clause {
                    xtor: "Ret".to_string(),
                    context: vec![ContextBinding {
                        var: Var {
                            name: "r".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    }]
                    .into(),
                    body: Rc::new(Statement::PrintI64(PrintI64 {
                        newline: true,
                        var: Var {
                            name: "r".to_string(),
                            id: 0,
                        },
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 0,
                            var: Var {
                                name: "ret".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Exit(Exit {
                                var: Var {
                                    name: "ret".to_string(),
                                    id: 0,
                                },
                            })),
                            free_vars_next: None,
                        })),
                        free_vars_next: None,
                    })),
                }],
                free_vars_clauses: None,
                next: Rc::new(Statement::Literal(Literal {
                    lit: 1,
                    var: Var {
                        name: "y".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: Var {
                                        name: "y".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                Var {
                                    name: "y".to_string(),
                                    id: 0,
                                },
                            ),
                            (
                                ContextBinding {
                                    var: Var {
                                        name: "k".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("Cont".to_string()),
                                },
                                Var {
                                    name: "k".to_string(),
                                    id: 0,
                                },
                            ),
                            (
                                ContextBinding {
                                    var: Var {
                                        name: "f".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Prd,
                                    ty: Ty::Decl("Fun".to_string()),
                                },
                                Var {
                                    name: "f".to_string(),
                                    id: 0,
                                },
                            ),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: Var {
                                name: "f".to_string(),
                                id: 0,
                            },
                            tag: "apply".to_string(),
                            ty: Ty::Decl("Fun".to_string()),
                            args: vec![].into(),
                        })),
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
        types: vec![ty_cont, ty_func],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
