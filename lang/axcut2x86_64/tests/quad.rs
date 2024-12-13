use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2x86_64::into_routine::into_x86_64_routine;
use axcut2x86_64::Backend;
use goldenfile::Mint;
use printer::Print;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

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
            ]
            .into(),
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
                                ]
                                .into(),
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
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![ty_quad],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("quad.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
