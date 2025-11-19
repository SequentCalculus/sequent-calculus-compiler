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
fn test_closure() {
    let ty_cont = TypeDeclaration {
        name: "Cont".to_string(),
        xtors: vec![XtorSig {
            name: "Ret".to_string(),
            args: vec![ContextBinding {
                var: "r".to_string(),
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
                    var: "x".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                },
                ContextBinding {
                    var: "k".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::Decl("Cont".to_string()),
                },
            ]
            .into(),
        }],
    };

    let main_body = Statement::Literal(Literal {
        lit: 9,
        var: "a".to_string(),
        next: Rc::new(Statement::Create(Create {
            var: "f".to_string(),
            ty: Ty::Decl("Fun".to_string()),
            context: Some(
                vec![ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            ),
            clauses: vec![Clause {
                xtor: "apply".to_string(),
                context: vec![
                    ContextBinding {
                        var: "x".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::I64,
                    },
                    ContextBinding {
                        var: "k".to_string(),
                        chi: Chirality::Cns,
                        ty: Ty::Decl("Cont".to_string()),
                    },
                ]
                .into(),
                body: Rc::new(Statement::Op(Op {
                    fst: "a".to_string(),
                    op: BinOp::Sum,
                    snd: "x".to_string(),
                    var: "b".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: "b".to_string(),
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                "b".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "k".to_string(),
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("Cont".to_string()),
                                },
                                "k".to_string(),
                            ),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "k".to_string(),
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
                var: "k".to_string(),
                ty: Ty::Decl("Cont".to_string()),
                context: Some(Vec::new().into()),
                clauses: vec![Clause {
                    xtor: "Ret".to_string(),
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
                next: Rc::new(Statement::Literal(Literal {
                    lit: 1,
                    var: "y".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            (
                                ContextBinding {
                                    var: "y".to_string(),
                                    chi: Chirality::Ext,
                                    ty: Ty::I64,
                                },
                                "y".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "k".to_string(),
                                    chi: Chirality::Cns,
                                    ty: Ty::Decl("Cont".to_string()),
                                },
                                "k".to_string(),
                            ),
                            (
                                ContextBinding {
                                    var: "f".to_string(),
                                    chi: Chirality::Prd,
                                    ty: Ty::Decl("Fun".to_string()),
                                },
                                "f".to_string(),
                            ),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "f".to_string(),
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
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
