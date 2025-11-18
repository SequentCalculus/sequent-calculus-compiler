use axcut::syntax::statements::*;
use axcut::syntax::*;
use printer::Print;

use std::collections::HashSet;
use std::rc::Rc;

fn main() {
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
        }]
        .into(),
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
        }]
        .into(),
    };

    let main_body_switch_switch = Statement::Switch(Switch {
        var: "a2".to_string(),
        ty: Ty::Decl("Box".to_string()),
        clauses: vec![Clause {
            xtor: "B".to_string(),
            context: vec![ContextBinding {
                var: "y2".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::Switch(Switch {
                var: "a1".to_string(),
                ty: Ty::Decl("Box".to_string()),
                clauses: vec![Clause {
                    xtor: "B".to_string(),
                    context: vec![ContextBinding {
                        var: "y1".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    }]
                    .into(),
                    body: Rc::new(Statement::Op(Op {
                        fst: "y1".to_string(),
                        op: BinOp::Sum,
                        snd: "y2".to_string(),
                        var: "res".to_string(),
                        next: Rc::new(Statement::Exit(Exit {
                            var: "res".to_string(),
                        })),
                        free_vars_next: None,
                    })),
                }],
                free_vars_clauses: None,
            })),
        }],
        free_vars_clauses: None,
    });
    let main_body_switch = Statement::Switch(Switch {
        var: "bb".to_string(),
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
                        context: vec![ContextBinding {
                            var: "x1".to_string(),
                            ty: Ty::I64,
                            chi: Chirality::Ext,
                        }]
                        .into(),
                        next: Rc::new(Statement::Let(Let {
                            var: "dd1".to_string(),
                            ty: Ty::Decl("BoxBox".to_string()),
                            tag: "BB".to_string(),
                            context: vec![ContextBinding {
                                var: "d1".to_string(),
                                ty: Ty::Decl("Box".to_string()),
                                chi: Chirality::Prd,
                            }]
                            .into(),
                            next: Rc::new(Statement::Literal(Literal {
                                lit: 4,
                                var: "y".to_string(),
                                next: Rc::new(Statement::Let(Let {
                                    var: "a1".to_string(),
                                    ty: Ty::Decl("Box".to_string()),
                                    tag: "B".to_string(),
                                    context: vec![ContextBinding {
                                        var: "y".to_string(),
                                        ty: Ty::I64,
                                        chi: Chirality::Ext,
                                    }]
                                    .into(),
                                    next: Rc::new(Statement::Switch(Switch {
                                        var: "bb".to_string(),
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
                                                        context: vec![ContextBinding {
                                                            var: "x2".to_string(),
                                                            ty: Ty::I64,
                                                            chi: Chirality::Ext,
                                                        }]
                                                        .into(),
                                                        next: Rc::new(main_body_switch_switch),
                                                        free_vars_next: None,
                                                    })),
                                                }],
                                                free_vars_clauses: None,
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
                var: "x".to_string(),
                next: Rc::new(Statement::Let(Let {
                    var: "b".to_string(),
                    ty: Ty::Decl("Box".to_string()),
                    tag: "B".to_string(),
                    context: vec![ContextBinding {
                        var: "x".to_string(),
                        ty: Ty::I64,
                        chi: Chirality::Ext,
                    }]
                    .into(),
                    next: Rc::new(Statement::Let(Let {
                        var: "bb".to_string(),
                        ty: Ty::Decl("BoxBox".to_string()),
                        tag: "BB".to_string(),
                        context: vec![ContextBinding {
                            var: "b".to_string(),
                            ty: Ty::I64,
                            chi: Chirality::Ext,
                        }]
                        .into(),
                        next: Rc::new(main_body_switch),
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
        used_vars: HashSet::from([
            "bb".to_string(),
            "a2".to_string(),
            "f1".to_string(),
            "b1".to_string(),
            "b".to_string(),
            "b2".to_string(),
            "y1".to_string(),
            "a1".to_string(),
            "y".to_string(),
            "res".to_string(),
            "dd1".to_string(),
            "x1".to_string(),
            "x2".to_string(),
            "d1".to_string(),
            "x".to_string(),
            "f2".to_string(),
            "y2".to_string(),
        ]),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_box, ty_box_box],
    };

    println!("{}", program.linearize().print_to_string(None))
}
