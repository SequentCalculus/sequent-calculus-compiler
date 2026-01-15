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
fn test_arith() {
    let main_body = Statement::Literal(Literal {
        lit: 1,
        var: Var {
            name: "a".to_string(),
            id: 0,
        },
        next: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: Var {
                name: "b".to_string(),
                id: 0,
            },
            next: Rc::new(Statement::Op(Op {
                fst: Var {
                    name: "a".to_string(),
                    id: 0,
                },
                op: BinOp::Sub,
                snd: Var {
                    name: "b".to_string(),
                    id: 0,
                },
                var: Var {
                    name: "c".to_string(),
                    id: 0,
                },
                next: Rc::new(Statement::Literal(Literal {
                    lit: 8,
                    var: Var {
                        name: "d".to_string(),
                        id: 0,
                    },
                    next: Rc::new(Statement::Literal(Literal {
                        lit: -1,
                        var: Var {
                            name: "e".to_string(),
                            id: 0,
                        },
                        next: Rc::new(Statement::Op(Op {
                            fst: Var {
                                name: "e".to_string(),
                                id: 0,
                            },
                            op: BinOp::Prod,
                            snd: Var {
                                name: "d".to_string(),
                                id: 0,
                            },
                            var: Var {
                                name: "f".to_string(),
                                id: 0,
                            },
                            next: Rc::new(Statement::Op(Op {
                                fst: Var {
                                    name: "f".to_string(),
                                    id: 0,
                                },
                                op: BinOp::Sum,
                                snd: Var {
                                    name: "c".to_string(),
                                    id: 0,
                                },
                                var: Var {
                                    name: "g".to_string(),
                                    id: 0,
                                },
                                next: Rc::new(Statement::Literal(Literal {
                                    lit: -6,
                                    var: Var {
                                        name: "h".to_string(),
                                        id: 0,
                                    },
                                    next: Rc::new(Statement::Op(Op {
                                        fst: Var {
                                            name: "h".to_string(),
                                            id: 0,
                                        },
                                        op: BinOp::Prod,
                                        snd: Var {
                                            name: "g".to_string(),
                                            id: 0,
                                        },
                                        var: Var {
                                            name: "i".to_string(),
                                            id: 0,
                                        },
                                        next: Rc::new(Statement::Exit(Exit {
                                            var: Var {
                                                name: "i".to_string(),
                                                id: 0,
                                            },
                                        })),
                                        free_vars_next: None,
                                    })),
                                    free_vars_next: None,
                                })),
                                free_vars_next: None,
                            })),
                            free_vars_next: None,
                        })),
                        free_vars_next: None,
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
        types: vec![],
    };

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("arith.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
