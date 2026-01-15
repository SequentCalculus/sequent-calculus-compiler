use axcut::syntax::statements::*;
use axcut::syntax::*;
use printer::Print;

use std::collections::HashSet;
use std::rc::Rc;

fn main() {
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

    let ty_cont_list = TypeDeclaration {
        name: "ContList".to_string(),
        xtors: vec![XtorSig {
            name: "Retl".to_string(),
            args: vec![ContextBinding {
                var: Var {
                    name: "kl".to_string(),
                    id: 0,
                },
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            }]
            .into(),
        }],
    };

    let ty_cont_int = TypeDeclaration {
        name: "ContInt".to_string(),
        xtors: vec![XtorSig {
            name: "Reti".to_string(),
            args: vec![ContextBinding {
                var: Var {
                    name: "ki".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
        }],
    };

    let main_body = Statement::Create(Create {
        var: Var {
            name: "t".to_string(),
            id: 0,
        },
        ty: Ty::Decl("ContInt".to_string()),
        context: None,
        clauses: vec![Clause {
            xtor: "Reti".to_string(),
            context: vec![ContextBinding {
                var: Var {
                    name: "r".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::Exit(Exit {
                var: Var {
                    name: "r".to_string(),
                    id: 0,
                },
            })),
        }],
        free_vars_clauses: None,
        next: Rc::new(Statement::Create(Create {
            var: Var {
                name: "k".to_string(),
                id: 0,
            },
            ty: Ty::Decl("ContList".to_string()),
            context: None,
            clauses: vec![Clause {
                xtor: "Retl".to_string(),
                context: vec![ContextBinding {
                    var: Var {
                        name: "as".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Prd,
                    ty: Ty::Decl("List".to_string()),
                }]
                .into(),
                body: Rc::new(Statement::Call(Call {
                    label: "sum".to_string(),
                    args: vec![
                        ContextBinding {
                            var: Var {
                                name: "t".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Cns,
                            ty: Ty::Decl("ContInt".to_string()),
                        },
                        ContextBinding {
                            var: Var {
                                name: "as".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Prd,
                            ty: Ty::Decl("List".to_string()),
                        },
                    ]
                    .into(),
                })),
            }],
            free_vars_clauses: None,
            next: Rc::new(Statement::Let(Let {
                var: Var {
                    name: "zs".to_string(),
                    id: 0,
                },
                ty: Ty::Decl("List".to_string()),
                tag: "Nil".to_string(),
                args: vec![].into(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: Var {
                        name: "n".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec![
                            ContextBinding {
                                var: Var {
                                    name: "k".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Cns,
                                ty: Ty::Decl("ContList".to_string()),
                            },
                            ContextBinding {
                                var: Var {
                                    name: "zs".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Prd,
                                ty: Ty::Decl("List".to_string()),
                            },
                            ContextBinding {
                                var: Var {
                                    name: "n".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
                        ]
                        .into(),
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
        used_vars: HashSet::from([
            Var {
                name: "t".to_string(),
                id: 0,
            },
            Var {
                name: "zs".to_string(),
                id: 0,
            },
            Var {
                name: "n".to_string(),
                id: 0,
            },
            Var {
                name: "k".to_string(),
                id: 0,
            },
            Var {
                name: "as".to_string(),
                id: 0,
            },
            Var {
                name: "r".to_string(),
                id: 0,
            },
        ]),
    };

    let range_body = Statement::IfC(IfC {
        sort: ifc::IfSort::Equal,
        fst: Var {
            name: "i".to_string(),
            id: 0,
        },
        snd: None,
        thenc: Rc::new(Statement::Invoke(Invoke {
            var: Var {
                name: "k".to_string(),
                id: 0,
            },
            tag: "Retl".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            args: vec![ContextBinding {
                var: Var {
                    name: "xs".to_string(),
                    id: 0,
                },
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            }]
            .into(),
        })),
        elsec: Rc::new(Statement::Let(Let {
            var: Var {
                name: "ys".to_string(),
                id: 0,
            },
            ty: Ty::Decl("List".to_string()),
            tag: "Cons".to_string(),
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
                        name: "i".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                },
            ]
            .into(),
            next: Rc::new(Statement::Literal(Literal {
                lit: -1,
                var: Var {
                    name: "o".to_string(),
                    id: 0,
                },
                next: Rc::new(Statement::Op(Op {
                    fst: Var {
                        name: "i".to_string(),
                        id: 0,
                    },
                    op: BinOp::Sum,
                    snd: Var {
                        name: "o".to_string(),
                        id: 0,
                    },
                    var: Var {
                        name: "j".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec![
                            ContextBinding {
                                var: Var {
                                    name: "k".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Cns,
                                ty: Ty::Decl("ContList".to_string()),
                            },
                            ContextBinding {
                                var: Var {
                                    name: "ys".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Prd,
                                ty: Ty::Decl("List".to_owned()),
                            },
                            ContextBinding {
                                var: Var {
                                    name: "j".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
                        ]
                        .into(),
                    })),
                    free_vars_next: None,
                })),
                free_vars_next: None,
            })),
            free_vars_next: None,
        })),
    });
    let range = Def {
        name: "range".to_string(),
        context: vec![
            ContextBinding {
                var: Var {
                    name: "k".to_string(),
                    id: 0,
                },
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
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
                    name: "i".to_string(),
                    id: 0,
                },
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
        ]
        .into(),
        body: range_body,
        used_vars: HashSet::from([
            Var {
                name: "k".to_string(),
                id: 0,
            },
            Var {
                name: "xs".to_string(),
                id: 0,
            },
            Var {
                name: "i".to_string(),
                id: 0,
            },
            Var {
                name: "j".to_string(),
                id: 0,
            },
            Var {
                name: "o".to_string(),
                id: 0,
            },
            Var {
                name: "ys".to_string(),
                id: 0,
            },
        ]),
    };

    let sum_body = Statement::Switch(Switch {
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
                    lit: 0,
                    var: Var {
                        name: "z".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Invoke(Invoke {
                        var: Var {
                            name: "k".to_string(),
                            id: 0,
                        },
                        tag: "Reti".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        args: vec![ContextBinding {
                            var: Var {
                                name: "z".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                    })),
                    free_vars_next: None,
                })),
            },
            Clause {
                xtor: "Cons".to_string(),
                context: vec![
                    ContextBinding {
                        var: Var {
                            name: "ys".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: Var {
                            name: "y".to_string(),
                            id: 0,
                        },
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                ]
                .into(),
                body: Rc::new(Statement::Create(Create {
                    var: Var {
                        name: "j".to_string(),
                        id: 0,
                    },
                    ty: Ty::Decl("ContInt".to_string()),
                    context: None,
                    clauses: vec![Clause {
                        xtor: "Reti".to_string(),
                        context: vec![ContextBinding {
                            var: Var {
                                name: "r".to_string(),
                                id: 0,
                            },
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                        body: Rc::new(Statement::Op(Op {
                            fst: Var {
                                name: "y".to_string(),
                                id: 0,
                            },
                            op: BinOp::Sum,
                            snd: Var {
                                name: "r".to_string(),
                                id: 0,
                            },
                            var: Var {
                                name: "s".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Invoke(Invoke {
                                var: Var {
                                    name: "k".to_string(),
                                    id: 0,
                                },
                                tag: "Reti".to_string(),
                                ty: Ty::Decl("ContInt".to_string()),
                                args: vec![ContextBinding {
                                    var: Var {
                                        name: "s".to_string(),
                                        id: 0,
                                    },
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                }]
                                .into(),
                            })),
                            free_vars_next: None,
                        })),
                    }],
                    free_vars_clauses: None,
                    next: Rc::new(Statement::Call(Call {
                        label: "sum".to_string(),
                        args: vec![
                            ContextBinding {
                                var: Var {
                                    name: "j".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Cns,
                                ty: Ty::Decl("ContInt".to_string()),
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
                    })),
                    free_vars_next: None,
                })),
            },
        ],
        free_vars_clauses: None,
    });
    let sum = Def {
        name: "sum".to_string(),
        context: vec![
            ContextBinding {
                var: Var {
                    name: "k".to_string(),
                    id: 0,
                },
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
            ContextBinding {
                var: Var {
                    name: "xs".to_string(),
                    id: 0,
                },
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            },
        ]
        .into(),
        body: sum_body,
        used_vars: HashSet::from([
            Var {
                name: "ys".to_string(),
                id: 0,
            },
            Var {
                name: "xs".to_string(),
                id: 0,
            },
            Var {
                name: "y".to_string(),
                id: 0,
            },
            Var {
                name: "j".to_string(),
                id: 0,
            },
            Var {
                name: "s".to_string(),
                id: 0,
            },
            Var {
                name: "r".to_string(),
                id: 0,
            },
            Var {
                name: "k".to_string(),
                id: 0,
            },
            Var {
                name: "z".to_string(),
                id: 0,
            },
        ]),
    };

    let program = Prog {
        defs: vec![main, range, sum],
        types: vec![ty_list, ty_cont_list, ty_cont_int],
    };

    println!("{}", program.linearize().print_to_string(None))
}
