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

    let main_body = Statement::Let(Let {
        var: "ws".to_string(),
        ty: Ty::Decl("List".to_string()),
        tag: "Nil".to_string(),
        args: vec![],
        next: Rc::new(Statement::Literal(Literal {
            lit: 5,
            var: "z".to_string(),
            next: Rc::new(Statement::Let(Let {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec!["z".to_string(), "ws".to_string()],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 7,
                    var: "y".to_string(),
                    next: Rc::new(Statement::Let(Let {
                        var: "ys".to_string(),
                        ty: Ty::Decl("List".to_string()),
                        tag: "Cons".to_string(),
                        args: vec!["y".to_string(), "zs".to_string()],
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 9,
                            var: "x".to_string(),
                            next: Rc::new(Statement::Let(Let {
                                var: "xs".to_string(),
                                ty: Ty::Decl("List".to_string()),
                                tag: "Cons".to_string(),
                                args: vec!["x".to_string(), "ys".to_string()],
                                next: Rc::new(Statement::Switch(Switch {
                                    var: "xs".to_string(),
                                    ty: Ty::Decl("List".to_string()),
                                    clauses: vec![
                                        Clause {
                                            xtor: "Nil".to_string(),
                                            context: vec![].into(),
                                            body: Rc::new(Statement::Done),
                                        },
                                        Clause {
                                            xtor: "Cons".to_string(),
                                            context: vec![
                                                ContextBinding {
                                                    var: "as".to_string(),
                                                    chi: Chirality::Prd,
                                                    ty: Ty::Decl("List".to_string()),
                                                },
                                                ContextBinding {
                                                    var: "a".to_string(),
                                                    chi: Chirality::Ext,
                                                    ty: Ty::I64,
                                                },
                                            ]
                                            .into(),
                                            body: Rc::new(Statement::Exit(Exit {
                                                var: "a".to_string(),
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
