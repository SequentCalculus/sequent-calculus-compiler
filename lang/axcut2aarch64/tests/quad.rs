use axcut::syntax::*;
use axcut2aarch64::code::pretty;
use axcut2aarch64::coder::compile;
use axcut2aarch64::into_routine::into_aarch64_routine;

use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_quad() {
    let ty_quad = TypeDeclaration {
        name: "Quad".to_string(),
        xtors: vec![XtorSig {
            name: "Q".to_string(),
            args: vec![
                ContextBinding {
                    var: "d".to_string(),
                    pol: Polarity::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "c".to_string(),
                    pol: Polarity::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "b".to_string(),
                    pol: Polarity::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "a".to_string(),
                    pol: Polarity::Ext,
                    ty: Ty::Int,
                },
            ],
        }],
    };

    let main_body = Statement::Literal(Literal {
        lit: 8,
        var: "z".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 6,
            var: "y".to_string(),
            case: Rc::new(Statement::Literal(Literal {
                lit: 4,
                var: "x".to_string(),
                case: Rc::new(Statement::Literal(Literal {
                    lit: 2,
                    var: "w".to_string(),
                    case: Rc::new(Statement::Leta(Leta {
                        var: "q".to_string(),
                        ty: Ty::Decl("Quad".to_string()),
                        tag: "Q".to_string(),
                        args: vec![
                            ContextBinding {
                                var: "z".to_string(),
                                pol: Polarity::Ext,
                                ty: Ty::Int,
                            },
                            ContextBinding {
                                var: "y".to_string(),
                                pol: Polarity::Ext,
                                ty: Ty::Int,
                            },
                            ContextBinding {
                                var: "x".to_string(),
                                pol: Polarity::Ext,
                                ty: Ty::Int,
                            },
                            ContextBinding {
                                var: "w".to_string(),
                                pol: Polarity::Ext,
                                ty: Ty::Int,
                            },
                        ],
                        next: Rc::new(Statement::Switch(Switch {
                            var: "q".to_string(),
                            clauses: vec![Clause {
                                env: vec![
                                    ContextBinding {
                                        var: "d".to_string(),
                                        pol: Polarity::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "c".to_string(),
                                        pol: Polarity::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "b".to_string(),
                                        pol: Polarity::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "a".to_string(),
                                        pol: Polarity::Ext,
                                        ty: Ty::Int,
                                    },
                                ],
                                case: Rc::new(Statement::Literal(Literal {
                                    lit: 7,
                                    var: "z".to_string(),
                                    case: Rc::new(Statement::Op(Op {
                                        fst: "d".to_string(),
                                        op: BinOp::Sum,
                                        snd: "z".to_string(),
                                        var: "e".to_string(),
                                        case: Rc::new(Statement::Return(Return {
                                            var: "e".to_string(),
                                        })),
                                    })),
                                })),
                            }],
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
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_quad],
    };

    let (code, arg_num) = compile(program);
    let assembler_code = into_aarch64_routine("quad", &pretty(code), arg_num);

    let mut file = File::open("tests/asm/quad.aarch64.asm")
        .expect("Cannot open file tests/asm/quad.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/quad.aarch64.asm");

    assert_eq!(assembler_code, reference_code);
}
