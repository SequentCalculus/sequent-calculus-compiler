use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::code::pretty;
use axcut2backend::coder::compile;
use axcut2rv64::into_routine::into_rv64_routine;
use axcut2rv64::Backend;

use std::collections::HashSet;
use std::rc::Rc;

use pretty_assertions::assert_eq;

use std::fs::File;
use std::io::prelude::*;

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
                        ty: Ty::Int,
                    },
                ]
                .into(),
            },
        ],
    };

    let main_body = Statement::Leta(Leta {
        var: "ws".to_string(),
        ty: Ty::Decl("List".to_string()),
        tag: "Nil".to_string(),
        args: vec![],
        next: Rc::new(Statement::Literal(Literal {
            lit: 5,
            var: "z".to_string(),
            case: Rc::new(Statement::Leta(Leta {
                var: "zs".to_string(),
                ty: Ty::Decl("List".to_string()),
                tag: "Cons".to_string(),
                args: vec!["z".to_string(), "ws".to_string()],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 7,
                    var: "y".to_string(),
                    case: Rc::new(Statement::Leta(Leta {
                        var: "ys".to_string(),
                        ty: Ty::Decl("List".to_string()),
                        tag: "Cons".to_string(),
                        args: vec!["y".to_string(), "zs".to_string()],
                        next: Rc::new(Statement::Literal(Literal {
                            lit: 9,
                            var: "x".to_string(),
                            case: Rc::new(Statement::Leta(Leta {
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
                                            case: Rc::new(Statement::Done),
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
                                                    ty: Ty::Int,
                                                },
                                            ]
                                            .into(),
                                            case: Rc::new(Statement::Return(Return {
                                                var: "a".to_string(),
                                            })),
                                        },
                                    ],
                                })),
                            })),
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

    let program = Prog {
        defs: vec![main],
        types: vec![ty_list],
    };

    let (code, arg_num) = compile(program, &Backend);
    let assembler_code = into_rv64_routine(&pretty(code), arg_num);

    //let mut file = File::create("tests/asm/list.rv64.asm")
    //    .expect("Cannot create file tests/asm/list.rv64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/list.rv64.asm");
    let mut file =
        File::open("tests/asm/list.rv64.asm").expect("Cannot open file tests/asm/list.rv64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/list.rv64.asm");

    assert_eq!(assembler_code, reference_code);
}
