use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::code::pretty;
use axcut2backend::coder::compile;
use axcut2x86_64::into_routine::into_x86_64_routine;
use axcut2x86_64::Backend;

use std::collections::HashSet;
use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_closure() {
    let ty_cont = TypeDeclaration {
        name: "Cont".to_string(),
        xtors: vec![XtorSig {
            name: "Ret".to_string(),
            args: vec![ContextBinding {
                var: "r".to_string(),
                chi: Chirality::Ext,
                ty: Ty::Int,
            }],
        }],
    };

    let ty_func = TypeDeclaration {
        name: "Func".to_string(),
        xtors: vec![XtorSig {
            name: "Ap".to_string(),
            args: vec![
                ContextBinding {
                    var: "x".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "k".to_string(),
                    chi: Chirality::Cns,
                    ty: Ty::Decl("Cont".to_string()),
                },
            ],
        }],
    };

    let main_body = Statement::Literal(Literal {
        lit: 9,
        var: "a".to_string(),
        case: Rc::new(Statement::New(New {
            var: "f".to_string(),
            ty: Ty::Decl("Func".to_string()),
            context: Some(vec!["a".to_string()]),
            clauses: vec![Clause {
                xtor: "Ap".to_string(),
                context: vec![
                    ContextBinding {
                        var: "x".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    },
                    ContextBinding {
                        var: "k".to_string(),
                        chi: Chirality::Cns,
                        ty: Ty::Decl("Cont".to_string()),
                    },
                ],
                case: Rc::new(Statement::Op(Op {
                    fst: "a".to_string(),
                    op: BinOp::Sum,
                    snd: "x".to_string(),
                    var: "b".to_string(),
                    case: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            ("b".to_string(), "b".to_string()),
                            ("k".to_string(), "k".to_string()),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "k".to_string(),
                            tag: "Ret".to_string(),
                            ty: Ty::Decl("Cont".to_string()),
                            args: vec![],
                        })),
                    })),
                })),
            }],
            next: Rc::new(Statement::New(New {
                var: "k".to_string(),
                ty: Ty::Decl("Cont".to_string()),
                context: Some(Vec::new()),
                clauses: vec![Clause {
                    xtor: "Ret".to_string(),
                    context: vec![ContextBinding {
                        var: "r".to_string(),
                        chi: Chirality::Ext,
                        ty: Ty::Int,
                    }],
                    case: Rc::new(Statement::Return(Return {
                        var: "r".to_string(),
                    })),
                }],
                next: Rc::new(Statement::Literal(Literal {
                    lit: 1,
                    var: "y".to_string(),
                    case: Rc::new(Statement::Substitute(Substitute {
                        rearrange: vec![
                            ("y".to_string(), "y".to_string()),
                            ("k".to_string(), "k".to_string()),
                            ("f".to_string(), "f".to_string()),
                        ],
                        next: Rc::new(Statement::Invoke(Invoke {
                            var: "f".to_string(),
                            tag: "Ap".to_string(),
                            ty: Ty::Decl("Func".to_string()),
                            args: vec![],
                        })),
                    })),
                })),
            })),
        })),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_cont, ty_func],
    };

    let (code, arg_num) = compile(program, &Backend);
    let assembler_code = into_x86_64_routine(&pretty(code), arg_num);

    //let mut file = File::create("tests/asm/closure.x86_64.asm")
    //    .expect("Cannot create file tests/asm/closure.x86_64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/closure.x86_64.asm");
    let mut file = File::open("tests/asm/closure.x86_64.asm")
        .expect("Cannot open file tests/asm/closure.x86_64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/closure.x86_64.asm");

    assert_eq!(assembler_code, reference_code);
}
