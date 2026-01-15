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
fn test_non_linear() {
    let ty_box = TypeDeclaration {
        name: "Box".to_string(),
        xtors: vec![XtorSig {
            name: "B".to_string(),
            args: vec![ContextBinding {
                var: Var {
                    name: "b".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
        }],
    };
    let ty_box_box = TypeDeclaration {
        name: "BoxBox".to_string(),
        xtors: vec![XtorSig {
            name: "BB".to_string(),
            args: vec![ContextBinding {
                var: Var {
                    name: "bb".to_string(),
                    id: 0,
                },
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }]
            .into(),
        }],
    };

    let main_body_switch_switch = Statement::Switch(Switch {
        var: Var {
            name: "a2".to_string(),
            id: 0,
        },
        ty: Ty::Decl("Box".to_string()),
        clauses: vec![Clause {
            xtor: "B".to_string(),
            context: vec![ContextBinding {
                var: Var {
                    name: "x2".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::Substitute(Substitute {
                rearrange: vec![
                    (
                        ContextBinding {
                            var: Var {
                                name: "x2".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        },
                        Var {
                            name: "x2".to_string(),
                            id: 0,
                        },
                    ),
                    (
                        ContextBinding {
                            var: Var {
                                name: "a1".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Prd,
                            ty: Ty::Decl("Box".to_string()),
                        },
                        Var {
                            name: "a1".to_string(),
                            id: 0,
                        },
                    ),
                ],
                next: Rc::new(Statement::Switch(Switch {
                    var: Var {
                        name: "a1".to_string(),
                        id: 0,
                    },
                    ty: Ty::Decl("Box".to_string()),
                    clauses: vec![Clause {
                        xtor: "B".to_string(),
                        context: vec![ContextBinding {
                            var: Var {
                                name: "x1".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                        body: Rc::new(Statement::Op(Op {
                            fst: Var {
                                name: "x1".to_string(),
                                id: 0,
                            },
                            op: BinOp::Sum,
                            snd: Var {
                                name: "x2".to_string(),
                                id: 0,
                            },
                            var: Var {
                                name: "res".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Exit(Exit {
                                var: Var {
                                    name: "res".to_string(),
                                    id: 0,
                                },
                            })),
                            free_vars_next: None,
                        })),
                    }],
                    free_vars_clauses: None,
                })),
            })),
        }],
        free_vars_clauses: None,
    });
    let main_body_switch = Statement::Switch(Switch {
        var: Var {
            name: "bb1".to_string(),
            id: 0,
        },
        ty: Ty::Decl("BoxBox".to_string()),
        clauses: vec![Clause {
            xtor: "BB".to_string(),
            context: vec![ContextBinding {
                var: Var {
                    name: "b1".to_string(),
                    id: 0,
                },
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }]
            .into(),
            body: Rc::new(Statement::Switch(Switch {
                var: Var {
                    name: "b1".to_string(),
                    id: 0,
                },
                ty: Ty::Decl("Box".to_string()),
                clauses: vec![Clause {
                    xtor: "B".to_string(),
                    context: vec![ContextBinding {
                        var: Var {
                            name: "x1".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    }]
                    .into(),
                    body: Rc::new(Statement::Let(Let {
                        var: Var {
                            name: "d1".to_string(),
                            id: 0,
                        },
                        ty: Ty::Decl("Box".to_string()),
                        tag: "B".to_string(),
                        args: vec![ContextBinding {
                            var: Var {
                                name: "x1".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                        next: Rc::new(Statement::Let(Let {
                            var: Var {
                                name: "dd1".to_string(),
                                id: 0,
                            },
                            ty: Ty::Decl("BoxBox".to_string()),
                            tag: "BB".to_string(),
                            args: vec![ContextBinding {
                                var: Var {
                                    name: "d1".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Prd,
                                ty: Ty::Decl("Box".to_string()),
                            }]
                            .into(),
                            next: Rc::new(Statement::Substitute(Substitute {
                                rearrange: vec![(
                                    ContextBinding {
                                        var: Var {
                                            name: "bb2".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Prd,
                                        ty: Ty::Decl("BoxBox".to_string()),
                                    },
                                    Var {
                                        name: "bb2".to_string(),
                                        id: 0,
                                    },
                                )],
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: 4,
                                    var: Var {
                                        name: "y".to_string(),
                                        id: 0,
                                    },
                                    next: Rc::new(Statement::Let(Let {
                                        var: Var {
                                            name: "a1".to_string(),
                                            id: 0,
                                        },
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: Var {
                                                name: "y".to_string(),
                                                id: 0,
                                            },
                                            chi: Chirality::Ext,
                                            ty: Ty::I64,
                                        }]
                                        .into(),
                                        next: Rc::new(Statement::Substitute(Substitute {
                                            rearrange: vec![
                                                (
                                                    ContextBinding {
                                                        var: Var {
                                                            name: "a1".to_string(),
                                                            id: 0,
                                                        },
                                                        chi: Chirality::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    },
                                                    Var {
                                                        name: "a1".to_string(),
                                                        id: 0,
                                                    },
                                                ),
                                                (
                                                    ContextBinding {
                                                        var: Var {
                                                            name: "bb2".to_string(),
                                                            id: 0,
                                                        },
                                                        chi: Chirality::Prd,
                                                        ty: Ty::Decl("BoxBox".to_string()),
                                                    },
                                                    Var {
                                                        name: "bb2".to_string(),
                                                        id: 0,
                                                    },
                                                ),
                                            ],
                                            next: Rc::new(Statement::Switch(Switch {
                                                var: Var {
                                                    name: "bb2".to_string(),
                                                    id: 0,
                                                },
                                                ty: Ty::Decl("BoxBox".to_string()),
                                                clauses: vec![Clause {
                                                    xtor: "BB".to_string(),
                                                    context: vec![ContextBinding {
                                                        var: Var {
                                                            name: "b2".to_string(),
                                                            id: 0,
                                                        },
                                                        chi: Chirality::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    }]
                                                    .into(),
                                                    body: Rc::new(Statement::Switch(Switch {
                                                        var: Var {
                                                            name: "b2".to_string(),
                                                            id: 0,
                                                        },
                                                        ty: Ty::Decl("Box".to_string()),
                                                        clauses: vec![Clause {
                                                            xtor: "B".to_string(),
                                                            context: vec![ContextBinding {
                                                                var: Var {
                                                                    name: "x2".to_string(),
                                                                    id: 0,
                                                                },
                                                                chi: Chirality::Ext,
                                                                ty: Ty::I64,
                                                            }]
                                                            .into(),
                                                            body: Rc::new(Statement::Let(Let {
                                                                var: Var {
                                                                    name: "a2".to_string(),
                                                                    id: 0,
                                                                },
                                                                ty: Ty::Decl("Box".to_string()),
                                                                tag: "B".to_string(),
                                                                args: vec![ContextBinding {
                                                                    var: Var {
                                                                        name: "x2".to_string(),
                                                                        id: 0,
                                                                    },
                                                                    chi: Chirality::Ext,
                                                                    ty: Ty::I64,
                                                                }]
                                                                .into(),
                                                                next: Rc::new(
                                                                    main_body_switch_switch,
                                                                ),
                                                                free_vars_next: None,
                                                            })),
                                                        }],
                                                        free_vars_clauses: None,
                                                    })),
                                                }],
                                                free_vars_clauses: None,
                                            })),
                                        })),
                                        free_vars_next: None,
                                    })),
                                    free_vars_next: None,
                                })),
                            })),
                            free_vars_next: None,
                        })),
                        free_vars_next: None,
                    })),
                }],
                free_vars_clauses: None,
            })),
        }],
        free_vars_clauses: None,
    });
    let main_body = Statement::Literal(Literal {
        lit: 3,
        var: Var {
            name: "f1".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: Var {
                name: "f2".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Literal(Literal {
                lit: 3,
                var: Var {
                    name: "f3".to_string(),
                    id: 0,
                },
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: Var {
                        name: "f4".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Literal(Literal {
                        lit: 3,
                        var: Var {
                            name: "f5".to_string(),
                            id: 0,
                        },
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 3,
                            var: Var {
                                name: "f6".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Literal(Literal {
                                lit: 3,
                                var: Var {
                                    name: "f7".to_string(),
                                    id: 0,
                                },
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: 3,
                                    var: Var {
                                        name: "x".to_string(),
                                        id: 0,
                                    },
                                    next: Rc::new(Statement::Let(Let {
                                        var: Var {
                                            name: "b".to_string(),
                                            id: 0,
                                        },
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: Var {
                                                name: "x".to_string(),
                                                id: 0,
                                            },
                                            chi: Chirality::Ext,
                                            ty: Ty::I64,
                                        }]
                                        .into(),
                                        next: Rc::new(Statement::Let(Let {
                                            var: Var {
                                                name: "bb".to_string(),
                                                id: 0,
                                            },
                                            ty: Ty::Decl("BoxBox".to_string()),
                                            tag: "BB".to_string(),
                                            args: vec![ContextBinding {
                                                var: Var {
                                                    name: "b".to_string(),
                                                    id: 0,
                                                },
                                                chi: Chirality::Prd,
                                                ty: Ty::Decl("Box".to_string()),
                                            }]
                                            .into(),
                                            next: Rc::new(Statement::Substitute(Substitute {
                                                rearrange: vec![
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f1".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f1".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f2".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f2".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f3".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f3".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f5".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f5".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f6".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f6".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f7".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f7".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "f4".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Ext,
                                                            ty: Ty::I64,
                                                        },
                                                        Var {
                                                            name: "f4".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "bb3".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        Var {
                                                            name: "bb".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "bb2".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        Var {
                                                            name: "bb".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: Var {
                                                                name: "bb1".to_string(),
                                                                id: 0,
                                                            },
                                                            chi: Chirality::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        Var {
                                                            name: "bb".to_string(),
                                                            id: 0,
                                                        },
                                                    ),
                                                ],
                                                next: Rc::new(main_body_switch),
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
        types: vec![ty_box, ty_box_box],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("nonLinear.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
