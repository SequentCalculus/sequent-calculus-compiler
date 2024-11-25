use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2aarch64::Backend;
use axcut2backend::code::pretty;
use axcut2backend::coder::compile;

use std::collections::HashSet;
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
                    chi: Chirality::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "c".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "b".to_string(),
                    chi: Chirality::Ext,
                    ty: Ty::Int,
                },
                ContextBinding {
                    var: "a".to_string(),
                    chi: Chirality::Ext,
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
                            "z".to_string(),
                            "y".to_string(),
                            "x".to_string(),
                            "w".to_string(),
                        ],
                        next: Rc::new(Statement::Switch(Switch {
                            var: "q".to_string(),
                            ty: Ty::Decl("Quad".to_string()),
                            clauses: vec![Clause {
                                xtor: "Q".to_string(),
                                context: vec![
                                    ContextBinding {
                                        var: "d".to_string(),
                                        chi: Chirality::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "c".to_string(),
                                        chi: Chirality::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "b".to_string(),
                                        chi: Chirality::Ext,
                                        ty: Ty::Int,
                                    },
                                    ContextBinding {
                                        var: "a".to_string(),
                                        chi: Chirality::Ext,
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
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_quad],
    };

    let (code, arg_num) = compile(program, &Backend);
    let assembler_code = into_aarch64_routine(&pretty(code), arg_num);

    //let mut file = File::create("tests/asm/quad.aarch64.asm")
    //    .expect("Cannot create file tests/asm/quad.aarch64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/quad.aarch64.asm");
    let mut file = File::open("tests/asm/quad.aarch64.asm")
        .expect("Cannot open file tests/asm/quad.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/quad.aarch64.asm");

    assert_eq!(assembler_code, reference_code);
}
