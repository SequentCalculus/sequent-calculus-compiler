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
fn test_buffer() {
    let main_body = Statement::Literal(Literal {
        lit: 0,
        var: "a".to_string(),
        case: Rc::new(Statement::Literal(Literal {
            lit: 3,
            var: "b".to_string(),
            case: Rc::new(Statement::Literal(Literal {
                lit: 5,
                var: "c".to_string(),
                case: Rc::new(Statement::MMapAnonymousPage(MMapAnonymousPage {
                    var: "m".to_string(),
                    case: Rc::new(Statement::SetByte(SetByte {
                        buffer: "m".to_string(),
                        offset: "a".to_string(),
                        var: "c".to_string(),
                        case: Rc::new(Statement::GetByte(GetByte {
                            buffer: "m".to_string(),
                            offset: "a".to_string(),
                            var: "d".to_string(),
                            case: Rc::new(Statement::Op(Op {
                                fst: "b".to_string(),
                                op: BinOp::Sum,
                                snd: "d".to_string(),
                                var: "e".to_string(),
                                case: Rc::new(Statement::Literal(Literal {
                                    lit: 7,
                                    var: "f".to_string(),
                                    case: Rc::new(Statement::SetByte(SetByte {
                                        buffer: "m".to_string(),
                                        offset: "f".to_string(),
                                        var: "e".to_string(),
                                        case: Rc::new(Statement::GetByte(GetByte {
                                            buffer: "m".to_string(),
                                            offset: "f".to_string(),
                                            var: "g".to_string(),
                                            case: Rc::new(Statement::MUnmapPage(MUnmapPage {
                                                var: "m".to_string(),
                                                case: Rc::new(Statement::Return(Return {
                                                    var: "g".to_string(),
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

    let assembly_prog = compile::<Backend, _, _, _>(program);
    let assembler_code = into_x86_64_routine(assembly_prog);

    let mut mint = Mint::new("tests/asm");
    let mut file = mint.new_goldenfile("buffer.x86_64.asm").unwrap();
    file.write(assembler_code.print_to_string(None).as_bytes())
        .unwrap();
}
