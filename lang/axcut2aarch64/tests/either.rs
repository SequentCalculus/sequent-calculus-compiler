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
fn test_either() {
    let ty_either = TypeDeclaration {
        name: "Either".to_string(),
        xtors: vec![
            XtorSig {
                name: "Left".to_string(),
                args: vec![ContextBinding {
                    var: "x".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            },
            XtorSig {
                name: "Right".to_string(),
                args: vec![ContextBinding {
                    var: "y".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::I64,
                }]
                .into(),
            },
        ],
    };

    let main_body = Statement::Literal(Literal {
        lit: 1,
        var: "z".to_string(),
        next: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: "x".to_string(),
            next: Rc::new(Statement::Let(Let {
                var: "p".to_string(),
                ty: Ty::Decl("Either".to_string()),
                tag: "Right".to_string(),
                args: vec!["x".to_string()],
                next: Rc::new(Statement::Switch(Switch {
                    var: "p".to_string(),
                    ty: Ty::Decl("Either".to_string()),
                    clauses: vec![
                        Clause {
                            xtor: "Left".to_string(),
                            context: vec![ContextBinding {
                                var: "a".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            body: Rc::new(Statement::Done),
                        },
                        Clause {
                            xtor: "Right".to_string(),
                            context: vec![ContextBinding {
                                var: "b".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::I64,
                            }]
                            .into(),
                            body: Rc::new(Statement::Op(Op {
                                fst: "b".to_string(),
                                op: BinOp::Sum,
                                snd: "z".to_string(),
                                var: "c".to_string(),
                                next: Rc::new(Statement::PrintI64(PrintI64 {
                                    newline: true,
                                    var: "c".to_string(),
                                    next: Rc::new(Statement::Literal(Literal {
                                        lit: 0,
                                        var: "ret".to_string(),
                                        next: Rc::new(Statement::Return(Return {
                                            var: "ret".to_string(),
                                        })),
                                        free_vars_next: None,
                                    })),
                                    free_vars_next: None,
                                })),
                                free_vars_next: None,
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
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_either],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_aarch64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("either.aarch64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
