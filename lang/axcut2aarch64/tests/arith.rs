use axcut::syntax::statements::*;
use axcut::syntax::*;
use axcut2aarch64::into_routine::into_aarch64_routine;
use axcut2aarch64::Backend;
use axcut2backend::coder::compile;

use pretty_assertions::assert_eq;
use printer::Print;

use std::collections::HashSet;
use std::rc::Rc;

use std::fs::File;
use std::io::prelude::*;

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

    let assembly_prog = compile(program, &Backend);
    let assembler_code = into_aarch64_routine(assembly_prog);

    //let mut file = File::create("tests/asm/arith.aarch64.asm")
    //    .expect("Cannot create file tests/asm/arith.aarch64.asm");
    //file.write_all(&mut assembler_code.as_bytes())
    //    .expect("Cannot write to file tests/asm/arith.aarch64.asm");
    let mut file = File::open("tests/asm/arith.aarch64.asm")
        .expect("Cannot open file tests/asm/arith.aarch64.asm");
    let mut reference_code = String::new();
    file.read_to_string(&mut reference_code)
        .expect("Cannot read from file tests/asm/arith.aarch64.asm");

    assert_eq!(assembler_code.print_to_string(None), reference_code);
}
