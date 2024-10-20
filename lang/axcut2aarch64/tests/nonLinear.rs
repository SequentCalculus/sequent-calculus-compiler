use axcut::syntax::*;
use axcut2aarch64::code::pretty;
use axcut2aarch64::coder::compile;
use axcut2aarch64::into_routine::into_aarch64_routine;

use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_non_linear() {
    let ty_box = TypeDeclaration {
        name: "Box".to_string(),
        xtors: vec![XtorSig {
            name: "B".to_string(),
            args: vec![ContextBinding {
                var: "b".to_string(),
                pol: Polarity::Ext,
                ty: Ty::Int,
            }],
        }],
    };
    let ty_box_box = TypeDeclaration {
        name: "BoxBox".to_string(),
        xtors: vec![XtorSig {
            name: "BB".to_string(),
            args: vec![ContextBinding {
                var: "bb".to_string(),
                pol: Polarity::Prd,
                ty: Ty::Decl("Box".to_string()),
            }],
        }],
    };

    let main_body_switch_switch = Statement::Switch(Switch {
        var: "a2".to_string(),
        clauses: vec![Clause {
            env: vec![ContextBinding {
                var: "x2".to_string(),
                pol: Polarity::Ext,
                ty: Ty::Int,
            }],
            case: Rc::new(Statement::Substitute(Substitute {
                rearrange: vec![
                    (
                        ContextBinding {
                            var: "x2".to_string(),
                            pol: Polarity::Ext,
                            ty: Ty::Int,
                        },
                        ContextBinding {
                            var: "x2".to_string(),
                            pol: Polarity::Ext,
                            ty: Ty::Int,
                        },
                    ),
                    (
                        ContextBinding {
                            var: "a1".to_string(),
                            pol: Polarity::Prd,
                            ty: Ty::Decl("Box".to_string()),
                        },
                        ContextBinding {
                            var: "a1".to_string(),
                            pol: Polarity::Prd,
                            ty: Ty::Decl("Box".to_string()),
                        },
                    ),
                ],
                next: Rc::new(Statement::Switch(Switch {
                    var: "a1".to_string(),
                    clauses: vec![Clause {
                        env: vec![ContextBinding {
                            var: "x1".to_string(),
                            pol: Polarity::Ext,
                            ty: Ty::Int,
                        }],
                        case: Rc::new(Statement::Op(Op {
                            fst: "x1".to_string(),
                            op: BinOp::Sum,
                            snd: "x2".to_string(),
                            var: "res".to_string(),
                            case: Rc::new(Statement::Return(Return {
                                var: "res".to_string(),
                            })),
                        })),
                    }],
                })),
            })),
        }],
    });
    let main_body_switch = Statement::Switch(Switch {
        var: "bb1".to_string(),
        clauses: vec![Clause {
            env: vec![ContextBinding {
                var: "b1".to_string(),
                pol: Polarity::Prd,
                ty: Ty::Decl("Box".to_string()),
            }],
            case: Rc::new(Statement::Switch(Switch {
                var: "b1".to_string(),
                clauses: vec![Clause {
                    env: vec![ContextBinding {
                        var: "x1".to_string(),
                        pol: Polarity::Ext,
                        ty: Ty::Int,
                    }],
                    case: Rc::new(Statement::Leta(Leta {
                        var: "d1".to_string(),
                        ty: Ty::Decl("Box".to_string()),
                        tag: "B".to_string(),
                        args: vec![ContextBinding {
                            var: "x1".to_string(),
                            pol: Polarity::Ext,
                            ty: Ty::Int,
                        }],
                        next: Rc::new(Statement::Leta(Leta {
                            var: "dd1".to_string(),
                            ty: Ty::Decl("BoxBox".to_string()),
                            tag: "BB".to_string(),
                            args: vec![ContextBinding {
                                var: "d1".to_string(),
                                pol: Polarity::Prd,
                                ty: Ty::Decl("Box".to_string()),
                            }],
                            next: Rc::new(Statement::Substitute(Substitute {
                                rearrange: vec![(
                                    ContextBinding {
                                        var: "bb2".to_string(),
                                        pol: Polarity::Prd,
                                        ty: Ty::Decl("BoxBox".to_string()),
                                    },
                                    ContextBinding {
                                        var: "bb2".to_string(),
                                        pol: Polarity::Prd,
                                        ty: Ty::Decl("BoxBox".to_string()),
                                    },
                                )],
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: 4,
                                    var: "y".to_string(),
                                    case: Rc::new(Statement::Leta(Leta {
                                        var: "a1".to_string(),
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: "y".to_string(),
                                            pol: Polarity::Ext,
                                            ty: Ty::Int,
                                        }],
                                        next: Rc::new(Statement::Substitute(Substitute {
                                            rearrange: vec![
                                                (
                                                    ContextBinding {
                                                        var: "a1".to_string(),
                                                        pol: Polarity::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    },
                                                    ContextBinding {
                                                        var: "a1".to_string(),
                                                        pol: Polarity::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    },
                                                ),
                                                (
                                                    ContextBinding {
                                                        var: "bb2".to_string(),
                                                        pol: Polarity::Prd,
                                                        ty: Ty::Decl("BoxBox".to_string()),
                                                    },
                                                    ContextBinding {
                                                        var: "bb2".to_string(),
                                                        pol: Polarity::Prd,
                                                        ty: Ty::Decl("BoxBox".to_string()),
                                                    },
                                                ),
                                            ],
                                            next: Rc::new(Statement::Switch(Switch {
                                                var: "bb2".to_string(),
                                                clauses: vec![Clause {
                                                    env: vec![ContextBinding {
                                                        var: "b2".to_string(),
                                                        pol: Polarity::Prd,
                                                        ty: Ty::Decl("Box".to_string()),
                                                    }],
                                                    case: Rc::new(Statement::Switch(Switch {
                                                        var: "b2".to_string(),
                                                        clauses: vec![Clause {
                                                            env: vec![ContextBinding {
                                                                var: "x2".to_string(),
                                                                pol: Polarity::Ext,
                                                                ty: Ty::Int,
                                                            }],
                                                            case: Rc::new(Statement::Leta(Leta {
                                                                var: "a2".to_string(),
                                                                ty: Ty::Decl("Box".to_string()),
                                                                tag: "B".to_string(),
                                                                args: vec![ContextBinding {
                                                                    var: "x2".to_string(),
                                                                    pol: Polarity::Ext,
                                                                    ty: Ty::Int,
                                                                }],
                                                                next: Rc::new(
                                                                    main_body_switch_switch,
                                                                ),
                                                            })),
                                                        }],
                                                    })),
                                                }],
                                            })),
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                }],
            })),
        }],
    });
    let main_body = Statement::Literal(Literal {
        lit: 3,
        var: "f1".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: "f2".to_string(),
            case: Rc::new(Statement::Literal(Literal {
                lit: 3,
                var: "f3".to_string(),
                case: Rc::new(Statement::Literal(Literal {
                    lit: 3,
                    var: "f4".to_string(),
                    case: Rc::new(Statement::Literal(Literal {
                        lit: 3,
                        var: "f5".to_string(),
                        case: Rc::new(Statement::Literal(Literal {
                            lit: 3,
                            var: "f6".to_string(),
                            case: Rc::new(Statement::Literal(Literal {
                                lit: 3,
                                var: "f7".to_string(),
                                case: Rc::new(Statement::Literal(Literal {
                                    lit: 3,
                                    var: "x".to_string(),
                                    case: Rc::new(Statement::Leta(Leta {
                                        var: "b".to_string(),
                                        ty: Ty::Decl("Box".to_string()),
                                        tag: "B".to_string(),
                                        args: vec![ContextBinding {
                                            var: "x".to_string(),
                                            pol: Polarity::Ext,
                                            ty: Ty::Int,
                                        }],
                                        next: Rc::new(Statement::Leta(Leta {
                                            var: "bb".to_string(),
                                            ty: Ty::Decl("BoxBox".to_string()),
                                            tag: "BB".to_string(),
                                            args: vec![ContextBinding {
                                                var: "b".to_string(),
                                                pol: Polarity::Prd,
                                                ty: Ty::Decl("Box".to_string()),
                                            }],
                                            next: Rc::new(Statement::Substitute(Substitute {
                                                rearrange: vec![
                                                    (
                                                        ContextBinding {
                                                            var: "f1".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f1".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f2".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f2".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f3".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f3".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f5".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f5".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f6".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f6".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f7".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f7".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "f4".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                        ContextBinding {
                                                            var: "f4".to_string(),
                                                            pol: Polarity::Ext,
                                                            ty: Ty::Int,
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "bb3".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        ContextBinding {
                                                            var: "bb".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "bb2".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        ContextBinding {
                                                            var: "bb".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                    ),
                                                    (
                                                        ContextBinding {
                                                            var: "bb1".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                        ContextBinding {
                                                            var: "bb".to_string(),
                                                            pol: Polarity::Prd,
                                                            ty: Ty::Decl("BoxBox".to_string()),
                                                        },
                                                    ),
                                                ],
                                                next: Rc::new(main_body_switch),
                                            })),
                                        })),
                                    })),
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
        context: Vec::new(),
        body: main_body,
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_box, ty_box_box],
    };

    let (code, arg_num) = compile(program);
    let assembler_code = into_aarch64_routine("nonLinear", &pretty(code), arg_num);

    let mut file = File::open("tests/asm/nonLinear.aarch64.asm")
        .expect("Cannot open file tests/asm/nonLinear.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/nonLinear.aarch64.asm");

    assert_eq!(assembler_code, reference_code);
}
