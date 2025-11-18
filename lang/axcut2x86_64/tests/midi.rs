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
fn test_midi() {
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
                        var: "xs".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: "x".to_string(),
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
                var: "kl".to_string(),
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
                var: "ki".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
        }],
    };

    let main_body = Statement::Create(Create {
        var: "t".to_string(),
        ty: Ty::Decl("ContInt".to_string()),
        context: Some(Vec::new().into()),
        clauses: vec![Clause {
            xtor: "Reti".to_string(),
            context: vec![ContextBinding {
                var: "r".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::PrintI64(PrintI64 {
                newline: true,
                var: "r".to_string(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "ret".to_string(),
                    next: Rc::new(Statement::Exit(Exit {
                        var: "ret".to_string(),
                    })),
                    free_vars_next: None,
                })),
                free_vars_next: None,
            })),
        }],
        free_vars_clauses: None,
        next: Rc::new(Statement::Create(Create {
            var: "k".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            context: Some(
                vec![ContextBinding {
                    var: "t".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::Decl("ContInt".to_string()),
                }]
                .into(),
            ),
            clauses: vec![Clause {
                xtor: "Retl".to_string(),
                context: vec![ContextBinding {
                    var: "as".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("List".to_string()),
                }]
                .into(),
                body: Rc::new(Statement::Substitute(Substitute {
                    rearrange: vec![
                        (
                            ContextBinding {
                                var: "t".to_string(),
                                chi: Chirality::Cns,
                                ty: Ty::Decl("ContInt".to_string()),
                            },
                            "t".to_string(),
                        ),
                        (
                            ContextBinding {
                                var: "as".to_string(),
                                chi: Chirality::Prd,
                                ty: Ty::Decl("List".to_string()),
                            },
                            "as".to_string(),
                        ),
                    ],
                    next: Rc::new(Statement::Call(Call {
                        label: "sum".to_string(),
                        context: vec![].into(),
                    })),
                })),
            }],
            free_vars_clauses: None,
            next: Rc::new(Statement::Let(Let {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Nil".to_string(),
                context: vec![].into(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "n".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: "k".to_string(),
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("ContInt".to_string()),
                                },
                                "k".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "zs".to_string(),
                                    chi: Chirality::Prd,
                                    ty: Ty::Decl("List".to_string()),
                                },
                                "zs".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "n".to_string(),
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                "n".to_string(),
                            ),
                        ],
                        next: Rc::new(Statement::Call(Call {
                            label: "range".to_string(),
                            context: vec![].into(),
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

    let range_body = Statement::IfC(IfC {
        sort: ifc::IfSort::Equal,
        fst: "i".to_string(),
        snd: None,
        thenc: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                (
                    ContextBinding {
                        var: "xs".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    "xs".to_string(),
                ),
                (
                    ContextBinding {
                        var: "k".to_string(),
                        chi: Chirality::Cns,
                        ty: Ty::Decl("ContList".to_string()),
                    },
                    "k".to_string(),
                ),
            ],
            next: Rc::new(Statement::Invoke(Invoke {
                var: "k".to_string(),
                tag: "Retl".to_string(),
                ty: Ty::Decl("ContList".to_string()),
                context: vec![].into(),
            })),
        })),
        elsec: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                (
                    ContextBinding {
                        var: "n".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                    "i".to_string(),
                ),
                (
                    ContextBinding {
                        var: "k".to_string(),
                        chi: Chirality::Cns,
                        ty: Ty::Decl("ContList".to_string()),
                    },
                    "k".to_string(),
                ),
                (
                    ContextBinding {
                        var: "xs".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    "xs".to_string(),
                ),
                (
                    ContextBinding {
                        var: "i".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                    "i".to_string(),
                ),
            ],
            next: Rc::new(Statement::Let(Let {
                var: "ys".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec![
                    ContextBinding {
                        var: "xs".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: "i".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                ]
                .into(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: -1,
                    var: "o".to_string(),
                    next: Rc::new(Statement::Op(Op {
                        fst: "n".to_string(),
                        op: BinOp::Sum,
                        snd: "o".to_string(),
                        var: "j".to_string(),
                        next: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                (
                                    ContextBinding {
                                        var: "k".to_string(),
                                        chi: Chirality::Cns,
                                        ty: Ty::Decl("ContList".to_string()),
                                    },
                                    "k".to_string(),
                                ),
                                (
                                    ContextBinding {
                                        var: "ys".to_string(),
                                        chi: Chirality::Prd,
                                        ty: Ty::Decl("List".to_string()),
                                    },
                                    "ys".to_string(),
                                ),
                                (
                                    ContextBinding {
                                        var: "j".to_string(),
                                        chi: Chirality::Ext,
                                        ty: Ty::I64,
                                    },
                                    "j".to_string(),
                                ),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "range".to_string(),
                                context: vec![].into(),
                            })),
                        })),
                        free_vars_next: None,
                    })),
                    free_vars_next: None,
                })),
                free_vars_next: None,
            })),
        })),
    });
    let range = Def {
        name: "range".to_string(),
        context: vec![
            ContextBinding {
                var: "k".to_string(),
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            },
            ContextBinding {
                var: "i".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            },
        ]
        .into(),
        body: range_body,
        used_vars: HashSet::new(),
    };

    let sum_body = Statement::Switch(Switch {
        var: "xs".to_string(),
        ty: Ty::Decl("List".to_string()),
        clauses: vec![
            Clause {
                xtor: "Nil".to_string(),
                context: vec![].into(),
                body: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "z".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: "z".to_string(),
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                "z".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "k".to_string(),
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("ContInt".to_string()),
                                },
                                "k".to_string(),
                            ),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "k".to_string(),
                            tag: "Reti".to_string(),
                            ty: Ty::Decl("ContInt".to_string()),
                            context: vec![].into(),
                        })),
                    })),
                    free_vars_next: None,
                })),
            },
            Clause {
                xtor: "Cons".to_string(),
                context: vec![
                    ContextBinding {
                        var: "ys".to_string(),
                        chi: Chirality::Prd,
                        ty: Ty::Decl("List".to_string()),
                    },
                    ContextBinding {
                        var: "y".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                ]
                .into(),
                body: Rc::new(Statement::Substitute(Substitute {
                    rearrange: vec![
                        (
                            ContextBinding {
                                var: "ys".to_string(),
                                chi: Chirality::Prd,
                                ty: Ty::Decl("List".to_string()),
                            },
                            "ys".to_string(),
                        ),
                        (
                            ContextBinding {
                                var: "k".to_string(),
                                chi: Chirality::Cns,
                                ty: Ty::Decl("ContInt".to_string()),
                            },
                            "k".to_string(),
                        ),
                        (
                            ContextBinding {
                                var: "y".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
                            "y".to_string(),
                        ),
                    ],
                    next: Rc::new(Statement::Create(Create {
                        var: "j".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        context: Some(
                            vec![
                                ContextBinding {
                                    var: "k".to_string(),
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("ContInt".to_string()),
                                },
                                ContextBinding {
                                    var: "y".to_string(),
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                            ]
                            .into(),
                        ),
                        clauses: vec![Clause {
                            xtor: "Reti".to_string(),
                            context: vec![ContextBinding {
                                var: "r".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            body: Rc::new(Statement::Op(Op {
                                fst: "y".to_string(),
                                op: BinOp::Sum,
                                snd: "r".to_string(),
                                var: "s".to_string(),
                                next: Rc::new(Statement::Substitute(Substitute {
                                    rearrange: vec![
                                        (
                                            ContextBinding {
                                                var: "s".to_string(),
                                                chi: Chirality::Ext,
                                                ty: Ty::I64,
                                            },
                                            "s".to_string(),
                                        ),
                                        (
                                            ContextBinding {
                                                var: "k".to_string(),
                                                chi: Chirality::Cns,
                                                ty: Ty::Decl("ContInt".to_string()),
                                            },
                                            "k".to_string(),
                                        ),
                                    ],
                                    next: Rc::new(Statement::Invoke(Invoke {
                                        var: "k".to_string(),
                                        tag: "Reti".to_string(),
                                        ty: Ty::Decl("ContInt".to_string()),
                                        context: vec![].into(),
                                    })),
                                })),
                                free_vars_next: None,
                            })),
                        }],
                        free_vars_clauses: None,
                        next: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                (
                                    ContextBinding {
                                        var: "j".to_string(),
                                        chi: Chirality::Cns,
                                        ty: Ty::Decl("ContInt".to_string()),
                                    },
                                    "j".to_string(),
                                ),
                                (
                                    ContextBinding {
                                        var: "ys".to_string(),
                                        chi: Chirality::Prd,
                                        ty: Ty::Decl("List".to_string()),
                                    },
                                    "ys".to_string(),
                                ),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "sum".to_string(),
                                context: vec![].into(),
                            })),
                        })),
                        free_vars_next: None,
                    })),
                })),
            },
        ],
        free_vars_clauses: None,
    });
    let sum = Def {
        name: "sum".to_string(),
        context: vec![
            ContextBinding {
                var: "k".to_string(),
                chi: Chirality::Cns,
                ty: Ty::Decl("ContList".to_string()),
            },
            ContextBinding {
                var: "xs".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("List".to_string()),
            },
        ]
        .into(),
        body: sum_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main, range, sum],
        types: vec![ty_list, ty_cont_list, ty_cont_int],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
