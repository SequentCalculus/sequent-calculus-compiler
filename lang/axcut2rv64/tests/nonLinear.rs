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
                var: "b".to_string(),
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
                var: "bb".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }]
            .into(),
        }],
    };

    let main_body_switch_switch = Statement::Switch(Switch {
        var: "a2".to_string(),
        ty: Ty::Decl("Box".to_string()),
        clauses: vec![Clause {
            xtor: "B".to_string(),
            context: vec![ContextBinding {
                var: "x2".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::Substitute(Substitute {
                rearrange: vec![
                    ("x2".to_string(), "x2".to_string()),
                    ("a1".to_string(), "a1".to_string()),
                ],
                next: Rc::new(Statement::Switch(Switch {
                    var: "a1".to_string(),
                    ty: Ty::Decl("Box".to_string()),
                    clauses: vec![Clause {
                        xtor: "B".to_string(),
                        context: vec![ContextBinding {
                            var: "x1".to_string(),
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                        body: Rc::new(Statement::Op(Op {
                            fst: "x1".to_string(),
                            op: BinOp::Sum,
                            snd: "x2".to_string(),
                            var: "res".to_string(),
                            next: Rc::new(Statement::Exit(Exit {
                                var: "res".to_string(),
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
        var: "bb1".to_string(),
        ty: Ty::Decl("BoxBox".to_string()),
        clauses: vec![Clause {
            xtor: "BB".to_string(),
            context: vec![ContextBinding {
                var: "b1".to_string(),
                chi: Chirality::Prd,
                ty: Ty::Decl("Box".to_string()),
            }]
            .into(),
            body: Rc::new(Statement::Switch(Switch {
                var: "b1".to_string(),
                ty: Ty::Decl("Box".to_string()),
                clauses: vec![Clause {
                    xtor: "B".to_string(),
                    context: vec![ContextBinding {
                        var: "x1".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    }]
                    .into(),
                    body: Rc::new(Statement::Let(Let {
                        var: "d1".to_string(),
                        ty: Ty::Decl("Box".to_string()),
                        tag: "B".to_string(),
                        args: vec![ContextBinding {
                            var: "x1".to_string(),
                            chi: Chirality::Ext,
                            ty: Ty::I64,
                        }]
                        .into(),
                        next: Rc::new(Statement::Let(Let {
                            var: "dd1".to_string(),
                            ty: Ty::Decl("BoxBox".to_string()),
                            tag: "BB".to_string(),
                            args: vec![ContextBinding {
                                var: "d1".to_string(),
                                chi: Chirality::Prd,
                                ty: Ty::Decl("Box".to_string()),
                            }]
                            .into(),
                            next: Rc::new(Statement::Substitute(Substitute {
                                rearrange: vec![("bb2".to_string(), "bb2".to_string())],
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: 4,
                                    var: "y".to_string(),
                                    next: Rc::new(Statement::Let(Let {
                                        var: "a1".to_string(),
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: "y".to_string(),
                                            chi: Chirality::Ext,
                                            ty: Ty::I64,
                                        }]
                                        .into(),
                                        next: Rc::new(Statement::Substitute(Substitute {
                                            rearrange: vec![
                                                ("a1".to_string(), "a1".to_string()),
                                                ("bb2".to_string(), "bb2".to_string()),
                                            ],
                                            next: Rc::new(Statement::Switch(Switch {
                                                var: "bb2".to_string(),
                                                ty: Ty::Decl("BoxBox".to_string()),
                                                clauses: vec![Clause {
                                                    xtor: "BB".to_string(),
                                                    context: vec![ContextBinding {
                                                        var: "b2".to_string(),
                                                        chi: Chirality::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    }]
                                                    .into(),
                                                    body: Rc::new(Statement::Switch(Switch {
                                                        var: "b2".to_string(),
                                                        ty: Ty::Decl("Box".to_string()),
                                                        clauses: vec![Clause {
                                                            xtor: "B".to_string(),
                                                            context: vec![ContextBinding {
                                                                var: "x2".to_string(),
                                                                chi: Chirality::Ext,
                                                                ty: Ty::I64,
                                                            }]
                                                            .into(),
                                                            body: Rc::new(Statement::Let(Let {
                                                                var: "a2".to_string(),
                                                                ty: Ty::Decl("Box".to_string()),
                                                                tag: "B".to_string(),
                                                                args: vec![ContextBinding {
                                                                    var: "x2".to_string(),
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
        var: "f1".to_string(),
        next: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: "f2".to_string(),
            next: Rc::new(Statement::Literal(Literal {
                lit: 3,
                var: "f3".to_string(),
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "f4".to_string(),
                    next: Rc::new(Statement::Literal(Literal {
                        lit: 3,
                        var: "f5".to_string(),
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 3,
                            var: "f6".to_string(),
                            next: Rc::new(Statement::Literal(Literal {
                                lit: 3,
                                var: "f7".to_string(),
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: 3,
                                    var: "x".to_string(),
                                    next: Rc::new(Statement::Let(Let {
                                        var: "b".to_string(),
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: "x".to_string(),
                                            chi: Chirality::Ext,
                                            ty: Ty::I64,
                                        }]
                                        .into(),
                                        next: Rc::new(Statement::Let(Let {
                                            var: "bb".to_string(),
                                            ty: Ty::Decl("BoxBox".to_string()),
                                            tag: "BB".to_string(),
                                            args: vec![ContextBinding {
                                                var: "b".to_string(),
                                                chi: Chirality::Prd,
                                                ty: Ty::Decl("Box".to_string()),
                                            }]
                                            .into(),
                                            next: Rc::new(Statement::Substitute(Substitute {
                                                rearrange: vec![
                                                    ("f1".to_string(), "f1".to_string()),
                                                    ("f2".to_string(), "f2".to_string()),
                                                    ("f3".to_string(), "f3".to_string()),
                                                    ("f5".to_string(), "f5".to_string()),
                                                    ("f6".to_string(), "f6".to_string()),
                                                    ("f7".to_string(), "f7".to_string()),
                                                    ("f4".to_string(), "f4".to_string()),
                                                    ("bb3".to_string(), "bb".to_string()),
                                                    ("bb2".to_string(), "bb".to_string()),
                                                    ("bb1".to_string(), "bb".to_string()),
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
