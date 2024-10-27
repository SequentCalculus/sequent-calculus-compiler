use axcut::syntax::*;
use axcut2aarch64::code::pretty;
use axcut2aarch64::coder::compile;
use axcut2aarch64::into_routine::into_aarch64_routine;

use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

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
                    ty: Ty::Int,
                }],
            },
            XtorSig {
                name: "Right".to_string(),
                args: vec![ContextBinding {
                    var: "y".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::Int,
                }],
            },
        ],
    };

    let main_body = Statement::Literal(Literal {
        lit: 1,
        var: "z".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 9,
            var: "x".to_string(),
            case: Rc::new(Statement::Leta(Leta {
                var: "p".to_string(),
                ty: Ty::Decl("Either".to_string()),
                tag: "Right".to_string(),
                args: vec!["x".to_string()],
                next: Rc::new(Statement::Switch(Switch {
                    var: "p".to_string(),
                    clauses: vec![
                        Clause {
                            context: vec![ContextBinding {
                                var: "a".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::Int,
                            }],
                            case: Rc::new(Statement::Done),
                        },
                        Clause {
                            context: vec![ContextBinding {
                                var: "b".to_string(),
                                chi: Chirality::Ext,
                                ty: Ty::Int,
                            }],
                            case: Rc::new(Statement::Op(Op {
                                fst: "b".to_string(),
                                op: BinOp::Sum,
                                snd: "z".to_string(),
                                var: "c".to_string(),
                                case: Rc::new(Statement::Return(Return {
                                    var: "c".to_string(),
                                })),
                            })),
                        },
                    ],
                })),
            })),
        })),
    });
    let main = Def {
        name: "main".to_string(),
        context: Vec::new(),
        body: main_body,
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_either],
    };

    let (code, arg_num) = compile(program);
    let assembler_code = into_aarch64_routine("either", &pretty(code), arg_num);

    let mut file = File::open("tests/asm/either.aarch64.asm")
        .expect("Cannot open file tests/asm/either.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/either.aarch64.asm");

    assert_eq!(assembler_code, reference_code);
}
