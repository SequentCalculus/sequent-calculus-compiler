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
            context: Some(vec!["a".to_string()].into()),
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
                            ("b".to_string(), "b".to_string()),
                            ("k".to_string(), "k".to_string()),
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
                next: Rc::new(Statement::Literal(Literal {
                    lit: 1,
                    var: "y".to_string(),
                    next: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            ("y".to_string(), "y".to_string()),
                            ("k".to_string(), "k".to_string()),
                            ("f".to_string(), "f".to_string()),
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
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("closure.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
