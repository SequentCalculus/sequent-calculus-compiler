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
        context: None,
        clauses: vec![Clause {
            xtor: "Reti".to_string(),
            context: vec![ContextBinding {
                var: "r".to_string(),
                chi: Chirality::Ext,
                ty: Ty::I64,
            }]
            .into(),
            body: Rc::new(Statement::Exit(Exit {
                var: "r".to_string(),
            })),
        }],
        free_vars_clauses: None,
        next: Rc::new(Statement::Create(Create {
            var: "k".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            context: None,
            clauses: vec![Clause {
                xtor: "Retl".to_string(),
                context: vec![ContextBinding {
                    var: "as".to_string(),
                    chi: Chirality::Prd,
                    ty: Ty::Decl("List".to_string()),
                }]
                .into(),
                body: Rc::new(Statement::Call(Call {
                    label: "sum".to_string(),
                    args: vec!["t".to_string(), "as".to_string()],
                })),
            }],
            free_vars_clauses: None,
            next: Rc::new(Statement::Let(Let {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Nil".to_string(),
                args: vec![],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "n".to_string(),
                    next: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec!["k".to_string(), "zs".to_string(), "n".to_string()],
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
            "t".to_string(),
            "zs".to_string(),
            "n".to_string(),
            "k".to_string(),
            "as".to_string(),
            "r".to_string(),
        ]),
    };

    let range_body = Statement::IfZ(IfZ {
        sort: ifz::IfZSort::Equal,
        ifc: "i".to_string(),
        thenc: Rc::new(Statement::Invoke(Invoke {
            var: "k".to_string(),
            tag: "Retl".to_string(),
            ty: Ty::Decl("ContList".to_string()),
            args: vec!["xs".to_string()],
        })),
        elsec: Rc::new(Statement::Let(Let {
            var: "ys".to_string(),
            ty: Ty::Decl("List".to_string()),
            tag: "Cons".to_string(),
            args: vec!["xs".to_string(), "i".to_string()],
            next: Rc::new(Statement::Literal(Literal {
                lit: -1,
                var: "o".to_string(),
                next: Rc::new(Statement::Op(Op {
                    fst: "i".to_string(),
                    op: BinOp::Sum,
                    snd: "o".to_string(),
                    var: "j".to_string(),
                    next: Rc::new(Statement::Call(Call {
                        label: "range".to_string(),
                        args: vec!["k".to_string(), "ys".to_string(), "j".to_string()],
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
        used_vars: HashSet::from([
            "k".to_string(),
            "xs".to_string(),
            "i".to_string(),
            "j".to_string(),
            "o".to_string(),
            "ys".to_string(),
        ]),
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
                    next: Rc::new(Statement::Invoke(Invoke {
                        var: "k".to_string(),
                        tag: "Reti".to_string(),
                        ty: Ty::Decl("ContInt".to_string()),
                        args: vec!["z".to_string()],
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
                body: Rc::new(Statement::Create(Create {
                    var: "j".to_string(),
                    ty: Ty::Decl("ContInt".to_string()),
                    context: None,
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
                            next: Rc::new(Statement::Invoke(Invoke {
                                var: "k".to_string(),
                                tag: "Reti".to_string(),
                                ty: Ty::Decl("ContInt".to_string()),
                                args: vec!["s".to_string()],
                            })),
                            free_vars_next: None,
                        })),
                    }],
                    free_vars_clauses: None,
                    next: Rc::new(Statement::Call(Call {
                        label: "sum".to_string(),
                        args: vec!["j".to_string(), "ys".to_string()],
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
        used_vars: HashSet::from([
            "ys".to_string(),
            "xs".to_string(),
            "y".to_string(),
            "j".to_string(),
            "s".to_string(),
            "r".to_string(),
            "k".to_string(),
            "z".to_string(),
        ]),
    };

    let program = Prog {
        defs: vec![main, range, sum],
        types: vec![ty_list, ty_cont_list, ty_cont_int],
    };

    println!("{}", program.linearize().print_to_string(None))
}
