use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::Backend;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2backend::coder::compile;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_quad() {
    let ty_quad = TypeDeclaration {
        name: "Quad".to_string(),
        xtors: vec![XtorSig {
            name: "Q".to_string(),
            args: vec![
                ContextBinding {
                    var: Var {
                        name: "d".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: Var {
                        name: "c".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: Var {
                        name: "b".to_string(),
                        id: 0,
                    },
                    chi: Chirality::Ext,
                    ty: Ty::I64,
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
        }],
    };

    let main_body = Statement::Literal(Literal {
        lit: 8,
        var: Var {
            name: "z".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Literal(Literal {
            lit: 6,
            var: Var {
                name: "y".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Literal(Literal {
                lit: 4,
                var: Var {
                    name: "x".to_string(),
                    id: 0,
                },
                next: Rc::new(Statement::Literal(Literal {
                    lit: 2,
                    var: Var {
                        name: "w".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Let(Let {
                        var: Var {
                            name: "q".to_string(),
                            id: 0,
                        },
                        ty: Ty::Decl("Quad".to_string()),
                        tag: "Q".to_string(),
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
                                    name: "y".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
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
                                    name: "w".to_string(),
                                    id: 0,
                                },
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            },
                        ]
                        .into(),
                        next: Rc::new(Statement::Switch(Switch {
                            var: Var {
                                name: "q".to_string(),
                                id: 0,
                            },
                            ty: Ty::Decl("Quad".to_string()),
                            clauses: vec![Clause {
                                xtor: "Q".to_string(),
                                context: vec![
                                    ContextBinding {
                                        var: Var {
                                            name: "d".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Ext,
                                        ty: Ty::I64,
                                    },
                                    ContextBinding {
                                        var: Var {
                                            name: "c".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Ext,
                                        ty: Ty::I64,
                                    },
                                    ContextBinding {
                                        var: Var {
                                            name: "b".to_string(),
                                            id: 0,
                                        },
                                        chi: Chirality::Ext,
                                        ty: Ty::I64,
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
                                body: Rc::new(Statement::Literal(Literal {
                                    lit: 7,
                                    var: Var {
                                        name: "z".to_string(),
                                        id: 0,
                                    },
                                    next: Rc::new(Statement::Op(Op {
                                        fst: Var {
                                            name: "d".to_string(),
                                            id: 0,
                                        },
                                        op: BinOp::Sum,
                                        snd: Var {
                                            name: "z".to_string(),
                                            id: 0,
                                        },
                                        var: Var {
                                            name: "e".to_string(),
                                            id: 0,
                                        },
                                        next: Rc::new(Statement::PrintI64(PrintI64 {
                                            newline: true,
                                            var: Var {
                                                name: "e".to_string(),
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
                                        free_vars_next: None,
                                    })),
                                    free_vars_next: None,
                                })),
                            }],
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
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_quad],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("quad.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
