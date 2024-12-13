use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2backend::coder::compile;
use axcut2rv64::into_routine::into_rv64_routine;
use axcut2rv64::Backend;
use goldenfile::Mint;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::Rc;

#[test]
fn test_arith() {
    let main_body = Statement::Literal(Literal {
        lit: 1,
        var: "a".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: "b".to_string(),
            case: Rc::new(Statement::Op(Op {
                fst: "a".to_string(),
                op: BinOp::Sub,
                snd: "b".to_string(),
                var: "c".to_string(),
                case: Rc::new(Statement::Literal(Literal {
                    lit: 8,
                    var: "d".to_string(),
                    case: Rc::new(Statement::Literal(Literal {
                        lit: -1,
                        var: "e".to_string(),
                        case: Rc::new(Statement::Op(Op {
                            fst: "e".to_string(),
                            op: BinOp::Prod,
                            snd: "d".to_string(),
                            var: "f".to_string(),
                            case: Rc::new(Statement::Op(Op {
                                fst: "f".to_string(),
                                op: BinOp::Sum,
                                snd: "c".to_string(),
                                var: "g".to_string(),
                                case: Rc::new(Statement::Literal(Literal {
                                    lit: -6,
                                    var: "h".to_string(),
                                    case: Rc::new(Statement::Op(Op {
                                        fst: "h".to_string(),
                                        op: BinOp::Prod,
                                        snd: "g".to_string(),
                                        var: "i".to_string(),
                                        case: Rc::new(Statement::Return(Return {
                                            var: "i".to_string(),
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
        context: Vec::new().into(),
        body: main_body,
        used_vars: HashSet::new(),
    };

    let program = Prog {
        defs: vec![main],
        types: vec![],
    };

    let assembler_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_rv64_routine(assembler_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("arith.rv64.asm").unwrap();
    file.write(assembler_code.as_bytes()).unwrap();
}
