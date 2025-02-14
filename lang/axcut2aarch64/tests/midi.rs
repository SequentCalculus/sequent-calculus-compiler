use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2aarch64::Backend;
use axcut2backend::coder::compile;
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

    let main_body = Statement::New(New {
        var: "t".to_string(),
        ty: Ty::Decl("ContInt".to_string()),
        context: Some(Vec::new()),
        clauses: vec![Clause {
            xtor: "Reti".to_string(),
            context: vec![ContextBinding {
                var: "r".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            case: Rc::new(Statement::PrintI64(PrintI64 {
                newline: true,
                var: "r".to_string(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "ret".to_string(),
                    case: Rc::new(Statement::Return(Return {
                        var: "ret".to_string(),
                    })),
                })),
            })),
        }],
        next: Rc::new(Statement::New(New {
            var: "k".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            context: Some(vec!["t".to_string()]),
            clauses: vec![Clause {
                xtor: "Retl".to_string(),
                context: vec![ContextBinding {
                    var: "as".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("List".to_string()),
                }]
                .into(),
                case: Rc::new(Statement::Substitute(Substitute {
                    rearrange: vec![
                        ("t".to_string(), "t".to_string()),
                        ("as".to_string(), "as".to_string()),
                    ],
                    next: Rc::new(Statement::Call(Call {
                        label: "sum".to_string(),
                        args: vec![],
                    })),
                })),
            }],
            next: Rc::new(Statement::Let(Let {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Nil".to_string(),
                args: vec![],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "n".to_string(),
                    case: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            ("k".to_string(), "k".to_string()),
                            ("zs".to_string(), "zs".to_string()),
                            ("n".to_string(), "n".to_string()),
                        ],
                        next: Rc::new(Statement::Call(Call {
                            label: "range".to_string(),
                            args: vec![],
                        })),
                    })),
                })),
            })),
        })),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let range_body = Statement::IfZ(IfZ {
        sort: ifz::IfZSort::Equal,
        ifc: "i".to_string(),
        thenc: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                ("xs".to_string(), "xs".to_string()),
                ("k".to_string(), "k".to_string()),
            ],
            next: Rc::new(Statement::Invoke(Invoke {
                var: "k".to_string(),
                tag: "Retl".to_string(),
                ty: Ty::Decl("ContList".to_string()),
                args: vec![],
            })),
        })),
        elsec: Rc::new(Statement::Substitute(Substitute {
            rearrange: vec![
                ("n".to_string(), "i".to_string()),
                ("k".to_string(), "k".to_string()),
                ("xs".to_string(), "xs".to_string()),
                ("i".to_string(), "i".to_string()),
            ],
            next: Rc::new(Statement::Let(Let {
                var: "ys".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec!["xs".to_string(), "i".to_string()],
                next: Rc::new(Statement::Literal(Literal {
                    lit: -1,
                    var: "o".to_string(),
                    case: Rc::new(Statement::Op(Op {
                        fst: "n".to_string(),
                        op: BinOp::Sum,
                        snd: "o".to_string(),
                        var: "j".to_string(),
                        case: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                ("k".to_string(), "k".to_string()),
                                ("ys".to_string(), "ys".to_string()),
                                ("j".to_string(), "j".to_string()),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "range".to_string(),
                                args: vec![],
                            })),
                        })),
                    })),
                })),
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
                case: Rc::new(Statement::Literal(Literal {
                    lit: 0,
                    var: "z".to_string(),
                    case: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            ("z".to_string(), "z".to_string()),
                            ("k".to_string(), "k".to_string()),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "k".to_string(),
                            tag: "Reti".to_string(),
                            ty: Ty::Decl("ContInt".to_string()),
                            args: vec![],
                        })),
                    })),
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
                case: Rc::new(Statement::Substitute(Substitute {
                    rearrange: vec![
                        ("ys".to_string(), "ys".to_string()),
                        ("k".to_string(), "k".to_string()),
                        ("y".to_string(), "y".to_string()),
                    ],
                    next: Rc::new(Statement::New(New {
                        var: "j".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        context: Some(vec!["k".to_string(), "y".to_string()]),
                        clauses: vec![Clause {
                            xtor: "Reti".to_string(),
                            context: vec![ContextBinding {
                                var: "r".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            case: Rc::new(Statement::Op(Op {
                                fst: "y".to_string(),
                                op: BinOp::Sum,
                                snd: "r".to_string(),
                                var: "s".to_string(),
                                case: Rc::new(Statement::Substitute(Substitute {
                                    rearrange: vec![
                                        ("s".to_string(), "s".to_string()),
                                        ("k".to_string(), "k".to_string()),
                                    ],
                                    next: Rc::new(Statement::Invoke(Invoke {
                                        var: "k".to_string(),
                                        tag: "Reti".to_string(),
                                        ty: Ty::Decl("ContInt".to_string()),
                                        args: vec![],
                                    })),
                                })),
                            })),
                        }],
                        next: Rc::new(Statement::Substitute(Substitute {
                            rearrange: vec![
                                ("j".to_string(), "j".to_string()),
                                ("ys".to_string(), "ys".to_string()),
                            ],
                            next: Rc::new(Statement::Call(Call {
                                label: "sum".to_string(),
                                args: vec![],
                            })),
                        })),
                    })),
                })),
            },
        ],
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
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("midi.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
